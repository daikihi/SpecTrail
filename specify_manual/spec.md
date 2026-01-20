/// [spec-specification] layer: abstract, type: Structure, name: SpecTrail Specification
# SpecTrail Specification

/// [spec-overview] layer: abstract, type: Convention, name: Overview
## 1. Overview

/// [spec-goal] layer: abstract, type: Philosophy, name: Goal of SpecTrail, links: [meta-model-doc]
### 1.1 Goal of SpecTrail

The goal of SpecTrail is supporting for a team of software developers and designers.

Basically, a software specification is written in natural language as this document.

It is difficult to map between software specifications and software implementations.

SpecTrail supports mappig / bridging such gaps.


/// [spec-user-assumption] layer: abstract, type: Guideline, name: User Assumption and Use Case
### 1.2 User Assumption and Use Case

In this subsection, we describe the types of users SpecTrail is designed for and introduce example use cases.

/// [spec-target-users] layer: abstract, type: Convention, name: Target Users
#### Target Users

Software development involves many kinds of engineers. Backend engineers build server-side systems, frontend engineers design user interfaces and user experiences, and many others—such as QA engineers, infrastructure specialists, and technical writers—contribute to the overall process.

SpecTrail is designed to support all of these roles by bridging the gap between specifications and implementation. It helps teams maintain clarity, traceability, and alignment between what is planned and what is built.

In addition, SpecTrail assists project managers in tracking progress and ensuring that development stays aligned with the original specifications.

/// [spec-usecase-examples] layer: abstract, type: Convention, name: Use Case Examples
#### Use Case Examples

In this section, we describe about some use case of SpecTrail.

/// [spec-usecase-spectrail] layer: spec-detail, type: Convention, name: SpecTrail Use Case, links: [spec-usecase-examples]
#### SpecTrail

First use case is SpecTrail project.

SpecTrail project has no visual user interface.  Most user persona is software designer and developer.

At the first time, designer and developer discuss about SpecTrail System.

And then, designer starts to describe a specification about SpecTrail.

Sometime, bouthe of designer and developer makes disscussion about specification.

All engineers repeat this cycle.

Last of a term of a cycle, engineers should discuss about annotation marking.

/// [spec-usecase-webserver] layer: spec-detail, type: Convention, name: Web Server Use Case, links: [spec-usecase-examples]
#### Web Server



/// [spec-system-abstraction] layer: abstract, type: Structure, name: System Abstraction
### 1.3 System Abstraction

/// [spec-components] layer: abstract, type: Structure, name: SpecTrail Components
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

/// [spec-annotation-structure] layer: abstract, type: Structure, name: Annotation Structure Overview, links: [meta-spectrail-annotation]
#### 1.3.2 Annotation Structure Overview

SpecTrail uses a three-layered annotation model to bridge specifications and implementation. Both specification documents and source code are tagged with corresponding annotations to enable traceability and alignment.

SpecTrail annotations are categorized into three types: AbstractAnnotation, SpecDetailAnnotation, and ImplementationAnnotation. Each serves a different level of abstraction and purpose within the system. The details of these annotation categories will be discussed in Section 3: Data Model.

/// [spec-mapping] layer: spec-detail, type: Rule, name: Mapping Specification and Implementation
#### 1.3.3 How to map specification between Specification and Implementation
In this subsection, we decribe about how to map a specification among the documents and the codes.

SpecTrail uses annotation on both of specitifcation documents and implementation codes.
And those annotations should be maped on SpecTrail Engine.
Basically, both of those annotations should be exact same name for readability to engineers.

/// [spec-design-concept] layer: meta, type: Philosophy, name: Design Concept
#### 1.3.4 Design Concept




/// [spec-functional-specs] layer: abstract, type: Structure, name: Functional Specifications
## 2. Functional Specifications

/// [spec-cli] layer: spec-detail, type: Rule, name: Command Line Interface, links: [spec-components]
### 2.1 Command Line Interface

/// [spec-cli-check-command] layer: spec-detail, type: Func, name: check-command, links: [spec-cli]
#### check-command

- **概要**: ソースコードをスキャンして、仕様とコードの対応をチェック
- **入力**: コマンドライン引数（パス、オプション）
- **出力**: レポート（標準出力、ファイル出力）
- **レイヤ**: cli
- **備考**: CIパイプラインから呼び出し可能にする

/// [spec-cli-report-ui] layer: spec-detail, type: Func, name: report-ui, links: [spec-cli]
#### report-ui

- **概要**: 結果をブラウザで可視化
- **入出力**: JSONファイル → Web UI
- **レイヤ**: service / view

/// [spec-annotation-parser] layer: spec-detail, type: Func, name: Annotation Parser, links: [spec-components]
### 2.2 Annotation Parser

/// [spec-parser-code-annotations] layer: spec-detail, type: Func, name: parse-code-annotations, links: [spec-annotation-parser]
#### parse-code-annotations

- **概要**: Rustソースのコメントからアノテーションを抽出
- **アルゴリズム**: syn crate + 正規表現
- **レイヤ**: usecase

/// [spec-parser-spec-docs] layer: spec-detail, type: Func, name: parse-spec-docs, links: [spec-annotation-parser]
#### parse-spec-docs

- **概要**: `.specify/*.md` から仕様IDとメタ情報を抽出
- **レイヤ**: usecase

/// [spec-mapping-engine] layer: spec-detail, type: Func, name: Mapping Engine, links: [spec-components]
### 2.3 Mapping Engine

/// [spec-mapping-match-spec-to-code] layer: spec-detail, type: Func, name: match-spec-to-code, links: [spec-mapping-engine]
#### match-spec-to-code

- **概要**: 仕様とコードの対応をマッピング
- **出力**: マッピング結果の内部表現（HashMap<SpecId, Vec<CodeLocation>>）
- **レイヤ**: model

/// [spec-mapping-generate-report] layer: spec-detail, type: Func, name: generate-report, links: [spec-mapping-engine]
#### generate-report

- **概要**: 差分・警告・孤立コードを出力
- **フォーマット**: text, json
- **レイヤ**: service

/// [spec-non-functional-specs] layer: spec-detail, type: NonFunc, name: Non-functional Requirements
## 3. Non-functional Requirements
- パフォーマンス要件
- 解析対象ソースコードの最大サイズ
- CLIのレスポンス速度目標
- 将来拡張（IDE連携、Graph出力）

/// [spec-data-model] layer: abstract, type: Structure, name: Data Model, links: [meta-specification]
## 4. Data Model
- アノテーション構造体（Rust struct）
- マッピング結果のデータ構造
- JSONスキーマ（もし出力するなら）

/// [spec-open-questions] layer: meta, type: Rule, name: Open Questions
## 5. Open Questions
- ID of naming convention (is it kebab-case?)
- Should we fail CI if unimplemented specifications exist?
- Handling functions with multiple @spec
