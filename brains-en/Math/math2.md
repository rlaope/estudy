# Gauss-Jordan Elimination and Various Matrices

## Gauss-Jordan Elimination: A Systematic Algorithm for Finding Solutions

### Addition/Subtraction Method in Computer Language

Thinking back to solving simultaneous equations in middle school math, we used the **addition/subtraction method**, which involves adding or subtracting two equations to eliminate one variable, finding the value of the remaining variable, and then substituting it back.

For example, consider the following system of equations:

$$\begin{aligned} x + 2y &= 8 \quad \text{--- (1식)} \\ 2x + 5y &= 19 \quad \text{--- (2식)} \end{aligned}$$

We multiplied equation (1) by 2 ($2x + 4y = 16$) and then subtracted it from equation (2) to eliminate $x$.

$$(2x + 5y) - (2x + 4y) = 19 - 16 \implies y = 3$$

Substituting $y = 3$ into equation (1) gives us the solution $x = 2$.

While this method is easy to solve for 2-3 equations, what if there are **1,000 unknowns and 1,000 equations**?

It's impossible for a human to individually decide which equation to multiply by what and then subtract. A **standardized procedure that a computer can process mechanically (algorithmically) and sequentially**, regardless of the number of unknowns, is needed.

This procedure is precisely Gauss-Jordan Elimination.

Think of it like a puzzle game where you tidy up a complex number board using only three buttons.

Imagine you're playing a board game with a friend. The goal of the game is to transform a jumbled number board, like the one below, into a **neat shape where only the diagonal elements are 1 and the rest are 0.**

```
[ 1  2 |  8 ]   ───(퍼즐 조작)───►   [ 1  0 |  2 ]  ->  x = 2
[ 2  5 | 19 ]                         [ 0  1 |  3 ]  ->  y = 3
```

1.  **Row Swap Button**: Swaps the positions of row 1 and row 2.
2.  **Scalar Multiply Button**: Multiplies all numbers in a row by 2 or 3.
3.  **Row Subtraction Button**: Subtracts the result of multiplying row 1 by 2 from row 2.

If you press these three buttons correctly in sequence to transform the left side into a [1 0 / 0 1] shape, the numbers 2 and 3 remaining on the far right are the answers we were looking for ($x=2, y=3$). This process of solving the puzzle according to these defined rules is Gauss-Jordan elimination.

### Augmented Matrix and Elementary Row Operations (Professional Explanation of the Above Analogy)

To begin Gauss-Jordan elimination, we first create an Augmented Matrix by combining the coefficient matrix $A$ and the result vector $\mathbf{b}$.

$$\left[\begin{array}{cc\|c}  1 & 2 & 8 \\  2 & 5 & 19  \end{array}\right]$$

When transforming this matrix, we can only use three rules. These operations, which change the rows while keeping the solution of the equation unchanged, are called Elementary Row Operations (ERO).

1.  **Row Swap**: Swaps the positions of two rows. Changing the order of equations does not change the solution.
2.  **Scalar Multiplication**: Multiplies an entire row by a non-zero number. (Multiplying both sides of an equation by the same number does not change the solution.)
3.  **Row Addition/Subtraction**: Adds or subtracts the result of multiplying one row by a specific number to or from another row (Adding or subtracting two equations does not change the solution.)

### Mathematical Principle

The three puzzle buttons mentioned above are called Elementary Row Operations in mathematics.

The table combining coefficients and results is called an Augmented Matrix.

$$\left[\begin{array}{cc\|c}  1 & 2 & 8 \\  2 & 5 & 19  \end{array}\right]$$

-   Step 1 (Make element at row 2, column 1 zero): Calculate (Row 2) $-$ $2 \times$ (Row 1).$$\left[\begin{array}{cc\|c} 1 & 2 & 8 \\ 0 & 1 & 3 \end{array}\right]$$
-   Step 2 (Make element at row 1, column 2 zero): Calculate (Row 1) $-$ $2 \times$ (Row 2).$$\left[\begin{array}{cc\|c} 1 & 0 & 2 \\ 0 & 1 & 3 \end{array}\right]$$

Since the left side has become an **identity matrix** with only 1s on the diagonal, we obtain the final solution $x = 2, y = 3$.

### Significance of Gauss Elimination in AI

Computers cannot ponder, 'Should I move x to the other side in this equation?' like humans do.

Instead, they execute a predefined algorithm, like Gauss elimination, which dictates to **make elements zero sequentially starting from column 1**.

When solving AI linear systems with millions of unknowns, computers internally repeat these puzzle operations billions of times to find the optimal solution.

<br>

## Inverse Matrix: The Ctrl + Z Matrix for Reverting to Original State

**It's like the Ctrl + Z button that restores accidentally deleted text.**

In the world of ordinary numbers, to revert something that was multiplied, you perform **division**.

-   Multiplying 5 by 2 gives 10 ($5 \times 2 = 10$).
-   To revert to the original state, you multiply by the inverse of 2, which is $\frac{1}{2}$ ($10 \times \frac{1}{2} = 5$).

In the world of matrices, there is no division. Instead, when a matrix A transforms data, there exists a **'undo password' matrix that perfectly restores it to its original state**, which is called the inverse matrix ($A^{-1}$).

```
[Original Data] ─── (Multiply by Matrix A) ───► [Encrypted Data] ─── (Multiply by Inverse Matrix A⁻¹) ───► [Original Data Restored]
```

### So, can all matrices be reverted (have an inverse)?

Let's say you have a cardboard box.

If you gently fold the box, you can unfold it and restore it to its original state.

However, if you **stomp on the box and flatten it, you can never restore what was inside.**

Similarly in mathematics, matrices that flatten data to a lower dimension do not have an inverse and are called Singular Matrices.

### Mathematical Principle

When a matrix A is multiplied by its inverse $A^{-1}$, the result is the identity matrix I, which changes nothing.

$$A A^{-1} = I$$

For a $2 \times 2$ matrix $A = \begin{bmatrix} a & b \\ c & d \end{bmatrix}$, the criterion for determining if an inverse matrix exists is called the determinant.

$$\text{Determinant } \det(A) = ad - bc$$

-   When $\det(A) \neq 0$: An inverse matrix exists.$$A^{-1} = \frac{1}{ad - bc} \begin{bmatrix} d & -b \\ -c & a \end{bmatrix}$$
-   When $\det(A) = 0$: The box is flattened, so no inverse matrix exists.

Theoretically, the solution to $A\mathbf{x} = \mathbf{b}$ is found by $\mathbf{x} = A^{-1}\mathbf{b}$, but

In Python/PyTorch code, you rarely directly compute the inverse matrix using an `inv(A)` function.

This is because directly calculating the inverse matrix involves too many operations and can easily accumulate small floating-point errors.

Instead, using the `p.linalg.solve(A, b)` function, which internally uses the elimination method we learned, is much faster and more accurate.

## Various Matrices

| Special Matrix Name | Analogy | Key Feature |
|---------------------|-------------------------|---------------------------------------------------|
| Identity Matrix (I) | Transparent Mirror | Multiplying by it yields itself (`A × I = A`) |
| Zero Matrix (O) | Black Hole | Multiplying or adding it turns all numbers into 0 |
| Transpose Matrix ($A^T$) | Smartphone Screen Rotation | A matrix where rows and columns are swapped |
| Diagonal Matrix (D) | Independent Adjustment Switch | Only has numbers on the diagonal, very fast computation |
| Orthogonal Matrix (Q) | Object Rotation Handle | Maintains size and shape, only rotates direction |

### 1. Transpose Matrix ($A^T$)

An operation that swaps the rows and columns of a matrix, like rotating a smartphone from landscape to portrait.

.$$A = \begin{bmatrix} 1 & 2 & 3 \\ 4 & 5 & 6 \end{bmatrix} \implies A^T = \begin{bmatrix} 1 & 4 \\ 2 & 5 \\ 3 & 6 \end{bmatrix}$$

### 2. Diagonal Matrix ($D$)

A matrix that only has numbers on its diagonal positions, with all other entries being 0.

$$D = \begin{bmatrix} \mathbf{3} & 0 \\ 0 & \mathbf{5} \end{bmatrix}$$

-   Why is it good?: While multiplication with general matrices is complex, with diagonal matrices, you only need to multiply the numbers on the diagonal. This saves a tremendous amount of memory and time for computer computations.

### 3. Orthogonal Matrix ($Q$):

An orthogonal matrix is a very interesting and useful matrix because its transpose ($Q^T$) is directly its inverse ($Q^{-1}$).

Think of it as a rotation tool used when modeling 3D characters to rotate their arms or legs 360 degrees without distorting their shape.
