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

/// [@st-manual-overview-show-selection] layer: spec-detail, type: Convention, name: Show Target and View Selection, links: [@st-manual-cli-show-output]
## Available Functions

`target` selects the annotation sources: `all`, `document`, or `code`. `view` determines how the selected annotations are shown:

- `summary`: counts by source, Layer, and Type.
- `list`: a file-based identification list.
- `group`: annotations grouped by Layer and then Type.
- `detail`: available annotation details and nonrecursive link references.

The initial supported mode is `list`. Search by `scope` and trace exploration are future operations. Text and JSON follow the same target and view semantics; see the [Show Output Contract](output.md).

# Reference

- [Input / output : io.md](./io.md)
- [usecases](./use_case.md)
- [flow](./flow.md)
- [usage](./useage.md)
- [output](./output.md)
