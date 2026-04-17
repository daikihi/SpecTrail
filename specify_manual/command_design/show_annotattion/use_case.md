# UseCase of show annotation command

In this section, we describe the UseCase of show annotation command.

We listed all functions of showing annotations in [Available functions](./overview.md#available-functions).

/// [@st-manual-usecase-show-config-split] layer: spec-detail, type: Convention, name: Show Config Split
The `show` command can switch its scan roots by config, so the main project and sample projects can be traced separately.
The recommended config set is `config/default.toml` for the main project and `config/simple_sample.toml` for the sample project.

## Individual UseCases

### List all annotations in your project

- **Purpose:**

  This mode lists all annotations in your project.
  This mode is the most standard use case of this command.
  The target annotations are both document and code annotations.
  When you want to list all annotations in your project, this mode is the most convenient way.
  This is easy to understand your project and current status of development.
  This also helps you to check all mapping between annotations.

- **Precondition:**

  At least one annotation is defined in your project.
  The annotation does not require to be defined on both document and code.
  This means your project contains at least one annotation, but we don't require that it is on a document or on a code.

- **Steps (command example):**

  Execute the following command:

```bash
$ strail show --mode list --target all --config src/config/default.toml
```

- **Expected Result:**

  This mode shows both document and code annotations.
  If an annotation is defined on both document and code, then the annotation is shown twice and they are mapped.
  If an annotation is defined only on document, then the annotation is shown only once. And the annotation has no mapping on code annotation.
  If an annotation is defined only on code, then the annotation is shown only once. And the annotation has no mapping on document annotation.

- **Example:**

```text
Code Anno [0]: @st-code-use-case-show-show-use-case-file (Layer: Abstract, Type: Structure)
Document Anno [0]: @st-manual-spec-specification (Layer: Abstract, Type: Structure)
```

### List all document annotations in your project

- **Purpose:**

  This mode lists all document annotations in your project.
  When you want to list only document annotations, this mode is the most convenient way.
  Most projects which are using SpecTrail start to write documents before developing codes.


- **Precondition:**
  
  This mode requires that your project contains at least one document annotation.
  Even if your project does not contain any document annotation, this mode does not show any error but shows an empty list.

- **Steps:**

```bash
$ strail show --mode list --target document --config src/config/default.toml
```

- **Expected Result:**
  
  A list of all annotations defined in documents is displayed. Annotations defined only in code are not shown.


- **Example:**
  ```text
  Document Anno [0]: @st-manual-spec-specification (Layer: Abstract, Type: Structure)
  ```

### List all code annotations in your project

- **Purpose:**
  
  This mode lists all code annotations in your project on the programming code.
  This mode shows all code annotations on your project codes.

  It helps you to check the status of your project especially when you are developing codes or you are a manager of your project.
  And, it helps you to update your code when your project gets to change a specification of the function.

- **Precondition:**

  At least one code annotation is defined in your project on the code.
  However, this mode does not require that the annotation is defined on the document.
  The mode will respond with an empty list if there is no code annotation in your project.

- **Steps:**

```bash
$ strail show --mode list --target code --config src/config/default.toml
```

- **Expected Result:**

You can see all code annotations in your project.

- **Example:**
  ```text
  Code Anno [0]: @st-code-use-case-show-show-use-case-file (Layer: Abstract, Type: Structure)
  ```

### List all annotation groups in your project

- **Purpose:**

  Sometimes, a developer wants to see all annotations that are in the same annotation layer or share the same type.
  This helps in identifying patterns and ensuring consistency across the project.
  For example, viewing all "Philosophy" meta-annotations together provides a high-level view of the project's core principles.

- **Precondition:**

  The project contains at least one annotation.

- **Steps:**

```bash
$ strail show --mode list --target group
```

- **Expected Result:**

  The command outputs annotations grouped by their **Layer** (Meta, Abstract, SpecDetail, Implementation) and then by their **Type**.

- **Example:**

```text
[Layer: Meta]
  - [Type: Philosophy]
    - @st-manual-meta-model-doc: Specification Model: Formal Definition
  - [Type: Guideline]
    - @st-manual-spec-user-assumption: User Assumption and Use Case

[Layer: Abstract]
  - [Type: Structure]
    - @st-manual-spec-specification: SpecTrail Specification
```

### Trace-related annotations starting from a specific annotation

- **Purpose:**

  A developer sometimes wants to know related specifications or codes which are related to a specific annotation.
  This mode allows tracing the chain of links from a single point of interest.
  It helps in impact analysis when a specification changes.

- **Precondition:**

  The project contains at least one annotation with links to other annotations.

- **Steps:**

```bash
$ strail show --mode search --target all --scope "@st-manual-spec-cli-show-command" --config src/config/simple_sample.toml
```
*(Note: Currently, tracing is often achieved via `search` mode with a specific ID, but future versions might have a dedicated trace mode.)*

- **Expected Result:**

  The mode shows the target annotation and all annotations that are directly or indirectly linked to it.

- **Example:**

```text
Target: @st-manual-spec-cli-show-command
Links to:
  - @st-manual-spec-cli (Layer: SpecDetail, Type: Rule)
  - @st-code-use-case-show-show-use-case (Layer: Abstract, Type: Structure)
```