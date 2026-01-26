# SpecTrail

> A specification is a map, and code is a road. SpecTrail is the compass that helps navigate this environment.

SpecTrail links documentation and implementation code through annotations, enabling teams to understand the relationship between specifications and code, and to track whether items are implemented or unimplemented.

## Vision

Writing documentation is one of the best ways to understand a project and reflect on its direction. However, mapping documentation to program code is a challenging task. SpecTrail aims to establish a clear and maintainable link between specifications and source code, reducing divergence and improving traceability.

## Core Features

### 📋 Annotation-Based Linking
- Add annotations to both specification documents and code to explicitly link them
- Support multi-layer abstraction: AbstractAnnotation → SpecDetailAnnotation → ImplementationAnnotation
- Both inline code annotations (comments) and document annotations are supported

### 🔍 Scan & Integrity Checks
- Scan repository for all annotations (documents and code)
- Verify integrity of annotation references (detect broken links)
- Identify unlinked annotations (document-only, code-only, implementation-only)

### 📊 Status Tracking & Reports
- Track implementation status: Implemented / In Progress / Unimplemented / Deprecated / Verified
- Autonomous status estimation based on heuristics (presence of implementation, tests, PR merge status)
- Generate coverage reports showing what percentage of specs have implementation references

### 🔄 PR & Version Tracking
- View annotation diffs in pull requests (added/changed/removed)
- Version annotations to track when they were introduced (commit, PR, author)
- Support PR-level change reporting

## Annotations Guide

### In Code

SpecTrail annotations in code should start with `///` comment lines. Use a structured format with tags to provide clear linking.

#### Annotation Format

```
/// @AbstractAnnotation @name="<name>" @type="<type>" @spec="<spec-id>"
```

#### Key Tags

| Tag        | Description                          | Example Value       |
|------------|--------------------------------------|---------------------|
| `@spec`    | Specification ID linked to the feature or item | `check-command`, `report-ui`, `FR-001` |
| `@type`    | Implementation type                  | `func`, `non-func`, `test`, `infra` |
| `@layer`   | Architectural layer                  | `controller`, `usecase`, `model`, `dao`, `cli`, `service` |

#### Type Definitions

| Type       | Meaning                                                                 |
|------------|-------------------------------------------------------------------------|
| `func`     | Functional implementation (e.g. feature logic, API behavior)            |
| `non-func` | Non-functional code (e.g. model definitions, config handling)           |
| `test`     | Test code related to the specification                                  |
| `infra`    | Infrastructure-related code (e.g. DB access, logging, external services)|

#### Annotation Layers

SpecTrail defines three abstraction layers for organizing annotations:

1. **AbstractAnnotation**: High-level specification (e.g. feature overview, user story)
2. **SpecDetailAnnotation**: Detailed specification item (e.g. API endpoint, validation rule)
3. **ImplementationAnnotation**: Implementation detail (e.g. database model, business logic)

#### Example

```rust
/// @AbstractAnnotation @name="CheckCommand" @type="Feature" @spec="check-command"
/// Analyzes annotations in source code and compares them with specification documents.
fn check_command(args: Vec<String>) {
    /// @SpecDetailAnnotation @id="CHK-001" @name="AnalyzeAnnotations" @type="func" @layer="cli"
    scan_annotations();
    
    /// @ImplementationAnnotation @id="CHK-001-impl" @type="func" @layer="model"
    compare_specs_and_code();
}

```

#### Architectural Layers (customizable per architecture)

| Layer       | Description                                  |
|-------------|----------------------------------------------|
| `controller`| Handles external input (e.g. HTTP requests)  |
| `usecase`   | Application-specific business logic          |
| `entity`    | Core domain models and rules                 |
| `gateway`   | Interfaces to external systems or DB         |

##### Domain-Driven Design (DDD)

| Layer           | Description                                  |
|-----------------|----------------------------------------------|
| `application`   | Coordinates tasks and workflows              |
| `domain`        | Business logic and domain models             |
| `infrastructure`| Technical implementation (DB, messaging)     |
| `interface`     | External interfaces (e.g. REST, CLI)         |

##### MVC / Web App

| Layer       | Description                                  |
|-------------|----------------------------------------------|
| `controller`| Handles user input and routes requests        |
| `model`     | Data and business logic                       |
| `view`      | UI rendering and presentation                 |
| `dao`       | Data access layer                             |

##### Hexagonal Architecture

| Layer       | Description                                  |
|-------------|----------------------------------------------|
| `domain`    | Core business logic                          |
| `port`      | Abstract interfaces                          |
| `adapter`   | Concrete implementations (e.g. REST, DB)     |
| `service`   | Shared utilities or helpers                  |

### In Documents

Specification documents include annotations to define specification items and their context.

#### Document Annotation Format

Use the `@spec:` tag in markdown to mark specification items:

```markdown
## Check Command @spec: check-command

This command analyzes annotations in source code and compares them with specification documents.

- **Type**: func
- **Layer**: cli
- **Status**: Implemented / In Progress / Unimplemented
```

#### Key Attributes

- `@spec`: Specification ID (unique identifier for linking to code)
- **Type**: Same types as code annotations (func, non-func, test, infra)
- **Layer**: Architectural layer
- **Status**: Implementation status (optional; can be auto-detected by scanner)

## Typical Workflow

### 1. Create Specification with Annotations

Add annotations to specification documents (markdown files in `specs/` directory):

```markdown
## Feature: User Authentication @spec: auth-feature

Users should be able to log in securely.

### Login Endpoint @spec: auth-login-endpoint
- **Type**: func
- **Layer**: api
- **Status**: In Progress
```

### 2. Add Code Annotations

Link implementation code using `///` comments:

```rust
/// @SpecDetailAnnotation @id="auth-login-endpoint" @type="func" @layer="controller"
async fn login(req: LoginRequest) -> Result<Token> {
    // Implementation here
}
```

### 3. Run Scanner

Scan repository to find all annotations and verify linking:

```bash
spectrail scan
```

Output: JSON manifest with all annotations, their relationships, and status.

### 4. Generate Reports

View coverage and broken references:

```bash
spectrail report --coverage
spectrail report --broken-refs
```

### 5. Review in PRs

See what specifications changed and their implementation status:

```bash
spectrail diff --pr
```

## Installation & Setup

### Prerequisites
- Python 3.9+
- Rust 1.56+ (for the scanner)

### Local Development

```bash
# Clone the repository
git clone https://github.com/daikihi/SpecTrail.git
cd SpecTrail

# Set up Python virtual environment
python -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install dependencies
pip install -r requirements-dev.txt

# Run tests
pytest

# Build Rust components (if applicable)
cargo build
```

## Project Structure

```
SpecTrail/
├── specify_manual/               # Canonical specification metamodel
│   ├── meta.md                  # Formal definition of SpecTrail annotation model
│   ├── spec.md                  # Core specification
│   ├── memory/                  # Constitution and design principles
│   ├── templates/               # Specification templates
│   └── scripts/                 # Specification management scripts
├── specs/                        # Feature specifications and implementations
│   └── 001-spec-impl-annotations/
│       ├── spec.md              # Feature specification (EN)
│       ├── spec.ja.md           # Feature specification (JA)
│       ├── metamodel.md         # Convenience copy of canonical metamodel
│       ├── tasks.md             # Implementation tasks and checklist
│       ├── quickstart.md        # Getting started guide
│       └── examples/            # Sample manifests for testing
├── src/                         # Source code
│   ├── main.rs
│   ├── cli/                     # CLI commands (scan, report, diff)
│   └── scanner/                 # Scanner and annotation parser
├── tests/                       # Test suite
│   ├── contracts/               # Contract/schema validation tests
│   ├── unit/                    # Unit tests
│   └── integration/             # Integration tests
└── .github/workflows/           # CI/CD pipelines
```

### Key Directories

- **specify_manual/**: Canonical source for the SpecTrail metamodel and annotation specification. This is the authoritative reference for the annotation schema and semantic model.
- **specs/**: Feature-specific specifications and implementation guides. Each feature branch has its own specification directory with tasks, examples, and progress tracking.
- **src/**: Implementation of the scanner, CLI, and core SpecTrail functionality in Rust.
- **tests/**: Comprehensive test coverage including contract tests that validate against the schema.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:
- Adding new annotations
- Extending the scanner
- Improving detection heuristics
- Adding features

## License

See [LICENSE](LICENSE) for details.