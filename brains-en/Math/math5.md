# Eigenvalues and the Cayley-Hamilton Theorem

## Eigenvalues and Eigenvectors

To use an analogy, it's about **a special direction where your body doesn't turn but only gets pushed back when walking in a strong wind, and the intensity of that push.**

Imagine walking down a street where a fierce typhoon (transformation matrix A) is blowing. If you stand in most directions, the wind will hit you, causing your body to twist sideways like a top or rotate and be pushed back.

However, if you stand in **one special direction** that is perfectly aligned with the wind, your body won't turn sideways but will only be pushed straight back in the direction you were facing.

This special direction, where your body doesn't rotate but **maintains its original orientation** even when hit by the wind, is called an 'eigenvector,' and the ratio of how many times stronger you are pushed back in that direction compared to the original is the 'eigenvalue.'

### Definition of Eigenvalues and Eigenvectors

When an $n \times n$ matrix $A$ is multiplied by a non-zero vector $\mathbf{v}$, if the result is simply a scalar multiple of the original vector $\mathbf{v}$ by a real number $\lambda$ (lambda), then

$$A\mathbf{v} = \lambda\mathbf{v} \quad (\mathbf{v} \neq \mathbf{0})$$

- $\mathbf{v}$: Eigenvector of matrix $A$ (a vector whose direction does not change after transformation)
- $\lambda$: Eigenvalue of matrix $A$ (the scalar factor by which the vector is scaled, keeping its direction unchanged)

In other words, lambda represents the eigenvalue, and v is the eigenvector.

- **Eigen**: Meaning an inherent/unique property that an object possesses, derived from German.
- **Characteristic Equation:** An equation solved to find the eigenvalue $\lambda$ of matrix A by using the condition that the inverse matrix does not exist for $(A - \lambda I)\mathbf{v} = \mathbf{0}$ (i.e., $\det(A - \lambda I) = 0$).
- **Diagonalization**: The process of transforming a complex matrix with intertwined numbers into a simpler diagonal matrix where only eigenvalues remain on the diagonal, by using eigenvectors as new basis vectors ($A = PDP^{-1}$).

> In mathematics, 'det' is a mathematical symbol meaning Determinant, pronounced "determinant," and it represents a unique characteristic of a square matrix as a single scalar value.

### Process for Finding Eigenvalues

Transposing $A\mathbf{v} = \lambda\mathbf{v}$ gives $(A - \lambda I)\mathbf{v} = \mathbf{0}$. For $\mathbf{v}$ to have a non-zero, meaningful solution, the determinant of the matrix $(A - \lambda I)$ must be $0$.

$$\det(A - \lambda I) = 0$$

The roots $\lambda$ obtained by solving this equation (the characteristic equation) are the eigenvalues.

### AI Engineering Context

- **PCA, Principal Component Analysis**: When reducing the dimensionality of high-dimensional data (hundreds of dimensions) to retain only the most important information, it's necessary to find the principal axes along which the data is most spread out. After calculating the covariance matrix of the data, **the direction of the eigenvector corresponding to the largest eigenvalue is selected as the first principal component axis to minimize information loss.**
- **RNN / Deep Learning Neural Network Exploding / Vanishing Gradient**: In deep learning networks with dozens of layers or Recurrent Neural Networks (RNNs) that handle time-series data, the weight matrix W is repeatedly multiplied. If the maximum eigenvalue ($\lambda_{max}$) of W is greater than $1$, the derivative values diverge infinitely as they pass through more layers (exploding gradient). If it's less than $1$, the error vanishes to $0$, leading to a vanishing gradient, making learning impossible.

<br>

## Cayley-Hamilton Theorem

Let's use an intuitive analogy. Imagine a robot A. This robot has rules (a matrix) that change its position or shape every time it moves.

1.  **Robot's Physical Exam**: Measure a few unique characteristics of robot A, like its height and weight, and create an equation. This equation is originally designed to calculate by plugging in a number x. (e.g., $x^2 - 5x + 6 = 0$)
2.  **Inputting the Robot Itself into the Custom Equation**: Now, instead of the number x, let's plug the robot A itself directly into this equation. $A^2 - 5A + 6I = ?$ (Here, $I$ represents the default state that changes nothing.)
3.  **The Result is Always 0 (Stopped)**: Amazingly, when you input the robot itself into an equation created from its unique characteristics, all its movements perfectly cancel out, and the result is always 0 (motion stopped).

The key takeaway is the rule that every matrix, when plugged into **the skeletal equation (characteristic equation) derived from its own unique properties, always results in 0.**

### Mathematical Explanation

Mathematically, the Cayley-Hamilton theorem states that every square matrix satisfies its own characteristic equation.

Defining the characteristic equation and the theorem, for an $n \times n$ square matrix $A$, the characteristic polynomial $p(\lambda)$ used to find the eigenvalue $\lambda$ is defined as follows:

$$p(\lambda) = \det(\lambda I - A) = \lambda^n + c_{n-1}\lambda^{n-1} + \dots + c_1 \lambda + c_0$$

At this point, if we substitute the matrix $A$ instead of the scalar variable $\lambda$, the following holds true:

$$p(A) = A^n + c_{n-1}A^{n-1} + \dots + c_1 A + c_0 I = O$$

(where $I$ is the identity matrix, and $O$ is the zero matrix)

### Example with a $2 \times 2$ Matrix

Given a $2 \times 2$ matrix $A = \begin{pmatrix} a & b \ c & d \end{pmatrix}$

-   **Trace**: $\text{tr}(A) = a + d$
-   **Determinant**: $\det(A) = ad - bc$

The characteristic equation becomes $p(\lambda) = \lambda^2 - \text{tr}(A)\lambda + \det(A) = 0$, and by the Cayley-Hamilton theorem, the following equation must hold true:

$$A^2 - \text{tr}(A)A + \det(A)I = O$$

In summary, the Cayley-Hamilton theorem states that every matrix has its own unique formula, a dedicated equation that yields 0 when numbers are plugged in.

Even if you plug the matrix itself entirely into that formula instead of numbers, the result is 0.

The reason for using this is as a **shortcut for tedious calculations**.

When you need to repeat the same operation 1,000 times

($A^{1000}$), using this property allows for a trick (compression) to find the answer in just 1-2 steps, without needing to multiply it 1,000 times individually.

### AI Engineering Applications

#### Matrix Exponential Calculation in State Space Models (SSM - Mamba, S4)

State Space Models (SSMs) like Mamba and S4, which have recently gained attention as alternatives to Transformers, need to calculate matrix exponentials to process continuous-time signals.

-   Problem: $\exp(A) = I + A + \frac{A^2}{2!} + \frac{A^3}{3!} + \dots$ (requires infinite series computation)
-   Solution: By the Cayley-Hamilton theorem, the infinite series expression for an $n \times n$ matrix $A$ can ultimately be reduced to a sum of polynomials of degree $n-1$.

$$\exp(A) = \sum_{k=0}^{n-1} \alpha_k A^k$$

#### Spectral Filtering in Graph Neural Networks (GGN) with ChebNet

In Graph Neural Networks that handle social networks, molecular structures, etc., information between neighboring nodes is aggregated through powers ($L^k$) of the graph's adjacency matrix and Laplacian matrix $L$.

-   **Problem**: Directly calculating $L^k$ to obtain information from $k$-hop distant nodes leads to a surge in memory and computational requirements.
-   **Solution**: Based on the Cayley-Hamilton theorem, high-order polynomial filters of $L$ are approximated as a linear combination of low-order polynomials, such as Chebyshev polynomials.
-   **Effect**: Efficiently performs "near-neighbor influence" (information transfer between neighboring nodes) operations without multiplying the entire graph structure.
