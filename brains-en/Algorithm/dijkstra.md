# Dijkstra's Algorithm

Dijkstra's algorithm is a representative **shortest path finding algorithm** that utilizes dynamic programming.

It is commonly used in satellite GPS software and similar applications.

Dijkstra's algorithm finds the shortest path from a specific single vertex to all other vertices.

However, it cannot include negative edges. Of course, since negative edges do not exist in the real world, Dijkstra's algorithm is very suitable for real-world applications.

The reason Dijkstra's algorithm is a dynamic programming problem is because **a shortest path is composed of multiple shortest paths.**

It can be seen that smaller problems are subsets of larger problems.
Fundamentally, Dijkstra's algorithm has the characteristic of directly using the shortest path information calculated up to that point when finding a single shortest path.
