# 機能仕様書: 注釈でドキュメントと実装を紐付ける仕組み

**Feature Branch**: `001-spec-impl-annotations`  
**作成日**: 2025-12-30  
**Status**: Draft  
**Input**: ユーザー説明: "このプロジェクトは、ドキュメントと実装を注釈で結び、チームが仕様とコードの関係を理解し、項目が実装済みか未実装かを追跡できるようにすることを目的とします。"

---

/// @MetaAnnotation @MetaName="Spec-Impl-Annotations Feature" @MetaType=Philosophy
/// @AbstractAnnotation @name="SpecTrailFeature" @type="Feature" @spec="specs/001-spec-impl-annotations/spec.md"
/// @SpecDetailAnnotation @id="FR-ROOT" @name="FeatureTrace" @type="func" @meta="feature-level"



/// @AbstractAnnotation @name="Concept" @type="Overview"
## コンセプト

/// @AbstractAnnotation @name="Goal" @type="Goal"
### 1.1 SpecTrail の目的
SpecTrail の目的は、ソフトウェア開発チーム（デザイナー、開発者、QA、インフラ/運用など）を支援し、仕様ドキュメントと実装（コード）とのギャップを埋めることです。仕様は通常自然言語で記述され、実装と乖離しがちです。注釈を使って仕様と実装を結びつけることで、追跡性と整合性を提供します。

### 1.2 想定ユーザーとユースケース
**想定ユーザー**: バックエンド/フロントエンド開発者、QA、インフラ/運用、テクニカルライター、仕様と実装の対応が必要なあらゆるロール。プロジェクトマネージャも進捗把握のために利用します。

**ユースケース例**:
- UI を持たないツールチェーン（SpecTrail 自身など）では、デザイナーと開発者が仕様を議論し、注釈で実装意図をマークするサイクルを支援します。
- Web サービスの開発では、仕様項目に注釈を付け、実装参照やステータスを追跡してレビューや保守を簡素化します。

### 1.3 システム抽象
**1.3.1 SpecTrail コンポーネント**
- SpecTrail Batches: CLI ベースのエントリポイント（スキャン、整合性チェック、レポート生成など）。
- SpecTrail Reporter: 注釈カバレッジや参照切れのレポート機能。
- （将来）SpecTrail Engine / SpecTrail Server: 注釈の集計、検索、同期のためのサービス。

**1.3.2 注釈構造の概要**
3 層モデル（AbstractAnnotation、SpecDetailAnnotation、ImplementationAnnotation）を使って、ドキュメント側とコード側の注釈をマッピングし、トレース可能にします（詳細はデータモデル節参照）。

**1.3.3 仕様と実装のマッピング方法**
ドキュメントとコードの双方に注釈を付与し、SpecTrail Engine は名前や設定された紐付けルールに基づいてマッピングします。可読性確保のため、注釈名は明確で一貫した命名規約に従うことを推奨します。

**1.3.4 設計方針**
MVP としては実用的な CLI ベースのワークフロー（注釈の追加、スキャン、レポート出力、PR 差分表示）を優先し、将来的にサーバ/サービス化やエディタ統合を検討します。

---

## ユーザーシナリオとテスト *(必須)*

### ユーザーストーリー 1 - 開発者が仕様項目の実装状況を素早く確認したい (優先度: P1)

開発者は各仕様項目に紐づく実装と、その実装ステータス（例: 実装済み/未実装/進行中）を一覧で確認したい。

**なぜこの優先度か**: 日々の開発・レビューで高い価値を生み、仕様と実装の不整合を早期に検出できるため。

**独立した確認方法**: リポジトリの仕様一覧ページで「未実装」フィルタを適用し、実装がない項目のみが表示されることを確認する。

**受け入れシナリオ**:
1. **前提** リポジトリに注釈付き仕様が存在する、 **操作** 開発者が「未実装」フィルタを適用する、 **期待** 未実装の仕様のみが一覧に表示される。
2. **前提** 仕様項目に実装参照が存在する、 **操作** 開発者が参照リンクをクリックする、 **期待** 対応するファイル/行が IDE/エディタまたはブラウザで開く。

---

### ユーザーストーリー 2 - レビュアーが仕様と実装のマッピングをレビューしたい (優先度: P2)

レビュアーは PR や変更セット内の注釈を利用して、追加/変更された仕様とその実装を検証できるべきです。

**なぜこの優先度か**: 品質向上と変更差分の追跡に有用で、レビュー時間を短縮します。

**独立した確認方法**: PR の注釈一覧にて、変更された仕様が正しい実装参照に紐付いていることを確認する。

**受け入れシナリオ**:
1. **前提** PR に仕様注釈が含まれる、 **操作** レビュアーが注釈一覧を確認する、 **期待** 各注釈に実装参照（ファイル:行）または未実装のフラグが表示される。

---

### ユーザーストーリー 3 - メンテナが注釈の網羅率を把握したい (優先度: P3)

メンテナは注釈のカバレッジ（何％の仕様に実装参照があるか）や未実装項目の一覧を定期的に確認したい。

**なぜこの優先度か**: 技術的負債の可視化と優先度付けに役立つため。

**独立した確認方法**: カバレッジレポートを実行し、注釈付き項目数 / 総項目数の割合を検証する。

**受け入れシナリオ**:
1. **前提** リポジトリに仕様が100項目ある、 **操作** メンテナがカバレッジレポートを実行する、 **期待** 注釈付きかつ実装参照を持つ項目数がレポートとして返される。

---

### エッジケース

- 仕様側の行や ID が変更され、注釈参照が古くなった場合の扱い（警告、再リンク提案など）。
- 同一仕様に複数の実装参照（複数言語や複数パス）がある場合の扱い。
- 大規模なリファクタで参照先が移動した場合の検知と更新方針。

---

## 要件 *(必須)*

### 機能要件（すべてテスト可能に記述）

/// @SpecDetailAnnotation @id="FR-001" @name="AddAnnotation" @type="func" @spec_section="User Scenarios"
- **FR-001**: 開発者は任意の仕様項目（ドキュメント）に注釈を追加できること。
  - **受け入れ**: 仕様項目に注釈を追加した後、その項目が一覧で注釈付きとして表示される。

/// @SpecDetailAnnotation @id="FR-002" @name="SupportDocumentAndCodeAnnotations" @type="func" @spec_section="Requirements"
- **FR-002**: 注釈はインラインコード注釈（コメント／属性）とドキュメント注釈（本スペックに示した形式）の両方をサポートすること（段階的導入を許容）。
  - **受け入れ**:
    - DocumentAnnotation と CodeAnnotation が同じ正規化された識別子を共有する場合、自動的にリンクされ、一覧／詳細で併記される。
    - 片側のみ存在する場合（ドキュメントのみ／コードのみ）は明示的に表示され、フィルタ可能（例: "document-only", "code-only", "unimplemented"）。
    - スキャンの実行により "unlinked annotations"（ドキュメントのみ／コードのみ）の一覧が生成され、同一識別子の競合は手動レビュー用にフラグされる。

/// @SpecDetailAnnotation @id="FR-003" @name="MultiStageStatusWithBatchDetection" @type="infra" @spec_section="Requirements"
- **FR-003**: 注釈は多段階ステータスモデル（例: Implemented / In Progress / Unimplemented / Deprecated / Verified）をサポートし、バッチベースの自律検出（推定ステータス）に対応すること。
  - **受け入れ**:
    - バッチ（スキャン）実行時に、システムは注釈に対する "estimated status" を出力できる（例: 実装参照、関連テスト、PR マージ状況に基づく）。
    - 推定結果は一覧／レポートで "estimated status" として表示され、ユーザーは手動で最終ステータスを承認または変更できる。
    - 自動推定が重大な不整合（参照切れ、識別子の競合など）を検出した場合、アラート／フラグが出力され、詳細確認リストに含まれる。

- **FR-004**: リポジトリをスキャンして注釈の整合性チェック（参照先が存在するかの検証）を行い、レポートとして出力できること。
  - **受け入れ**: 整合性チェック後、参照切れがあれば "broken reference" として一覧に表示される。

- **FR-005**: 注釈を検索・フィルタ（ステータス、仕様タグ、ファイル、未実装など）できること。
  - **受け入れ**: 例: "Unimplemented" をフィルタした場合、未実装の項目のみが返る。

- **FR-006**: 注釈はバージョン管理が可能で、PR ごとの追加／変更注釈の差分を確認できること。
  - **受け入れ**: PR に追加された注釈は差分として表示される。

### 主要エンティティ（データを扱う場合）

- **Specification Item**: ドキュメント内の個々の仕様項目（ID、タイトル、所属ドキュメント、範囲）
- **Annotation**: 仕様項目に付与される注釈（注釈 ID、対象仕様 ID、実装参照／未実装フラグ、ステータス、メタデータ）
- **Implementation Reference**: 実装を指す参照（ファイルパス、行番号、シンボル名、リポジトリ URL など）
- **Report / Coverage**: 注釈の網羅率や参照切れなどの集計データ

---

## バージョン注釈（推奨）

**目的**: 注釈の導入や変更を PR／コミット単位で追跡し、差分や導入バージョンを明示する。

**提案フィールド**:
- マニフェストレベル:
  - `version`: string (semver またはタグ) — マニフェストのスナップショット識別子（任意）。
  - `generated_by`: { "tool": string, "tool_version": string, "timestamp": string (date-time) }
- 注釈レベル:
  - `version`: string | null — 注釈固有の導入バージョン（例: `v1.2.0`, `PR#123`, `2025-12-31`）。
  - `introduced_by`: { "type": "pr"|"commit"|"manual", "id": string, "author"?: string }
  - `introduced_at`: string (ISO-8601 タイムスタンプ)
  - `history`: array of { "version": string, "changed_by": string, "when": string, "note"?: string }

**スキーマ案（contracts/manifest.schema.json への追加）**:
- ルートに `version` と `generated_by` を追加（両方ともオプション）。
- 注釈オブジェクトに `version`、`introduced_by`、`introduced_at`、`history` を追加。

**運用ルール**:
- PR コンテキスト（CI / pre-merge）でスキャンを実行した場合、スキャナは `introduced_by` に PR ID をセットしてマニフェストを出力します。
- ローカル単発スキャンでは `version` は省略または null とすることができるが、ユーザーは手動で設定可能です。
- 差分機能は `annotation.version` と `introduced_by` の差分を基に変更を表示します。

**受け入れ基準（追加提案）**:
- **SC-005**: `contracts/manifest.schema.json` が `version` と `generated_by` を受け入れ、スキャナがそれらを出力できること。
- **SC-006**: PR スキャンでは、追加注釈に `introduced_by` が付与され、マニフェストに反映されること。
- **SC-007**: 注釈差分で `annotation.version` の追加／変更が差分として表示されること。

---

## 成功基準 *(必須、測定可能、技術中立)*

- **SC-001**: 開発者が「Unimplemented」項目一覧を取得できる（フィルタ応答は高速で、目標: 2 秒以内に結果が返る）。
- **SC-002**: レポートが注釈カバレッジを数値で出力する（例: 注釈付き項目数／総項目数）。
- **SC-003**: 整合性チェックが参照切れを検出し、重大な参照切れを一覧で可視化する。
- **SC-004**: PR レビュー時に注釈差分が表示され、承認プロセスで利用できる。

---

## 仮定

- 当初はこのリポジトリの主要言語とドキュメント形式（例: Markdown）に注力し、将来的に多言語対応を検討します。
- 注釈は主に人が追加する想定だが、クラス/関数名と仕様のマッチングのような軽量な自動検出を支援機能として提供する想定です。

---

## 仕様モデル: 形式定義（Formal Definition）

以下はサンプルの形式定義です。スペックの構造を厳密に記述し、ドキュメント注釈と実装注釈の関係を数学的／論理的に表現します。

/// @MetaAnnotation @MetaName="Definition of SpecTrailUnit" @MetaType=Philosophy
### 1.1 SpecTrailUnit
Let SpecTrailUnit = { CodeAnnotation, DocumentAnnotation }

CodeAnnotation は実際のプログラミングコード内に書かれる SpecTrail 注釈の実体です。
DocumentAnnotation は仕様ドキュメント内に書かれる SpecTrail 注釈の実体です。

SpecTrailUnit は、ドキュメント側と実装側の注釈のペアを表し、トレース可能な単位を構成します。

/// @MetaAnnotation @MetaName="Definition of SpecTrailAnnotation" @MetaType=Philosophy
### 1.2 SpecTrailAnnotation
SpecTrail は二つの補完的な注釈空間を定義します。

Let
SpecTrailUnit = { DocumentAnnotation, CodeAnnotation }

両者は共通の構造スキーマ（MetaAnnotation, AbstractAnnotation, SpecDetailAnnotation）を共有しますが、表現領域が異なります。

- **DocumentAnnotation**: 仕様ドメインに存在する注釈（自然言語や半構造化ドキュメント内）。
- **CodeAnnotation**: 実装ドメインに存在する注釈（コード内の埋め込み注釈やメタデータ）。

など（以降省略、英語版をマスターとしてください）。
