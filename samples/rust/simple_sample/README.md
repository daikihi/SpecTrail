# Simple Sample

/// @DocumentAnnotation @spec="ss-simple-sample" @type="non-func" @layer="document"

## Goal

This sample shows a tiny Rust CLI with a clean Presentation / Application / Domain split.

It accepts a single `name` argument, validates it, and returns a greeting.

For a concise explanation of how to use SpecTrail in this sample, see [spectrail_usage.md](./spectrail_usage.md).

This sample also assumes configuration can be switched with `config/*.toml` so the sample can stay separate from the main application settings.

## Structure

/// @DocumentAnnotation @spec="ss-simple-sample-structure" @type="non-func" @layer="document"

### `src/main.rs`

/// @DocumentAnnotation @spec="ss-simple-sample-structure-main" @type="non-func" @layer="document"

- CLI entry point

### `src/application.rs`

/// @DocumentAnnotation @spec="ss-simple-sample-structure-application" @type="non-func" @layer="document"

- Application orchestration

### `src/domain.rs`

/// @DocumentAnnotation @spec="ss-simple-sample-structure-domain" @type="non-func" @layer="document"

- Validation and greeting generation

## Architecture

### Presentation

/// @DocumentAnnotation @spec="ss-simple-sample-presentation" @type="non-func" @layer="presentation"
/// @DocumentAnnotation @spec="ss-simple-sample-structure-main" @type="non-func" @layer="document"

- `main.rs` reads CLI arguments and prints the result

### Application

/// @DocumentAnnotation @spec="ss-simple-sample-application" @type="non-func" @layer="application"
/// @DocumentAnnotation @spec="ss-simple-sample-structure-application" @type="non-func" @layer="document"

- `application.rs` coordinates validation and greeting generation

### Domain

/// @DocumentAnnotation @spec="ss-simple-sample-domain" @type="non-func" @layer="domain"
/// @DocumentAnnotation @spec="ss-simple-sample-structure-domain" @type="non-func" @layer="document"

- `domain.rs` validates the name and builds the greeting
- The sample keeps responsibilities separated so the flow is easy to follow

## Behavior

- Empty or whitespace-only input is rejected
- Valid input returns `Hello, <name>`

## SpecTrail Annotations

This sample keeps annotation markers in both the document and the code so the traceability flow can be checked in a small example.
