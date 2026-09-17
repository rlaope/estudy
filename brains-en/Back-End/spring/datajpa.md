# How Spring Data JPA Works

Spring Data JPA is a Spring Data module that helps developers easily implement JPA-based data access layers. It's a module commonly used with the Spring Framework for development utilizing RDB-based databases.

Spring JPA is a module that extends Hibernate's functionality to fit the Spring Framework. Therefore, its basic structure and operation are based on Hibernate's implementation.

### Persistence Context

A term frequently encountered with JPA is "Persistence Context," often referred to as the 1st-level cache. It manages and caches entities loaded from or to be loaded into the database. It has a 1:1 relationship with an EntityManager or Session. A Session is an implementation of an EntityManager and is created when a connection to the database is established.

![pc](image.png)

> This explanation is based on `SessionImpl`, the implementation of Sessions/EntityManager, and `StatefulPersistenceContext`, the implementation of PersistenceContext.

<br>

### Operations that occur when `jpaRepository.save()` is called

What operations take place when `JpaRepository.save()` is called?

When this method is called to save a newly created entity, `persist()` of `SessionImpl` is invoked. Hibernate operates on an event-driven model, where each event has an `EventListener Group` and default listeners are registered.

`persist` is also published as an event, and `DefaultPersistEventListener` is registered as the default listener. When `persist` is called, it's passed as a `PersistEvent`, an `AbstractEvent` implementation, which then calls `onPersist` of `DefaultPersistEventListener` within the persist event group. The `DefaultPersistEventListener` performs the entity persistence operation. Persistence can be understood as an entity becoming a `ManagedEntity` that is managed within the persistence context.

![alt text](image-2.png)

Persisted entities are managed in a list data structure within the `EntityEntryContext`. The most recently persisted entity is at the tail, and the first persisted entity is at the head.

Next, for a new entity to be persisted, a database insert must occur. At this point, the insert command is not executed immediately; instead, an `EntityInsertAction` is enqueued into the **ActionQueue**. In some cases, it might be executed immediately.

The `ActionQueue` contains actions such as INSERT, UPDATE, and DELETE. The secret to Spring JPA's lazy writing is precisely this `ActionQueue`.

![alt text](image-3.png)

Once all persistence operations are complete, `Session`'s `flush` is called. This passes a `FlushEvent` to the `Flush Event Group`, which then calls `onFlush` of `DefaultFlushEventListener`. The `DefaultFlushEventListener` executes `executeActions` of the `ActionQueue`, running all actions contained within the `ActionQueue` in a batch. The `execute` function of each `Action` is run, and an INSERT/UPDATE/DELETE statement is generated and executed depending on the type of `Action`.

![alt text](image-4.png)

In summary:

1.  When connected to the database, a session or entity manager is created.
2.  An event to persist a new entity is published.
3.  The entity is registered as a managed entity within the session's persistence context.
4.  An `EntityInsertAction` is added to the `ActionQueue`.
5.  A `Flush` event is published.
6.  Actions in the `ActionQueue` are executed in a batch.
7.  An INSERT statement is executed.

![alt text](image-1.png)
