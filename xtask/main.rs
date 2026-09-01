//! Development tasks for the `gladia` crate.
//!
//! ```sh
//! cargo xtask codegen          # regenerate src/model/generated.rs
//! cargo xtask codegen --check  # fail if the committed output is stale
//! cargo xtask fetch-spec       # refresh docs/openapi.json from the API
//! ```

use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod codegen;
mod fetch;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let task = args.next();
    let check = args.any(|a| a == "--check");

    let result = match task.as_deref() {
        Some("codegen") => codegen::run(&root(), check),
        Some("fetch-spec") => fetch::run(&root()),
        Some(other) => {
            Err(format!("unknown task `{other}`; expected `codegen` or `fetch-spec`").into())
        }
        None => Err("usage: cargo xtask <codegen [--check] | fetch-spec>".into()),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// The repository root, derived from this crate's location rather than the
/// current directory, so the task works from anywhere in the tree.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask/ always has a parent")
        .to_path_buf()
}
