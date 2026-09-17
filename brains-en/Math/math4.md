# Linear Transformations and the Rank-Nullity Theorem

## Linear Transformation

It can be **likened to photo editing on a smartphone, where you press and stretch or tilt a photo with your fingers.**

Imagine editing a photo on your smartphone, stretching it diagonally with two fingers, skewing it to one side, or rotating it.

If you look closely at the grid lines drawn over the photo, no matter how much you stretch or tilt it, **straight lines remain perfectly straight and unbent**, and the grid spacing changes uniformly. The central origin of the photo also stays in place.

In contrast, if you make the center of the photo bulge out, like a concave lens effect, the grid lines will bend.

**Such an operation that regularly transforms the entire space without bending grid lines, keeping straight lines straight and the origin at the origin, is called a linear transformation in mathematics.**

### Formal Definition of Linear Transformation

A rule (function) $T: V \to W$ that transforms a vector in the input space $V$ into a vector in the output space $W$ is called a linear transformation if it satisfies the following two conditions:

1.  **Additivity:** The result of adding inputs first and then transforming is the same as transforming each separately and then adding the results.

    $$T(\mathbf{u} + \mathbf{v}) = T(\mathbf{u}) + T(\mathbf{v})$$

2.  **Homogeneity (Scalar Multiplication):** The result of scaling an input and then transforming is the same as transforming and then scaling.

    $$T(c\mathbf{u}) = c T(\mathbf{u})$$

-   **Transformation**: A rule (synonymous with function) that takes a numerical vector as input and outputs another number or vector according to a defined rule.
-   **Mapping**: The process of establishing a one-to-one correspondence between a point in the input space and a point in the output space, like shooting an arrow.
-   **Additivity**: An honest and predictable property meaning "the result is the same whether you add and then process, or process and then add."

## Relationship between Linear Transformation and Matrices ($T(\mathbf{x}) = A\mathbf{x}$)

Every linear transformation can be perfectly expressed by a single matrix multiplication $A\mathbf{x}$.

There's no need to track where every single point in the entire space moves. If you only know where the basic unit arrows (basis vectors: 1 unit in the x-direction, 1 unit in the y-direction) move after transformation, then a matrix A is formed by listing their new positions as columns.

$$\mathbf{x} = \begin{bmatrix} x_1 \\ x_2 \end{bmatrix} \implies T(\mathbf{x}) = \begin{bmatrix} \vert{} & \vert{} \\ T(\mathbf{e}_1) & T(\mathbf{e}_2) \\ \vert{} & \vert{} \end{bmatrix} \begin{bmatrix} x_1 \\ x_2 \end{bmatrix} = A\mathbf{x}$$

-   **Basis Vectors ($\mathbf{e}_1, \mathbf{e}_2$):** The fundamental unit-length arrows that define the space (e.g., $[1, 0]^T$ in the x-direction, $[0, 1]^T$ in the y-direction).
-   **Standard Matrix:** A matrix that numerically encapsulates the rules of how a linear transformation deforms space.

### AI Engineering Context

-   **Linear / Dense Layer in Artificial Neural Networks**: In the fundamental operation $y = xW + b$ of deep learning, $xW$ is precisely the linear transformation that maps input data to a new space.
-   **Non-linear Activation Functions**: No matter how many linear transformations are composed (applied sequentially), they ultimately reduce to a single matrix multiplication $T_2(T_1(\mathbf{x})) = (A_2 A_1)\mathbf{x} = A_{total}\mathbf{x}$. Therefore, to enable neural networks to learn complex, non-linear data structures, non-linear functions like ReLU or Sigmoid are interspersed between layers, intentionally folding and bending the grid.
-   **Image Preprocessing Affine Transformation**: When training computer vision models, linear transformation matrices are used for data augmentation, such as rotating, scaling, or translating images.

<br>

## Rank-Nullity Theorem

**Imagine shining a flashlight in a dark room and making a shadow of your hand on the wall.**

This is a situation where you move your 3D hand under the flashlight to create a 2D shadow on the wall.

-   **Surviving Information (Rank)**: The shape of your outstretched fingers survives and is represented as a 2D plane (area) on the wall.
-   **Crushed and Lost Information (Nullity)**: Information about the front-to-back depth of your hand or the inside of your palm, which aligns with the direction of the light, is crushed into a point or line on the wall and disappears.

The most important fact here is that the relationship [number of dimensions surviving as a shadow on the wall] + [number of dimensions crushed and lost] = [original number of dimensions of your hand, 3D] is always perfectly maintained. This law of dimension preservation is precisely the Rank-Nullity Theorem.

### Image and Kernel

Given a linear transformation $T: \mathbb{R}^n \to \mathbb{R}^m$ (matrix $A$):

1.  **Image ($\text{Im}(A)$ or $\text{Col}(A)$)**: The set of all possible output vectors that can actually be reached by the transformation (the area of the shadow cast on the wall).
2.  **Kernel / Nullspace ($\text{Null}(A)$)**: The set of all input vectors that, when transformed, are crushed and disappear precisely into the zero origin. This is like the thickness direction of the hand, compressed to point 0 because it's parallel to the light.

    $$A\mathbf{x} = \mathbf{0} \quad \text{for all } \mathbf{x}\text{ satisfying this}$$

The image can be likened to the area where arrows hit the target board.

The nullspace is the collection of input values whose form is destroyed, resulting in a value of 0 after the transformation.

### Rank-Nullity Theorem Formula

If the dimension of the input space is n, the sum of the dimension that survives as output (rank) and the dimension that is compressed to 0 and disappears (nullity) is always equal to the original dimension n.

$$\text{Rank}(A) + \text{Nullity}(A) = n$$

$$\dim(\text{Im}(A)) + \dim(\text{Null}(A)) = \text{Dimension of input space } n$$

-   Rank is the true number of dimensions that a matrix can represent (the number of independent rows in the matrix).
-   Nullity is the number of dimensions lost, which became 0 during the transformation process.

```
[Input Space: 3D (n = 3)]
        │
   (Linear Transformation A)
        │
        ├──► [Plane surviving in output space: 2D] ──► Rank(A) = 2
        └──► [Line crushed to 0 and disappeared: 1D]  ──► Nullity(A) = 1
        
        ★ 2 + 1 = 3 (Rank + Nullity = n)
```

-   **LLM Fine-tuning Technique - LoRA (Low-Rank Adaptation):** While the parameter matrix W of large language models (LLMs) can have thousands of dimensions, the "core" dimensions of newly updated information when learning a specific single task are very low (typically 4-8). LoRA utilizes this principle by expressing the huge d x k matrix not by directly modifying it, but as the product of two very small low-rank matrices $A(d \times r)$ and $B(r \times k)$, thereby reducing memory and training time by over 99%.
-   **Dimension Reduction and Information Loss Assessment:** If the rank of a weight matrix A is less than the input dimension n ($\text{Rank}(A) < n$), it means that some information from the input data is lost into the nullspace when passing through that layer, becoming irrecoverably destroyed.
-   **Multicollinearity Detection:** In data analysis, if the rank of the input data is less than the number of features, it implies the existence of completely redundant and meaningless data columns, necessitating preprocessing to remove them. (Because there would be rows of zeros, for example.)
