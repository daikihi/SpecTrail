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

### Format Options

- `text`: Human-readable text (Standard Output/Error).
- `json`: Machine-readable structured representation.