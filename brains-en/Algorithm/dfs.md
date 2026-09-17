# Depth-First Search (DFS)

## DFS
It is a method that starts from the root node and completely explores a branch before moving on to the next branch.

Graph traversal means starting from one vertex and visiting all vertices once in sequence.

For example, it is an algorithm used to determine if one city can be reached from another, or if specific terminals in an electronic circuit are connected.

### Node Traversal Order in DFS

![](https://velog.velcdn.com/images%2Fsukong%2Fpost%2Fb9042f15-fb5b-4272-abe7-8cdeb3f0f22f%2FDFS.png)

Since it's a depth-first search, it explores in one direction until there are no more adjacent nodes (down to the deepest node), and then explores in another direction.

### Characteristics of DFS

- It is a good method to use when all nodes need to be explored.
- Depth-First Search (DFS) is generally simpler than Breadth-First Search (BFS).
- The raw search speed itself is slower compared to Breadth-First Search (BFS).

### DFS Implementation Algorithm

1. Start from the root node.
2. Visit an unvisited node adjacent to the root node (down to the deepest node).
3. If there are no more adjacent and unvisited nodes (after visiting the deepest node), backtrack to a branching point and visit nodes in another direction.

It follows the sequence below:

![](https://velog.velcdn.com/images%2Fsukong%2Fpost%2F9beaa6b5-2713-451b-aa7d-5cfb2ab219d2%2Fimage.png)

1. Visit the root node at step 1.
2. Repeatedly visit adjacent, unvisited nodes at steps 2, 3, and 4.
3. If there are no more adjacent, unvisited nodes, find nodes in another direction (i.e., unexplored nodes) through backtracking.

### Graph Implementation Methods

1. Adjacency Matrix
2. Adjacency List

### Code Implemented with Adjacency Matrix

1. Adjacency matrix array (int[][] graph)
2. Visited status array (boolean[] visited)
3. Array to store visited nodes in order (ArrayList visitedArr)

```java
static void dfs(int node) {
    visited[node] = true;
    visitedArr.add(node);

    for(int i = 1; i <= nodeNum; i++){
        if(graph[node][i] == 1 && visited[i] == false) {
            dfs(i);
        }
    }
}
```

Time complexity of DFS implemented with adjacency matrix: O(N^2)

### Code Implementing DFS with Adjacency List

1. Adjacency list array (ArrayList[] graph)
2. Visited status array (boolean[] visited)
3. Array to store visited nodes in order (ArrayList visitArr)

```java
static void dfs(int node) {
    visited[node] = true;
    visitArr.add(node);

    for(int i = 0; i < graph[node].size(); i++) {
        int adjNode = graph[node].get(i);
        if(visited[adjNode] == false) {
            dfs(adjNode);
        }
    }
}
```
