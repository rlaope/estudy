# Persistence Context

### What is a Persistence Context?
- It means **an environment for permanently storing entities**. It acts like a virtual database that holds objects between the application and the database.
- When an entity is stored or retrieved through an Entity Manager, the Entity Manager stores and manages the entity in the Persistence Context.

em.persist(member);
- This means storing a member entity in the Persistence Context using the Entity Manager.

### Characteristics of a Persistence Context
1. One is created when an Entity Manager is created.
2. The Persistence Context can be accessed and managed through the Entity Manager.

<br>

### Entity Lifecycle
- Non-persistent (new/transient): A state completely unrelated to the Persistence Context.
- Persistent (managed): A state where the entity is stored in the Persistence Context.
- Detached: A state where the entity was stored in the Persistence Context but has since been separated.
- Removed: A state where the entity has been deleted.

![](./image/entitylife.png)

#### Non-persistent
An entity object has been created but not yet stored in the Persistence Context.
```java
Member member = new Member();
```

#### Persistent
This refers to the state where an entity is stored in the Persistence Context via the Entity Manager, meaning it is managed by the Persistence Context.

```java
em.persist(member)
```

#### Detached
If the Persistence Context no longer manages a persistent entity, it becomes detached. To make a specific entity detached, `em.detach()` must be called.

```java
// Detaches the entity from the Persistence Context, making it detached.
em.detach(member);
// Even if the Persistence Context is cleared, managed entities become detached.
em.claer();
// Even if the Persistence Context is closed, managed entities become detached.
em.close();
```
**Characteristics of a detached state**
- None of the features provided by the Persistence Context, including the first-level cache, transactional write-behind, dirty checking, and lazy loading, operate.
- It still holds its identifier value.


#### Removed
Deletes the entity from the Persistence Context and the database.
```java
em.remove(member);
```

<br>

### Characteristics of the Persistence Context

**Identifier value of the Persistence Context**
The Persistence Context distinguishes entities by their identifier values. Therefore, a persistent state must always have an identifier value.

**Persistence Context and database storage**
JPA typically reflects newly stored entities in the Persistence Context to the database the moment a transaction is committed. This is called flushing.

**Advantages of the Persistence Context managing entities**
1. First-level cache
2. Identity guarantee
3. Transactional write-behind
4. Dirty checking
5. Lazy loading

#### First-level cache
Inside the Persistence Context, there is a cache called the first-level cache. Persistent entities are stored here. The key of the first-level cache is the identifier value (the primary key of the database), and the value is the entity instance. The lookup method is as follows:

```java
// em.find(entity class type, identifier value);
Member member = em.find(Member.class, "member1");
```

**Lookup flow**
1. Search for the entity in the first-level cache.
2. If found, retrieve the entity from the first-level cache in memory.
3. If not found, retrieve it from the database.
4. Create an entity with the retrieved data and store it in the first-level cache. (This makes the entity persistent.)
5. Return the retrieved entity.

#### Identity guarantee for persistent entities

The Persistence Context guarantees the identity of entities.

```java
Member a = em.find(Member.class, "member1");
Member b = em.find(Member.class, "member1");
System.out.print(a==b) // true
```
Identity comparison: The actual instances are the same. Compared using `==`.
Equality comparison: The actual instances may be different, but the values held by the instances are the same. Compared by implementing the `equals()` method.

#### Transactional write-behind

Even if `em.persist(member)` is used to save a member, the INSERT SQL is not immediately sent to the DB. The Entity Manager collects INSERT SQL statements in an internal query store until just before the transaction is committed. Then, when the transaction is committed, it sends the collected queries to the DB. This is called transactional write-behind.

#### Dirty checking
When modifying an entity with JPA, you simply retrieve the entity and change its data.

Dirty checking flow
1. When the transaction is committed, `flush` is first called internally by the Entity Manager.
2. It compares the entity with its snapshot to find changed entities.
3. If there are changed entities, it generates update queries and stores them in the transactional write-behind SQL store.
4. It flushes the SQL from the write-behind store.
5. It commits the database transaction.

Dirty checking only applies to persistent entities managed by the Persistence Context.

```java
EntityManager em = emf.createEntityManager();
EntityTransaction transaction = em.getTransaction();
transaction.begin();

Member member = em.find(Member.class, "member1");
member.setName("박주홍");

transaction.commit();
```
