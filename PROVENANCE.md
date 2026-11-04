# Provenance

## Origin

- Repository: `https://github.com/anreonyr/sqware.git`
- Commit: `94e114614bec53424ac55e640fc80cf19a3e7107`
- Original file hashes and authors: `provenance/source-files.json`
- Original-to-filtered commit map: `provenance/source-commit-map.txt`

## Path mapping

| Original path | Extracted path |
|---|---|
| `programs/src/user/login/main.rs` | `server/src/main.rs` |
| `programs/src/user/login/auth.rs` | `server/src/auth.rs` |
| `programs/src/user/login/startup.rs` | `server/src/startup/{mod.rs,context.rs,issuer.rs}` |
| `programs/src/user/login/mod.rs` | removed; module wiring lives in `server/src/main.rs` |
| `programs/src/user/login/README.md` | removed; replaced by this repository's `README.md` |
| `programs/tests/login/Cargo.toml` | `tests/login/Cargo.toml` |
| `programs/tests/login/lib.rs` | `tests/login/lib.rs` |
| `crates/composition/src/units/login.rs` | `declaration/src/login.rs` (plus `declaration/src/lib.rs`) |
| `crates/composition/src/image_builds.rs` (`image!("login")` entry) | `declaration/src/build.rs` (`BUILD`) |
| `programs/build.rs` | `server/build.rs` (reduced to the `login` bin) |
| `.cargo/config.toml` | `.cargo/config.toml` (original target and rustflags) |
| `AGENTS.md` | `AGENTS.md` |
| `rust-toolchain.toml` | `rust-toolchain.toml` |

## History

The target repository holds a local filtered history of 30 original Login
commits over paths under `server/src` and `tests/login`. That history is
derived, not an unmodified copy of the source history: paths, file sets, and
therefore commit hashes differ from the source commits.

`provenance/source-commit-map.txt` is the local-filter mapping from each of the
30 source commits to the filtered commit that carries its Login changes. It
contains only the nonzero mappings; the unrelated zero rows of the full filter
map were pruned. It records which source commit each filtered commit derives
from — it is not a claim that those derived hashes will be the remote hashes.

Publication is planned as follows, and is Root's work, not this repository's:
the remote target is empty, so publishing first creates a minimal `README`
bootstrap on `main`. The GitHub Git API is then used to upload the 30 filtered
commits, preserving every tree and the author and committer names, e-mails,
dates, and messages, with the original subsequent topology intact; only the
first imported commit is attached to the bootstrap. The resulting remote
hashes are therefore new objects, and Root evidence will carry an additional
publication map from filtered commit to published commit. No force push, PR
merge, or repository settings change is part of this plan.

So: authorship and change content are preserved and traceable through the
explicit source-to-filtered and filtered-to-published mappings. This history
is not byte-identically unrewritten.

This change adds the workspace, declaration, build wiring, `README.md`,
provenance copies, and the required structure split on top of that filtered
history.

The declaration source is `crates/composition/src/units/login.rs`. Its history
was not part of the 30-commit Login filter, so it is attributed here to the
original commit and to the authors recorded in
`provenance/source-files.json`.

## License

No source `LICENSE` file or license declaration was found at the original
commit. No new license is asserted.

## Behaviour changes and adaptations

All main function bodies are preserved exactly. The only program-body
replacements are:

- `extern crate programs;` is removed.
- `#[programs::entry]` becomes `#[mold::entry]`.
- The five `programs::debug::put` call sites become
  `system_support::debug::put`.

No runtime strings, marks, constants, opcodes, wire frames, or protocol names
were renamed. `server/src/auth.rs` is a byte-identical copy of the original
file.

`startup.rs` was split by structure: `context.rs` holds `MARK` and the 8-byte
`Context` frame, `issuer.rs` holds `Issuer` and `deliver`, and `mod.rs` only
wires the modules and re-exports the delivery entry point. The inspect, push,
and accord sequence, the `Revoke` drop, and all failure paths are unchanged.

The original two host tests and assertions are unchanged; only the `auth.rs`
path was updated. A new golden test pins the moved real `Context` against the
untouched Shell wire contract. It is new coverage, not parity evidence.

The declaration copies the unit file exactly and resolves `crate::unit`
through `pub use unit`, preserving the logical name/image `login`, the
dependency list, and `Ending::Told`.

The declaration also publishes `BUILD`, an `unit::ImageBuild` for the
standalone `login` image: image `login`, package `login-server`, bin `login`,
target `riscv64gc-unknown-none-elf`, profile `release`, no features, default
features on. Its `manifest` is provider-relative (`server/Cargo.toml`); the
Session/Vanllia cluster supplies its own manifest prefix, following the
existing Account/Terminal pattern, so the build facts are stated once here
instead of being duplicated at integration time. This mirrors the original
`image!("login")` entry in `crates/composition/src/image_builds.rs`, whose
manifest/package/bin were `programs/Cargo.toml`, `programs`, `prog-login`; the
manifest file, package, and bin names change with the extraction; the image
name, target, profile, and feature flags do not.

Known preserved coupling: the success greeting `"Hello, anran.\n"` is a
literal in the original body and is intentionally not derived from
`auth::ACCOUNT`.

## Validation status

Native checks, the original host tests plus the new golden test, both actual
RISC-V ELF builds, and body/token equivalence were run by Root against an
explicitly selected installed toolchain, `nightly-2026-10-09`.

The `rust-toolchain.toml` declared in this repository, `nightly-2026-09-04`,
is preserved from the source tree and is untested as delivered. Runtime
behaviour was not validated; no QEMU run was performed.
