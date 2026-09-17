# B-Tree Data Deletion

Let's explore data deletion in B-Trees.

Data deletion in B-Trees has the following characteristics:
1. Deletion always occurs in a leaf node.
2. If the number of keys in a node falls below the minimum key count after deletion, a rebalancing operation is performed.
    1. Additionally, if a parent node meets the rebalancing condition during the deletion process, rebalancing is performed starting from that node.

> The formula for the minimum number of keys in each node of an M-ary B-Tree is ⌈M/2⌉-1 (excluding the root node).

Let's examine the following cases to understand this.

1. Deleting from a leaf node without needing rebalancing
2. Two cases where deleting from a leaf node requires rebalancing
3. Deleting from an internal node

<br>

### Deleting from a Leaf Node Without Needing Rebalancing

**Simply put, if the number of keys in a node remains at or above the minimum key count after data deletion, no rebalancing is necessary.**

![](https://velog.velcdn.com/images/chanyoung1998/post/5d2911a7-733a-4e95-a4cc-a1f2302b96f2/image.png)

<br>

### Deleting from a Leaf Node Requiring Rebalancing: Case 1

**First, request support from a sibling. If a sibling node has spare keys, support is received from that sibling (typically, the left sibling node is asked for help first).**

In the figure below, deleting 31 causes the node to fall below the minimum key count, thus requiring rebalancing.

![](https://velog.velcdn.com/images/chanyoung1998/post/3a174208-6542-4a73-9975-28c30dd99eb8/image.png)

Looking at 31's left sibling node, it has keys 25 and 28, so it has spare capacity. It provides a key, but instead of directly moving 28, 28 is moved to the parent, and 30 is moved down, in accordance with B-Tree properties (sorting).

<br>

### Deleting from a Leaf Node Requiring Rebalancing: Case 2

**If sibling nodes do not have spare keys to provide, support is received from the parent, and the node is merged with a sibling.**

**Method of receiving support from the parent**
1. Merge the parent's key and my keys sequentially into the left node.
2. Delete my node.

If the left node (sibling node) does not exist, merge the parent's key and the right node's (sibling node's) keys sequentially into my node, and then delete the right node.

![](https://velog.velcdn.com/images/chanyoung1998/post/1468cd33-2f6d-45f0-b67e-bc78a14c17c8/image.png)

In the figure above, deleting 30 causes the node to fall below the minimum key count, thus requiring rebalancing.

Although an attempt was made to get support from a sibling with spare keys, the siblings also only had the minimum number of keys. Therefore, support is received from the parent node's 28, and it is merged with the left node.

If, in this process, the parent node provides support and then also requires rebalancing, the rebalancing process is performed again from that parent node.

1. If the parent is not the Root node, proceed with rebalancing.
2. If the parent node is the Root and the node becomes empty, delete the parent node. (The node that was merged just before then becomes the root node.)

<br>

### Deleting Data from an Internal Node

If an internal node is to be deleted, its data is swapped with data from a leaf node, and then the deletion proceeds.

The data is swapped with a leaf node's data by **choosing either its predecessor or successor.**
- Predecessor: The largest data among those smaller than me
- Successor: The smallest data among those larger than me

When deleting 33, since 33 is not a leaf node, its predecessor is found.

Since 33's predecessor is 32, 32 and 33 are swapped, and then the deletion proceeds.

![](https://velog.velcdn.com/images/chanyoung1998/post/bb41nfh6-f4db-45f0-b67e-bc78a14c17c8/image.png)
