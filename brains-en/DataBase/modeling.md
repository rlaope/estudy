# Data Modeling and Types of DBs Based on Data Characteristics

## Data Modeling
It is the task of constructing a logical data model from given concepts.

### Purpose of Data Modeling
- To accurately analyze the business content that is the target of information system construction by expressing the foundational information that constitutes business information using a consistent notation.
- To create an actual database using the analyzed model and use it for development and data management.

Data modeling should be performed considering redundancy, flexibility, and consistency.

- **Redundancy**: Storing duplicate data or attributes within the same database should be avoided.

- **Flexibility**: To prevent frequent modifications/changes to the model, query tuning should also be considered.

- **Consistency**: To avoid cases where data is updated by ignoring related information between data, relationships between data must be clearly defined.

### Relational Database Data Modeling Sequence
1. Requirements gathering
2. Conceptual data model design
3. Logical data model design
4. Physical data model design
