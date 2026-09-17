# Red-Black Tree: Red-Black Tree

### Red-Black Tree
A Red-Black tree is a type of binary-search tree.
In the case of a traditional binary search tree, if the input is already sorted, its height becomes n, and thus the time complexity also reaches O(n).

![](./image/redblack.png)

Red-Black Tree

However, a Red-Black tree maintains a balanced binary tree by adhering to specific conditions, allowing Search, Insert, and Delete operations to be performed in O(logN) time even in the worst case.

This summary covers the following:

1.  Explanation of NIL nodes
2.  Definition of a Red-Black tree
3.  Height of a Red-Black tree
4.  Left and Right rotation
5.  Insert operation and case classification
6.  Delete operation and case classification

<br>

### NIL node
First, before explaining Red-Black trees, let's define NIL nodes.

Let's consider a NIL node as a special node that exists when a child node is absent.
Therefore, all leaf nodes become NIL nodes. Also, let's assume the parent of the root is a NIL node. Nodes are divided into internal nodes and NIL nodes.

Visually, it's as follows:

![nilnode](./image/nilnode.png)

NIL Node

<br>

### Definition of a Red-Black Tree
A binary search tree that satisfies the following five conditions is called a Red-Black Tree.

1.  Each node's color is either red or black.
2.  The root node is black.
3.  All terminal nodes (leaf nodes) are black. (NIL becomes black.)
4.  The child nodes of a red node are all black. (That is, red nodes cannot appear consecutively.)
5.  Every path from the root node to a descendant leaf node contains the same number of black nodes.

![nilnode](image/nilnode.png)

NIL Node

In the diagram above:

-   case 1 : 13(b) -> 8 -> 11(b) -> NIL(b): traverses a total of 3 black nodes
-   case 2 : 13(b) -> 8 -> 1(b) -> NIL(b) : traverses a total of 3 black nodes
-   case 3 : 13(b) -> 17 -> 25(b) -> 22 -> NIL(b) : traverses a total of 3 black nodes

The number of black nodes encountered on the path from the root to NIL is the same, which is 3.
Looking at these cases, the number of black nodes traversed while descending through the path is consistently 3. Of course, there are a few more cases, but I will omit them. They are also consistent.

<br>

### Height of a Red-Black Tree
Height can be categorized into two types:

1.  h(x) is the number of edges included in the longest path from x itself to a leaf node.
2.  bh(x) is the number of black nodes on the path from x to a leaf node. (x itself is excluded from the count.)

Let's confirm this with the following diagram.

![rededge](image/rededge.png)

Red Edge

-   For a node with height h, its bh is bh >= h/2.
    -   According to condition 4, two red nodes cannot appear consecutively. If a red node appears, a black node must follow immediately, meaning the number of black nodes must be at least half.
-   Any subtree rooted at node x contains at least 2^bh(x) - 1 internal nodes.

For example, let's consider a node x as shown in the following diagram.

![](image/examplenode.png)
Example Node
bh(x) being 1 means that, excluding x itself, the count of black nodes is exactly one NIL node.
This is a situation where x itself is an internal node of the subtree.

By using mathematical induction, we can see that this is applicable to the entire tree.

-   The height of a Red-Black tree with n internal nodes is less than or equal to 2log(n+1).
    -   The total number of nodes n in a tree is greater than or equal to the number of nodes in a subtree rooted at x (2^bh - 1), which we just learned.
    -   Also, as confirmed above, bh >= h/2. Therefore, the following equation holds:
    -   n >= 2^bh-1 >= 2^k/2 - 1
    -   From this, we can derive the height h.

![](image/math.png)

Math

Therefore, the maximum height is 2log(n+1), guaranteeing a time complexity of O(logN).

<br>

### Left and Right Rotation

This can be confirmed at once through the diagram.

![](image/rotation.png)

Rotation

The time complexity is O(1), and it preserves the properties of a binary search tree.

```js
Left-Rotate(T,x)
y <- right[x] // set y
right[x] <- left[y] // x의 오른쪽 자식을 y의 왼쪽 자식으로 지정
p[left[y]] <- // B의 부모를 y에서 x로 변경
p[y] <- p[x] // x의 부모가 y의 부모노드가 됨
if p[x] = nil[T] // 만약 x의 부모가 NIL이라면, 즉 x가 root라면
    then root[T] <- // y가 새로운 tree의 root 가 됨.
    else if x = left[p[x]] // x가 x의 부모의 왼쪽 자식이라면
    then left[p[x]] <- y // y가 x부모의 왼쪽 자식이 되고
    else right[p[x]] <- y // y가 x부모의 오른쪽 자식이 되고

left[y] <- x // x와 y를 연결
p[x] <- y
```

<br>

### Insert

The newly inserted node z is colored red.

After insertion, our tree must satisfy the five conditions of an RB tree. Let's check them.
1.  Satisfied.
2.  If z is the root node, it's a violation; otherwise, condition 2 is satisfied.
    Since the newly added node is red, it's usually satisfied. However, if the tree was originally empty, the red node would become the root. In such cases, simply changing the root's color to black resolves it.

3.  Satisfied.
4.  This is a problem! If the parent y of the newly inserted z is black, it's fine, but if it's red, it's a violation of condition 4. Like in the following diagram.

![](image/violation.png)

Violation

To fix such red-red conflicts, an additional function called RB-Insert_Fixup must be called.
5.  Satisfied.

-   Implementation of RB-Insert_Fixup
    First, let's check when RB-Insert_Fixup terminates and when it's needed.

-   When needed
    -   Violation of condition 2, when z is the root and is red.
    -   Violation of condition 4, when both z and its parent p[z] are red.
-   Termination condition
    -   If the parent node p[z] is black, it terminates. If condition 2 is violated, simply change z to black and it's done.
There are a total of 3 cases in insertion.
Note that cases 1, 2, and 3 all involve p[z] (the parent) being the left child of p[p[z]] (the grandparent).
Cases 4, 5, and 6 involve p[z] being the right child of p[p[z]], which simply requires swapping left and right. Therefore, I will only describe cases 1, 2, and 3.

#### case 1 : If z's uncle y is red

![](image/case1.png)
Case 1
Node B is the z we newly inserted. This z could be either the right child or the left child of A.

In the diagram above, A and B are experiencing a red-red conflict, and z's uncle y (D) is red.

By changing A and D to black, and grandparent C to red, the red-red conflict between A and B is resolved.
However, this is not a complete solution. This is because if grandparent node C becomes red, a red-red conflict might occur with p[C].

However, by changing the colors this way, we can see that the red-red conflict problem has moved up two levels.
That is, the A-B problem has moved to a C-p[C] problem. If this continues all the way up to the root, eventually, only the root's color needs to be changed to black.

Furthermore, condition 5, the black height, also remains unchanged. Both before and after the color change, the number of black nodes on the path to a NIL node is the same. If you imagine coming from above C and counting the black nodes as you descend, it's the same.

For abyoe, since the parent is red, these are black. This is natural as consecutive red nodes are not possible.

#### case 2,3 : If z's uncle y is Black

![](image/case23.png)
Case 23
Uncle y can also be a NIL node, which is why it's not represented by a black circle. In any case, it should be considered black.

-   case 2 : If z is the right child
    -   Change p[z] (B) to black, p[p[z]] (C) to red, then perform a right-rotation on p[p[z]] (C). The final result is obtained.

#### Time Complexity of Insert
RB-Insert_Fixup
In case 1, z moves up by 2 levels. In cases 2 and 3, constant time is taken. Therefore, the time taken is proportional to the height of the tree.

This is because, in the worst case, case 1 might repeat, requiring the red-red conflict to be propagated up to the root. Once it finally reaches the root, only the root's color needs to be changed to black. Therefore, the worst case will correspond to the height.

The time complexity will be O(logN).

Finally, let's look at an example of Insert. White nodes are red.

![node](image/nodetree.png)

Node Tree

<br>

### DELETE

Let's take an example. The `successor` function refers to the value immediately following y to be deleted, i.e., the value right after y when the tree's elements are arranged in ascending order.

If all elements of the tree are arranged in ascending order as 1, 2, 5, 7(y), 8, 11, then y's successor is 8.

If the actually deleted node y was red, you can simply terminate. Since red nodes cannot be consecutive, the children and parent of a red node will be black, and thus deleting a red node in the middle does not cause a black-black sequence problem.

However, if y was black, RB-Delete-Fixup must be called. This will be clearer when you look at the following diagram.

(y is the node to be deleted, and x is y's child.)

![](image/deleteexam.png)

Delete Example

Deleting a black node in the middle causes a red-red conflict, violating condition 4. Additionally, a problem with condition 5 arises because a black node suddenly disappeared from the middle.

To solve these two problems, RB-Delete-Fixup is called.

After deletion, our tree must satisfy the five conditions of an RB tree. Let's check them.
1.  Satisfied.
2.  If y was the root and x is red, it's a violation.
    After y is deleted, x takes its place. That is, x becomes the root. However, x's color is red. The root cannot be red. This can be simply resolved by changing the color of the new root x to black.
3.  Satisfied.
4.  If both p[y] and x are red, it's not satisfied. This corresponds to the case shown in the diagram just before.

5.  All paths that originally included y (black) now have one fewer black node.
-   Assign an "extra black" to node x to forcibly satisfy condition 5 for now. Let's confirm this with the following diagram.

![](image/deleteexam2.png)

Delete Example 2
In the diagram above, one black node is deleted, causing a black height issue. To prevent this, let's first imagine that two black nodes are inserted into one node.

#### Implementation of RB-Delete-Fixup
Idea
-   Propagate the extra black upwards in the tree. If the node x becomes red-black during propagation, simply make it a black node and terminate.

![](image/deleteexam3.png)

Delete Example 3

-   If it reaches the root while propagating, simply remove the extra black.
-   Visually, it's as follows. Imagine the very top is the root.

![](./image/deleteexam4.png)

Delete Example 4

Loop Invariant (condition that remains true during function execution)
-   x is a double-black node that is not the root
-   w is x's sibling node
-   w cannot be a NIL node. If it becomes NIL, condition 5 is violated.

#### There are a total of 4 cases in Delete.

Note that cases 1, 2, 3, and 4 all involve x being the left child of its parent.
Cases 5, 6, 7, and 8 involve x being the right child of its parent, which simply requires swapping left and right.

-   case 1: If w is red

    w's children are black. They cannot be NIL. If they were NIL, condition 5 would be violated.

    Then, change w (D) to black, change p[x] (B) to red. After that, apply a left-rotation with p[x] as the pivot.
    C, which was previously w's child, is incorporated as B's child.

    Then, proceed to cases 2, 3, and 4.

-   case 2 : If w is black and w's children are also black

    Gray nodes can be either black or red.
    ![](./image/deletecase2.png)
    Delete Case 2
    Pass x's extra-black to p[x] (B) and change w to red. Designate p[x] (B) as the new x.
    If this case was reached from case 1, p[x] was red, and thus the new x became red&black. Simply change it to black and terminate.

-   case 3 : If w is black, and w's left child is red
    Change w to red and w's left child to black. Then apply a right-rotation on w. x's new sibling w will have a red right child, which corresponds to case 4.

-   case 4 : If w is black, and w's right child is red

    Swap the colors of w and B. Change w's right child to black.
