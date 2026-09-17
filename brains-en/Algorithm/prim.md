# Prim's Algorithm

An algorithm used to implement a **Minimum Spanning Tree**, which expands the tree step-by-step by adding vertices from a starting vertex.

## How Prim's Algorithm Works

Prim's algorithm is based on a greedy approach, selecting the best condition at each moment. That is, for a visited vertex, it selects the adjacent vertex connected by the edge with the lowest cost.

1.  Initially, only the starting node belongs to the MST set.
2.  Among the vertices adjacent to those already in the tree set, select the vertex connected by the edge with the lowest weight. Add this edge and vertex to the MST tree set. (To prevent cycles, if the connected vertex is already in the tree, consider the next lowest-weight option.)
3.  Repeat step 2 until the number of elements in the MST set equals the number of vertices in the graph. (Sum the edge weights to calculate the cost of the Minimum Spanning Tree.)

![](https://blog.kakaocdn.net/dn/t9Uqe/btra8zqXxvC/IAVNFqVxn75syj0N2qANZ0/img.png)

Let's find the Minimum Spanning Tree of the graph above using Prim's algorithm. Assume the starting vertex is A.

![](https://blog.kakaocdn.net/dn/qLXyd/btra34yiT04/exhnbGse9CVyZc321ojfn1/img.png)

Among nodes B and C adjacent to A, C is connected by the edge with the lowest weight, so add C to the set and add the AC weight to the cost.

![](https://blog.kakaocdn.net/dn/bErno8/btra8zYQnSg/ZCVLFloE1oM1wOJ1Yd1O0K/img.png)

Among the nodes adjacent to AC, the vertex connected by the lowest weight is B. Add B and add the CB weight.

![](https://blog.kakaocdn.net/dn/DGxpm/btra8AJ974c/z3h4sIbC6bFDaBSWggigT0/img.png)

Among the nodes adjacent to A, C, and B, the vertex connected by the lowest weight is D. Add D to the set and add the CD weight.

![](https://blog.kakaocdn.net/dn/nDRma/btra34kI0RY/ke0aIuzAZOQbzZrPn0cOKK/img.png)

Among the nodes adjacent to A, C, B, and D, the vertex connected by the lowest weight is E. Add E to the set and add the DE weight.

![](https://blog.kakaocdn.net/dn/9qPtN/btrbbTvHr2H/NeTnS5BWaf69DgngSvDXP0/img.png)

Among the nodes adjacent to A, C, B, D, and E, add vertex F, which is connected by the lowest weight, to the set and add the DF weight. Since the number of elements in the tree set has reached N, stop the search. The cost of building the Minimum Spanning Tree was confirmed to be 13.

## Implementation
Looking at the operational process, it seems that finding the vertex with the lowest weight among adjacent vertices will determine the time complexity. Therefore, it would be helpful to implement this by iterating through the vertices in the set, inserting them into a priority queue, and then popping them.

![](https://blog.kakaocdn.net/dn/bx4C47/btra1ZYioDT/uu2dFvaYSHxeVrmjFsUe31/img.png)

```py
from collections import defaultdict
import heapq

def mst():
    V,E = 6, 9
    edges = [[1, 2, 6], [1, 3, 3], [2, 3, 2], [2, 4, 5],
             [3, 4, 3], [3, 5, 4], [4, 5, 2], [4, 6, 3], [5, 6, 5]]
    graph = defaultdict(list)
    for srt, dst, weight in edges:
        graph[srt].append((dst, weight))
        graph[dst].append((srt, weight))
    mst_graph = [[0] * V for _ in range(V)]
    mst_nodes [[0] for _ in range(V)]
    visited = [True for _ in range(V)]
    q = [(0, 1, 1)]
    while q:
        cost, node, prev = heapq.heappop(q)
        if visited[node - 1] is False:
            continue
        visited[node - 1] = False
        mst_graph[node - 1][prev - 1] = 1
        mst_graph[prev - 1][node - 1] = 1
        mst_nodes[node - 1] = coust
        for dst, weight in graph[node]:
            if visited[dst - 1] is True:
                heapq.heappush(q, (weight, dst, node))
        print(f'MST cost is {sum(mst_nodes)}')
        mst_graph[0][0] = 1
        for row in mst_graph:
            print(*row)

mst()
```

![](https://blog.kakaocdn.net/dn/XuUeO/btrbbSp1YnE/dFLeC4gemekhIwj69YAhVK/img.png)
