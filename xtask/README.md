# xtask

Development tasks for the `gladia` crate, following the [cargo-xtask] convention: a
workspace binary invoked through a cargo alias, so tasks need no tool beyond a Rust
toolchain.

```sh
cargo xtask codegen          # regenerate src/model/generated.rs
cargo xtask codegen --check  # fail if the committed output is stale
cargo xtask fetch-spec       # refresh docs/openapi.json from the API
```

The alias lives in [`.cargo/config.toml`](../.cargo/config.toml); `cargo xtask` expands
to `cargo run --package xtask --`. This crate is `publish = false` and is not a
dependency of `gladia`, so `typify` and the other build-time crates never reach anyone
who depends on the library.

## codegen

Generates `src/model/generated.rs` from `docs/openapi.json` with [`typify`], covering
every request, response, config, and enum the API defines.

The output is committed rather than produced by a build script. That keeps `docs.rs`
builds offline, spares contributors an extra toolchain, keeps `cargo build` fast, and
makes a change to Gladia's API a reviewable diff instead of a silent change in the
public types. `cargo xtask codegen --check` runs in CI and fails when the committed
output no longer matches the spec.

Three things happen to the schemas on the way in:

- **`$ref` rewriting.** Typify resolves against `#/$defs`, so the OpenAPI pointers are
  repointed there.
- **Dropping defaults on optional properties.** Typify maps an optional property with a
  `default` to a plain `T` rather than `Option<T>`, which would make every request
  serialize a dozen-odd explicit `false`s and pin defaults the API is free to change.
  Removing the `default` restores `Option<T>` with `skip_serializing_if`.
- **Patching upstream mistakes.** `AudioUploadMetadataDTO.extension` holds a file
  extension but is declared `format: uuid`, contradicted by its own `"wav"` example;
  left alone, every upload response fails to deserialize. Each patch asserts the
  mistake is still present, so codegen fails once it is corrected upstream and the
  patch can be dropped.

## fetch-spec

Refreshes `docs/openapi.json` from `https://api.gladia.io/openapi.json`. Run it, review
the diff, then run `codegen`.

The spec is vendored rather than downloaded during codegen because the served document
is not byte-stable: the API stamps the current time into the `date`, `before_date`, and
`after_date` query-parameter examples, so two fetches minutes apart differ while every
schema is identical. This task pins those examples to a fixed instant, which is what
makes repeated refreshes idempotent, and what lets `codegen --check` mean something.

[cargo-xtask]: https://github.com/matklad/cargo-xtask
[`typify`]: https://github.com/oxidecomputer/typify
