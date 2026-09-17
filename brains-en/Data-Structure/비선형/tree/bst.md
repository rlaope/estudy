# Binary Search Tree (BST)

### Properties of Binary Search Trees
A Binary Search Tree is a sorted binary tree with the following properties:
- The left subtree of a node contains only nodes with keys less than the node's key.
- The right subtree of a node contains only nodes with keys greater than the node's key.
- Both the left and right subtrees must also be binary search trees.
- Duplicate keys are not allowed.

![](./image/bst.png)

Due to these characteristics of a Binary Search Tree, efficient searching is possible.

### Creation Example
```
50, 15, 62, 80, 7, 54, 11
```
The process of creating a BST using the given elements is as follows:

1. Insert 50 into the tree as the root.
2. Read the next element. If it is smaller than the root node's element, insert it into the left subtree.
3. Otherwise, insert it into the right subtree.

![](./image/insertbst.png)

### Characteristics of Binary Search Trees

1. By performing an Inorder Traversal of a BST, all keys can be retrieved in sorted order.

![](./image/bstdetail1.png)

The result of an inorder traversal of the above tree is as follows:
8 11 15 50 54 62 80

2. The time complexity for searching in a BST is O(logN) if it is balanced, and up to O(N) if it is unbalanced.

![](./image/bstdetail2.png)

<br>

### Binary Tree Operations

#### Search

Finds the position of a specific element in a Binary Search Tree.

The search process is as follows:
1. Start from the root.
2. Compare the search value with the root. If it is smaller than the root, recurse on the left; if larger, recurse on the right.
3. Repeat the procedure until a matching value is found.
4. If the search value is not found, return Null.

#### Insertion
Performs the operation of inserting data into a Binary Search Tree. Duplicates are not allowed.
New keys are always inserted at leaf nodes.

The insertion process is as follows:
1. Start from the root.
2. Compare the insertion value with the root. If it is smaller than the root, recurse left; if larger, recurse right.
3. After reaching a leaf node, if it is larger than the node, insert it to the right; if smaller, insert it to the left.

![](./image/bstinsert2.png)

#### Deletion

Deletes a specific node from a Binary Search Tree. There are three situations for deleting a node in a Binary Search Tree.

1. Case 1: The node to be deleted is a leaf node.

Simply delete the node.

![](./image/deletebst1.png)

2. Case 2: The node to be deleted has only one child.

Delete the node and directly connect its child node to the parent of the deleted node.

![](./image/bstdelete2.png)

3. Case 3: The node to be deleted has two children.

If there are two children, an additional step of finding the successor node is required.

What is a successor node?
- The minimum value in the right subtree.
- That is, the next node in an inorder traversal.

The deletion process is as follows:
1. Find the node to be deleted.
2. Find the successor node of the node to be deleted.
3. Swap the values of the node to be deleted and its successor node.
4. Delete the successor node.

![](./image/bstdelete3.png)
