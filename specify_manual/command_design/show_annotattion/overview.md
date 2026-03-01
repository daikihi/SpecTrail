# Show Annotations Command

This command shows lists of annotations in your project.
It provides a comprehensive overview of all annotations, including their types, categories, and associated metadata.

## Goal

The goal of the `show` command is to provide transparency and traceability between specifications and implementation. 
It allows users to quickly inspect what annotations exist, how they are categorized, and how they relate to each other.
By providing different targets and modes, it caters to various roles:
- **Developers** can find implementation details and their corresponding specifications.
- **Designers** can verify if their specifications are correctly annotated.
- **Managers** can get an overview of the project's state and coverage.

## Available functions

This command lists all annotations in your project for both document and programming code.
You can view annotations categorized as follows:

**Targets of this command:**
- **All**: List all annotations in your project on both document and programming code. This is the entry point for a full project audit.
- **Document**: List only document annotations. Useful for reviewing specifications.
- **Code**: List only code annotations. Useful for developers working on the implementation.
- **Group**: List all annotations grouped by their **layer** (Meta, Abstract, SpecDetail, Implementation) or **type** (Philosophy, Guideline, etc.). This helps in understanding the structural distribution of annotations.
- **Search**: Find specific annotations based on a query (ID, Name, or Metadata).
- **Trace**: (Experimental) Trace the relationship starting from a specific annotation to see its links across layers.

# Reference

- [Input / output : io.md](./io.md)
- [usecases](./use_case.md)
- [flow](./flow.md)
- [usage](./usage.md)