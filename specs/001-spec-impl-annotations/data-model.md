/// @MetaAnnotation @MetaName="Specification Data Model" @MetaType=Structure
/// @AbstractAnnotation @name="DataModelOverview" @type="DataModel"
# data-model.md — Spec-Impl-Annotations

**Created**: 2025-12-31

## Entities

/// @SpecDetailAnnotation @id="SPEC_ITEM" @name="SpecificationItem" @type="entity"
### SpecificationItem
- id: string (unique within repository, e.g., `docs:Auth:LoginFlow`)
- title: string
- doc_path: string (file path)
- anchor/line: optional (for precise location)
- tags: [string]
- created_at, updated_at

/// @SpecDetailAnnotation @id="ANNOTATION" @name="Annotation" @type="entity"
### Annotation
- annotation_id: string (UUID or human-readable stable id)
- spec_item_id: string (links to SpecificationItem)
- source: enum(`document`, `code`)
- annotation_type: enum(`meta`, `abstract`, `detail`, `implementation`)
- content: string (raw annotation body)
- location: { file: string, line?: number, column?: number }
- status: enum(`implemented`, `in-progress`, `unimplemented`, `deprecated`, `verified`)
- estimated_status?: enum(...) (if produced by batch heuristics)
- links: [trace_id]
- metadata: map (freeform for tools)
- created_by, created_at, updated_at

### ImplementationReference
- id: string
- annotation_id: string
- file: string
- line?: number
- symbol?: string
- language?: string
- artifact: enum(`database`,`domain`,`external_system`,`web_interface`,`other`)

### Trace
- id: string
- src: annotation_id
- dst: annotation_id
- kind: enum(`refines`,`implements`,`verifies`,`derives`)

### Report / Coverage
Aggregations computed from manifests and indexes (counts, ratios, lists of missing refs, broken refs)

## Validation Rules
- `annotation_id` uniqueness
- `spec_item_id` must exist for document-sourced annotations
- `location` must reference an existing file for code-sourced annotations (validated by scanner)
- `status` must be one of the allowed enum values

## Notes
- The manifest (contracts/manifest.schema.json) encodes these entities for storage and validation.  
- The runtime index (SQLite) contains tables for `annotations`, `spec_items`, `refs`, and `traces` to support fast filtering and reporting.