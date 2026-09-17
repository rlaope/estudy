# AVL Tree

### What is an AVL Tree?
An AVL tree is a self-balancing binary search tree.

The time complexity of a binary search tree is O(h), where h is the height of the tree.
If the tree becomes a skewed binary tree, leaning to one side, its height increases. To prevent this, AVL trees are used to maintain height balance.

AVL trees have the following characteristics:
- An AVL tree possesses the properties of a binary search tree.
- The height difference between the left and right subtrees is at most 1.
- If the height difference at any point exceeds 1, rotations are performed to restore balance and reduce the height difference.
- Since an AVL tree maintains a height of logN, the time complexity for insertion, search, and deletion is O(logN).

### Balance Factor BF

The Balance Factor is the height of the left subtree minus the height of the right subtree.

- A BF of 1 means the left subtree is one level taller than the right subtree.
- A BF of 0 means the left and right subtrees have the same height.
- A BF of -1 means the left subtree is one level shorter than the right subtree.

Below is an example of an AVL tree. You can see that the BF is between -1 and +1.

![](./image/avl.png)

<br>

### Time Complexity

Since an AVL tree maintains a tree height of logN, the time complexity for insertion, search, and deletion is O(logN).

### Rotation

An AVL tree is a binary search tree, so all operations are performed in the same way as in a binary search tree.

If an imbalance occurs during insertion or deletion (when the BF is not -1, 0, or 1), the AVL tree performs a rotation operation to adjust the positions of subtrees around the imbalanced node, thereby rebalancing the tree.

Depending on the arrangement of nodes during insertion or deletion, four types of imbalances can occur (LL, RR, LR, RL). For each situation, the rotation direction is varied to balance the tree.

#### LL Left Left Case

If y is the left child of z, and x is the left child of y, a right rotation is performed to restore balance.

Right rotation process:
- Change the right child of node y to node z.
- Change the left child of node z to the right subtree of node y.
- y becomes the new root node.

![](./image/LLcase.png)

#### Right Rotation Implementation

```c
struct node *rightRotate (struct node *z) {
  struct node *y = z->left;
  struct node *T2 = y->right;

// Perform right rotation
  y->right = z;
  z->left = T2;

// Update node height
  z->height = 1 + max(z->left->height, z->right->height);
  y->height = 1 + max(y->left->height, y->right->height);

// Return new root node y
  return y;
}
```

<br>

#### RR Right Right Case
If y is the right child of z, and x is the right child of y, a left rotation is performed to restore balance.

Left rotation process:
- Change the left child of node y to node z.
- Change the right child of node z to the left subtree of node y.

![](./image/rr.png)

#### Left Rotation Implementation

```c
struct node *leftRotate (struct node *z) {
  struct node *y = z->right;
  struct node *T2 = y->left;

// Perform left rotation
  y->left = z;
  z->right = T2;

// Update node height
  z->height = 1 + max(z->left->height, z->right->height);
  y->height = 1 + max(y->left->height, y->right->height);

// Return new root node y
  return y;
}
```

#### Left Right LR Case
If y is the left child of z and x is the right child of y, a total of two rotations are performed in Left Right order to restore balance.

![](./image/lr.png)

#### Implementation

```java
y = z->left;
y = leftRotate(y);
z = rightRotate(z);
```

### RL Right Left case
If y is the right child of z and x is the left child of y, a total of two rotations are performed in Right Left order to restore balance.

![](./image/rl.png)

#### Implementation
```
y = z->right;
y = rightRotate(y);
z = leftRotate(z);
```

<br>

### Insertion Implementation

```java
struct node {
  int key;
  struct node *left, *right;
  int height;
};

int max(int a, int b) {
  return (a > b)? a : b;
}

struct node* newNode(int key) {
  struct node *temp = (struct *node)malloc(sizeof(struct node));

  temp->data = key;
  temp->left = NULL;
  temp->right = NULL;
  temp->height = 1;
  return temp;
}

struct node *leftRotate (struct node *z) {
  struct node *y = z->right;
  struct node *T2 = y->left;

// Perform left rotation
  y->left = z;
  z->right = T2;

// Update node height
  z->height = 1 + max(z->left->height, z->right->height);
  y->height = 1 + max(y->left->height, y->right->height);

// Return new root node y
  return y;
}


struct node *rightRotate (struct node *z) {
  struct node *y = z->left;
  struct node *T2 = y->right;

// Perform right rotation
  y->right = z;
  z->left = T2;

// Update node height
  z->height = 1 + max(z->left->height, z->right->height);
  y->height = 1 + max(y->left->height, y->right->height);

// Return new root node y
  return y;
}

// Function to get BalanceFactor (BF) value.
int getBalanceFactor(struct node *n) {
  if (n == NULL)
    return 0;
  return n->left->height - n->right->height;
}

// Function to maintain tree height balance.
// Performs rotation based on 4 cases.
struct node* rebalance(struct node* root) {
  
  int bFactor = getBalanceFactor(root);
  
  // LL Case
  if (bFactor > 1 && key < node->left->key)
    return rightRotate(root);
  // RR Case
  if (bFactor < -1 && key > node->right->key)
    return leftRotate(root);
  // LR Case
  if (bFactor > 1 && key > node->left->key){
    root->left = leftRotate(root->left);
    return rightRotate(root);
  }
  // RL Case
  if (bFactor < -1 && key < node->right->key){
    root->right = rightRotate(root->right);
    return leftRotate(root);
  }

  return root;
}

// Insertion function.
struct node* insert(struct node* root, int key) {

// Perform insertion
  if (root == NULL)
    return newNode(key);
  if (key > root->data)
    root->right = insert(root->right, key);
  else if (key < root->data)
    root->left = insert(root->left, key);
  else
    return root;

// Update node height
  root->height = 1 + max(node->left->height, node->right->height);

// Maintain node balance
  root = rebalance(root);
  
  return root;
}
```
