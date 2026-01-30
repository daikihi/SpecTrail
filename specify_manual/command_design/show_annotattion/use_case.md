# UseCase of show annotation command

In this section, we describe the UseCase of show annotation command.

We listed all functions of showing annotations in [Available functions](./overview.md#available-functions).

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
$ strail show --mode list --target all
```

- **Expected Result:**

  This mode shows both document and code annotations.
  If an annotation is defined on both document and code, then the annotation is shown twice and they are mapped.
  If an annotation is defined only on document, then the annotation is shown only once. And the annotation has no mapping on code annotation.
  If an annotation is defined only on code, then the annotation is shown only once. And the annotation has no mapping on document annotation.

- **Example:**

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

- **Steps:**

```bash
$ strail show --mode list --target document
```

- **Expected Result:**
  
  A list of all annotations defined in documents is displayed. Annotations defined only in code are not shown.


- **Example:**
  ```bash
  @todo
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
$ strail show --mode list --target code
```

- **Expected Result:**

You can see all code annotations in your project.

- **Example:**
  ```bash
  @todo
  ```

### List all annotation groups in your project

This mode lists all annotation groups in your project.
The target annotations are both document and code annotations.
When you want to list annotations grouped by specific layer or types, this mode is the most convenient way.

- **Purpose:**

Sometimes, a developer wants to see all annotations that are the same annotation layer.

- **Precondition:**
  The project contains at least one annotation group.
  However, this mode does not require that the annotation group is defined on the document or on the code.

- **Steps:**

  ```bash
  @todo
  ```

- **Expected Result:**
  The mode can show all annotations in the same group.

- **Example:**
  ```bash
  @todo
  ```
  
### Trace-related annotations starting from a specific annotation

This mode lists all related annotations starting from the specific annotations.
A developer sometimes wants to know related specifications or codes which have related to the specific annotation.

- **Purpose:**

A developer can know the relation among annotations. 
The relation points to the context of the specification in annotation.
Annotations have contexts from application or higher level specifications to real implementation codes.

- **Precondition:**

The project contains at least two annotations, and they are related to each other.
Even if the project does not contain any annotations, or the system contains more than two annotations and they are not related to each other,
however, the mode can execute without any error. But the mode execution result is an empty list.

- **Steps:**

```bash
@todo
```

- **Expected Result:**

The mode can show related annotations not only in text format but also in a graphical view.
A user can choose one of them.

- **Example:**
  ```bash
  @todo
  ```
