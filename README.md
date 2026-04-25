# `cargo check`, `cargo clippy`, and `#[ctor]` inconsistencies

ctor v0.9.0 introduced an inconsistency in MacOS that causes global constructor
functions to behave differently between `cargo check` and `cargo clippy`
executions, and between attributes that do and do not included a `priority = _`
argument. This workspace demonstrates a minimal reproduction of these issues.

## Structure

This workspace includes two crates:

- `producer` exposes static values that are initialized with various forms of
  `#[ctor]` attributes.
- `consumer` reads those values into a build.rs that emits them as compiler
  warnings.


## Reproduction

Run these commands from this directory:

```bash
cargo clean
cargo check -p consumer

cargo clean
cargo clippy -p consumer
```

Expected behavior:

- `cargo check -p consumer` prints compiler warnings showing that all static
  values have been initialized.
- `cargo clippy -p consumer` prints compiler warnings showing that only _some_
  of the static values have been initialized.

## Compare against `ctor` 0.8.x

If you want to check the version boundary directly, change
`producer/Cargo.toml` from `ctor = "=0.10.1"` to `ctor = "=0.8.0"`, remove
`Cargo.lock`, and rerun the `cargo clippy` command above.
