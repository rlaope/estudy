# Breadth-First Search (BFS)

Graph traversal algorithms typically include DFS and BFS. Today, we'll explore BFS.

## BFS
A representative graph traversal algorithm

A method that first explores nodes at the same level as the current vertex (sibling nodes)

**Move as broadly as possible, then move down when there are no more options horizontally**

For example, it's an algorithm used to determine if one city can be reached from another, or if specific terminals in an electronic circuit are connected.

### Node Traversal Order in BFS

![](https://velog.velcdn.com/images%2Fsukong%2Fpost%2F103fbeed-3f70-4074-9a7d-76915a7764f2%2FBFS.png)

Because it's a breadth-first search, it explores all nodes at the shallowest depth first, then moves to deeper nodes. That is, in the diagram, it first explores nodes 1 and 2 at depth 1. Once all nodes at depth 1 are explored, it then explores nodes 3, 4, 5, and 6 at depth 2.

### Characteristics

- It's a good method for finding the shortest path between two nodes, as it explores distant nodes later.
- It uses a queue to store the order of nodes to be explored and explores them in the order they were stored in the queue. A queue is used because it requires a First-In, First-Out (FIFO) approach.

### BFS Implementation Algorithm

1. Start from the root node.
2. Add nodes that are adjacent to the root node, have not been visited, and are not already in the Queue, to the Queue.
3. Dequeue from the Queue and visit the node that was added to the queue first.

![](https://velog.velcdn.com/images%2Fsukong%2Fpost%2Fc64d33a0-6e43-43be-9c44-5937f9bf40e3%2Fimage.png)

1. Visit the root node at step 1.
2. At steps 2, 3, 4, and 5, add adjacent nodes that have not been visited and are not already in the queue, to the queue.
3. At step 6, move to node 1 (the first node stored in the queue) and check the conditions of its adjacent nodes.

Repeat this process until there are no more nodes in the queue.

### Graph Implementation Methods

1. Adjacency Matrix
2. Adjacency List

Let's take an example.

![](https://velog.velcdn.com/images%2Fsukong%2Fpost%2Fc209c54d-de4d-4ec3-9914-b70624cfeabd%2F%EA%B7%B8%EB%9E%98%ED%94%84%EC%9D%B4%EB%AF%B8%EC%A7%80.png)

When implementing a graph like the one above, it can be done using an adjacency matrix or an adjacency list, as follows.

An adjacency matrix can be implemented as a 2D array, while an adjacency list can be implemented using an array of linked lists, an array of ArrayLists, or an ArrayList of ArrayLists, among other methods.

![](https://velog.velcdn.com/images%2Fsukong%2Fpost%2F392b382a-5e93-4d94-9f1f-151976032f26%2F%EC%9D%B8%EC%A0%91%ED%96%89%EB%A0%AC%2C%20%EC%9D%B8%EC%A0%91%EB%A6%AC%EC%8A%A4%ED%8A%B82.png)

### Code for BFS Implemented with Adjacency Matrix

Structure required for adjacency matrix implementation

1. Adjacency matrix array (int[][] graph)
2. Visited status array (boolean[] visited)
3. Queue (Queue queue)
4. Array to store visited nodes in order (ArrayList arrList)

```java
static void bfs(int node) {
    visited[node] = true;
    arrList.add(node);

    for(int i = 1; i <= nodeNum; i++){
        if(graph[node][i] == 1 && visited[i] == false && queue.contains(i) == false) {
            queue.add(i);
        }
    }
    if(!queue.isEmpty())
        bfs(queue.poll());
}
```

### Code for BFS Implemented with Adjacency List

Structure required for adjacency list implementation

1. Adjacency list (ArrayList[] graph)
2. Visited status array (boolean[] visited)
3. Queue (Queue queue)
4. Array to store visited nodes in order (ArrayList arrList)

```java
static void bfs(int node) {
    visited[node] = true;
    arrList.add(node);

    for(int i = 0; i < graph[node].size(); i++){
        int adjNode = graph[node].get(i);
        if(visited[adjNode] == false && queue.contains(adjNode) == false) {
            queue.add(adjNode);
        }
    }

    if(!queue.isEmpty())
        bfs(queue.poll());
}
```
