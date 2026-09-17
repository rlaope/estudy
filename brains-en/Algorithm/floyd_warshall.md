# Floyd-Warshall Algorithm

If you want to find the shortest path from every vertex to every other vertex, use the Floyd-Warshall algorithm.
  
While Dijkstra's algorithm requires selecting the lowest cost one by one, the Floyd-Warshall algorithm is characterized by performing its operations based on intermediate vertices.
  
Similarly, Floyd-Warshall is also fundamentally based on dynamic programming techniques.

> The core idea of the Floyd-Warshall algorithm is to find the shortest distance based on intermediate vertices.

- An algorithm that can be used to find all shortest paths from all points to all other points.

- Its source code is very short compared to Dijkstra's, making it easy to implement.
- In Dijkstra's case, it iteratively selects one node with the shortest distance at each step. It then updates the shortest distance table by checking paths through that node. <-> The Floyd-Warshall algorithm also performs its operations based on intermediate nodes at each step. However, there's no need to find the node with the shortest distance among unvisited nodes at each step.
- Floyd-Warshall stores shortest distance information in a 2D table (because it needs to store the shortest distance from all points to all other points). <-> Dijkstra's stores it in a 1D list because it's the shortest distance from one point to other points.
