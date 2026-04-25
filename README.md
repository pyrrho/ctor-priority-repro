# `cargo clippy` inconsistencies

ctor v0.9.0 introduced an inconsistency in MacOS that causes global constructor
functions to behave inconsistently when evaluated under `cargo clippy`,
specifically between attributes that do and do not included a `priority = _`
argument. This workspace demonstrates a minimal reproduction of these issues.

## Structure

This workspace includes two crates:

- `producer` exposes static values that are initialized with various forms of
  `#[ctor]` attributes.
- `consumer` reads those values into a build.rs that emits them as compiler
  warnings.

## Reproduction

```bash
cargo clippy -p consumer
```

- On a Debian 13 linux system
  ```sh
  warning: consumer@0.1.0: plain                is false
  warning: consumer@0.1.0: simple               is false
  warning: consumer@0.1.0: plain_declarative    is false
  warning: consumer@0.1.0: simple_declarative   is false
  warning: consumer@0.1.0: priority             is false
  warning: consumer@0.1.0: priority_declarative is false
  ```

- On a MacOS 14.7.2 system
  ```sh
  warning: consumer@0.1.0: plain                is false
  warning: consumer@0.1.0: simple               is false
  warning: consumer@0.1.0: plain_declarative    is false
  warning: consumer@0.1.0: simple_declarative   is false
  warning: consumer@0.1.0: priority             is true
  warning: consumer@0.1.0: priority_declarative is true
  ```

Note that `cargo check -p consumer` yields `is true` for all log line under both
my linux and mac systems.
