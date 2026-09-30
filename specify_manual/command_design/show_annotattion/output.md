/// [@st-manual-cli-show-output] layer: abstract, type: Structure, name: Show Output Contract, links: [@st-manual-cli-show-output-selection, @st-manual-cli-show-output-views, @st-manual-cli-show-output-json, @st-manual-cli-show-output-annotations, @st-manual-cli-show-output-streams]
# Show Output Contract

This document defines the output behavior of the `show` command. The annotation concepts come from the [metamodel](../../meta.md); the command is introduced in the [product specification](../../spec.md#show-command). Both text and JSON represent the same selected annotation data.

/// [@st-manual-cli-show-output-boundary] layer: spec-detail, type: Rule, name: Presentation Output Boundary, links: [@st-manual-cli-show-output]
## Presentation Boundary

The Use Case returns annotation data and scan warnings through a response DTO. A Presentation adapter builds the output model for the selected view. Text and JSON renderers consume that model. Only the selected view model is built. The format changes representation; it does not change scanning, selection, link resolution, warning detection, success or failure, or the exit code. Domain and Use Case code return results instead of printing output.

/// [@st-manual-cli-show-output-selection] layer: spec-detail, type: Convention, name: Show Selection Semantics, links: [@st-manual-cli-show-output]
## Command Inputs

- `target` selects the source set: `all`, `document`, or `code`. It is required.
- `mode` selects the operation. The initial supported mode is `list`; `search` and its `scope` belong to the future search operation and are not implemented by this output contract.
- `view` selects `summary`, `list`, `group`, or `detail`. The default is `list`.
- `format` selects `text` or `json`. The default is `text`.
- `--compact` selects compact JSON rather than the default pretty JSON. It is valid only with `--format json`.
- `--config` may select a configuration file. If omitted, the configured default file is used.

The default mode is `list`. `target` determines which annotations are in the result. `view` determines how that selected set is presented. `format` determines how the view is encoded.

/// [@st-manual-cli-show-output-views] layer: spec-detail, type: Convention, name: Show View Semantics, links: [@st-manual-cli-show-output]
## Views

| View | Meaning |
|---|---|
| `summary` | Counts over the selected annotations. |
| `list` | A file-based list with the fields needed to identify annotations. |
| `group` | A list grouped by Layer, then Type. |
| `detail` | The available annotation details without recursively embedding link targets. |

The view names are values of a single `view` field. The exact fields in each view's `result` are defined by the versioned JSON contract; a view does not silently change the selected target set.

/// [@st-manual-cli-show-output-summary] layer: spec-detail, type: Rule, name: Show Summary Counts, links: [@st-manual-cli-show-output]
### Summary Counts

Summary reports the total annotation count and counts by source (`document`, `code`), Layer, and Annotation Type. A test annotation is a spec-detail annotation with `type = Test`; it is counted under its actual source and in the Test type count. It is not a separate source category. The Test count is present with value zero when there are no Test annotations. The source counts and type counts are different dimensions and must not be summed together.

/// [@st-manual-cli-show-output-group] layer: spec-detail, type: Rule, name: Show Grouping, links: [@st-manual-cli-show-output]
### Grouping

Group uses Layer, then Type, then Annotation. An annotation with no Type belongs to a no-Type group; JSON retains `type: null` rather than inventing a Type value.

/// [@st-manual-cli-show-output-json] layer: spec-detail, type: Rule, name: Show JSON Envelope, links: [@st-manual-cli-show-output]
## JSON Envelope

Every successful JSON result has the same root fields:

```json
{
  "schema_version": 1,
  "created_at": "2026-09-30T09:11:23Z",
  "view": "list",
  "request": {
    "target": "all",
    "mode": "list",
    "scope": null,
    "config": {
      "path": "src/config/default.toml"
    }
  },
  "result": {
    "documents": [],
    "code": []
  }
}
```

The example illustrates a list result. The root envelope is fixed; `result` depends on the selected view. The JSON root has no `format` or `warnings` field. Successful empty results remain valid JSON.

/// [@st-manual-cli-show-output-metadata] layer: spec-detail, type: Rule, name: Show Request Metadata, links: [@st-manual-cli-show-output]
### Request Metadata

`request` records the effective `target`, `mode`, `scope`, and configuration file. In list mode, `mode` is `list` and `scope` is `null`. `config.path` is the path of the file actually used, including the default when `--config` is omitted. Config fingerprint is not part of the initial output.

`created_at` records generation time in UTC RFC 3339 form. It helps compare saved results approximately; it does not establish causal or total ordering between machines.

/// [@st-manual-cli-show-output-annotations] layer: spec-detail, type: Rule, name: Show Annotation Representation, links: [@st-manual-cli-show-output]
## Annotation Representation

Document and code sources remain separate. Each source-file entry contains a project-relative `source_file` and a flat `annotations` array rather than four Layer-specific arrays. An annotation has the shared fields `id`, `name`, `layer`, `type`, and `links` when the selected view includes them. The `id` is present and nonempty in every emitted annotation entry. List and group favor identification fields; detail exposes available Domain fields. Implementation annotations can additionally expose `artifact` and `status`.

Test annotations use the same representation as other spec-detail annotations, with `layer: "spec-detail"` and `type: "Test"`. No separate Test domain model or test-specific JSON object is introduced.

/// [@st-manual-cli-show-output-links] layer: spec-detail, type: Rule, name: Show Link References, links: [@st-manual-cli-show-output]
### Links

An annotation's `links` are outgoing references. Each emitted Link is an object with `target_id` and `target_layer`:

```json
{
  "target_id": "st-example-detail",
  "target_layer": "spec-detail"
}
```

The target annotation is not embedded recursively. A target may be absent from a result filtered by `target` or represented by a less detailed view. The output does not infer a reverse link, relation kind, or Layer cardinality. `target_layer` lets consumers classify the target even when that target is absent from the same JSON result.

The Link object does not carry a document/code source domain. If the same `id` and `layer` appear in both domains, this object alone cannot distinguish the two occurrences; consumers must use surrounding or external context. This limitation does not change the initial two-field Link contract.

/// [@st-manual-cli-show-output-null] layer: spec-detail, type: Rule, name: Show Nullable Fields, links: [@st-manual-cli-show-output]
### Null and Empty Values

The initial nullable fields are Annotation `type`, Implementation `status`, and `request.scope`. When one of these fields is emitted without a value, JSON uses `null`. A field excluded by the selected view is omitted, which differs from an emitted field containing `null`. Collections are emitted as arrays and use `[]` when empty. `request.config.path`, `target`, `mode`, `view`, `schema_version`, and `created_at` are not nullable.

/// [@st-manual-cli-show-output-order] layer: spec-detail, type: Rule, name: Show Stable Ordering, links: [@st-manual-cli-show-output]
## Paths and Ordering

The initial project root is the process's current working directory. Source files and the effective configuration file are represented relative to that root. Absolute machine-specific paths are not exposed in normal output.

Both renderers use the same deterministic order: Layer groups follow `meta`, `abstract`, `spec-detail`, `implementation`; source files sort by relative path; annotations within a Layer sort by `id`; links sort by `target_id`; Type groups sort by Type name. A no-Type group retains a stable position in the versioned view contract.

/// [@st-manual-cli-show-output-text] layer: spec-detail, type: Rule, name: Show Text Equivalence, links: [@st-manual-cli-show-output]
## Text Rendering

Text and JSON convey the same selected data and view semantics. Text may use headings, labels, indentation, and readable missing-value markers. Machine-oriented JSON envelope fields do not need to appear as matching text fields. Text uses the same target selection, counts, annotation identities, link references, and ordering rules as JSON.

/// [@st-manual-cli-show-output-streams] layer: spec-detail, type: Rule, name: Show Standard Streams, links: [@st-manual-cli-show-output]
## Standard Streams and Exit Status

- Successful result: complete text or JSON on stdout, exit code zero.
- Nonfatal scan warning with a usable result: complete result on stdout, warning on stderr, exit code zero.
- Invalid argument, invalid configuration, inability to start a scan, or renderer failure: error on stderr and a nonzero exit code.
- The initial JSON contract has no JSON error object. JSON stdout contains no warning, log, or progress text.

/// [@st-manual-cli-show-output-schema-version] layer: spec-detail, type: Rule, name: Show Schema Version Management, links: [@st-manual-cli-show-output]
## Versioned JSON Contract

`schema_version` is a sequential integer identifying the external JSON schema. It is independent of `created_at` and the application release version. The versioned schemas are stored as `contracts/show-json/vN.schema.json`; prior versions are immutable. `contracts/show-json/LATEST` identifies the current version. A version-bump helper allocates the next sequential number, and CI checks that `LATEST`, the current schema file, and the emitted `schema_version` agree. A change's compatibility is reviewed before the version is advanced.

/// [@st-manual-cli-show-output-schema-validation] layer: spec-detail, type: Rule, name: Show JSON Schema Validation, links: [@st-manual-cli-show-output]
### Validation Scope

The schema checks the root envelope and allowed `view` values. Every emitted Annotation entry requires a nonempty `id`; known common fields have defined types when present. Link objects contain `target_id` and `target_layer`. Future or project-specific annotation fields are allowed. JSON Schema validates the shape of references; whether a target ID resolves to an actual annotation is checked separately when needed.

/// [@st-manual-cli-show-output-acceptance] layer: spec-detail, type: Test, name: Show Output Acceptance Criteria, links: [@st-manual-cli-show-output]
## Acceptance Criteria

- Each of the four views can render the same selected annotation data as text and JSON.
- The JSON envelope is valid for empty results, and every emitted Annotation has a nonempty `id`.
- Missing optional values, empty collections, and outgoing Link references follow the rules above.
- Pretty and compact JSON contain the same values; both formats use the same stable ordering.
- A nonfatal warning appears on stderr with a complete stdout result and exit code zero. A fatal error appears on stderr with a nonzero exit code.
