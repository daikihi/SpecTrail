# Quickstart — SpecTrail Scanner (MVP)

**Prerequisites**
- Rust toolchain (1.70+), `cargo` installed
- Git available on PATH

**Build**

```bash
cargo build --bin spec-trail
```

**Scan repository and output manifest**

```bash
# Scan current repo and write normalized manifest
cargo run --bin spec-trail -- scan --path . --out specs/annotations.json
```

**Generate coverage / integrity report**

```bash
cargo run --bin spec-trail -- report --manifest specs/annotations.json --format text
```

**Check PR diffs (local)**

```bash
# Compare current branch to main
cargo run --bin spec-trail -- diff --range main --format json
```

**Notes**
- For CI, prefer `--format json` and fail the job on integrity failures.
- The manifest schema is in `specs/001-spec-impl-annotations/contracts/manifest.schema.json`.
