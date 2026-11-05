# Validation

## Unit document validation, 2026-11-05

`server/unit.toml` adds the schema-1 Login document without changing the
legacy `PROGRAM`, `BUILD`, server code, or original Login tests. Its complete
field mapping, startup creation, provider closure, and System bootstrap
boundary are described in `README.md`.

The contract suite is an independent workspace at `tests/unit-document`,
explicitly excluded from the production workspace. It uses `login-declaration`
by path and `unit-contract`, a development alias for public Unit revision
`ad50b803c3bd5cd2ef90010d80badf23551e88d5`, for `document` and `record`.
The production `unit` dependency remains at
`90396f0fda376e0693fb1b9553aa21864d241057`; `login-declaration` has no
development dependencies. This keeps Cargo's production Unit patch from
collapsing two differently named direct dependencies into the same package.
The moved Rust test file is byte-identical to the original three-test suite,
and reads the original `PROGRAM` and `BUILD` through the path dependency.

Cargo generated both formal locks. The production root lock retains exactly
the 42 original external package version/source/checksum tuples and removes
the 22 packages introduced solely for the contract tests. The independent
test lock records both public Unit revisions. Neither tracked workspace
commits a local Unit path patch.

Validation sourced `/workspace/.vanllia-tools/env.sh`, selecting installed
`nightly-2026-10-09`, and used `/tmp/target-login-unit-public` for artifacts:

```sh
source /workspace/.vanllia-tools/env.sh
CARGO_TARGET_DIR=/tmp/target-login-unit-public cargo test \
  --manifest-path tests/unit-document/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked --offline
cargo tree -p login-declaration --edges normal --locked --offline
```

All 3 new tests passed with the generated lock and then with locked offline
resolution: every-field comparison to the legacy declaration, real Cargo
bin/build mapping, and complete borrowed binary-record roundtrip. The production
workspace contains only the original Unit revision; the new revision is
confined to the independent test workspace. Scoped
`rustfmt --edition 2024 --check tests/unit-document/unit.rs` and
`git diff --check` passed. Login's empty authority and resource collections
remain empty; no declaration mark, Ops, or Grant value was added.

The unchanged Login host suite also passed all 3 tests with the public
dependencies and its original independent lock:

```sh
CARGO_TARGET_DIR=/tmp/target-login-unit-existing cargo test \
  --manifest-path tests/login/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked --offline
```

Authentication, Account request framing, and the Shell startup-context frame
contract all passed with the original independent Login test lock unchanged.

The production declaration also passed an actual Cargo-native Unit overlay
check. Its generated overlay lock is outside the repository; comparison
against a saved copy confirmed the tracked root lock was unchanged:

```sh
CARGO_RESOLVER_LOCKFILE_PATH=/workspace/vanllia/target/login-unit-overlay/Cargo.lock \
CARGO_TARGET_DIR=/tmp/target-login-unit-overlay cargo check \
  -p login-declaration --target x86_64-unknown-linux-gnu --offline \
  --config 'patch."https://github.com/shuyra-arche/unit.git".unit.path="/workspace/unit"'
```

This compiled the local Unit package and production declaration successfully,
without the duplicate dependency-name error. The two test runs above used
their public dependencies without an overlay.

The original independent Login reviewer approved this workspace separation.
It repeated both test suites with `--locked --offline` (3 tests each) and
repeated the native Unit overlay check with `--locked` successfully. It also
verified that the moved test file, original Login test lock, and restored
root lock were byte-identical to their respective baselines, that production
workspace metadata contains only `server` and `declaration`, and that
`login-declaration` depends only on the original Unit revision. Scoped
formatting and diff checks passed independently.

No old scene migration, native server rebuild, QEMU run, or runtime acceptance
is claimed for this change.

## Extraction baseline

Source baseline: `anreonyr/sqware` at
`94e114614bec53424ac55e640fc80cf19a3e7107`.

Root selected and executed these checks in the isolated provider checkout,
with installed `nightly-2026-10-09`, on 2026-11-04:

| Check | Result |
|---|---|
| Original Login host suite | 2 passed |
| Extracted Login host suite, including the moved real Context golden frame | 3 passed; locked/offline repeat passed |
| `cargo check -p login-declaration --target x86_64-unknown-linux-gnu --locked --offline` | Passed, including `PROGRAM` and `BUILD` |
| `cargo build -p login-server --bin login --target riscv64gc-unknown-none-elf --locked` | Passed |
| Same RISC-V build with `--release --locked --offline` | Passed |
| Root `cargo metadata --format-version 1 --locked --offline` | Passed |
| Main body, authentication, issuer delivery/drop, declaration and original assertions | Equivalent; only documented imports/module wiring changed |
| Original source hashes | Verified |
| Root external lock tuples | All 42 match Source root lock tuples |
| Independent host suite lock | Byte-identical to Source's tracked Login lock |
| Normal dependency closure | No `sqware` or `programs` package |

The root lock is the selected subset of the Source workspace lock. The
independent test suite retains its own original versions; these two source
locks already differed in `bitflags` and `unicode-ident`. No unrelated
dependency version update was introduced.

Both artifacts are ELF64 little-endian RISC-V static `ET_EXEC`, with image
base `0x10000`, no interpreter, and exactly one definition of `_start`,
`clean_ret`, `tls_bootstrap`, the panic handler and allocator entry. The ELF
entry equals `_start`; it differs from the image base.

| Artifact | Entry | Bytes | SHA-256 |
|---|---|---|---|
| Debug `login` | `0x26784` | 8375408 | `5d0ed6d3a7ad02f4e5c465f12c88f6943d36414b85702c6b22f9068774f368a0` |
| Release `login` | `0x1443a` | 359224 | `a1c122b25bb4eb09ddd54cd8b55942c7b50bc61283eaed4262e3d9441b3cee8e` |

These hashes identify the actual artifacts built in this checkout, not a
claim of byte-for-byte reproducibility across directories or toolchains.

The first worker turn exhausted its response token limit before editing;
the same worker session continued and completed the implementation. Root's
first added ELF diagnostic incorrectly equated the image base with the
entry point, then encountered a symbol-table header parsing error. Both
diagnostic assumptions were corrected against the real ELF headers and
`_start` symbol; no runtime code was changed to satisfy those checks.

The genuine OpenCode Go `deepseek-v4.1-flash` roles use separate contexts:

- Namer: `session-dc091847-b967-427c-8820-87e8b45279ef`.
- Sole worker: `session-014c6927-88fc-46b8-ae21-5c7fa2b47be9`, continued for
  implementation and corrections in the same session.
- Independent reviewer: `session-a25b3f9b-14e2-401c-909b-7bbb9ece08e9`,
  PASS with no blocking findings. The first review request that included full
  raw ELF symbol tables exceeded the upstream context limit; the same role
  resumed with bounded tool views and completed the review.

Role tools expose constrained filesystem operations, without shell or
network execution. Root owns native checks and Git delivery. Actual model
responses and tool outcomes are retained in Root's local evidence.

No QEMU run, real terminal interaction, Account/System fault acceptance,
Session aggregate integration or Vanllia integration was performed. The
preserved declared `nightly-2026-09-04` toolchain was not exercised. Native
compilation and unchanged bodies do not establish runtime acceptance.
