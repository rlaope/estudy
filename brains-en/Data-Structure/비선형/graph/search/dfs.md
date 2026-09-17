# Depth-First Search (DFS)

### Graph Traversal
- The process of visiting all vertices one by one, starting from a single vertex.
- e.g., checking if one city can be reached from another, or if specific terminals in an electronic circuit are connected.

### What is Depth-First Search?
A method that starts from a root node or any arbitrary node and completely explores a branch before moving on to the next branch.

- It's similar to exploring a maze: you keep going in one direction until you can't go any further, then you return to the nearest fork and continue exploring in a different direction from there.
- In other words, it explores deeply before exploring broadly.
- When to use: Choose this method when you want to **visit all nodes**.
- Depth-first search is generally simpler than breadth-first search.
- The raw search speed itself is slower compared to breadth-first search.

### Characteristics of Depth-First Search
- It has a recursive algorithm structure that calls itself.
- Other forms of tree traversal, including pre-order traversal, are all types of DFS.
- The biggest difference when implementing this algorithm is that, for graph traversal, you must check whether a node has been visited.
  - Failure to check this risks falling into an infinite loop.

![](../image/dfs.png)

1. Visit node 'a', the starting node, then mark the visited node as visited.
2. Traverse the nodes adjacent to 'a' in order; if there are no adjacent nodes, terminate.
3. If you visit node 'b', which is adjacent to 'a', you must visit all of 'b's neighbors before visiting another node adjacent to 'a'.
   - Restart DFS with 'b' as the starting vertex to visit 'b's neighbors.
4. Once all branches of 'b' have been completely explored, find an unvisited vertex among those adjacent to 'a'.
    - This means you can only visit other neighbors of 'a' after all branches of 'b' have been completely explored.
    - If there are no unvisited vertices, terminate.
    - If there are, restart DFS with that vertex as the starting vertex.

### Implementation Code

```java
oid search(Node root) {
  if (root == null) return;
  // 1. root 노드 방문
  visit(root);
  root.visited = true; // 1-1. 방문한 노드를 표시
  // 2. root 노드와 인접한 정점을 모두 방문
  for each (Node n in root.adjacent) {
    if (n.visited == false) { // 4. 방문하지 않은 정점을 찾는다.
      search(n); // 3. root 노드와 인접한 정점 정점을 시작 정점으로 DFS를 시작
    }
  }
}
```

```java
순환 호출을 이용한 DFS 구현 (java 언어)
import java.io.*;
import java.util.*;

/* 인접 리스트를 이용한 방향성 있는 그래프 클래스 */
class Graph {
  private int V;   // 노드의 개수
  private LinkedList<Integer> adj[]; // 인접 리스트

  /** 생성자 */
  Graph(int v) {
      V = v;
      adj = new LinkedList[v];
      for (int i=0; i<v; ++i) // 인접 리스트 초기화
          adj[i] = new LinkedList();
  }

  /** 노드를 연결 v->w */
  void addEdge(int v, int w) { adj[v].add(w); }

  /** DFS에 의해 사용되는 함수 */
  void DFSUtil(int v, boolean visited[]) {
      // 현재 노드를 방문한 것으로 표시하고 값을 출력
      visited[v] = true;
      System.out.print(v + " ");

      // 방문한 노드와 인접한 모든 노드를 가져온다.
      Iterator<Integer> i = adj[v].listIterator();
      while (i.hasNext()) {
          int n = i.next();
          // 방문하지 않은 노드면 해당 노드를 시작 노드로 다시 DFSUtil 호출
          if (!visited[n])
              DFSUtil(n, visited); // 순환 호출
      }
  }

  /** 주어진 노드를 시작 노드로 DFS 탐색 */
  void DFS(int v) {
      // 노드의 방문 여부 판단 (초깃값: false)
      boolean visited[] = new boolean[V];

      // v를 시작 노드로 DFSUtil 순환 호출
      DFSUtil(v, visited);
  }

  /** DFS 탐색 */
  void DFS() {
      // 노드의 방문 여부 판단 (초깃값: false)
      boolean visited[] = new boolean[V];

      // 비연결형 그래프의 경우, 모든 정점을 하나씩 방문
      for (int i=0; i<V; ++i) {
          if (visited[i] == false)
              DFSUtil(i, visited);
      }
  }
}
/** 사용 방법 */
public static void main(String args[]) {
    Graph g = new Graph(4);

    g.addEdge(0, 1);
    g.addEdge(0, 2);
    g.addEdge(1, 2);
    g.addEdge(2, 0);
    g.addEdge(2, 3);
    g.addEdge(3, 3);

    g.DFS(2); /* 주어진 노드를 시작 노드로 DFS 탐색 */
    g.DFS(); /* 비연결형 그래프의 경우 */
}
```

- Time Complexity of Depth-First Search (DFS)

- DFS traverses all edges of a graph (number of vertices: N, number of edges: E).
  - Graph represented by an adjacency list: O(N+E)
  - Graph represented by an adjacency matrix: O(N^2)
- Therefore, for a sparse graph with a small number of edges, using an adjacency list is more advantageous than an adjacency matrix.
