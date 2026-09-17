# B-Tree

Let's learn about the B-Tree data structure.

First, a B-Tree is **one of the Balanced Trees that automatically balances itself so that all leaf nodes are at the same level.**

> In actual databases, B+Trees are used, which are an evolution of B-Trees.

### Characteristics

- To increase the maximum number of child nodes, a parent node stores one or more keys.
- The keys in the parent node are **sorted in ascending order.**
- The range of key values for child nodes is determined by the sorted order.

The maximum number of child nodes a B-Tree can have is an important parameter when using it.

Let's say `M` is the maximum number of child nodes for each node.

A B-Tree that can have a maximum of M children is called an M-ary B-Tree.

Here, the maximum number of keys in each node is `M - 1`.

The minimum number of child nodes in each node is `⌈ M / 2 ⌉`. (Always round up the value divided by 2, e.g., 1.5 -> 2)

The minimum number of keys in each node is `⌈ M / 2 ⌉ - 1`. **This condition excludes leaf nodes and the root node.**

To summarize:

- M | Maximum number of child nodes per node
- M - 1 | Maximum number of keys per node
- ⌈ M / 2 ⌉ | Minimum number of child nodes per node
- ⌈ M / 2 ⌉ - 1 | Minimum number of keys per node

Additionally, if an internal node has x keys, the number of child nodes must always be x + 1.

![](https://velog.velcdn.com/images%2Femplam27%2Fpost%2Fddbae2c9-da94-457d-bad8-77ff6791255b%2FB%ED%8A%B8%EB%A6%AC%20%EA%B8%B0%EB%B3%B8%20%ED%98%95%ED%83%9C.png)
B-Tree Basic Structure

Since each node has at least one key, an internal node always has at least two children, regardless of the B-Tree's order.

<br>

## B-Tree Key Search Process

Starting from the root node, a **top-down search** is performed.

Here's the search process when the key to be searched is k.

1. Start from the root node and iterate through the keys.
    1. If a key equal to k is found, terminate the search.
    2. Compare the search value with the key. If k falls between keys, descend to the child node between those keys.
2. Repeat the above process **until a leaf node is reached**. If k is not found even in the leaf node, the search fails.

![](https://velog.velcdn.com/images%2Femplam27%2Fpost%2Fb7df8287-2524-4ec0-ad03-b969a8830c8e%2FB%ED%8A%B8%EB%A6%AC%20%EA%B2%80%EC%83%89%201.png)
B-Tree Search 1
<br>
<br>

## B-Tree Data Insertion

Data is always added to a leaf node.

If a node overflows, the keys are split left and right based on the median key, and the median key is promoted.
