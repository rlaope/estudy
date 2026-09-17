# Object and Table Mapping

### Entity Mapping
- Object and table mapping: @Entity, @Table
- Field and column mapping: @Column
- Primary key mapping: @Id
- Association mapping: @ManyToOne, @JoinColumn

### Object and Table Mapping

#### Entity
- A class annotated with @Entity is managed by JPA and is called an entity.
- Classes to be mapped to tables using JPA must have @Entity.

Note
- Default constructor required (public or protected constructor with no parameters).
- Cannot use final classes, enums, interfaces, or inner classes.
- Cannot use final on fields to be persisted.

### @Table
- @Table specifies the table to map with the entity.
- name: The name of the table to map (default: uses the entity name).
- catalog: Maps the database catalog.
- schema: Maps the database schema.
- uniqueConstraints(DDL): Creates unique constraints when generating DDL.
