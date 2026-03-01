# Usage: show_annotation

In this section, we describe about how to use the show-command.

/// [@st-manual-usage-show-all] layer: spec-detail, type: Convention, name: Show All Annotations
To show all annotations (scan both `src/` and `specify_manual/`):
```bash
cargo run --bin show -- --mode list --target all
```

/// [@st-manual-usage-show-document] layer: spec-detail, type: Convention, name: Show Document Annotations
To show only document annotations (scan `specify_manual/`):
```bash
cargo run --bin show -- --mode list --target document
```

/// [@st-manual-usage-show-code] layer: spec-detail, type: Convention, name: Show Code Annotations
To show only code annotations (scan `src/`):
```bash
cargo run --bin show -- --mode list --target code
```

/// [@st-manual-usage-show-search] layer: spec-detail, type: Convention, name: Search Annotations
To search for specific annotations using a query:
```bash
cargo run --bin show -- --mode search --target all --scope "search_query"
```

/// [@st-manual-usage-show-group] layer: spec-detail, type: Convention, name: Show Grouped Annotations
To show grouped annotations (future implementation):
```bash
cargo run --bin show -- --mode list --target group
```

