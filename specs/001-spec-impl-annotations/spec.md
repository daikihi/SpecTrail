# Feature Specification: ドキュメントと実装を注釈(Annotation)で紐付ける仕組み

**Feature Branch**: `001-spec-impl-annotations`  
**Created**: 2025-12-30  
**Status**: Draft  
**Input**: User description: "このプロジェクトでは、Document と implementation をAnnotation で結ぶことで、実装と仕様の関係を把握し、実装済み、未実装の有無なども把握できるようにしたい"

---

## Concept / コンセプト

### 1.1 Goal of SpecTrail
SpecTrail の目的は、ソフトウェア開発チーム（デザイナー、開発者、QA、インフラ担当など）を支援し、仕様書（ドキュメント）と実装（コード）の間のギャップを埋めることです。仕様は通常自然言語で記述され、そのままでは実装との整合性を保つのが難しいため、注釈（Annotation）を用いて仕様と実装を結び付け、追跡性と整合性を提供します。

### 1.2 User Assumption and Use Case
**Target Users**: バックエンド/フロントエンド/QA/インフラ/テクニカルライター等、仕様と実装の対応を必要とするあらゆるロールを想定します。プロジェクトマネージャも進捗把握のために本機能を利用します。

**Use Case Examples**:
- SpecTrail プロジェクト自体のように、UI を持たないツールチェーンの設計・開発において、デザイナーと開発者が仕様を議論し、注釈で実装対応を明示するサイクルを支援します。
- Web サービス等の開発では、仕様項目ごとに注釈を付け、実装参照やステータスを追跡することでレビュー・保守を容易にします。

### 1.3 System Abstraction
**1.3.1 SpecTrail Components**
- SpecTrail Batches: CLI ベースのエントリポイント（スキャン、整合性チェック、レポート生成など）。
- SpecTrail Reporter: 注釈カバレッジや参照切れを出力するレポート機能。
- （将来的）SpecTrail Engine / SpecTrail Server: 注釈の集計・検索・同期を行うサービス。

**1.3.2 Annotation Structure Overview**
三層モデル（AbstractAnnotation、SpecDetailAnnotation、ImplementationAnnotation）を用いて、ドキュメント側とコード側の注釈を対応させることでトレースを実現します（詳細はデータモデル節参照）。

**1.3.3 How to map specification between Specification and Implementation**
ドキュメントとコードの双方に注釈を付与し、SpecTrail Engine が同名または紐づけルールに基づいてマッピングを行います。可読性確保のため、注釈名は明確かつ一貫した命名規約を採用することを推奨します。

**1.3.4 Design Concept**
設計方針としては、まずは CLI ベースでの実用的なワークフロー（注釈の追加・スキャン・レポート出力・PR 差分表示）を MVP とし、将来的にサーバ/サービス化やエディタ統合を図る方針です。

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 開発者が仕様項目の実装状況をすばやく確認したい (Priority: P1)

開発者は仕様書の各項目に対して、どの実装が紐づいているか、現在の実装ステータス（実装済み／未実装／進行中 等）を一覧で確認したい。

**Why this priority**: 日々の開発・レビューで最も直接的に価値を生むため。仕様と実装のズレを早期に発見できる。 

**Independent Test**: リポジトリの仕様一覧ページで「未実装」フィルタを適用し、実装が存在しない項目のみが表示されることを確認する。

**Acceptance Scenarios**:

1. **Given** リポジトリに注釈された仕様が存在する、 **When** 開発者が「未実装」フィルタを適用する、 **Then** 未実装の仕様のみが一覧に表示される。
2. **Given** 仕様項目に実装参照がある、 **When** 開発者が参照リンクをクリックする、 **Then** 対応する実装ファイル/位置に移動できる（IDE/エディタまたはブラウザで開ける）。

---

### User Story 2 - レビュアーが仕様と実装のマッピングをレビューしたい (Priority: P2)

レビュアーはプルリクエストや変更セット内の注釈を使って、追加/変更された仕様とその実装を検証できるようにしたい。

**Why this priority**: 品質向上と変更差分の追跡に有用。レビュー時間短縮に寄与する。

**Independent Test**: PR の注釈一覧で、変更された仕様に対して参照先実装が正しく紐付いているかを確認する。

**Acceptance Scenarios**:

1. **Given** PR に仕様注釈が含まれる、 **When** レビュアーが注釈一覧を確認する、 **Then** 注釈に対する実装参照（ファイル:行）または未実装のフラグが表示される。

---

### User Story 3 - メンテナが注釈の網羅率を把握したい (Priority: P3)

メンテナは仕様に対する注釈のカバレッジ（何％の仕様に実装参照があるか）や未実装項目の一覧を定期的に確認したい。

**Why this priority**: 技術的負債の可視化と優先度付けに役立つ。

**Independent Test**: レポート機能で「注釈カバレッジ」を出力し、総仕様数に対する注釈付き仕様数の割合を検証する。

**Acceptance Scenarios**:

1. **Given** リポジトリに仕様が100項目ある、 **When** メンテナがカバレッジレポートを実行する、 **Then** 何件が注釈され実装参照を持つかが数値として返される。

---

### Edge Cases

- 仕様側の行／ID が変更され、注釈の参照が古くなっている場合にどう扱うか（警告表示、再リンク提案など）。
- 同一仕様に複数の実装参照（複数言語や複数実装パス）がある場合の取り扱い。
- 大規模なリファクタで参照先が移動した場合の検知と更新方針。

---

## Requirements *(mandatory)*

### Functional Requirements (すべてテスト可能に記述)

- **FR-001**: 開発者が仕様（ドキュメント）の任意の項目に対して注釈（Annotation）を追加できること。
  - **Acceptance**: 仕様項目に注釈を追加後、一覧で該当項目が注釈付きとして表示される。

- **FR-002**: 注釈は**ソースコード内のインライン注釈（コメント/属性）とドキュメント内の注釈（本スペックに示した形式）の双方をサポートすること**（段階的に導入して両方を扱えることを目標とする）。
  - **Acceptance**:
    - ドキュメント注釈（DocumentAnnotation）とコード注釈（CodeAnnotation）が同じ正規化された識別子を持つ場合、それらは自動的にリンクされ、一覧／詳細画面で併記される。
    - どちらか一方のみ存在する（ドキュメントのみ／コードのみ）のケースは明示的に表示され、フィルタ可能である（例: 「ドキュメントのみ」「コードのみ」「未実装」）。
    - スキャン実行により「未リンク注釈（ドキュメントのみ／コードのみ）」の一覧と、同一識別子でも競合情報がある場合は手動レビュー用にフラグが出力される。

- **FR-003**: 注釈は**多段階のステータスモデル（例: 実装済み / 進行中 / 未実装 / 廃止 / 検証済み）を持ち、SpecTrail のバッチ実行による自律検出（ステータス推定）をサポートすること**。
  - **Acceptance**:
    - バッチ（スキャン）実行により、システムは注釈に対する「推定ステータス」を出力できる（例: 実装参照の有無、関連テストの存在、PR マージ状況等に基づく推定）。
    - 推定結果は一覧とレポートで「推定ステータス」として表示され、ユーザーは手動で最終ステータスを承認・変更できる。
    - 自動推定で重大な不整合（例: 参照切れ、同一識別子での競合）が検出された場合、アラート/フラグが出力され、詳細確認のための一覧に含まれる。

- **FR-004**: リポジトリをスキャンして注釈の整合性チェック（参照先が存在するかの検証）を行い、レポートとして出力できること。
  - **Acceptance**: 整合性チェック実行後、参照切れが検出されれば一覧に「参照切れ」として表示される。

- **FR-005**: 注釈の検索・フィルタ（ステータス、仕様タグ、ファイル、未実装など）による一覧表示ができること。
  - **Acceptance**: 例えば「未実装」をフィルタすると未実装の行のみが返る。

- **FR-006**: 注釈はバージョン管理でき、PR単位で追加/変更された注釈の差分を確認できること。
  - **Acceptance**: PR に追加された注釈が差分として表示される。

### Key Entities *(include if feature involves data)*

- **Specification Item**: ドキュメントや仕様の個別項目（ID, タイトル, 所属ドキュメント, 範囲）
- **Annotation**: 仕様項目に紐づく注釈（注釈ID, 対象仕様ID, 実装参照／未実装フラグ, ステータス, メタデータ）
- **Implementation Reference**: 実装を指す参照（ファイルパス, 行番号, シンボル名, リポジトリURL 等）
- **Report / Coverage**: 注釈の網羅率や参照切れ等の集計データ

---

## Success Criteria *(mandatory, measurable & technology-agnostic)*

- **SC-001**: 開発者が一覧で「未実装」項目を取得できる（フィルタ時間は体感で遅くないこと、目安: 2秒以内に結果が返ること）。
- **SC-002**: レポートで注釈カバレッジが数値として出力される（例: 注釈付き項目数 / 総項目数）。
- **SC-003**: 整合性チェックで参照切れを検出でき、重大な参照切れは一覧で可視化される。
- **SC-004**: PR レビュー時に注釈差分が確認でき、レビューが行える（レビュープロセスで注釈を参照したうえでの承認が可能）。

---

## Assumptions

- 初期はこのリポジトリの主要言語/ドキュメント形式（例: Markdown、プロジェクトの主たる言語）にフォーカスする。将来的に多言語対応を検討する。
- 注釈は人が追加することを想定するが、簡易な自動検出（コードベースのクラス/関数名と仕様のマッチ）を補助機能として想定する。

---

## Specification Model: Formal Definition

以下はサンプルの形式定義（Formal Definition）です。スペックの構造を厳密に記述し、ドキュメントと実装注釈の関係を数学的/論理的に表現します。

/// @MetaAnnotation @MetaName="Definition of SpecTrailUnit" @MetaType=Philosophy
### 1.1 SpecTrailUnit
Let SpecTrailUnit = { CodeAnnotation, DocumentAnnotation }

CodeAnnotation is an entity of SpecTrail annotation written in actual programming code.  
DocumentAnnotation is an entity of SpecTrail annotation written in specification documents.

SpecTrailUnit represents a pair of annotations—one from code, one from documentation—  
that together form a traceable unit within the SpecTrail system.

/// @MetaAnnotation @MetaName="Definition of SpecTrailAnnotation" @MetaType=Philosophy
### 1.2 SpecTrailAnnotation
SpecTrail defines two complementary annotation spaces:

Let  
SpecTrailUnit = { DocumentAnnotation, CodeAnnotation }

Both represent collections of annotations that share a common structural schema
(MetaAnnotation, AbstractAnnotation, SpecDetailAnnotation),
but exist in different ontological domains.

- **DocumentAnnotation** exists in the *specification domain* —  
  annotations written within natural language or semi-structured documents.

- **CodeAnnotation** exists in the *implementation domain* —  
  annotations embedded within programming code or related metadata.

Formally:

```
DocumentAnnotation = { Mᴰ, Aᴰ, Dᴰ }
CodeAnnotation = { Mᶜ, Aᶜ, Dᶜ }
```

where each set corresponds to the three core layers of annotation structure:

- **MetaAnnotation (M)** — expresses design principles and conventions.  
- **AbstractAnnotation (A)** — expresses conceptual or domain-level intent.  
- **SpecDetailAnnotation (D)** — expresses concrete functional or structural specifications.

Each of these annotation types shares a *common schema* across spaces,  
but instances differ because they belong to distinct representation domains (textual vs code).

---

#### 1.2.1 Structural Mapping

A **Trace relation** establishes correspondence between DocumentAnnotation and CodeAnnotation.  
That is, each annotation in DocumentAnnotation may have one or more semantic counterparts in CodeAnnotation.

```
∀ aᴰ ∈ DocumentAnnotation,
∃ aᶜ ∈ CodeAnnotation such that Trace(aᴰ, aᶜ)
```

The mapping is **not required to be one-to-one**;  
it allows partial, compositional, or derived mappings to represent real-world divergence  
between written specifications and implemented systems.

---

#### 1.2.2 Philosophical Note

The DocumentAnnotation and CodeAnnotation spaces are *isomorphic by structure* but *distinct by substance*.

They share the same logical annotation model but live in different ontological strata:
- The **Document space** describes *what is intended*.
- The **Code space** describes *what exists*.

SpecTrail does not collapse these into a single ontology.  
Instead, it maintains both and enforces structural symmetry and semantic traceability between them.


/// @MetaAnnotation @MetaName="Definition of MetaType" @MetaType=Philosophy
#### 1.2.3 MetaAnnotation
MetaAnnotation is an annotation used to describe design directions, naming rules, and system management principles.  
It does not define features directly, but supports the structure and philosophy behind specification design.

In most cases, MetaAnnotations do not appear in programming code.  
However, they help readers of specification documents understand why certain specifications are not reflected in the source code.

```
M = {m₁, m₂, ..., mₙ} is a finite set of all possible MetaAnnotations.

∀m ∈ M:  
MetaName is a finite set of identifiers for MetaAnnotations.  
MetaType is a finite set of conceptual categories.  
m = {n, t}, where n ∈ MetaName, t ∈ MetaType.
```

MetaType can take the following values: `Philosophy`, `Guideline`, `Convention`, `Structure`, `Rule`.

/// @MetaAnnotation @MetaName="Definition of AbstractAnnotation" #MetaType=Philosophy
#### 1.2.4 AbstractAnnotation
AbstractAnnotation defines the high-level concept of the target software.  
This concept answers questions like:  

- Why does the team want to create the software?
- What kind of use cases or user needs are being prioritized?

In the context of a web service, an AbstractAnnotation often corresponds to a single web page or screen-level concept.  
It represents the overall purpose or user-facing role of that page within the system.

In addition, an AbstractAnnotation may also describe what kinds of components are involved—such as APIs, batch processes, or background jobs.

An AbstractAnnotation may contain several SpecDetailAnnotations.  
These detail annotations emerge when the abstract concept is split into sub-specifications—similar to how a task is broken down into subtasks.

Each AbstractAnnotation must include a `@name` tag to identify its concept.  
We recommend using a consistent naming convention—such as `CamelCase` or screen-level identifiers (e.g., `UserAuthPage`, `ProductListView`)—to ensure clarity and traceability across the project.

We will define SpecDetailAnnotation separately,  
but it's important to note that each SpecDetailAnnotation is structurally linked to an AbstractAnnotation via `@spec`.

```
A = {a₁, a₂, ..., aₙ} is a finite set of AbstractAnnotations.

For each a ∈ A:  
a = {na, ta, link}, where  
- na ∈ AbstractName  
- ta ∈ AbstractType  
- link ⊆ SpecDetailAnnotation

Here,

na identifies the abstract concept.

ta represents its type, such as domain concept, use case, or system role.

link connects this abstract concept to one or more SpecDetailAnnotations that concretize it.
```

/// @MetaAnnotation @MetaName="Definition of SpecDetailAnnotation" #MetaType=Philosophy
#### 1.2.5 SpecDetailAnnotation
SpecDetailAnnotation defines a specific behavior or functional aspect derived from an AbstractAnnotation.  
It represents a concrete specification that guides implementation.

In the context of a web service,  
a SpecDetailAnnotation often corresponds to an API specification, a user interaction flow, or a page-level behavior.

In the context of a one-shot batch process, a SpecDetailAnnotation often corresponds to a batch specification such as loading master data from a filesystem and inserting it into a database.

```
D = {d₁, d₂, ..., dₖ} is a finite set of SpecDetailAnnotations.

For each d ∈ D:  
d = {nd, td, link}, where  
- nd ∈ SpecDetailName  
- td ∈ SpecDetailType  
- link ⊆ {AbstractAnnotation ∪ ImplementationAnnotation}

Here,

nd identifies the detailed specification.

td specifies its type, such as entity, relation, operation, or rule.

link connects this detail both upward (to AbstractAnnotation) and downward (to ImplementationAnnotation), forming a bidirectional specification trace.
```

/// @MetaAnnotation @MetaName="Definition of ImplementationAnnotation" #MetaType=Philosophy
#### 1.2.6 ImplementationAnnotation

ImplementationAnnotation defines a concrete implementation-level specification that realizes a particular SpecDetailAnnotation.
It provides the semantic bridge between specification and source code, representing how a detailed specification is technically realized.

Unlike SpecDetailAnnotation, which describes what should be done,
ImplementationAnnotation focuses on the conceptual role of the implementation rather than its physical code location or language.
Information such as file paths or programming languages is managed separately by CodeAnnotation and connected through trace relations.

A single SpecDetailAnnotation may correspond to multiple ImplementationAnnotations,
each describing a distinct implementation aspect (e.g., database access, repository design, gateway integration).

ImplementationAnnotation is typically used to express:

Database-related implementation semantics (tables, columns, constraints)

Design of data access layers (DAO modules)

Structure definitions for domain entities

Repository or gateway interface specifications

Web API design and data model definitions

Any other conceptually distinct technical realization of a specification

```
I = {i₁, i₂, ..., iₗ} is a finite set of ImplementationAnnotations.

For each i ∈ I:  
i = {ni, ti, link, art, status}, where  
- ni ∈ ImplementationSpecName  
- ti ∈ ImplementationType  
- link ⊆ {SpecDetailAnnotation ∪ AbstractAnnotation}
- art ∈ ImplementationArtifact  
- status ∈ ImplementationStatus
```

Here,
- ni identifies the ImplementationAnnotation.

- ti classifies its role, such as DAO, Repository, Gateway, or API.

- link associates this implementation with its related SpecDetailAnnotation(s) and AbstractAnnotation(s).

- art defines the semantic target of the implementation, such as database, domain, external_system, or web_interface.

- status represents the implementation’s maturity or verification state, such as draft, implemented, or verified.

#### 1.2.7 Annotation Trace
In combination with link references, Traces form the formal mapping between conceptual, detailed, and implementation layers, ensuring full bidirectional traceability within the SpecTrail system.

```
T = {t₁, t₂, ..., tₘ} is a finite set of Traces.

For each t ∈ T:  
t = {src, dst, kind}, where  
- src ∈ {A ∪ D ∪ I}  
- dst ∈ {A ∪ D ∪ I}  
- kind ∈ TraceKind
```

Here,

src and dst denote the source and destination of the trace link.

kind indicates the semantics of the relationship (e.g., refines, implements, verifies, derives).
Traces thus form the structural backbone of the SpecTrail, enabling complete bidirectional traceability across all layers.

/// @MetaAnnotation @MetaName="SpecDetailType Vocabulary" #MetaType=Structure
#### 2.1.1 SpecDetailType

SpecDetailType defines the structural category of a SpecDetailAnnotation.  
It helps clarify what kind of implementation or behavior the detail refers to.

Available types include:

- `func`: Functional specification—describes logic, behavior, and expected outcomes.  
- `non-func`: Non-functional specification—covers static structures such as enums, data types, and configuration schemas.  
- `test`: Test specification—defines validation logic, test cases, and expected assertions.  
- `infra`: Infrastructure specification—includes database schemas, gateways, file formats, and system-level configurations.

---

## Open Questions / [NEEDS CLARIFICATION]

- **Q1 (FR-002)**: 解決 — 注釈はソース内のインライン注釈とドキュメント注釈の**双方をサポート**する方針に決定しました（段階的導入可）。

- **Q2 (FR-003)**: 解決 — ステータスは多段階（例: 実装済み / 進行中 / 未実装 / 廃止 / 検証済み）とし、SpecTrail のバッチによる自律検出（推定ステータス出力）をサポートする方針に決定しました。

---

## Next steps

1. 上記の **Q1/Q2** の選択をお願いします。回答があれば仕様を確定して実装タスク分解に進みます。
2. 優先度 P1 を実装するための最小の実装案（MVP）を定義して見積もりを作成します。

```