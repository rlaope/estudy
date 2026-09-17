# Lock-Free Data Structures and STM

While CAS enables lightweight concurrency by performing atomic get-set operations without acquiring a separate monitor lock, it suffers from CPU resource waste, cache thrashing in multi-core environments, starvation unfairness, and limitations in manipulating multiple variables.

Implementing lock-free data structures with multiple intertwined pointers, beyond simple single-variable counting, is extremely difficult. The core challenge is **how to make multi-step modifications appear as a single atomic operation.**

Let's explore common approaches like implementing lock-free data structures or using STM to ensure atomicity for operations involving multiple variables.

### Michael-Scott Queue

This is the most standard lock-free queue algorithm. It served as the basis for `java.util.concurrent.ConcurrentLinkedQueue`.

**Complexity**: It requires managing two pointers: `head` and `tail`. When inserting a new node, you must update the `next` pointer of the node currently pointed to by `tail`, and then move `tail` itself. Since another thread can intervene between these two steps, the algorithm includes a Helping logic that uses CAS to detect intermediate states and allows other threads to assist in completing the operation.

```cpp
#include <atomic>

template<typename T>
class LockFreeQueue {
private:
    struct Node {
        T data;
        std::atomic<Node*> next;
        Node(T val) : data(val), next(nullptr) {}
    };
    std::atomic<Node*> head;
    std::atomic<Node*> tail;

public:
    void enqueue(T value) {
        Node* newNode = new Node(value);
        while (true) {
            Node* currentTail = tail.load();
            Node* tailNext = currentTail->next.load();
            
            if (currentTail == tail.load()) {
                if (tailNext == nullptr) {
                    // 1. Attempt to change Tail's next to the new node (CAS)
                    if (currentTail->next.compare_exchange_weak(tailNext, newNode)) {
                        // 2. On success, move the Tail pointer to the new node (CAS)
                        tail.compare_exchange_strong(currentTail, newNode);
                        return;
                    }
                } else {
                    // Another thread added a node but failed to update Tail (Helping)
                    tail.compare_exchange_strong(currentTail, tailNext);
                }
            }
        }
    }
};
```

### Harris's Linked List (Lock-Free Linked List)

Deleting a node in a linked list is tricky.

**Complexity**: If you have nodes a-b-c and want to delete b, you need to change a's `next` to c. However, in that brief moment, if another thread tries to insert a new node d after b, it might modify b's `next`, leading to data loss or a corrupted structure.

**Solution**: Use a Logical Delete technique. The last bit of the pointer is marked atomically to indicate that "this node will be deleted soon," and the actual physical link change occurs later.

```cpp
#include <atomic>
#include <cstdint>

struct Node {
    int value;
    std::atomic<Node*> next;
};

// Helper function to mark the last bit of a pointer as 1 to indicate a 'logical delete' state
Node* getMarkedReference(Node* ref) {
    return reinterpret_cast<Node*>(reinterpret_cast<uintptr_t>(ref) | 1);
}

bool logicalDelete(Node* target, Node* targetNext) {
    // Attempt to mark the target node's next pointer for deletion (CAS)
    Node* markedNext = getMarkedReference(targetNext);
    
    // If target->next is still targetNext, change it to markedNext (on success, it becomes deleted)
    return target->next.compare_exchange_strong(targetNext, markedNext);
}
```

### Lock-free Skip List

This is a sorted data structure often used in lock-free environments instead of binary search trees.

**Complexity**: Since it requires modifying links across multiple levels simultaneously, implementing it with only CAS is extremely complex. However, it offers excellent scalability because insertions and deletions only affect specific regions.

```cpp
#include <atomic>

struct Node {
    int key;
    // Multi-level link array (simplified to 1 level for illustration)
    std::atomic<Node*> next[1]; 
};

bool insertBottomLevel(Node* pred, Node* succ, Node* newNode) {
    newNode->next[0].store(succ);
    
    // If the predecessor node (pred) still points to the successor node (succ),
    // insert my node (newNode) in between them (CAS)
    return pred->next[0].compare_exchange_strong(succ, newNode);
    // If it fails, another thread must have added something after pred, so we need to retry the search.
}
```

#### Memory Reclamation Issues: Hazard Pointer, EBR

The true hidden complexity of lock-free data structures lies in safely reclaiming memory that might still be referenced by someone.

-   **Hazard Pointers**: A technique where a thread declares pointers it is currently accessing as "hazardous," preventing other threads from deallocating that memory.
-   **Epoch-based Reclamation (EBR)**: Divides operations into epochs. Memory is reclaimed in batches only after all threads have confirmed they have moved past a certain epoch.

## Software Transactional Memory STM

STM (Software Transactional Memory) is a concurrency control model that brings the concept of transactions to the memory domain. Instead of writing complex lock or CAS logic directly, developers declare a block of code that should execute atomically.

### atomic { ... }

Developers designate specific code blocks as transactions instead of dealing with complex synchronization logic.

-   **Optimistic Concurrency**: It optimistically assumes that other threads will not interfere and freely performs operations within the transaction.
-   **Rollback & Retry**: When attempting to commit the work, it checks if the memory addresses it read or wrote have been changed by another thread. If a conflict occurs, it rolls back the changes and retries from the beginning.

Working Mechanism:

1.  **Read Set / Write Set**: Values read and values to be written within the transaction are recorded in separate logs. Actual memory is not touched yet.
2.  **Validation**: Immediately before committing, it verifies if the current values of the addresses recorded in the read set are the same as they were at the start of the transaction.
3.  **Commit**: If validation succeeds, the contents of the Write Set are applied to physical memory all at once.

#### Pros and Cons

-   **Pros**: No deadlocks, and it's very easy to write logic that modifies multiple memory locations simultaneously. Its composability is excellent, making it easy to combine small transactions into larger ones.
-   **Cons**: In high-contention environments, frequent rollbacks can lead to worse performance degradation than CAS spinlocks. Also, logging all memory accesses incurs higher read/write overhead.

```clojure
;; STM using Clojure (the most representative and elegant STM implementation)
;; Bank Transfer Example

;; Create refs (memory spaces managed by STM)
(def account-A (ref 1000)) ; Account A: 1000 won
(def account-B (ref 0))    ; Account B: 0 won

(defn transfer [from to amount]
  ;; dosync block: all operations within this block are treated as a single atomic transaction
  (dosync
    (if (>= @from amount)
      (do
        ;; alter: attempt to change value within the transaction
        (alter from - amount)
        (alter to + amount)
        "Transfer successful")
      "Insufficient balance")))

;; Execution: Even if multiple threads call transfer concurrently,
;; Clojure's STM mechanism detects conflicts and performs rollback/retry in the background.
(transfer account-A account-B 100)
```

Examples of STM implementations include languages like Clojure and Haskell, and databases like Datomic.

You might ask, "Isn't this different from a database?" To answer that question, STM can be thought of as bringing the database transaction model directly onto the program's RAM memory.

The difference lies in the physical environment where they operate (memory vs. disk), and there are clear distinctions in their actual implementation and purpose.

**Database**: Must safely write changed data to disk (write-ahead logging, fsync) and guarantee durability even if the server loses power immediately after a commit. Rollback costs are very high due to disk I/O and network communication involved in conflicts. To avoid frequent rollbacks, databases actively use pessimistic locking or complex MVCC, operating in the millisecond range. No matter how fast, it's still storage.

**STM**: Manages only volatile memory states during program execution. Even if a commit succeeds, data disappears if the computer is turned off. This means it guarantees ACI (Atomicity, Consistency, Isolation) but sacrifices D (Durability). Rollbacks are cheap because it simply involves resetting local variables or pointers in memory. It primarily adopts an optimistic approach: modify freely, and if a conflict occurs during commit, just recalculate from scratch. Processing times are in the nano to microsecond range, synchronizing variable reads and writes by countless threads in real-time, aligned with CPU cache and RAM speeds.
