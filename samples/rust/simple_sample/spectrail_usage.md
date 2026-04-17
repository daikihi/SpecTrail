# SpecTrail Usage Guide for Simple Sample

/// [@ss-simple-sample-spectrail-usage] layer: meta, type: Philosophy, name: SpecTrail Usage Guide

## Purpose

This document explains how to use SpecTrail in `simple_sample/`.

The sample is intentionally small, so the documentation shows the annotation flow first and the code follows it.

## Rules

### 1. Write documentation before code

/// [@ss-simple-sample-spectrail-usage-doc-first] layer: abstract, type: Philosophy, name: Write documentation before code

- Start with the specification document
- Add annotations to the document sections
- Agree on the content before changing code

### 2. Use `///` for SpecTrail annotation lines in markdown

/// [@ss-simple-sample-spectrail-usage-doc-annotation] layer: abstract, type: Philosophy, name: Use /// for SpecTrail annotation lines in markdown

- Put the annotation line directly above the section or item it describes
- Keep the annotation line short and explicit
- Use the same ID in the code when the item is implemented there

### 3. Keep IDs aligned between document and code

/// [@ss-simple-sample-spectrail-usage-id-alignment] layer: abstract, type: Philosophy, name: Keep IDs aligned between document and code

- Document requirement IDs and code requirement IDs should match
- Structure annotations should point to the same logical part of the sample
- The `application` layer in this sample is named `application` on purpose

### 4. Use separate config files per application

/// [@ss-simple-sample-spectrail-usage-config] layer: abstract, type: Philosophy, name: Use separate config files per application

- Prefer `config/default.toml` for the main project
- Prefer `config/simple_sample.toml` for this sample
- Select the config explicitly when running commands that inspect annotations
- Keep the sample documentation independent from the main project configuration

## Example Mapping

### Requirement example

/// [@ss-simple-sample-spectrail-usage-example-requirement] layer: abstract, type: Philosophy, name: Requirement example

- Document: `/// [@ss-simple-sample-requirement-1] layer: abstract, type: Philosophy, name: Requirement 1`
- Code: `/// [@ss-simple-sample-requirement-1] layer: implementation, type: Structure, name: Requirement 1`

### Structure example

/// [@ss-simple-sample-spectrail-usage-example-structure] layer: abstract, type: Philosophy, name: Structure example

- Document: `### Structure: \`src/main.rs\``
- Code: `src/main.rs` carries the matching annotation

## Files in This Sample

/// [@ss-simple-sample-spectrail-usage-files] layer: abstract, type: Philosophy, name: Files in This Sample

- `spec.md`: main specification for the sample
- `README.md`: human-friendly overview and structure summary
- `spectrail_usage.md`: how to use SpecTrail in this sample
- `src/main.rs`: Presentation layer
- `src/application.rs`: Application layer
- `src/domain.rs`: Domain layer

## Recommended Flow

/// [@ss-simple-sample-spectrail-usage-flow] layer: abstract, type: Philosophy, name: Recommended Flow

1. Update `spec.md`
2. Update `README.md` if needed
3. Add or adjust annotations in the sample code
4. Run a build check

## Using `show`

/// [@ss-simple-sample-spectrail-usage-show] layer: abstract, type: Philosophy, name: Using show

Use the `show` command to confirm that the sample annotations are visible from the main SpecTrail project.

If the project supports config selection, point `show` at the config that matches the application you want to inspect.

- Show all annotations:

```bash
cargo run --bin show -- --mode list --target all
```

- Show annotations with a sample config:

```bash
cargo run --bin show -- --mode list --target all --config config/simple_sample.toml
```

- Focus on the sample scope:

```bash
cargo run --bin show -- --mode list --target all --scope ss-simple-sample
```

- Check a specific annotation ID:

```bash
cargo run --bin show -- --mode search --target all --scope ss-simple-sample-requirement-1
```

If the command output is noisy, narrow the target or scope to the sample document and code annotations first.

## Notes

/// [@ss-simple-sample-spectrail-usage-notes] layer: abstract, type: Philosophy, name: Notes

- Keep this sample small
- Prefer clarity over abstraction
- Treat annotation IDs as part of the sample contract
