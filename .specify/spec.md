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


### 1.2 User Assamption and Use Case

### 1.3 System Abstraction

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
