/// [@st-manual-data-model-meta-file] layer: abstract, type: File, name: meta.md

# Meta Annotation

## Overview
Meta-layer annotations represent top-level policies such as design philosophy, guidelines, and rules.

## Structure
/// [@st-manual-data-model-meta-meta-annotation-id] layer: abstract, type: Structure, name: MetaAnnotationId
- `MetaAnnotationId`
  - wrapper around `String` (id = tag)
/// [@st-manual-data-model-meta-meta-name] layer: abstract, type: Structure, name: MetaName
- `MetaName`
  - wrapper around `String`
/// [@st-manual-data-model-meta-meta-type] layer: abstract, type: Structure, name: MetaType
- `MetaType`
  - `Philosophy`
  - `Guideline`
  - `Convention`
  - `Structure`
  - `Rule`
/// [@st-manual-data-model-meta-meta-annotation] layer: abstract, type: Structure, name: MetaAnnotation
- `MetaAnnotation`
  - `id: MetaAnnotationId`
  - `name: MetaName`
  - `type: MetaType`
  - `layer: Layer` (typically `Layer::Meta`)
  - `links: Vec<MetaAnnotation>`

## Notes
`links` represents relationships between meta annotations and can form a self-referential graph.
