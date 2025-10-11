# SpecTrail Specification

## 1. Overview
- プロジェクトの目的
- 想定する利用者・ユースケース
- 全体構成図（Mermaidの簡単な図）

### 1.1 Goal of SpecTrail
The goal of SpecTrail is supporting for a team of software developers and designers.

Basically, a software specification is written in natural language as this document.

It is difficult to map between software specifications and software implementations.

SpecTrail supports mappig / bridging such gaps.


### 1.2 User Assumption and Use Case
In this subsection, we describe the types of users SpecTrail is designed for and introduce example use cases.

#### Target Users
Software development involves many kinds of engineers. Backend engineers build server-side systems, frontend engineers design user interfaces and user experiences, and many others—such as QA engineers, infrastructure specialists, and technical writers—contribute to the overall process.

SpecTrail is designed to support all of these roles by bridging the gap between specifications and implementation. It helps teams maintain clarity, traceability, and alignment between what is planned and what is built.

In addition, SpecTrail assists project managers in tracking progress and ensuring that development stays aligned with the original specifications.

#### Use Case Examples
In this section, we describe about some use case of SpecTrail.

#### SpecTrail
First use case is SpecTrail project.

SpecTrail project has no visual user interface.  Most user persona is software designer and developer.

At the first time, designer and developer discuss about SpecTrail System.

And then, designer starts to describe a specification about SpecTrail.

Sometime, bouthe of designer and developer makes disscussion about specification.

All engineers repeat this cycle.

Last of a term of a cycle, engineers should discuss about annotation marking.

#### Web Server



### 1.3 System Abstraction

In this subsection, we describe the abstraction of SpecTrail's system model.

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

#### 1.3.2 Annotation Structure Overview

SpecTrail uses a three-layered annotation model to bridge specifications and implementation. Both specification documents and source code are tagged with corresponding annotations to enable traceability and alignment.

SpecTrail annotations are categorized into three types: AbstractAnnotation, SpecDetailAnnotation, and ImplementationAnnotation. Each serves a different level of abstraction and purpose within the system. The details of these annotation categories will be discussed in Section 3: Data Model.

#### 1.3.3 How to map specification between Specification and Implementation
In this subsection, we decribe about how to map a specification among the documents and the codes.

SpecTrail uses annotation on both of specitifcation documents and implementation codes.
And those annotations should be maped on SpecTrail Engine.
Basically, both of those annotations should be exact same name for readability to engineers.

#### 1.3.4 Design Concept




## 2. Functional Specifications

### 2.1 Command Line Interface
#### check-command `@spec: check-command`
- **概要**: ソースコードをスキャンして、仕様とコードの対応をチェック
- **入力**: コマンドライン引数（パス、オプション）
- **出力**: レポート（標準出力、ファイル出力）
- **レイヤ**: cli
- **備考**: CIパイプラインから呼び出し可能にする

#### report-ui `@spec: report-ui`
- **概要**: 結果をブラウザで可視化
- **入出力**: JSONファイル → Web UI
- **レイヤ**: service / view

### 2.2 Annotation Parser
#### parse-code-annotations `@spec: parse-code-annotations`
- **概要**: Rustソースのコメントからアノテーションを抽出
- **アルゴリズム**: syn crate + 正規表現
- **レイヤ**: usecase

#### parse-spec-docs `@spec: parse-spec-docs`
- **概要**: `.specify/*.md` から仕様IDとメタ情報を抽出
- **レイヤ**: usecase

### 2.3 Mapping Engine
#### match-spec-to-code `@spec: match-spec-to-code`
- **概要**: 仕様とコードの対応をマッピング
- **出力**: マッピング結果の内部表現（HashMap<SpecId, Vec<CodeLocation>>）
- **レイヤ**: model

#### generate-report `@spec: generate-report`
- **概要**: 差分・警告・孤立コードを出力
- **フォーマット**: text, json
- **レイヤ**: service

## 3. Non-functional Requirements
- パフォーマンス要件
- 解析対象ソースコードの最大サイズ
- CLIのレスポンス速度目標
- 将来拡張（IDE連携、Graph出力）

## 4. Data Model
- アノテーション構造体（Rust struct）
- マッピング結果のデータ構造
- JSONスキーマ（もし出力するなら）

## 5. Open Questions
- IDの命名規則（kebab-caseか？）
- 未実装仕様をCIでfailさせるか？
- 複数@specを持つ関数の扱い
