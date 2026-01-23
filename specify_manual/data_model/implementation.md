/// [@st-manual-data-model-implementation-file] layer: abstract, type: File, name: implementation.md

# Implementation Annotation

## Overview
Implementation-layer annotations describe how specifications are realized at the technical level.

## Structure
/// [@st-manual-data-model-implementation-implementation-annotation-id] layer: abstract, type: Structure, name: ImplementationAnnotationId
- `ImplementationAnnotationId`
  - wrapper around `String` (id = tag)
/// [@st-manual-data-model-implementation-implementation-spec-name] layer: abstract, type: Structure, name: ImplementationSpecName
- `ImplementationSpecName`
  - wrapper around `String`
/// [@st-manual-data-model-implementation-implementation-type] layer: abstract, type: Structure, name: ImplementationType
- `ImplementationType`
  - `DatabaseSchema`
  - `DaoRepository`
  - `DomainEntity`
  - `ExternalApiGateway`
  - `WebInterfaceDataModel`
/// [@st-manual-data-model-implementation-implementation-link] layer: abstract, type: Structure, name: ImplementationLink
- `ImplementationLink`
  - `SpecDetail(Box<SpecDetailAnnotation>)`
  - `Abstract(Box<AbstractAnnotation>)`
/// [@st-manual-data-model-implementation-implementation-artifact] layer: abstract, type: Structure, name: ImplementationArtifact
- `ImplementationArtifact`
  - wrapper around `String`
/// [@st-manual-data-model-implementation-implementation-status] layer: abstract, type: Structure, name: ImplementationStatus
- `ImplementationStatus`
  - `Planned`
  - `InProgress`
  - `Completed`
/// [@st-manual-data-model-implementation-implementation-annotation] layer: abstract, type: Structure, name: ImplementationAnnotation
- `ImplementationAnnotation`
  - `id: ImplementationAnnotationId`
  - `name: ImplementationSpecName`
  - `type: ImplementationType`
  - `layer: Layer` (typically `Layer::Implementation`)
  - `links: Vec<ImplementationLink>`
  - `artifact: ImplementationArtifact`
  - `status: ImplementationStatus`

## Relationships
`links` connects to either `SpecDetailAnnotation` or `AbstractAnnotation`.
