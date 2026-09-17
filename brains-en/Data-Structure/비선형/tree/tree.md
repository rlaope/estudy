# Tree

### What is a Tree?
- A tree can be thought of as similar to a real tree. A tree has a single root, from which branches extend. These branches then sprout smaller branches, and those smaller branches sprout even more. This type of data structure is called a tree.
- Trees differ fundamentally in their purpose from previous data structures like stacks, queues, and deques.
- Data structures like linked lists and arrays serve as foundational structures for building other data structures.
- Stacks, queues, and deques are used in internal system implementations, so their internals don't need to be exposed; they are data structures focused purely on performance.
- Trees, finally, are data structures designed for human convenience, meaning they are made to be easy for people to use.

![](./image/tree.png)

A tree has this structure.
There is a starting node, and other nodes are connected as child nodes based on it.
In a general tree, there is no limit to the number of child nodes. That is, a node like A can have 1, 100, or even no connected child nodes. There's also no limit to how deep it can go.
  
For example, a path like A > C > F > I could be 4 levels deep, but it could also be 100 or even 1000 levels deep.
- Here, the topmost node, A, which is the entry point to all other nodes, is called the `Root node`.
- And based on A, B, C, and D are directly connected to A. These nodes are called `child nodes`.
- Conversely, A is called the `parent node` relative to B, C, and D.
- Also, C and D are on the same level as B. These nodes are called `sibling nodes`.
- And a node like D, which has no children, is called a `leaf node`.
- Conversely, any node that has at least one child, excluding the root, is called a `branch node`.

#### Summary
```
Root node - The topmost node

Child node - A subordinate node of another node

Parent node - A superior node of another node

Brother, Sibling node - Nodes at the same level as another node

Leaf node - A node that has no child nodes

Branch node - A node that has at least one child node, but is not the Root
```

### Tree Terminology

- From the root node A to I, the shortest path traverses 4 nodes. From the root to node E, it traverses 3 nodes. The number of paths from the root to any given node is called its `depth`.
- This depth is also sometimes called `level`, though with a slight nuance. Node E has a depth of 3 and a level of 3. You can think of it this way: someone might ask, 'What is the depth of node E?', but they wouldn't typically say, 'What nodes have a depth of 3?' Instead, they'd ask, 'What nodes are at level 3?' They are almost synonymous.
- And nodes H, I, J, K have the greatest depth in this tree. This largest depth value is called the tree's `height`.
- And the number of children a node has is called its `degree`. For example, node A has a degree of 3.

#### Summary
```
Depth - The number of paths from the root to a given node

Level - A collection of nodes at the same depth

Height - The greatest depth in this tree

Degree - The number of children a node has
```

<br>

### Uses of Trees

![](./image/linuxtree.png)

![](./image/windowtree.gif)

These are the file system structures of Linux and Windows, respectively. Both file systems are built using trees.
Typically, file systems are often constructed using variations of trees.
Additionally, tree structures are used for various data that require a hierarchical structure.
Trees are also often suitable for storing large amounts of data.

### Tree Algorithms and Time Complexity

- Left-Child/Right-Sibling Representation

![](./image/express.jpeg)

This algorithm is easier to implement than the one above, though less intuitive, but it is widely used due to its ease of implementation.
The context of its creation is essentially the same as a binary tree. If you unravel this tree, it's actually identical to a binary tree.
Going left visits a child, and going right visits a sibling node.
For example, in the tree above, there are three nodes at level 2, but they can only be visited through the first data element among the level 2 nodes.
The advantages of this so-called LCRS (Left Child Right Sibling) method are:
It is much easier to implement compared to the N-link method, and it has the advantage of being effective for growing data. (In fact, in the N-link method, you can use dynamic arrays instead of complicating the algorithm, so this might not always be considered an advantage.)
However, a disadvantage is that there is no direct way to visit child nodes from a parent node.
That is, even if they are my children, to check how many children I have or what data they hold, traversal is required, which is a drawback.
The time complexity is the same as the N-link method for general function implementations, but this does not mean the time complexity is the same in most situations.
For example, in the N-link method, visiting the nth child node of a given node takes O(1) time complexity, whereas in LCRS, it's O(n).
Let's assume n is the number of children of an arbitrary node and m is the total number of data elements, and examine the time complexity.

```
Add - O(n)

The time complexity for adding data is O(n). This is because there is no way to add data directly in the LCRS method. Therefore, you must traverse through to the nth child, resulting in an O(n) time complexity.


Search - O(m)

Since all nodes must be traversed, the worst-case scenario is O(m). The probability of a best-case scenario is particularly lower than with the N-link method; in the N-link method, there's a higher chance of finding the element midway and terminating the search, but this is difficult with the LCRS method.


Delete - O(1) Or O(m)

Similar to the N-link method, in C/C++, the time complexity is O(m), while in Java, it takes constant time. 


Change - O(1)

Needless to say. Since you only need to change it, it takes O(1) time.
```

### Implementation

```java
package Tree;

public class Tree {
	int count;
	
	public Tree() {
		count = 0;
	}
	
	public class Node {
		Object data;
		Node left;
		Node right;
	
		// 생성 시 매개변수를 받아 초기화하는 방법으로만 선언 가능
		public Node(Object data) {
			this.data = data;
			left = null;
			right = null;
		}

		public void addLeft(Node node) {
			left = node;
			count++;
		}

		public void addRight(Node node) {
			right = node;
			count++;
		}

		public void deleteLeft() {
			left = null;
			count--;
		}

		public void deleteRight() {
			right = null;
			count--;
		}
	}
	
	public Node addNode(Object data) {
		Node n = new Node(data);
		return n;
	}
	
	public void preOrder(Node node) {
		if(node == null) {
			return;
		}
		
		System.out.print(node.data + " ");
		preOrder(node.left);
		preOrder(node.right);
	}

	public void inOrder(Node node) {
		if(node == null) {
			return;
		}
		
		inOrder(node.left);
		System.out.print(node.data + " ");
		inOrder(node.right);
	}

	public void postOrder(Node node) {
		if(node == null) {
			return;
		}
		
		postOrder(node.left);
		postOrder(node.right);
		System.out.print(node.data + " ");
	}
}

 

- Run Class (Code Execution Class)

import Tree.*;
import Tree.Tree.Node;

public class Run {
	public static void main(String[] args) {
		// 트리 생성
		Tree tree = new Tree();
		
		// 노드 생성
		Node node1 = tree.addNode(1);
		Node node2 = tree.addNode(2);
		Node node3 = tree.addNode(3);
		Node node4 = tree.addNode(4);
		Node node5 = tree.addNode(5);
		Node node6 = tree.addNode(6);
		Node node7 = tree.addNode(7);
		
		// 트리 연결관계 생성
		/*  트리 모양       
		 *        1
		 *     2     3
		 *   4  5  6   7
		 */
		node1.addLeft(node2);
		node1.addRight(node3);
		node2.addLeft(node4);
		node2.addRight(node5);
		node3.addLeft(node6);
		node3.addRight(node7);
		
		// 순회
		tree.preOrder(node1);
		System.out.println();
		tree.inOrder(node1);
		System.out.println();
		tree.postOrder(node1);
		System.out.println();
		
		// 삭제
		node2.deleteLeft();
		node3.deleteRight();
		/* 삭제 이후 트리 모양
		 *        1
		 *     2     3
		 *      5  6   
		 */
		
		// 순회
		System.out.println();
		tree.preOrder(node1);
		System.out.println();
		tree.inOrder(node1);
		System.out.println();
		tree.postOrder(node1);
		System.out.println();
	}
}
```
