# Login

Login authenticates the user and hands session startup to Account using Terminal
I/O. It is a startup Unit in Shuyra Vanllia.

## Layout and startup contract

- `server/`: `login-server`, the `login` executable, authentication and startup code.
- `server/unit.toml`: runtime policy and dependency ordering.
- `declaration/`: retained Rust `PROGRAM` and `BUILD` compatibility declarations.
- `tests/login` and `tests/unit-document`: independent host test workspaces.

Login requires Terminal and Account and waits for both. `requires` selects
packaging dependencies; `after` orders creation. Account owns Shell construction.
Packaging the requested Shell template must not start it or give Login additional
construction rights. System completes Operator and Identity bootstrap first.

## Build and test

Install Rust through rustup. `rust-toolchain.toml` selects `nightly-2026-09-04`;
`.cargo/config.toml` defaults to `riscv64gc-unknown-none-elf`. From the root:

```sh
cargo build -p login-server --bin login --release --locked
cargo test --manifest-path tests/login/Cargo.toml --target x86_64-unknown-linux-gnu --locked
cargo test --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu --locked
```

The first run needs network access. `server/build.rs` uses Execution's linker
metadata. Host tests cover authentication, startup frames and Unit conversion;
actual startup and handover need the selected Vanllia deployment.

See [PROVENANCE.md](PROVENANCE.md) for attribution and source-history mappings.
Follow the [Shuyra maintenance standard](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
