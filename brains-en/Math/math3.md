# Vector Spaces and Inner Products

## Vectors and Vector Spaces

Imagine it as **three primary colors that can create any desired color, along with a regular canvas.**

When painting, if you only have red, green, and blue paints, you can mix them to create almost every color in the world.

- The act of adding 2 drops of red and 1 drop of green can be represented as an arrow (vector) with numbers $(2, 1, 0)$.
- Even if you double the amount of paint (scalar multiplication) or combine two mixed paints (addition), they still remain within the framework of colors expressible on the canvas.

Similarly, in mathematics, **a vector space is defined as the collection of all possible results that can be created by adding or scaling basic elements, and a safe space that is not exceeded no matter how much they are mixed or scaled.**

### Mathematical and Engineering Principles

In mathematics, the definition of a vector is not simply limited to an 'arrow'.

**Any object for which two operations, addition and scalar multiplication (real number multiplication), are defined is called a vector.**

A set of n-dimensional elements with n numbers arranged in order is usually expressed as follows.

$$\mathbf{x} = \begin{bmatrix} x_1 \\ x_2 \\ \vdots \\ x_n \end{bmatrix} \in \mathbb{R}^n$$

### Conditions for a Vector Space (Closure)

For a set $V$ to be a vector space, the results of the following two operations must exist (closure) within the set $V$ for any vectors $\mathbf{u}, \mathbf{v}$ and scalar $c$ in the set, and it must satisfy 8 linear axioms (commutativity, associativity, existence of identity element, inverse element, etc.).

1. Closed under addition: $\mathbf{u} + \mathbf{v} \in V$
2. Closed under scalar multiplication: $c\mathbf{u} \in V$

### Basis and Dimension

- **Linear Combination:** It's a form obtained by multiplying vectors by scalars and adding them. ($c_1\mathbf{v}_1 + c_2\mathbf{v}_2 + \dots + c_k\mathbf{v}_k$)
- **Linear Independence:** A state where none of the vectors in a collection can be formed by a combination of the others. (A state without unnecessary redundancy)
- **Basis:** The minimal set of vectors that can generate the entire space (Span) and are perfectly independent of each other.
- **Dimension:** The number of vectors in the basis of that space.

$$\text{Dimension } \dim(V) = \text{Number of Basis Vectors}$$

```
[Standard Basis of 2D Space ℝ²]
e₁ = [1, 0]ᵀ (Basic unit in X-axis direction)
e₂ = [0, 1]ᵀ (Basic unit in Y-axis direction)
-> Any point in the 2D plane can be expressed as a linear combination c₁e₁ + c₂e₂ of e₁ and e₂.
```

### AI Engineering Context

- **Embedding Space:** AI transforms data such as words, images, and audio into a point in a high-dimensional vector space. For example, LLMs (Large Language Models) represent a single word as coordinates in a 4,096-dimensional or 12,288-dimensional vector space $\mathbb{R}^{d}$.
- **Latent Space:** This is a space where deep learning encoders extract only the essential features of high-dimensional data (e.g., 1024 x 1024 image) and compress them into a lower-dimensional subspace.
- **Dimensionality Reduction:** If the dimension of the data is too large, the computational load explodes. Therefore, techniques (like PCA) that reconstruct the basis of the space to retain only important dimensions are essential.

<br>

## Inner Product of Vectors

**The product of the length of the shadow cast on the floor when sunlight shines on it and the arrow on the floor.**

When there are two arrows A and B, if light is shone directly above arrow A, a shadow of A is cast on arrow B.

- The more the two arrows point in the same direction, the longer the shadow, and the larger the resulting inner product value.
- If the two arrows are perpendicular at 90 degrees, no shadow is cast at all, so the inner product value is exactly 0.
- If the two arrows point in opposite directions, the inner product value becomes negative.

In other words, the inner product is a method of calculating a single number that indicates **how much two arrows point in the same direction, i.e., how similar they are.**

### Algebraic Definition

The inner product of two vectors $\mathbf{u} = [u_1, u_2, \dots, u_n]^T$ and $\mathbf{v} = [v_1, v_2, \dots, v_n]^T$ in n-dimensional space is a scalar value obtained by multiplying their corresponding components and summing them all.

$$\mathbf{u} \cdot \mathbf{v} = \mathbf{u}^T \mathbf{v} = \sum_{i=1}^{n} u_i v_i = u_1 v_1 + u_2 v_2 + \dots + u_n v_n$$

### Geometric Definition

This definition uses the lengths (L2 Norm, $\Vert{}\mathbf{u}\Vert{}$) of the two vectors and the angle $\theta$ between them.

$$\mathbf{u} \cdot \mathbf{v} = \Vert{}\mathbf{u}\Vert{} \Vert{}\mathbf{v}\Vert{} \cos\theta$$$$\text{where, } \Vert{}\mathbf{u}\Vert{} = \sqrt{u_1^2 + u_2^2 + \dots + u_n^2} = \sqrt{\mathbf{u} \cdot \mathbf{u}}$$

### Cosine Similarity

To measure only **the similarity of direction**, unaffected by the magnitude of the vectors, the inner product value is normalized by dividing it by the product of the lengths of the two vectors. The range of values is -1 to 1.

$$\text{Cosine Similarity}(\mathbf{u}, \mathbf{v}) = \frac{\mathbf{u} \cdot \mathbf{v}}{\Vert{}\mathbf{u}\Vert{} \Vert{}\mathbf{v}\Vert{}} = \cos\theta$$

### Orthogonality

If the inner product of two vectors is 0, the two vectors are said to be Orthogonal, which means that the two data points are completely independent of each other and have no correlation.

$$\mathbf{u} \cdot \mathbf{v} = 0 \iff \mathbf{u} \perp \mathbf{v}$$

### AI Engineering Context

- **Self-Attention (Transformer / LLM)**: In Transformer models, when calculating the correlation between words, the inner product of the Query vector and Key vector is performed. This is called the Attention Score $\text{Attention Score} = Q K^T$, and the larger the inner product value, the more strongly the model determines that the two words are contextually linked.
- **Vector Search, RAG (Retrieval-Augmented Generation)**: It quickly calculates the inner product (or cosine similarity) between a user's query vector and document vectors stored in a VectorDB to find the most relevant documents.
- **GPU Acceleration**: Inner product operations are the basic units of matrix multiplication ($A B^T$), so they are processed rapidly in parallel through hardware like TensorCores.

<br>

## Vector Calculus & Gradient

Consider gradient descent. To use an analogy, it tells you the direction and steepness of the steepest downhill path from a mountaintop.

Imagine a computer standing at a point in a 2D space on a map, where the height of that point represents the loss error, or Loss.

Our goal is to descend to the bottom of the valley where the error is minimal.

At this point, the mathematical tool that calculates the terrain's slope beneath your feet and tells you, in the form of an arrow, which direction to step to descend fastest is the Vector Derivative, or Gradient.

### Differentiating a Scalar with Respect to a Vector: Gradient

When there is a function $f(\mathbf{x})$ whose input is an n-dimensional vector $\mathbf{x} = [x_1, x_2, \dots, x_n]^T$ and whose output is a single scalar value, the vector formed by collecting the partial derivatives with respect to each component is called the **gradient**. It is denoted as $\nabla f$ (nabla $f$).

$$\nabla f(\mathbf{x}) = \frac{\partial f}{\partial \mathbf{x}} = \begin{bmatrix} \frac{\partial f}{\partial x_1} \\ \frac{\partial f}{\partial x_2} \\ \vdots \\ \frac{\partial f}{\partial x_n} \end{bmatrix}$$

Partial differentiation, in a multivariable function with two or more variables, means differentiating by considering only one specific variable as changing and treating the others as constants. In other words, it means differentiating only one 'side' of a specific variable. If there are too many variables, it's impossible to calculate by considering every single change, so it fixes one case and proceeds.

As a **property**, the vector $\nabla f(\mathbf{x})$ points in the direction where the function $f$ increases most steeply. Therefore, to find the minimum value, one must move in the opposite direction, $-\nabla f(\mathbf{x})$.

### Differentiating a Vector with Respect to a Vector: Jacobian Matrix

When both the input is an n-dimensional vector $\mathbf{x}$ and the output is an m-dimensional vector $\mathbf{f}(\mathbf{x}) = [f_1(\mathbf{x}), f_2(\mathbf{x}), \dots, f_m(\mathbf{x})]^T$, the m x n matrix collecting all partial derivatives of the input is called the Jacobian.

$$J = \frac{\partial \mathbf{f}}{\partial \mathbf{x}} = \begin{bmatrix}  \frac{\partial f_1}{\partial x_1} & \frac{\partial f_1}{\partial x_2} & \dots & \frac{\partial f_1}{\partial x_n} \\ \frac{\partial f_2}{\partial x_1} & \frac{\partial f_2}{\partial x_2} & \dots & \frac{\partial f_2}{\partial x_n} \\ \vdots & \vdots & \ddots & \vdots \\ \frac{\partial f_m}{\partial x_1} & \frac{\partial f_m}{\partial x_2} & \dots & \frac{\partial f_m}{\partial x_n} \end{bmatrix}$$

### Key Vector Differentiation Formulas

These are frequently used differentiation forms when dealing with matrix/vector operations. (When $\mathbf{A}$ is a symmetric matrix)

1. / $\frac{\partial}{\partial \mathbf{x}} (\mathbf{a}^T \mathbf{x}) = \mathbf{a}$
2. $\frac{\partial}{\partial \mathbf{x}} (\mathbf{x}^T \mathbf{A} \mathbf{x}) = 2\math$bf{A}\mathbf{x}$  (Quadratic form differentiation)

### AI Engineering Context

- **Gradient Descent**: This is the gradient descent method. It is the core rule for updating the weight vector $\mathbf{w}$ of an artificial intelligence model in the direction that minimizes the loss function $L(\mathbf{w})$.

$$\mathbf{w}_{t+1} = \mathbf{w}_t - \eta \nabla L(\mathbf{w}_t) \quad (\eta: \text{Learning Rate})$$

- **Backpropagation**: This is the process in deep learning where the scalar error value from the output layer is differentiated with respect to the numerous weight vectors in the neural network by applying the chain rule. In terms of equations, this involves a chained product of Jacobian matrices and gradients.
- **Pytorch Autograd Engine**: Inside the framework, the Vector-Jacobian Product, which is the derivative of the weight vector with respect to the scalar loss value, is automatically calculated for each operation graph node to perform optimization.
