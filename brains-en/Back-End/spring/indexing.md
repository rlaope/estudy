### How to Index Databases in Spring Boot

### Indexing Operations
Indexing operations are highly inefficient when modifications occur. Please keep this in mind.

### How to Configure Indexes
Above your entity class, add:

`@Table(indexes = @Index(name = "인덱스 명", columnList = "인덱싱 할 컬럼"))`
You can attach it like this.
