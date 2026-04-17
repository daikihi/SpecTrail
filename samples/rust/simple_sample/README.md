# Simple Sample

/// [@ss-simple-sample] layer: meta, type: Philosophy, name: Simple Sample

## Goal

This sample shows a tiny Rust CLI with a clean Presentation / Application / Domain split.

It accepts a single `name` argument, validates it, and returns a greeting.

For a concise explanation of how to use SpecTrail in this sample, see [spectrail_usage.md](./spectrail_usage.md).

This sample also assumes configuration can be switched with `config/*.toml` so the sample can stay separate from the main application settings.

## Structure

/// [@ss-simple-sample-structure] layer: abstract, type: Structure, name: Simple Sample Structure

### `src/main.rs`

/// [@ss-simple-sample-structure-main] layer: abstract, type: Structure, name: Main Structure

- CLI entry point

### `src/application.rs`

/// [@ss-simple-sample-structure-application] layer: spec-detail, type: NonFunc, name: Application Structure

- Application orchestration

### `src/domain.rs`

/// [@ss-simple-sample-structure-domain] layer: implementation, type: Structure, name: Domain Structure

- Validation and greeting generation

## Architecture

### Presentation

/// [@ss-simple-sample-presentation] layer: abstract, type: Philosophy, name: Presentation
/// [@ss-simple-sample-structure-main] layer: abstract, type: Structure, name: Main Structure

- `main.rs` reads CLI arguments and prints the result

### Application

/// [@ss-simple-sample-application] layer: spec-detail, type: NonFunc, name: Application
/// [@ss-simple-sample-structure-application] layer: spec-detail, type: NonFunc, name: Application Structure

- `application.rs` coordinates validation and greeting generation

### Domain

/// [@ss-simple-sample-domain] layer: implementation, type: Structure, name: Domain
/// [@ss-simple-sample-structure-domain] layer: implementation, type: Structure, name: Domain Structure

- `domain.rs` validates the name and builds the greeting
- The sample keeps responsibilities separated so the flow is easy to follow

## Behavior

- Empty or whitespace-only input is rejected
- Valid input returns `Hello, <name>`

## SpecTrail Annotations

This sample keeps annotation markers in both the document and the code so the traceability flow can be checked in a small example.
