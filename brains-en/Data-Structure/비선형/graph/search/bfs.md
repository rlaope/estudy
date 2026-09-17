# Breadth-First Search (BFS)


### Breadth-First Search

**Breadth-First Search (BFS)** is a method that starts from the root node and explores adjacent nodes first.

### Graph Traversal
It is the process of visiting all vertices one by one, starting from a single vertex. For example, it is an algorithm used to determine if one city can be reached from another, or if specific terminals in an electronic circuit are connected to each other.

### Node Traversal Order in BFS

![](./image/bfsexam.png)

Since it's a breadth-first search, it explores all nodes at the shallowest depth first, then moves on to nodes at deeper levels.
That is, in the diagram, it first explores nodes 1 and 2, which are at depth 1. Once all nodes at depth 1 have been explored, it then explores nodes 3, 4, 5, and 6, which are at depth 2.

### Characteristics of BFS
- It is a good method for finding the shortest path between two nodes, as it explores distant nodes later.
- It uses a queue to store the order of nodes to be explored and explores them in the order they were stored in the queue. A queue is used because it requires a First-In, First-Out (FIFO) approach.

### BFS Implementation Algorithm

1. Start from the root node.
2. Add nodes that are adjacent to the root node, have not been visited, and are not already in the queue, to the Queue.
3. Dequeue from the Queue and visit the node that was added to the queue first.

![](./image/bfsal.png)

1. Visit the root node at step 1.
2. At steps 2, 3, 4, 5, add nodes that are adjacent, unvisited, and not in the queue, to the queue.
3. At step 6, move to the node that was added to the queue first (node 1) and check the conditions of its adjacent nodes.

Repeat this process until there are no nodes left in the queue.

<br>

### Graph Implementation Methods
1. Adjacency Matrix
2. Adjacency List

Let's take an example.

![](./image/bfs2.png)

When implementing a graph like the one above, it can be done using an adjacency matrix or an adjacency list as follows.
An adjacency matrix can be implemented as a 2D array, while an adjacency list can be implemented as an array of linked lists, an array of ArrayLists, or an ArrayList storing ArrayLists, etc.

![](./image/bfsimpl.png)

<br>

### BFS Code Implemented with Adjacency Matrix

Structures required for implementation with an adjacency matrix:
1. Adjacency matrix array
2. Visited status array
3. Queue
4. Array to store visited nodes in order

```java
static void bfs(int node) {
	BFSisVisited[node] = true;  //노드방문여부를 true로 저장
	BFSvisitArr.add(node);   //방문한 노드를 순서대로 저장하는 리스트에 해당노드 추가
	for( int i = 1 ; i <= nodeNum ; i++) {
		if( graph[node][i] == 1 && BFSisVisited[i] == false && queue.contains(i)==false) {
           		//인접, 방문된적X, 큐에저장되지X 를 만족하는 노드를 큐에 추가
			queue.add(i);
		}
	}	
	if(!queue.isEmpty())
		bfs(queue.poll());   //큐에 가장 먼저 저장한 노드를 방문
}
```
Time complexity of BFS implemented with an adjacency matrix: O(N^2)

<br>

### BFS Code Implemented with Adjacency List

Structures required for implementation with an adjacency list:

1. Adjacency list
2. Visited status array
3. Queue
4. Array to store visited nodes in order

```java
static void bfs(int node) {
	BFSisVisited[node] = true;  //노드방문여부를 true로 저장
	BFSvisitArr.add(node);   //방문한 노드를 순서대로 저장하는 리스트에 해당노드 추가
	for( int i = 0 ; i < graph[node].size() ; i++ ) {   
		//graph[node]에 인접한 노드만 저장되어있음
		int adjNode = graph[node].get(i);   
		if(BFSisVisited[adjNode] == false && queue.contains(adjNode) == false) {
			//방문된적X, 큐에저장되지X 를 만족하는 노드를 큐에 추가
			//adjNode에는 인접노드만 저장되므로 인접조건O
			queue.add(adjNode);
		}				
	}		
	if(!queue.isEmpty())
		bfs(queue.poll());   //큐에 가장 먼저 저장한 노드를 방문
}
```

Time complexity of BFS implemented with an adjacency list: O( N + E )
