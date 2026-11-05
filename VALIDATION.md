# Validation

## Unit document validation, 2026-11-05

`server/unit.toml` adds the schema-1 Login document without changing the
legacy `PROGRAM`, `BUILD`, server code, or original Login tests. Its complete
field mapping, startup creation, provider closure, and System bootstrap
boundary are described in `README.md`.

The contract suite uses `unit-contract`, a development alias for the public
Unit revision `ad50b803c3bd5cd2ef90010d80badf23551e88d5`, for `document`
and `record`. The production `unit` dependency remains at
`90396f0fda376e0693fb1b9553aa21864d241057`, and the test reads the original
`PROGRAM` and `BUILD` through that dependency. Cargo generated the actual
root lock in the unchanged repository workspace, retaining all 42 prior
external package version/source/checksum tuples and adding 22 packages for
the development dependency closure. Both Unit revisions have public Git
sources in the lock; no local path patch is committed or used.

Validation sourced `/workspace/.vanllia-tools/env.sh`, selecting installed
`nightly-2026-10-09`, and used `/tmp/target-login-unit-public` for artifacts:

```sh
source /workspace/.vanllia-tools/env.sh
CARGO_TARGET_DIR=/tmp/target-login-unit-public cargo test \
  -p login-declaration --target x86_64-unknown-linux-gnu --locked --offline
cargo tree -p login-declaration --edges normal --locked --offline
```

All 3 new tests passed with the generated lock and then with locked offline
resolution: every-field comparison to the legacy declaration, real Cargo
bin/build mapping, and complete borrowed binary-record roundtrip. The normal
dependency tree contains only the original Unit revision; the new revision
is confined to development dependencies. Scoped
`rustfmt --edition 2024 --check declaration/tests/unit.rs` and
`git diff --check` passed. Login's empty authority and resource collections
remain empty; no declaration mark, Ops, or Grant value was added.

The unchanged Login host suite also passed all 3 tests with the public
dependencies and its original independent lock:

```sh
CARGO_TARGET_DIR=/tmp/target-login-unit-existing cargo test \
  --manifest-path tests/login/Cargo.toml --target x86_64-unknown-linux-gnu --locked
```

Its initial offline attempt required uncached registry packages; the locked
online run fetched them and passed authentication, Account request framing,
and the Shell startup-context frame contract.

The original independent Login reviewer approved the public-dependency change
and repeated both suites with `--locked --offline`: 3 contract tests and
3 existing host tests passed. The reviewer also verified the development-only
alias in Cargo metadata, the original production Unit tree, all 42 preserved
external lock tuples, unchanged production sources and independent test lock,
and scoped formatting and diff checks. An additional full-workspace offline
metadata query was blocked by uncached `libc 0.2.190`; it was outside these
test checks and does not establish a full-workspace offline cache.

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
