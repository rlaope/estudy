# Identifier Types and Classification System

## Types of Identifiers
Identifiers are classified into primary identifiers and secondary identifiers based on whether they represent an entity.

They are classified into internal identifiers and external identifiers based on whether they are self-generated within the entity.

They are classified into single identifiers and composite identifiers based on whether they can be identified by a single attribute.

Intrinsic identifiers and artificial identifiers are used to distinguish between identifiers that originally had business meaning and newly created identifiers, such as serial numbers, that replace the attributes of the original identifiers.

## Identifier Classification System

### Representativeness
- `Primary Identifier`: An identifier that can distinguish each occurrence within an entity and connect reference relationships with other entities.
- `Secondary Identifier`: An identifier that can distinguish each occurrence within an entity but cannot establish reference relationships due to lack of representativeness.

### Self-Generation

- `Internal Identifier`: An identifier generated internally within an entity.
- `External Identifier`: An identifier received from another entity through a relationship with that entity.

### Number of Attributes
- `Single Identifier`: An identifier composed of a single attribute.
- `Composite Identifier`: An identifier composed of two or more attributes.

### Substitutability
- `Intrinsic Identifier`: An identifier created by business requirements.
- `Artificial Identifier`: An identifier not created by business requirements, but artificially created because the original identifier has a complex structure.

> Occurrence: Data storing concrete and actual information in a database according to the structure of a defined record.

## Characteristics of Primary Identifiers
- Uniqueness: All instances within an entity are uniquely distinguished by the primary identifier.
- Minimality: The number of attributes comprising the primary identifier must be the minimum number to satisfy uniqueness.
- Immutability: Once a primary identifier is assigned to a specific entity, its value should not change.
- Existence: When a primary identifier is assigned, a data value must exist. Null is not allowed.

## Comparison of Identifying vs. Non-Identifying Relationships

### Purpose

- Identifying: Representing strong relationships
- Non-Identifying: Representing weak relationships

### Impact on Child Primary Identifier
- Identifying: Included in the composition of the child primary identifier
- Non-Identifying: Included in the child's regular attributes

### Notation
- Identifying: Represented by a solid line
- Non-Identifying: Represented by a dashed line

### Connection Considerations
- Identifying
  - Must be dependent on the parent entity
  - Requires inclusion of the parent primary identifier in the child primary identifier's composition
  - Requires transferring inherited primary identifier attributes to other entities
- Non-Identifying
  - Weak dependency relationship
  - Composes the child primary identifier independently
  - Requires a portion of the parent primary identifier in the child primary identifier's composition
  - Requires blocking the inherited primary identifier attributes from being transferred to other entities
  - Parent's participation in the relationship is optional
