# 너비 우선 탐색(BFS)


그래프 탐색 알고리즘은 대표적으로 DFS, BFS가 있다. 그중에서 오늘은 BFS를 알아보겠다.

## BFS
대표적인 그래프 탐색 알고리즘

정점들과 같은 레벨에 있는 노드들(형제 노드들)을 먼저 탐색하는 방식


**최대한 넓게 이동한 다음, 더 이상 갈 수 없을 때 아래로 이동**

예를 들어서 특정도시에서 다른 도시로 갈 수 있는지, 전자회로에서 특정 단자와 단자가 서로 연결되어 있는지를 탐색하는 알고리즘이다.

### BFS에서의 노드 탐색 순서

![](https://velog.velcdn.com/images%2Fsukong%2Fpost%2F103fbeed-3f70-4074-9a7d-76915a7764f2%2FBFS.png)

너비우선탐색이기 때문에 깊이가 가장 얕은 노드부터 모두 탐색한뒤 깊이가 깊은 노드를 탐색하는 방법. 즉, 그림에서 깇이가 1인 노드1, 2를 먼저 탐색한 뒤, 깊이가 1인 노드를 모두 탐색하였으므로, 깊이가 2인 노드 3,4,5,6을 탐색하는 순서다.

### 특징

- 두 노드사이의 최단경로를 탐색할 때 활용하기 좋은 방식이다. 멀리 떨어진 노드는 나중에 탐색하는 방식이기 때문
- 큐를 활용해 탐색할 노드의 순서를 저장하고 큐에 저장된 순서대로 탐색한다. 선입선출의 방식을 활용해야하기 때문에 큐를 활용한다.

### BFS 구현 알고리즘

1. 루트노드에서 시작한다.
2. 루트노드와 인접하고 방문된적 없고, 큐에 저장되지 않은 노드를 Queue에 넣는다.
3. Queue에서 dequeue하여 가장 먼저 큐에 저장한 노드를 방문한다.

![](https://velog.velcdn.com/images%2Fsukong%2Fpost%2Fc64d33a0-6e43-43be-9c44-5937f9bf40e3%2Fimage.png)

1. 1번에서 루트노드에 방문하고
2. 2 3 4 5에서 인접하고 방문된적 없으며 큐에 저장되지 않은 노드를 큐에 저장한다.
3. 6에서 가장 먼저 큐에 저장된 1번 노드로 이동해서 인접노드들의 조건을 확인한다.

이 방식을 큐에 저장된 노드가 없을 때까지 반복

### 그래프 구현 방식

1. 인접 행렬
2. 인접 리스트

예를 들어보자

![](https://velog.velcdn.com/images%2Fsukong%2Fpost%2Fc209c54d-de4d-4ec3-9914-b70624cfeabd%2F%EA%B7%B8%EB%9E%98%ED%94%84%EC%9D%B4%EB%AF%B8%EC%A7%80.png)

위와 같은 그래프를 구현할 때 다음과 같이 인접행렬과 인접 리스트로 구현할 수 있다.

인접 행렬은 이차원배열, 인접리스트는 링크드리스트 배열, 어레이리스트 배열, 어레이리스트를 저장하는 어레이리스트 등과 같은 방식으로 구현할 수 있다.

![](https://velog.velcdn.com/images%2Fsukong%2Fpost%2F392b382a-5e93-4d94-9f1f-151976032f26%2F%EC%9D%B8%EC%A0%91%ED%96%89%EB%A0%AC%2C%20%EC%9D%B8%EC%A0%91%EB%A6%AC%EC%8A%A4%ED%8A%B82.png)

### BFS를 인접행렬로 구현한 코드

인접행렬로 구현할 때 필요한 구조

1. 인접 행렬 배열 (int[][] graph)
2. 방문여부 배열 (boolean[] visited)
3. 큐 (Queue queue)
4. 방문한 노드를 순서대로 저장하는 배열 (ArrayList arrList)

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

### BFS를 인접리스트로 구현한 코드

인접리스트로 구현할 때 필요한 구조

1. 인접리스트(ArrayList[] graph)
2. 방문여부 배열 (boolean[] visited)
3. 큐(Queue queue)
4. 방문한 노드를 순서대로 저장하는 배열(ArrayList arrList)

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