# UseCase of show annotation command

In this section, we describe the UseCase of show annotation command.

We listed all functions of showing annotations in [Available functions](./overview.md#available-functions).

## Individual UseCases

### List all annotations in your project

- purpose:

This mode lists all annotations in your project.
This mode is the most standard use case of this command.
The target annotations are both document and code annotations.
When you want to list all annotations in your project, this mode is the most convenient way.
This is easy to understand your project and current status of development.
This also helps you to check all mapping between annotations.

- precondition:

At least one annotation is defined in your project.
The annotation does not require to be defined on both document and code.
This means your project contains at least one annotation, but we don't require that it is on a document or on a code.

- steps (command example):

Execute the following command:

```bash
$ strail show --mode list --target all
```

- expected result:

This mode shows both document and code annotations.
If an annotation is defined on both document and code, then the annotation is shown twice and they are mapped.
If an annotation is defined only on document, then the annotation is shown only once. And the annotation has no mapping on code annotation.
If an annotation is defined only on code, then the annotation is shown only once. And the annotation has no mapping on document annotation.

- example:

```bash
@todo
```

### List all document annotations in your project

- **Purpose:**

This mode lists all document annotations in your project.
When you want to list only document annotations, this mode is the most convenient way.
Most projects which are using SpecTrail start to write documents before developing codes.


- **Precondition:**
  
This mode requires that your project contains at least one document annotation.
Even if your project does not contain any document annotation, this mode does not show any error but shows an empty list.

- **操作手順 (Steps):**
  1. コマンド `speck show manifest --type=document` を実行する。
- **期待結果 (Expected Result):**
  ドキュメントアノテーションのみが表示される。
- **利用例 (Example):**
  ```
  $ speck show manifest --type=document
  ```

### List all code annotations in your project

- **目的 (Purpose):**
  コードアノテーションのみを一覧表示する。
- **前提 (Precondition):**
  プロジェクトが正しくセットアップされていること。
- **操作手順 (Steps):**
  1. コマンド `speck show manifest --type=code` を実行する。
- **期待結果 (Expected Result):**
  コードアノテーションのみが表示される。
- **利用例 (Example):**
  ```
  $ speck show manifest --type=code
  ```

### List all annotation groups in your project

- **目的 (Purpose):**
  アノテーショングループを一覧表示する。
- **前提 (Precondition):**
  プロジェクトが正しくセットアップされていること。
- **操作手順 (Steps):**
  1. コマンド `speck show manifest --group` を実行する。
- **期待結果 (Expected Result):**
  アノテーショングループが表示される。
- **利用例 (Example):**
  ```
  $ speck show manifest --group
  ```
