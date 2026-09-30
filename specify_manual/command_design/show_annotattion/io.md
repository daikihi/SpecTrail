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
show --target all --view list --format json
show --target all --view list --format json --compact
```

/// [@st-manual-io-show-config] layer: spec-detail, type: Convention, name: Show Config Selection
The `show` command can load a config file passed as a parameter and switch the scanning target accordingly.
This allows the main project and sample applications to be traced with different `src/config/*.toml` files.

### target: all

Selects annotations from both configured source domains. The selected view determines the result structure. JSON keeps document and code sources distinct inside `result`; file entries use a flat `annotations` array. See the [Show Output Contract](output.md).

### target: document / code

Returns a list of annotations filtered by source.

### view: group

When `--view group` is specified, the output is organized hierarchically by Layer and Type.
This is a presentation choice applied on top of normal targets (`all`, `document`, `code`).
It only applies when `--view group` is used.

The Layer order is `meta`, `abstract`, `spec-detail`, `implementation`. Type groups follow within each Layer. An annotation without a Type belongs to a no-Type group; JSON retains `type: null`.

### config

The config determines the scan roots and extensions for document and code.
It also makes it possible to switch between `config/default.toml` and `config/simple_sample.toml`.

/// [@st-manual-io-show-output] layer: spec-detail, type: Rule, name: Show Output Selection, links: [@st-manual-cli-show-output]
## Output

The Presentation layer builds the selected view once and renders it as text or JSON. Both formats represent the same selected data. The [Show Output Contract](output.md) defines the full behavior.

### View Options

- `summary`: Overall statistics (count, types, etc.).
- `list`: File-based summary list (default).
- `group`: Grouped by attributes (Layer, Type, etc.).
- `detail`: Full annotation details.

### Format Options

- `text`: Human-readable text on stdout; the default.
- `json`: Machine-readable structured representation on stdout, pretty-printed by default. `--compact` changes only whitespace and is valid only with JSON.

Warnings and errors go to stderr. Warnings with a usable result do not change the success exit code.
