# command I/O

This command shows annotations for both source code and document.

## Input

```bash
show --mode list --target all --config src/config/default.toml
show --mode list --target all --config src/config/simple_sample.toml
show --mode list --target all
show --mode list --target document
show --mode list --target code
show --mode list --target group
show --mode search --target code --scope src/
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

### target: group

Returns annotations organized hierarchically by Layer and Type.

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

The output can be provided in **Standard Output (Text)** or **JSON** format.

### Text Output (Standard)

A human-readable list or tree, as shown in [UseCase examples](./use_case.md).

### JSON Output

A structured representation suitable for machine processing or the `report-ui`.