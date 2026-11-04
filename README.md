# Login

Standalone extraction of the Login user program from the sqware source tree
at commit `94e114614bec53424ac55e640fc80cf19a3e7107`.

## Packages

- `server` (`login-server`): the `login` executable. Its `main.rs` keeps the
  original Login body; only the program-crate paths were replaced with the
  published `mold::entry` attribute and `system_support::debug::put`.
- `declaration` (`login-declaration`): the host-safe `no_std` unit
  declaration. It re-exports `unit` and `PROGRAM`, a byte-identical copy of
  the original `crates/composition/src/units/login.rs` with `crate::unit`
  resolved by `pub use unit`. It also publishes `BUILD`, the `ImageBuild` for
  the standalone `login` image (package `login-server`, bin `login`, target
  `riscv64gc-unknown-none-elf`, profile `release`), with a provider-relative
  manifest that the Session/Vanllia cluster will prefix.
- `tests/login` (`login-tests`): the original host suite plus one new startup
  frame golden test. It is an independent workspace, excluded from the root
  workspace.

## Build

The root workspace targets the default RISC-V program target:

```sh
cargo build -p login-server --bin login --locked
cargo build -p login-server --bin login --release --locked
```

`.cargo/config.toml` selects `riscv64gc-unknown-none-elf` and the original
program link flags (`-Crelocation-model=static`,
`-Cforce-frame-pointers=yes`, `-Ccode-model=medium`). `server/build.rs`
consumes `DEP_EXECUTION_LINKER_SCRIPT` and passes it to the `login` binary as
`-T<path>`.

`rust-toolchain.toml` declares `nightly-2026-09-04`, preserved from the
source tree. The Root validation that passed used an explicitly selected
installed `nightly-2026-10-09`; the declared channel is untested as delivered.

The declaration is host-safe:

```sh
cargo check -p login-declaration --target x86_64-unknown-linux-gnu
```

## Tests

The Login host suite uses an explicit host target:

```sh
cargo test --manifest-path tests/login/Cargo.toml --target x86_64-unknown-linux-gnu
```

The original two tests and their assertions are unchanged; only the `auth.rs`
path points at `server/src/auth.rs`. The new golden test path-includes the
moved real `Context` and pins the untouched Shell startup contract: an exact
8-byte little-endian `TaskId` frame with no trailing bytes. It does not claim
any mocked native runtime behaviour.

## Integration boundary

Login keeps its original authorities and failure responsibilities:

- It owns local Argon2 password verification and zeroization.
- It asks Account to create a paused Shell instance, reconciles `NotReady` by
  inspecting and, when unresolved, cancelling the same `CreateId`.
- It forwards the startup resolve authority through `forward_resolve`.
- It delivers the marked issuer context, lends terminal foreground, then
  embarks, joins, releases, restores, and revokes in the original order.
- It does not own Account construction, System/Control lifecycle, the
  Terminal, or the Shell receiver. Those published providers and the original
  Shell are unchanged.

The unit declaration preserves the logical name/image `login`, its original
dependencies, and `Ending::Told`.

## Limitations

- Runtime behaviour is unvalidated in this repository. No QEMU, kernel, or
  initrd is present here.
- Session/Vanllia cluster integration and the Account/System fault probes
  (claim timeout, release failure, Login-death cleanup) are still pending on
  the integration side.
- This extraction does not claim that the whole independence effort is
  complete; it only provides the standalone build, declaration, body, and host
  contract tests described above.
- Build, host-test, ELF, and equivalence checks were run by Root with
  `nightly-2026-10-09`. Runtime behaviour remains unvalidated: no QEMU run
  was performed. Root will add the final validation record.

No source license or license declaration was found at the original commit; no
new license is asserted. See `PROVENANCE.md`.
