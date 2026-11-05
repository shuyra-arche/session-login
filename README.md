# Login

Standalone extraction of the Login user program from the sqware source tree
at commit `94e114614bec53424ac55e640fc80cf19a3e7107`.

## Packages

- `server` (`login-server`): the `login` executable. Its `main.rs` keeps the
  original Login body; only the program-crate paths were replaced with the
  published `mold::entry` attribute and `system_support::debug::put`.
- `declaration` (`login-declaration`): the host-safe `no_std` unit
  compatibility declaration. It re-exports `unit` and `PROGRAM`, a byte-identical
  copy of
  the original `crates/composition/src/units/login.rs` with `crate::unit`
  resolved by `pub use unit`. It also publishes `BUILD`, the `ImageBuild` for
  the standalone `login` image (package `login-server`, bin `login`, target
  `riscv64gc-unknown-none-elf`, profile `release`), with a provider-relative
  manifest that the Session/Vanllia cluster will prefix.
- `server/unit.toml`: the schema-1 declaration for the Unit document and
  binary-record runtime. This file is the single source for the new model;
  the old `PROGRAM` and `BUILD` remain for scenes that have not migrated.
- `tests/login` (`login-tests`): the original host suite plus one new startup
  frame golden test. It is an independent workspace, excluded from the root
  workspace.
- `tests/unit-document` (`login-unit-document-tests`): the three Unit document
  contract tests and their own lock, in a separate workspace excluded from
  the production workspace.

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

## Unit document model

`server/unit.toml` preserves the stable Unit ID `login` and the real Cargo
binary `login`. Loader space is `user`, arguments are empty, and stack is
zero, as in the legacy declaration. Namespace publications, identity aliases
and grants, construction permissions, retirement hooks, and resource bindings
remain empty. Login has no declaration marks, Ops, or Grant values to encode.
Readiness remains spawn and ending remains told; supervision remains absent.

`control.creation = "startup"` makes Login a startup instance. `requires`
includes the Terminal and Account providers needed by the program. `after`
waits for those providers to be ready. These are separate facts: `requires`
selects the dependency closure for packaging, while `after` orders creation.
Account's construction dependency on the `shell` template belongs to Account
or the deployment's dependency closure. Shell uses `creation = "requested"`:
packaging the template must not launch it, and Login does not wait for a
startup Shell or gain Shell construction rights.

System must finish both Bootstrap Operator and Bootstrap Identity before
processing Unit documents. They are System bootstrap steps, not Units. The
legacy `SCENE` dependency is a composition target and its membership and
ordering belong to the deployment; it is not a provider named `scene` in this
document. These two legacy dependency forms are therefore not copied into
`requires` or `after`.

The contract suite parses this file as
`unit_contract::document::Unit<String>`, validates it, and encodes it with
`unit_contract::record::encode`. Record decoding yields a validated
`Unit<&str>` whose strings borrow the binary record. The independent contract tests
compare all schema fields to the legacy `PROGRAM`, including the explicit
bootstrap/composition mapping above, and compare every field again after a
binary roundtrip. They also verify the image against the actual Cargo bin.
Future full-width `u64` values use quoted decimal or quoted hexadecimal TOML
strings; labels and signed integers must not stand in for marks.

The independent contract workspace uses `login-declaration` by path and the
development dependency `unit-contract`, an alias for the published Unit package at revision
`ad50b803c3bd5cd2ef90010d80badf23551e88d5`. Its `document` and `record` APIs
are compared directly with the legacy production Unit types. The production
`unit` dependency remains at `90396f0fda376e0693fb1b9553aa21864d241057`,
preserving the existing `PROGRAM` and `BUILD` API for unmigrated scenes.
The root lock contains the original production dependency closure; the test
workspace's lock records both published Unit revisions. Tests use no local
path patch. Keeping their dependencies outside `login-declaration` also lets
production Cargo patches select one local Unit package without introducing
duplicate dependency names in the declaration package.

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

The new declaration contract suite runs on the host with the published
development dependency:

```sh
cargo test --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu --locked
```

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

The legacy compatibility declaration preserves the original dependencies.
The new Unit document preserves the logical name/image `login` and told
ending, with dependency ownership described above.

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
