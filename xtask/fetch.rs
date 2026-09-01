//! Refreshes the vendored OpenAPI document from the live API.
//!
//! ```sh
//! cargo xtask fetch-spec
//! ```
//!
//! Run this deliberately, review the diff, then run `cargo xtask codegen`. The spec is
//! vendored rather than fetched during codegen for two reasons:
//!
//! - **It is not byte-stable.** The server stamps the current time into the `date`
//!   query-parameter examples on every request, so two fetches minutes apart differ
//!   even when nothing about the API has changed. A codegen that downloaded would make
//!   `--check` fail at random in CI. (This task normalizes those examples away, which
//!   is what keeps the vendored copy stable across refreshes.)
//! - **Builds should not depend on a third-party endpoint.** Vendoring keeps `cargo
//!   build`, `cargo test`, and CI working offline and reproducible, and it means a
//!   change to Gladia's API arrives as a reviewable diff rather than silently altering
//!   the generated types.

use std::error::Error;
use std::path::Path;
use std::process::Command;

use serde_json::Value;

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

/// Where the spec is served.
const SOURCE: &str = "https://api.gladia.io/openapi.json";
/// Where it is vendored, relative to the repository root.
const OUTPUT: &str = "docs/openapi.json";

/// Downloads the spec, normalizes it, and writes it to `OUTPUT`.
pub(crate) fn run(root: &Path) -> Result<()> {
    let output = root.join(OUTPUT);

    // `curl` rather than an HTTP crate: this runs by hand a few times a year, and it
    // keeps the xtask's dependency tree (and so every contributor's build) small.
    let response = Command::new("curl")
        .args(["--fail", "--silent", "--show-error", "--location", SOURCE])
        .output()
        .map_err(|e| format!("could not run curl: {e}"))?;

    if !response.status.success() {
        let stderr = String::from_utf8_lossy(&response.stderr);
        return Err(format!("fetching {SOURCE} failed: {}", stderr.trim()).into());
    }

    let mut document: Value = serde_json::from_slice(&response.stdout)
        .map_err(|e| format!("{SOURCE} is not valid JSON: {e}"))?;

    let normalized = normalize(&mut document);

    // Pretty-printed so the vendored copy diffs line-by-line rather than as one line.
    let mut rendered = serde_json::to_string_pretty(&document)?;
    rendered.push('\n');

    let changed = match std::fs::read_to_string(&output) {
        Ok(previous) => previous != rendered,
        Err(_) => true,
    };

    std::fs::write(&output, &rendered)?;

    println!("wrote {OUTPUT} ({normalized} volatile examples normalized)");
    if changed {
        println!("the spec changed: review the diff, then run `cargo xtask codegen`");
    } else {
        println!("no change");
    }
    Ok(())
}

/// Pins the timestamp examples the API regenerates on every request.
///
/// The API stamps the current time into the examples for its `date`, `before_date`,
/// and `after_date` query parameters, so the served document differs on every fetch
/// while every schema is identical. Pinning them is what makes repeated refreshes
/// idempotent, and what lets `codegen --check` mean something.
///
/// Only those three query parameters are touched. The document holds two dozen other
/// timestamp examples, on `created_at`, `completed_at` and the error envelopes'
/// `timestamp`, which are static prose; rewriting those would edit the spec rather
/// than stabilise it.
///
/// Returns how many were rewritten.
fn normalize(document: &mut Value) -> usize {
    /// The value volatile timestamp examples are pinned to.
    const PINNED: &str = "2026-01-01T00:00:00.000Z";
    /// The query parameters whose examples the API regenerates.
    const VOLATILE: [&str; 3] = ["date", "before_date", "after_date"];

    fn visit(value: &mut Value, count: &mut usize) {
        match value {
            Value::Object(object) => {
                // Both halves matter: `in` distinguishes an OpenAPI parameter from
                // any other object that happens to carry a `name`, and the location
                // narrows it to the query parameters the API regenerates.
                let volatile = object.get("in").and_then(Value::as_str) == Some("query")
                    && object
                        .get("name")
                        .and_then(Value::as_str)
                        .is_some_and(|name| VOLATILE.contains(&name));

                if volatile {
                    pin(object, PINNED, count);
                }

                for nested in object.values_mut() {
                    visit(nested, count);
                }
            }
            Value::Array(items) => {
                for item in items {
                    visit(item, count);
                }
            }
            _ => {}
        }
    }

    let mut count = 0;
    visit(document, &mut count);
    count
}

/// Rewrites every timestamp `example` inside one parameter definition.
fn pin(parameter: &mut serde_json::Map<String, Value>, pinned: &str, count: &mut usize) {
    fn visit(value: &mut Value, pinned: &str, count: &mut usize) {
        match value {
            Value::Object(object) => {
                if let Some(example @ Value::String(_)) = object.get_mut("example")
                    && is_timestamp(example.as_str().unwrap_or_default())
                {
                    *example = Value::String(pinned.to_owned());
                    *count += 1;
                }
                for nested in object.values_mut() {
                    visit(nested, pinned, count);
                }
            }
            Value::Array(items) => {
                for item in items {
                    visit(item, pinned, count);
                }
            }
            _ => {}
        }
    }

    for value in parameter.values_mut() {
        visit(value, pinned, count);
    }
}

/// Whether a string looks like an ISO-8601 instant (`2026-01-01T00:00:00.000Z`).
///
/// Deliberately shape-based rather than a full parse: it only needs to separate
/// generated timestamps from the spec's other string examples.
fn is_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 20
        && bytes.ends_with(b"Z")
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[..4].iter().all(u8::is_ascii_digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_are_recognized() {
        assert!(is_timestamp("2026-08-31T21:00:37.157Z"));
        assert!(is_timestamp("2026-01-01T00:00:00.000Z"));

        assert!(!is_timestamp("https://files.gladia.io/example/audio.wav"));
        assert!(!is_timestamp("solaria-1"));
        assert!(!is_timestamp(""));
        assert!(!is_timestamp("2026-08-31"));
    }

    #[test]
    fn only_the_volatile_query_parameters_are_pinned() {
        let mut document = serde_json::json!({
            "paths": {
                "/v2/pre-recorded": {
                    "get": {
                        "parameters": [
                            {
                                "name": "date",
                                "in": "query",
                                "schema": { "example": "2026-08-31T21:00:37.157Z" }
                            },
                            {
                                "name": "model",
                                "in": "query",
                                "schema": { "example": "solaria-1" }
                            },
                            // Same name, but not a query parameter.
                            {
                                "name": "date",
                                "in": "header",
                                "schema": { "example": "2026-03-04T05:06:07.008Z" }
                            }
                        ]
                    }
                }
            },
            "components": {
                "schemas": {
                    // A static schema example, not something the server regenerates.
                    "PreRecordedResponse": {
                        "properties": {
                            "created_at": { "example": "2026-03-04T05:06:07.008Z" }
                        }
                    },
                    // A property named like a volatile parameter, but not one.
                    "Filter": {
                        "properties": {
                            "date": { "example": "2026-03-04T05:06:07.008Z" }
                        }
                    }
                }
            }
        });

        assert_eq!(normalize(&mut document), 1);

        let parameters = &document["paths"]["/v2/pre-recorded"]["get"]["parameters"];
        assert_eq!(
            parameters[0]["schema"]["example"],
            "2026-01-01T00:00:00.000Z"
        );
        assert_eq!(parameters[1]["schema"]["example"], "solaria-1");
        assert_eq!(
            parameters[2]["schema"]["example"], "2026-03-04T05:06:07.008Z",
            "only query parameters are regenerated by the API"
        );

        let schema = &document["components"]["schemas"]["PreRecordedResponse"];
        assert_eq!(
            schema["properties"]["created_at"]["example"], "2026-03-04T05:06:07.008Z",
            "schema examples are documentation, not volatile"
        );
    }
}
