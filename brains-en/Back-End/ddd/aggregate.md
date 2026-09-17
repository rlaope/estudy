# [DDD] Understanding Domain-Driven Design Aggregates

### Overview

Today, on the third day of studying Domain-Driven Design, we will explore Aggregates.

![](image/aggregate_0.png)

### What is an Aggregate?

An Aggregate is a concept that treats related **objects** as a single unit. In object-oriented programming, an Aggregate is one way to define relationships between objects, grouping logically related objects and treating them as a single bundle.

***Simply put, it's about bundling multiple objects and treating them as one large object.***

An Aggregate typically consists of Entities and Values. It includes a **root** entity that represents the relationship between entities and values, and this root entity defines the relationships with other objects belonging to the Aggregate. An Aggregate maintains **immutability** and encapsulates its internal implementation.

Aggregates are one of the important concepts in DDD design patterns. By using Aggregates, you can express complex relationships between objects more clearly and improve the maintainability and scalability of applications. Additionally, Aggregates can simplify database transaction processing and maintain consistency.

For example, you can group a forum post and its comments into an Aggregate. This allows for a clearer representation of the relationship between posts and comments and simplifies database processing.

### Role and Responsibilities of the Root Entity

The Aggregate root entity holds the core role and responsibilities of the Aggregate, and all objects within the Aggregate are managed centered around the Aggregate root entity.

**Key Responsibilities of the Aggregate Root Entity**

1. Manages and controls all objects within the Aggregate.
2. Ensures the immutability of the Aggregate root entity.
3. Manages the relationships between the Aggregate root entity and other objects.
4. Controls the entire Aggregate through the Aggregate root entity.

### Maintaining Immutability and Encapsulation in Aggregates

Aggregates must maintain immutability while adhering to encapsulation.

**Maintaining Immutability**

The state of an Aggregate must be changed through its root entity, and once created, an Aggregate's state does not change. Similarly, the internal objects of an Aggregate must also maintain immutability. This allows the Aggregate to protect its internal objects from external influence while maintaining consistency.

**Encapsulation**

This means protecting the Aggregate's internal objects from direct external access. The internal objects of an Aggregate should only be accessible through the Aggregate root entity, which allows the Aggregate to protect its internal objects from external influence while maintaining consistency.

The reasons for maintaining immutability and encapsulation are to protect the Aggregate's internal objects from external access while maintaining consistency, and to reduce coupling between objects, thereby enhancing maintainability and scalability.

**Implemented an Aggregate Root Entity In Kotlin**

```kotlin
@Entity
@Table(name = "order")
class Order(
    @Id
    val id: Long,
    val customerName: String,
    @OneToMany(mappedBy = "order", cascade = [CascadeType.ALL], orphanRemoval = true)
    val orderItems: List<OrderItem>
) {
    fun addItem(item: OrderItem) {
        orderItems.add(item)
    }
}

@Entity
@Table(name = "order_item")
class OrderItem(
    @Id
    val id: Long,
    val itemName: String,
    val price: Int,
    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "order_id")
    val order: Order
)
```

By setting the fields of `Order` as `val`, we can see that the principle of immutability is upheld. In the logic for adding items via `addItem`, `orderItem` is only called from `Order`, and the `orderItem` field is designed to prevent direct manipulation.

### Understanding Maintainability and Scalability of Aggregates

An Aggregate represents a collection of related objects bundled into a single logical unit.

This allows for **reducing dependencies between objects and simplifying complex relationships between them.** As a result, maintainability is improved. For example, changes occurring within an Aggregate do not affect the outside of the Aggregate, thus not impacting other parts when changes are made.

Aggregates are encapsulated to maintain immutability, preventing external modification. This enhances the scalability of the system. For instance, if an Aggregate's immutability is guaranteed, the object's state can remain consistent even when the system is used concurrently by multiple threads.

### Conclusion

Thus, we have explored the concept of Aggregates. Now, it seems I need to study DDD further by applying this in actual development.
