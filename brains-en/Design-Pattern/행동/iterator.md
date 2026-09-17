# Iterator Pattern

The Iterator pattern is a design pattern that provides a way to traverse a collection of objects more easily **without knowing its specific collection type**.

Representative examples include Java's Iterator interface, Spring's CompositeIterator, and StAX.

## Example

For example, let's say we have a list containing Book entities. If we want to sort these entities by time, we would use `Collections.sort()` with a specified condition to sort them, and then iterate through them one by one using a for loop, referencing by index or calling `get()` to print them.

Instead, by implementing an iterator interface like `RecentPostIterator`, we can provide object traversal functionality as follows.

```java
public class RecentPostIterator implements Iterator<Post> {

    private Iterator<Post> internalIterator;

    public RecentPostIterator(List<Post> posts) {
        Collections.sort(posts, (p1, p2) -> p2.getCreatedDateTime().compareTo(p1.getCreatedDateTime()));
        this.internalIterator = posts.iterator();
    }

    @Override
    public boolean hasNext() {
        return this.internalIterator.hasNext();
    }

    @Override
    public Post next() {
        return this.internalIterator.next();
    }

}
```

In this way, if we use an iterator interface, we can traverse objects without knowing the specific collection type (Set, List), which simplifies the traversal method.

### Pros and Cons

Of course, if we create a new Iterator class for each traversal method and perform the traversal, there is a disadvantage of slower performance,

but it becomes easier to maintain and more accessible, and **by delegating the responsibility of object traversal to the iterator class**, cohesion is reduced, enabling a more extensible code design.
