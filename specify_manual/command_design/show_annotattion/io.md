# command I/O

This command shows annotations for both source code and document.

## Input

```bash
show --target all --config src/config/default.toml --view list --format text
show --target all --config src/config/simple_sample.toml --view detail
show --target all --view summary
show --target document
show --target code
show --target all --view group
```

/// [@st-manual-io-show-config] layer: spec-detail, type: Convention, name: Show Config Selection
The `show` command can load a config file passed as a parameter and switch the scanning target accordingly.
This allows the main project and sample applications to be traced with different `config/*.toml` files.

### target: all

Returns a flat list of all annotations from both `src/` and `specify_manual/`.

```json
{
  "document_annotations": [...],
  "code_annotations": [...]
}
```

### target: document / code

Returns a list of annotations filtered by source.

### view: group

When `--view group` is specified, the output is organized hierarchically by Layer and Type.
This is a presentation choice applied on top of normal targets (`all`, `document`, `code`).
It only applies when `--view group` is used.

```json
{
  "Meta": {
    "Philosophy": [
      { "id": "@st-manual-meta-model-doc", "name": "..." }
    ],
    "Guideline": [...]
  },
  "Abstract": { ... }
}
```

### config

The config determines the scan roots and extensions for document and code.
It also makes it possible to switch between `config/default.toml` and `config/simple_sample.toml`.

## Output

The output is managed by the `Presentation` layer (`output` module) and can be controlled via `--view` and `--format` options.

### View Options

- `summary`: Overall statistics (count, types, etc.).
- `list`: File-based summary list (default).
- `group`: Grouped by attributes (Layer, Type, etc.).
- `detail`: Full annotation details.

### Traces in detail view

The JSON `traces` array contains one entry per directed relationship between two annotation locations. The `src_` and `dst_` fields identify each endpoint. Lines start at 1. For example, the bundled sample produces this entry as well as a separate one for `domain.rs`:

```json
{
  "src_space": "code",
  "src_file": "samples/rust/simple_sample/src/application.rs",
  "src_line": 1,
  "src_layer": "Implementation",
  "src_id": "ss-simple-sample",
  "dst_space": "document",
  "dst_file": "samples/rust/simple_sample/spec.md",
  "dst_line": 3,
  "dst_layer": "Meta",
  "dst_id": "ss-simple-sample",
  "kind": "Implements"
}
```

A link originating in a document has `src_space: "document"` and `dst_space: "code"`. Summary view counts the same entries.

### Format Options

- `text`: Human-readable text (Standard Output/Error).
- `json`: Machine-readable structured representation.