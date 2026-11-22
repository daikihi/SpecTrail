/// [@st-manual-data-model-abstract-annotation-file] layer: abstract, type: File, name: abstract_annotation.md

# Abstract Annotation

## Overview
Abstract-layer annotations represent conceptual units and structures of the system.

## Structure
/// [@st-manual-data-model-abstract-annotation-abstract-annotation-id] layer: abstract, type: Structure, name: AbstractAnnotationId
- `AbstractAnnotationId`
  - wrapper around `String` (id = tag)
/// [@st-manual-data-model-abstract-annotation-abstract-name] layer: abstract, type: Structure, name: AbstractName
- `AbstractName`
  - wrapper around `String`
/// [@st-manual-data-model-abstract-annotation-abstract-type] layer: abstract, type: Structure, name: AbstractType
- `AbstractType`
  - `Page`
  - `Application`
  - `BackgroundComponent`
  - `Structure`
  - `Convention`
/// [@st-manual-data-model-abstract-annotation-abstract-link] layer: abstract, type: Structure, name: AbstractLink
- `AbstractLink`
  - `Meta(MetaAnnotationId)`
  - `Abstract(AbstractAnnotationId)`
  - `SpecDetail(SpecDetailAnnotationId)`
/// [@st-manual-data-model-abstract-annotation-abstract-annotation] layer: abstract, type: Structure, name: AbstractAnnotation
- `AbstractAnnotation`
  - `id: AbstractAnnotationId`
  - `name: AbstractName`
  - `type: AbstractType`
  - `layer: Layer` (typically `Layer::Abstract`)
  - `links: Vec<AbstractLink>`

## Relationships
`links` points to associated `MetaAnnotation`, `AbstractAnnotation`, or `SpecDetailAnnotation` identifiers.
