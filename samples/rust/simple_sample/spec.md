# 005 Simple Sample

/// [@ss-simple-sample] layer: meta, type: Philosophy, name: Simple Sample

## Overview

This sample is a minimal Rust CLI that demonstrates documentation-first development and a small three-layer architecture.

## Requirements

/// [@ss-simple-sample-requirements] layer: abstract, type: Philosophy, name: Requirements

### Requirement 1

/// [@ss-simple-sample-requirement-1] layer: abstract, type: Philosophy, name: Requirement 1

- Accept one CLI argument named `name`

### Requirement 2

/// [@ss-simple-sample-requirement-2] layer: abstract, type: Philosophy, name: Requirement 2

- Reject empty or whitespace-only values

### Requirement 3

/// [@ss-simple-sample-requirement-3] layer: abstract, type: Philosophy, name: Requirement 3

- Return a greeting for valid input

### Requirement 4

/// [@ss-simple-sample-requirement-4] layer: abstract, type: Philosophy, name: Requirement 4

- Keep the code split into Presentation, Application, and Domain responsibilities

## Architecture

### Presentation

/// [@ss-simple-sample-presentation] layer: abstract, type: Philosophy, name: Presentation
/// [@ss-simple-sample-structure-main] layer: abstract, type: Philosophy, name: Main Structure

The Presentation layer owns the boundary with the command line.

- `src/main.rs` owns the CLI boundary
- Reads the raw argument
- Delegates execution to the Application layer
- Prints the success message or the error message
- Keeps parsing and output concerns out of business logic

### Application

/// [@ss-simple-sample-application] layer: spec-detail, type: NonFunc, name: Application
/// [@ss-simple-sample-structure-application] layer: spec-detail, type: NonFunc, name: Application Structure

The Application layer coordinates the flow for greeting a user.

- `src/application.rs` orchestrates validation and greeting creation
- Receives raw input from Presentation
- Calls Domain validation
- Assembles the final response string
- Keeps orchestration separate from low-level validation

### Domain

/// [@ss-simple-sample-domain] layer: implementation, type: Structure, name: Domain
/// [@ss-simple-sample-structure-domain] layer: implementation, type: Structure, name: Domain Structure

The Domain layer contains the pure business rules for the sample.

- `src/domain.rs` validates the input name
- Normalizes the value by trimming whitespace
- Rejects empty values after trimming
- Generates the greeting text
- Avoids CLI knowledge and output formatting

### Configuration

/// [@ss-simple-sample-configuration] layer: abstract, type: Philosophy, name: Configuration

The sample should be able to point to different `config/*.toml` files depending on the target application.

- `config/default.toml` represents the main project settings
- `config/simple_sample.toml` represents the sample settings
- The selected config makes it clear which application is being traced
- The sample does not force every application to share one config file

## Annotation Policy

- Add SpecTrail annotations to this document
- Add matching SpecTrail annotations to the sample code
- Keep the sample small enough to understand at a glance
- Document configuration selection before implementing config loading
