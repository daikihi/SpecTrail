/// [@st-manual-data-model-annotation-file] layer: abstract, type: File, name: annotation.md

# Annotation Aggregation

## Overview
Defines aggregation models for document-space and code-space annotations, plus a diff model for comparison.

## Aggregation Models
/// [@st-manual-data-model-annotation-document-annotation] layer: abstract, type: Structure, name: DocumentAnnotation
- `DocumentAnnotation`
  - `metas: Vec<MetaAnnotation>`
  - `abstracts: Vec<AbstractAnnotation>`
  - `details: Vec<SpecDetailAnnotation>`
  - `implementations: Vec<ImplementationAnnotation>`
/// [@st-manual-data-model-annotation-code-annotation] layer: abstract, type: Structure, name: CodeAnnotation
- `CodeAnnotation`
  - `metas: Vec<MetaAnnotation>`
  - `abstracts: Vec<AbstractAnnotation>`
  - `details: Vec<SpecDetailAnnotation>`
  - `implementations: Vec<ImplementationAnnotation>`

## Unit Representation
/// [@st-manual-data-model-annotation-spec-trail-unit] layer: abstract, type: Structure, name: SpecTrailUnit
- `SpecTrailUnit`
  - `Code(CodeAnnotation)`
  - `Document(DocumentAnnotation)`

## Diff Model
/// [@st-manual-data-model-annotation-meta-annotation-diff] layer: abstract, type: Structure, name: MetaAnnotationDiff
- `MetaAnnotationDiff`
  - `common: Vec<MetaAnnotation>`
  - `only_in_document: Vec<MetaAnnotation>`
  - `only_in_code: Vec<MetaAnnotation>`

### Behavior
- `compare(doc, code)`
  - compares `metas` between `DocumentAnnotation` and `CodeAnnotation` using a `HashSet` and extracts common/one-sided values
- `is_empty()`
  - checks whether `only_in_document` and `only_in_code` are empty
