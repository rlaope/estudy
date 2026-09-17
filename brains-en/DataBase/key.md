# Concept and Types of Keys

### Key
- In a database, it refers to an attribute that serves as a criterion for finding tuples that satisfy a condition or for sorting them in order.

![table](./Image/table.png)

### Super Key
- A key composed of a set of attributes within a relation. Among all tuples constituting the relation, no two tuples will have the same value for the set of attributes that make up the super key.
- It satisfies `uniqueness` for all tuples constituting the relation but does not satisfy `minimality`.

e.g., In a relation, student ID, social security number, (student ID, social security number), (student ID, social security number, name), etc., are super keys.

### Candidate Key
- A subset of attributes among those constituting a relation, used to uniquely identify tuples.
- Satisfies both uniqueness and minimality.

e.g.,
1. In the `<Student>` relation, student ID or social security number satisfy uniqueness and minimality, thus becoming candidate keys.
2. In the `<Enrollment>` relation, the combination of student ID and course name satisfies uniqueness and minimality, so the combination (student ID, course name) becomes a candidate key.

> A key formed by combining two or more fields, such as student ID and course name, is called a composite key.

### Primary Key
- A key specially selected from candidate keys, which cannot have duplicate values.
- It possesses the properties of a candidate key, meaning it has uniqueness and minimality, and is an essential key for identifying tuples.
- Cannot have NULL values. That is, attributes set as primary keys in a tuple must not contain NULL values.

e.g.,
1. In the `<Student>` relation, either student ID or social security number (which are candidate keys) can be selected as the primary key. In the `<Enrollment>` relation, student ID and course name must be combined to set a primary key.
2. If student ID is defined as the primary key in the `<Student>` relation, the already entered student ID `190001` cannot be entered as the student ID attribute value for another tuple.

### Alternate Key
- Refers to the remaining candidate keys after a primary key has been selected from the set of candidate keys.

e.g.,
1. If student ID is set as the primary key in the `<Student>` relation, then social security number becomes an alternate key.

### Foreign Key
- Refers to an attribute or set of attributes that references the primary key of another relation.
- Used to express relationships between relations.
- When attribute A in one relation and primary key B in a referenced relation are defined over the same domain, attribute A is called a foreign key.
- Has the same key attributes as the primary key of the referenced relation.
e.g.,
1. The student ID in the `<Enrollment>` relation references the student ID, which is the primary key of the `<Student>` relation. Therefore, the student ID in the `<Enrollment>` relation becomes a foreign key.
2. You cannot enter a value into the student ID attribute of the `<Enrollment>` relation that does not exist in the student ID attribute of the `<Student>` relation.
3. The `<Student>` relation and the `<Enrollment>` relation have a relationship established based on student ID.

<br>

### Uniqueness
- A single key value must be able to uniquely identify only one tuple.

### Minimality
- If one attribute constituting the key is removed, it should no longer be able to uniquely identify, meaning it must be composed of the minimum necessary attributes. However, just because a student ID or social security number satisfies minimality does not mean it is not a super key. A super key only needs to uniquely distinguish tuples, regardless of minimality.
- Candidate keys are those super keys that satisfy both uniqueness and minimality.
