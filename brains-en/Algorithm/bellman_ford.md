# Bellman-Ford Algorithm

![](https://velog.velcdn.com/images%2Fkimdukbae%2Fpost%2Fa80f3ef5-4540-45f2-9913-895b653755e3%2Fimage.png)

## What is the Bellman-Ford Algorithm?

The Bellman-Ford algorithm is an algorithm that finds the shortest path from one node to another.
  
**It can find the shortest path even when edge weights are negative.**
  
Dijkstra's algorithm also finds the shortest path. You might wonder, what then is Bellman-Ford? Let's explore the differences between Dijkstra and Bellman-Ford.

## Bellman-Ford vs. Dijkstra

![](https://velog.velcdn.com/images%2Fkimdukbae%2Fpost%2F66ce61df-775e-458c-83f1-119a2bcf46db%2Fimage.png)

Let's look at the image above. Assume we want to find the shortest path from node 1 to node 3. Visually, there are two paths from 1 to 3.
  
`1 -> 3 cost 10` and `1 -> 2 -> 3 cost 5`. The shortest path from node 1 to node 3 is 5.
  
Now, if we use Dijkstra's algorithm instead of visual inspection, it always selects the unvisited node with the shortest distance, leading it to choose the path `1 -> 3` with cost 10. As shown, if negative edges exist, situations can arise where the shortest path cannot be found.
  
On the other hand, if we use the Bellman-Ford algorithm, it checks all edges every time, allowing it to select the path `1 -> 2 -> 3` with cost 5 and find the shortest path.

### Dijkstra's Algorithm
- It iteratively finds the shortest path by selecting the unvisited node with the shortest distance at each step.
- It can find the optimal solution if there are no negative edges.
- It has a fast time complexity of `O(ElogV)`.

### Bellman-Ford Algorithm
- At each of the (V-1) steps, it checks all edges to find the shortest path between all nodes. The difference from Dijkstra's is that it checks all edges in every iteration (Dijkstra only visits the unvisited node with the closest shortest distance).
  - It always includes the optimal solution found by Dijkstra's algorithm.
- It can find the optimal solution even if negative edges exist.
- It has a slow time complexity of `O(VE)`.

If all edge costs are positive, use Dijkstra; if negative edges are included, use Bellman-Ford.

## Bellman-Ford Algorithm Execution Process
1. Set the starting node.
2. Initialize the shortest distance table.
3. Repeat the following process (V - 1) times.
   1. Check each of the E edges one by one.
   2. Calculate the cost to reach other nodes via each edge and update the shortest distance table.
4. If you want to check for negative cycle occurrences, perform step 3 one more time. -> If the shortest distance table is updated at this point, a negative cycle exists.
