# Graph

A graph is a data structure that expresses the relationships between connected elements.

### Concept of a Graph
Simply a data structure that collects nodes and the edges connecting them.

- In other words, it's a data structure that can represent relationships between connected objects.
  - e.g., maps, shortest paths in subway lines, components in electrical circuits, roads, prerequisite courses, etc.
- A graph can consist of multiple isolated subgraphs.

![](image/differgraph.png)

### Reference
- Eulerian Path
  - Refers to a path that traverses every edge in a graph exactly once and returns to the starting vertex.
  - An Eulerian path exists only when the number of edges connected to every vertex in the graph is even.

### Graph-related Terms
- Vertex: The concept of a location (same meaning as node)
- Edge: The relationship between locations; a line connecting nodes (also called link, branch)
- Adjacent Vertex: A vertex directly connected by an edge
- Degree of a Vertex: In an undirected graph, the number of vertices adjacent to a single vertex
  - Sum of all degrees of vertices in an undirected graph = 2 * number of edges in the graph
- In-degree: In a directed graph, the number of incoming edges from outside
- Out-degree: In a directed graph, the number of outgoing edges
  - Sum of in-degrees or out-degrees of vertices in a directed graph = number of edges in the directed graph
- Path Length: The number of edges used to form a path
- Simple Path: A path where no vertex is repeated
- Cycle: A simple path where the starting and ending vertices are the same

### Characteristics of a Graph
- A graph is a **network model**.
- Two or more paths are possible.
  - That is, nodes can have bidirectional paths in undirected/directed graphs.
  - Both self-loops and general loops/circuits are possible.
  - There is no concept of a root node.
  - There is no concept of a parent-child relationship.
  - Traversal is performed using DFS and BFS.
  - A graph is either cyclic or acyclic.
  - Graphs are broadly categorized into directed graphs and undirected graphs.
  - The presence or absence of edges varies depending on the graph.

### Types of Graphs

#### Undirected vs. Directed
- Undirected Graph
  - Edges in an undirected graph allow movement in both directions.
  - An edge connecting vertex A and vertex B is represented as a pair of vertices, such as (A,B).
    - (A,B) == (B,A)
    - e.g., two-way roads
- Directed Graph
  - A graph where edges have directionality.
  - An edge that can only go from A to B is represented as <\A,B>.
    - <\A,B> != <\B,A>
  - e.g., one-way street

<br>

### Weighted Graph
- A graph where costs or weights are assigned to edges.
- Also called a network.
    - e.g., city-to-city connections, road lengths, circuit component capacities, communication network usage fees, etc.

### Connected Graph vs. Disconnected Graph
- Connected Graph
  - A case where a path always exists for every pair of vertices in an undirected graph.
  - e.g., Tree: A connected graph without cycles
- Disconnected Graph
  - A case where no path exists between a specific pair of vertices in an undirected graph.

### Cycle vs. Acyclic Graph
- Cycle
  - A case where the starting and ending vertices of a simple path are the same.
  - Simple Path: A path where no vertex is repeated.
- Acyclic Graph
  - A graph without cycles.

### Complete Graph
- Complete Graph
  - A graph where all vertices belonging to the graph are connected to each other.
  - Undirected Complete Graph
    - Number of vertices: n, then number of edges: n * (n-1)/2
