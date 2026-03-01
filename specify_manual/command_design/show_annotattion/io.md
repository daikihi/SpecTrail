# command I/O

This command shows annotations for both source code and document.

## Input

```bash
show --mode list --target all
show --mode list --target document
show --mode list --target code
show --mode list --target group
show --mode search --target code --scope src/
```

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

## Output

The output can be provided in **Standard Output (Text)** or **JSON** format.

### Text Output (Standard)

A human-readable list or tree, as shown in [UseCase examples](./use_case.md).

### JSON Output

A structured representation suitable for machine processing or the `report-ui`.