/// [@st-manual-spec-specification] layer: abstract, type: Structure, name: SpecTrail Specification
# SpecTrail Specification

/// [@st-manual-spec-overview] layer: abstract, type: Convention, name: Overview
## 1. Overview

/// [@st-manual-spec-goal] layer: abstract, type: Philosophy, name: Goal of SpecTrail, links: [@st-manual-meta-model-doc]
### 1.1 Goal of SpecTrail

The goal of SpecTrail is supporting for a team of software developers and designers.

Basically, a software specification is written in natural language as this document.

It is difficult to map between software specifications and software implementations.

SpecTrail supports mappig / bridging such gaps.


/// [@st-manual-spec-user-assumption] layer: abstract, type: Guideline, name: User Assumption and Use Case
### 1.2 User Assumption and Use Case

In this subsection, we describe the types of users SpecTrail is designed for and introduce example use cases.

/// [@st-manual-spec-target-users] layer: abstract, type: Convention, name: Target Users
#### Target Users

Software development involves many kinds of engineers. Backend engineers build server-side systems, frontend engineers design user interfaces and user experiences, and many others—such as QA engineers, infrastructure specialists, and technical writers—contribute to the overall process.

SpecTrail is designed to support all of these roles by bridging the gap between specifications and implementation. It helps teams maintain clarity, traceability, and alignment between what is planned and what is built.

In addition, SpecTrail assists project managers in tracking progress and ensuring that development stays aligned with the original specifications.

/// [@st-manual-spec-usecase-examples] layer: abstract, type: Convention, name: Use Case Examples
#### Use Case Examples

In this section, we describe about some use case of SpecTrail.

/// [@st-manual-spec-usecase-spectrail] layer: spec-detail, type: Convention, name: SpecTrail Use Case, links: [@st-manual-spec-usecase-examples]
#### SpecTrail

First use case is SpecTrail project.

SpecTrail project has no visual user interface.  Most user persona is software designer and developer.

At the first time, designer and developer discuss about SpecTrail System.

And then, designer starts to describe a specification about SpecTrail.

Sometime, bouthe of designer and developer makes disscussion about specification.

All engineers repeat this cycle.

Last of a term of a cycle, engineers should discuss about annotation marking.

/// [@st-manual-spec-usecase-webserver] layer: spec-detail, type: Convention, name: Web Server Use Case, links: [@st-manual-spec-usecase-examples]
#### Web Server



/// [@st-manual-spec-system-abstraction] layer: abstract, type: Structure, name: System Abstraction
### 1.3 System Abstraction

/// [@st-manual-spec-components] layer: abstract, type: Structure, name: SpecTrail Components
#### 1.3.1 SpecTrail Compoenents

In this Subsection , we describe about what kind of components the SpecTrail contains.

- SpecTrail Batchs ( entry point of the system )
- SpecTrail Reporter

SpecTrail Batchs contains following services.
In this context, the batches are like as CLI.
- 
-


In the future, there are several more tools as following

- SpecTrail Engine
- SpecTrail Server

/// [@st-manual-spec-annotation-structure] layer: abstract, type: Structure, name: Annotation Structure Overview, links: [@st-manual-meta-spectrail-annotation]
#### 1.3.2 Annotation Structure Overview

SpecTrail uses a three-layered annotation model to bridge specifications and implementation. Both specification documents and source code are tagged with corresponding annotations to enable traceability and alignment.

SpecTrail annotations are categorized into three types: AbstractAnnotation, SpecDetailAnnotation, and ImplementationAnnotation. Each serves a different level of abstraction and purpose within the system. The details of these annotation categories will be discussed in Section 3: Data Model.

/// [@st-manual-spec-mapping] layer: spec-detail, type: Rule, name: Mapping Specification and Implementation
#### 1.3.3 How to map specification between Specification and Implementation
In this subsection, we decribe about how to map a specification among the documents and the codes.

SpecTrail uses annotation on both of specitifcation documents and implementation codes.
And those annotations should be maped on SpecTrail Engine.
Basically, both of those annotations should be exact same name for readability to engineers.

/// [@st-manual-spec-design-concept] layer: meta, type: Philosophy, name: Design Concept
#### 1.3.4 Design Concept




/// [@st-manual-spec-functional-specs] layer: abstract, type: Structure, name: Functional Specifications
## 2. Functional Specifications

/// [@st-manual-spec-cli] layer: spec-detail, type: Rule, name: Command Line Interface, links: [@st-manual-spec-components]
### 2.1 Command Line Interface

/// [@st-manual-spec-cli-check-command] layer: spec-detail, type: Func, name: check-command, links: [@st-manual-spec-cli]
#### check-command

- **概要**: ソースコードをスキャンして、仕様とコードの対応をチェック
- **入力**: コマンドライン引数（パス、オプション）
- **出力**: レポート（標準出力、ファイル出力）
- **レイヤ**: cli
- **備考**: CIパイプラインから呼び出し可能にする
 
/// [@st-manual-spec-cli-show-command] layer: spec-detail, type: Func, name: show-command, links: [@st-manual-spec-cli]
#### show-command
 
- **overview**: This command shows or finds annotations from specification documents and/or source code.
- **input**: 
  - `target` (`--target all|document|code|group`):
    - `all`: Show all annotations (scans `src/` and `specify_manual/`).
    - `document`: Show annotations from documents (scans `specify_manual/`).
    - `code`: Show annotations from source code (scans `src/`).
    - `group`: (Future implementation) Group annotations by specific criteria.
  - `mode` (`--mode list|search`):
    - `list`: List all found annotations.
    - `search`: Search for specific annotations (requires `--scope`).
  - `scope` (`--scope <query>`): Search query for `search` mode.
- **output**: reports of annotations (stdout/json)
- **layer**: cli, usecase
- **note**: Show command has annotation filtering and searching capabilities. It provides a way to inspect the state of annotations across the project.
 
#### report-ui

- **概要**: 結果をブラウザで可視化
- **入出力**: JSONファイル → Web UI
- **レイヤ**: service / view

/// [@st-manual-spec-annotation-parser] layer: spec-detail, type: Func, name: Annotation Parser, links: [@st-manual-spec-components]
### 2.2 Annotation Parser

/// [@st-manual-spec-parser-code-annotations] layer: spec-detail, type: Func, name: parse-code-annotations, links: [@st-manual-spec-annotation-parser]
#### parse-code-annotations

- **概要**: Rustソースのコメントからアノテーションを抽出
- **アルゴリズム**: syn crate + 正規表現
- **レイヤ**: usecase

/// [@st-manual-spec-parser-spec-docs] layer: spec-detail, type: Func, name: parse-spec-docs, links: [@st-manual-spec-annotation-parser]
#### parse-spec-docs

- **概要**: `.specify/*.md` から仕様IDとメタ情報を抽出
- **レイヤ**: usecase

/// [@st-manual-spec-mapping-engine] layer: spec-detail, type: Func, name: Mapping Engine, links: [@st-manual-spec-components]
### 2.3 Mapping Engine

/// [@st-manual-spec-mapping-match-spec-to-code] layer: spec-detail, type: Func, name: match-spec-to-code, links: [@st-manual-spec-mapping-engine]
#### match-spec-to-code

- **概要**: 仕様とコードの対応をマッピング
- **出力**: マッピング結果の内部表現（HashMap<SpecId, Vec<CodeLocation>>）
- **レイヤ**: model

/// [@st-manual-spec-mapping-generate-report] layer: spec-detail, type: Func, name: generate-report, links: [@st-manual-spec-mapping-engine]
#### generate-report

- **概要**: 差分・警告・孤立コードを出力
- **フォーマット**: text, json
- **レイヤ**: service

/// [@st-manual-spec-non-functional-specs] layer: spec-detail, type: NonFunc, name: Non-functional Requirements
## 3. Non-functional Requirements
- パフォーマンス要件
- 解析対象ソースコードの最大サイズ
- CLIのレスポンス速度目標
- 将来拡張（IDE連携、Graph出力）

/// [@st-manual-spec-data-model] layer: abstract, type: Structure, name: Data Model, links: [@st-manual-meta-specification]
## 4. Data Model

SpecTrail uses a unified data model for annotations in both documents and source code. This section describes the internal data structures (Rust structs) and their corresponding JSON representations used for CLI output and reporting.

### 4.1 Common Fields
All annotation layers share some common attributes:
- `id`: Unique identifier (e.g., `@st-manual-spec-goal`)
- `name`: Human-readable name
- `type`: Specific category within the layer (e.g., Philosophy, Func)
- `layer`: The layer this annotation belongs to (Meta, Abstract, SpecDetail, Implementation)
- `links`: References to other annotations

### 4.2 Annotation Layers

#### MetaAnnotation
Represents high-level philosophies, guidelines, and rules.
- **Types**: Philosophy, Guideline, Convention, Structure, Rule
- **Links**: Can link to other `MetaAnnotation`s.

#### AbstractAnnotation
Represents conceptual units of the system.
- **Types**: Page, Application, BackgroundComponent, Structure, Convention, Philosophy, Guideline
- **Links**: Links to `SpecDetailAnnotation`s.

#### SpecDetailAnnotation
Represents concrete functional or structural specifications.
- **Types**: Func, NonFunc, Test, Infra, Convention, Rule
- **Links**: Links to `AbstractAnnotation` or `ImplementationAnnotation`.

#### ImplementationAnnotation
Describes technical realization.
- **Types**: DatabaseSchema, DaoRepository, DomainEntity, ExternalApiGateway, WebInterfaceDataModel, Structure
- **Artifact**: Path or identifier of the code artifact (e.g., file path, function name).
- **Status**: Planned, InProgress, Completed.
- **Links**: Links to `SpecDetailAnnotation` or `AbstractAnnotation`.

### 4.3 Container Structures

#### CodeAnnotation / DocumentAnnotation
These are aggregators that group annotations found in a specific file or artifact.
```json
{
  "metas": [ ...MetaAnnotation ],
  "abstracts": [ ...AbstractAnnotation ],
  "details": [ ...SpecDetailAnnotation ],
  "implementations": [ ...ImplementationAnnotation ]
}
```

### 4.4 JSON Schema Example (Show Command Output)
When running `show --mode list --target all`, the output follows this structure:

```json
{
  "document_annotations": [
    {
      "metas": [
        {
          "id": "@st-manual-meta-model-doc",
          "name": "Specification Model: Formal Definition",
          "type": "Philosophy",
          "layer": "Meta",
          "links": [
            {
              "id": "@st-manual-meta-vocabulary",
              "name": "Vocabulary",
              "type": "Convention",
              "layer": "Meta"
            }
          ]
        }
      ],
      "abstracts": [
        {
          "id": "@st-manual-spec-goal",
          "name": "Goal of SpecTrail",
          "type": "Philosophy",
          "layer": "Abstract",
          "links": [
            {
              "id": "@st-manual-spec-cli-show-command",
              "name": "show-command",
              "type": "Func",
              "layer": "SpecDetail"
            }
          ]
        }
      ],
      "details": [
        {
          "id": "@st-manual-spec-cli-show-command",
          "name": "show-command",
          "type": "Func",
          "layer": "SpecDetail",
          "links": [
            {
              "id": "@st-code-use-case-show-show-use-case",
              "name": "ShowUseCase",
              "type": "Structure",
              "layer": "Implementation"
            }
          ]
        }
      ],
      "implementations": [
        {
          "id": "@st-impl-report-json-format",
          "name": "Show Command JSON Output Format",
          "type": "WebInterfaceDataModel",
          "layer": "Implementation",
          "artifact": "specify_manual/command/show/io.md",
          "status": "InProgress",
          "links": [
            {
              "id": "@st-manual-spec-cli-show-command",
              "name": "show-command",
              "type": "Func",
              "layer": "SpecDetail"
            }
          ]
        }
      ]
    }
  ],
  "code_annotations": [
    {
      "metas": [
        {
          "id": "@st-meta-naming-convention",
          "name": "Annotation ID Naming Convention",
          "type": "Convention",
          "layer": "Meta",
          "links": []
        }
      ],
      "abstracts": [
        {
          "id": "@st-abstract-cli",
          "name": "Command Line Interface",
          "type": "Structure",
          "layer": "Abstract",
          "links": [
            {
              "id": "@st-manual-spec-cli-show-command",
              "name": "show-command",
              "type": "Func",
              "layer": "SpecDetail"
            }
          ]
        }
      ],
      "details": [
        {
          "id": "@st-detail-show-list",
          "name": "List annotations (show --mode list)",
          "type": "Func",
          "layer": "SpecDetail",
          "links": [
            {
              "id": "@st-code-use-case-show-show-use-case",
              "name": "ShowUseCase",
              "type": "Structure",
              "layer": "Implementation"
            }
          ]
        }
      ],
      "implementations": [
        {
          "id": "@st-code-use-case-show-show-use-case",
          "name": "ShowUseCase",
          "type": "Structure",
          "layer": "Implementation",
          "artifact": "src/use_case/show/show_use_case.rs",
          "status": "Completed",
          "links": [
            {
              "id": "@st-detail-show-list",
              "name": "List annotations (show --mode list)",
              "type": "Func",
              "layer": "SpecDetail"
            }
          ]
        }
      ]
    }
  ]
}
```

/// [@st-manual-spec-open-questions] layer: meta, type: Rule, name: Open Questions
## 5. Open Questions
- ID of naming convention (is it kebab-case?)
- Should we fail CI if unimplemented specifications exist?
- Handling functions with multiple @spec
