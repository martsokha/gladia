//! Generates `src/model/generated.rs` from the vendored OpenAPI document.
//!
//! The types are generated with [`typify`] rather than hand-written: the spec carries
//! schemas, and hand-written mirrors drift from it silently. Generation is a
//! committed artifact rather than a build script, so `docs.rs` builds offline,
//! contributors need no extra toolchain, and a spec change lands as a reviewable diff.
//!
//! [`typify`]: https://github.com/oxidecomputer/typify

use std::error::Error;
use std::path::Path;

use serde_json::{Map, Value};

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

/// Where the vendored spec lives, relative to the repository root.
const SPEC: &str = "docs/openapi.json";
/// Where the generated module is written, relative to the repository root.
const OUTPUT: &str = "src/model/generated.rs";

/// The header prepended to the generated file.
const HEADER: &str = "\
//! Types generated from the Gladia OpenAPI document.
//!
//! Do not edit by hand. Regenerate with `cargo xtask codegen` after updating
//! `docs/openapi.json`; CI fails if this file is out of date with the spec.

// The API's own descriptions carry `[Deprecated]`/`[Alpha]` markers and a bare URL,
// which rustdoc reads as a malformed link and a missing hyperlink respectively. They
// are Gladia's prose, reproduced verbatim, so the lints are silenced rather than the
// text rewritten.
#![allow(
    clippy::all,
    missing_docs,
    unreachable_pub,
    rustdoc::broken_intra_doc_links,
    rustdoc::bare_urls
)]
#![cfg_attr(rustfmt, rustfmt::skip)]
";

/// Generates the model, writing it to `OUTPUT` or, with `check`, comparing against it.
pub(crate) fn run(root: &Path, check: bool) -> Result<()> {
    let spec = root.join(SPEC);
    let output = root.join(OUTPUT);

    let generated = generate(&spec)?;

    if check {
        let committed =
            std::fs::read_to_string(&output).map_err(|e| format!("{}: {e}", output.display()))?;
        if committed != generated {
            return Err(format!(
                "{} is out of date with {}; run `cargo xtask codegen`",
                OUTPUT, SPEC
            )
            .into());
        }
        println!("{OUTPUT} is up to date");
        return Ok(());
    }

    if let Some(dir) = output.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(&output, &generated)?;
    println!("wrote {OUTPUT} ({} lines)", generated.lines().count());
    Ok(())
}

/// Reads the OpenAPI document and renders the Rust module as a string.
fn generate(spec: &Path) -> Result<String> {
    let raw = std::fs::read_to_string(spec).map_err(|e| format!("{}: {e}", spec.display()))?;
    let document: Value = serde_json::from_str(&raw)?;

    let schemas = document
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(Value::as_object)
        .ok_or("no components.schemas in the spec")?
        .clone();

    let root = root_schema(schemas)?;

    let mut settings = typify::TypeSpaceSettings::default();
    settings
        .with_struct_builder(true)
        .with_derive("PartialEq".to_owned());

    let mut space = typify::TypeSpace::new(&settings);
    space.add_root_schema(root)?;

    let syntax = syn::parse2::<syn::File>(space.to_stream())?;
    Ok(format!("{HEADER}\n{}", prettyplease::unparse(&syntax)))
}

/// Turns `components.schemas` into the JSON Schema document typify consumes,
/// applying the fixes the spec needs on the way in.
fn root_schema(mut schemas: Map<String, Value>) -> Result<schemars::schema::RootSchema> {
    for schema in schemas.values_mut() {
        strip_optional_defaults(schema);
    }

    patch_upstream_errors(&mut schemas)?;

    // Typify resolves `$ref`s against `#/$defs`, so the OpenAPI pointers are
    // rewritten to match. Done textually because the refs are nested arbitrarily
    // deep inside `allOf`, `items`, and property schemas.
    let defs = serde_json::to_string(&schemas)?.replace("#/components/schemas/", "#/$defs/");

    let document = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "GladiaApi",
        "$defs": serde_json::from_str::<Value>(&defs)?,
    });

    Ok(serde_json::from_value(document)?)
}

/// Removes `default` from every property that is not `required`.
///
/// Typify maps an optional property carrying a `default` to a plain `T` with a
/// `Default` impl, rather than `Option<T>` with `skip_serializing_if`. The spec marks
/// most of the request feature toggles `default: false`, so leaving these in place
/// makes every request serialize a dozen-odd explicit `false`s, which is noise on the
/// wire and pins defaults the API is free to change. Dropping the `default` restores
/// `Option<T>`, so a request carries exactly the fields the caller set.
///
/// Defaults on *required* properties are left alone: those are always serialized
/// anyway, and the default is useful documentation.
fn strip_optional_defaults(schema: &mut Value) {
    let required: Vec<String> = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|r| {
            r.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();

    let Some(properties) = schema.get_mut("properties").and_then(Value::as_object_mut) else {
        return;
    };

    for (name, property) in properties.iter_mut() {
        if required.contains(name) {
            continue;
        }
        if let Some(property) = property.as_object_mut() {
            property.remove("default");
        }
    }
}

/// Corrects known mistakes in the upstream spec.
///
/// A wrong `format` in the spec becomes a wrong Rust type, so the mistakes are
/// corrected on the way in. They are fixed here rather than in `docs/openapi.json` so
/// the vendored copy stays a true record of what the API serves, and `fetch-spec` does
/// not revert them.
///
/// Each patch asserts the mistake is still present: once Gladia corrects it upstream,
/// codegen fails and the patch can be removed.
fn patch_upstream_errors(schemas: &mut Map<String, Value>) -> Result<()> {
    // `AudioUploadMetadataDTO.extension` holds a file extension ("wav"), but is
    // declared `format: uuid`, copied from the `id` field above it and contradicted
    // by its own example. Left alone, every upload response fails to deserialize.
    let extension = schemas
        .get_mut("AudioUploadMetadataDTO")
        .and_then(|s| s.get_mut("properties"))
        .and_then(|p| p.get_mut("extension"))
        .and_then(Value::as_object_mut)
        .ok_or("AudioUploadMetadataDTO.extension is missing from the spec")?;

    match extension.get("format").and_then(Value::as_str) {
        Some("uuid") => {
            extension.remove("format");
        }
        // Fixed upstream: drop this patch.
        None => {
            return Err("AudioUploadMetadataDTO.extension no longer declares a \
                            format; remove this patch from patch_upstream_errors"
                .into());
        }
        Some(other) => {
            return Err(format!(
                "AudioUploadMetadataDTO.extension declares an unexpected format `{other}`; \
                 review patch_upstream_errors"
            )
            .into());
        }
    }

    Ok(())
}
