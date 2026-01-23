/// [@st-manual-data-model-spec-detail-file] layer: abstract, type: File, name: spec_detail.md

# Spec Detail Annotation

## Overview
Spec-detail-layer annotations represent concrete specifications such as functional/non-functional requirements, tests, and infrastructure.

## Structure
/// [@st-manual-data-model-spec-detail-spec-detail-annotation-id] layer: abstract, type: Structure, name: SpecDetailAnnotationId
- `SpecDetailAnnotationId`
  - wrapper around `String` (id = tag)
/// [@st-manual-data-model-spec-detail-spec-detail-name] layer: abstract, type: Structure, name: SpecDetailName
- `SpecDetailName`
  - wrapper around `String`
/// [@st-manual-data-model-spec-detail-spec-detail-type] layer: abstract, type: Structure, name: SpecDetailType
- `SpecDetailType`
  - `Func`
  - `NonFunc`
  - `Test`
  - `Infra`
  - `Convention`
  - `Rule`
/// [@st-manual-data-model-spec-detail-spec-detail-link] layer: abstract, type: Structure, name: SpecDetailLink
- `SpecDetailLink`
  - `Abstract(Box<AbstractAnnotation>)`
  - `Implementation(Box<ImplementationAnnotation>)`
/// [@st-manual-data-model-spec-detail-spec-detail-annotation] layer: abstract, type: Structure, name: SpecDetailAnnotation
- `SpecDetailAnnotation`
  - `id: SpecDetailAnnotationId`
  - `name: SpecDetailName`
  - `type: SpecDetailType`
  - `layer: Layer` (typically `Layer::SpecDetail`)
  - `links: Vec<SpecDetailLink>`

## Relationships
`links` connects to the abstract layer and/or the implementation layer.
