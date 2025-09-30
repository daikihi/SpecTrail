# SpecTrail
A specification is a map, and code is a road. SpecTrail is the compass that helps navigate this environment.

Writing documentation is one of the best ways to understand a project and reflect on its direction.  
However, mapping documentation to program code is a challenging task.  
SpecTrail aims to establish a clear and maintainable link between documentation and source code.

## Features

- SpecTrail analyzes specification documents in the `.specify/` directory and evaluates their structure and content.
- SpecTrail scans source code to find embedded annotations.
- SpecTrail maps and analyzes both specification annotations and code annotations, identifying unimplemented functions and undocumented features.

## annotations

### in code 
SpecTrail annotation should start with `///` comment line.

#### Annotation for Specification Abstraction Layer

SpecTrail defines a three-layered annotation model to represent different levels of specification abstraction. This model helps organize and trace the relationship between high-level concepts and low-level implementation.

1. AbstractAnnotation
Represents high-level specifications, such as the overall purpose or behavior of a feature. In a web service, this might correspond to a screen or page. In a microservice architecture, it could describe the role of a service. An AbstractAnnotation typically links to multiple SpecDetailAnnotations.

2. SpecDetailAnnotation
Describes more specific aspects of a feature, breaking down an AbstractAnnotation into smaller parts. This includes API endpoints used by a page, UI behaviors, or validation rules. Each SpecDetailAnnotation may link to several ImplementationAnnotations.

3. ImplementationAnnotation
Captures implementation-level details such as database models, domain logic, use cases, repositories, and gateways. This layer connects the specification directly to the code structure and is used when deeper traceability is needed.


#### Annotation Types

SpecTrail supports structured annotations to describe the role and context of each code fragment.  
Each annotation consists of three parts: `@spec`, `@type`, and `@layer`.

| Tag        | Description                          | Example Value       |
|------------|--------------------------------------|---------------------|
| `@spec`    | Specification ID linked to the feature or task | `check-command`, `report-ui` |
| `@type`    | Type of the implementation           | `func`, `non-func`, `test`, `infra` |
| `@layer`   | Architectural layer of the code      | `controller`, `usecase`, `model`, `dao`, `cli`, `service` |

---

#### Type Definitions

| Type       | Meaning                                                                 |
|------------|-------------------------------------------------------------------------|
| `func`     | Functional implementation (e.g. feature logic, API behavior)            |
| `non-func` | Non-functional code (e.g. model definitions, config handling)           |
| `test`     | Test code related to the specification                                  |
| `infra`    | Infrastructure-related code (e.g. DB access, logging, external services)|

---

#### Layer Examples (customizable per architecture)

SpecTrail allows flexible layer tagging depending on the architecture of your project.  
Below are common examples across various architectural styles.

---

##### 🧭 Clean Architecture

| Layer       | Description                                  |
|-------------|----------------------------------------------|
| `controller`| Handles external input (e.g. HTTP requests)  |
| `usecase`   | Application-specific business logic          |
| `entity`    | Core domain models and rules                 |
| `gateway`   | Interfaces to external systems or DB         |

---

##### 🧱 Domain-Driven Design (DDD)

| Layer           | Description                                  |
|-----------------|----------------------------------------------|
| `application`   | Coordinates tasks and workflows              |
| `domain`        | Business logic and domain models             |
| `infrastructure`| Technical implementation (DB, messaging)     |
| `interface`     | External interfaces (e.g. REST, CLI)         |

---

##### 🧪 MVC / Web App

| Layer       | Description                                  |
|-------------|----------------------------------------------|
| `controller`| Handles user input and routes requests        |
| `model`     | Data and business logic                       |
| `view`      | UI rendering and presentation                 |
| `dao`       | Data access layer                             |

---

##### 🛠 Hexagonal Architecture

| Layer       | Description                                  |
|-------------|----------------------------------------------|
| `domain`    | Core business logic                          |
| `port`      | Abstract interfaces                          |
| `adapter`   | Concrete implementations (e.g. REST, DB)     |
| `service`   | Shared utilities or helpers                  |

---

##### 🌀 Others (customizable)

| Layer       | Description                                  |
|-------------|----------------------------------------------|
| `cli`       | Command-line interface layer                  |
| `test`      | Test-related code                             |
| `infra`     | Infrastructure setup and configuration        |
| `config`    | Configuration definitions                     |



### in document
Specification documents in `.specify/` can include annotations to define feature IDs and their context.  
These annotations help SpecTrail link documentation to corresponding code fragments.

#### Example

```markdown
#### Check Command `@spec: check-command`

This command analyzes annotations in source code and compares them with specification documents.  
It reports missing implementations, undocumented features, and potential duplicates.

- Type: func
- Layer: cli
```