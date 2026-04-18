# Usage: show_annotation

In this section, we describe about how to use the show-command.

/// [@st-manual-usage-show-all] layer: spec-detail, type: Convention, name: Show All Annotations
To show all annotations, choose the config that matches the application and scan both document and code roots:
```bash
cargo run --bin show -- --target all --config src/config/default.toml
```

/// [@st-manual-usage-show-document] layer: spec-detail, type: Convention, name: Show Document Annotations
To show only document annotations, use the config for the application and target documents:
```bash
cargo run --bin show -- --target document --config src/config/simple_sample.toml
```

/// [@st-manual-usage-show-code] layer: spec-detail, type: Convention, name: Show Code Annotations
To show only code annotations, use the config for the application and target code:
```bash
cargo run --bin show -- --target code --config src/config/default.toml
```

/// [@st-manual-usage-show-search] layer: spec-detail, type: Convention, name: Search Annotations
To search for specific annotations using a query, and point `show` at the appropriate config:
```bash
cargo run --bin show -- --mode search --target all --scope "search_query" --config src/config/simple_sample.toml
```

/// [@st-manual-usage-show-group] layer: spec-detail, type: Convention, name: Show Grouped Annotations
To show grouped annotations:
```bash
cargo run --bin show -- --view group
```
