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

## History and license

Filtered history retains original authorship and changes with derived IDs.
`provenance/source-commit-map.txt` maps source to filtered commits;
`provenance/published-source-commits.json` maps GitHub imports. Declaration
authors are recorded in `provenance/source-files.json`. No source LICENSE or
license declaration was found at the baseline; no new license is asserted.
