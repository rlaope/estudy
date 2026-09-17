# Matrix Decomposition

### Why Decompose Matrices?

In mathematics, when you prime factorize the number 12 into 2 x 2 x 3, its divisors and structure become immediately apparent.

Matrix decomposition is the process of breaking down a complex and massive data 'plate' (matrix) into **2-3 simpler matrix products with clear characteristics.**

In other words, it's used to make complex calculations much faster and to extract only the core features hidden within the data.

## LU Decomposition

It's a technique that breaks down a large system of linear equations into two matrices: a lower triangular matrix L and an **upper triangular matrix U**, making it much easier to find solutions.

$$A = L \cdot U$$

- **A (Original Matrix)**: The complex overall data or system of equations that needs to be solved.
- **L (Lower Triangular Matrix):** A matrix where all numbers above the main diagonal are zero. Values only remain in the lower triangular region.
- **U (Upper Triangular Matrix):** A matrix where all numbers below the main diagonal are zero. Values only remain in the upper triangular region.

### Why It's Used and What Values It Represents

It's due to **computational efficiency**. When solving a system of linear equations (Ax = b) with tens of thousands of variables on a computer, calculating it every time takes a long time.

However, once the matrix is decomposed into L and U, even if the right-hand side 'b' changes repeatedly, **solutions can be found consecutively within milliseconds just by substitution.**

In terms of meaning, L represents the record/steps of transformations made while simplifying the equations, and U represents the final coefficients remaining after Gaussian elimination.

### Applications in AI Engineering

- **Second-Order Optimization Algorithms (Newton-Raphson / Hessian Operations):** In second-order optimization techniques that converge faster than SGD (Stochastic Gradient Descent) during AI model training, LU decomposition is used instead of directly calculating the inverse matrix, thereby reducing computational burden.
- **Physics-Informed Neural Networks (PINN):** Used to rapidly derive system solutions within AI models that learn differential equations.

<br>

## Singular Value Decomposition (SVD)

It's a technique that decomposes any matrix, regardless of its form, into three basic operations: 'first rotation -> scaling/stretching -> second rotation', thereby **extracting only the most important information (features) from the data.**

### Formula and Notation

$$A = U \cdot \Sigma \cdot V^T$$

- $A$ ($m \times n$ original matrix): The entire data set of arbitrary form (e.g., [user $\times$ movie] rating table).
- $U$ ($m \times m$ orthogonal matrix): The core axes/directions from a row perspective. (e.g., user preference characteristics)
- $\Sigma$ ($m \times n$ diagonal matrix, 'Sigma'): A matrix with values only on its diagonal, representing the importance (singular value) of each feature. Values are sorted in descending order from largest to smallest.
- $V^T$ ($n \times n$ transpose of an orthogonal matrix): The core axes/directions from a column perspective. (e.g., movie genre characteristics)

Even if **only the top few most important values on the diagonal of the Sigma matrix are kept and the rest are set to zero**, over 90% of the original data A can be reconstructed. This allows for noise reduction and significant capacity reduction.

U and V represent the hidden concepts/axes within the data, while Sigma indicates how important those concepts are in explaining the data.

### Applications in AI Engineering

- **Recommendation Systems (Collaborative Filtering):** In services like Netflix or Coupang, the user x item matrix is decomposed using SVD to predict and recommend preferences for items that users have not yet rated.
- **Natural Language Processing (LSA, Latent Semantic Analysis):** Decomposes the document x word matrix to identify hidden contexts and topics behind words.
- **LLM Model Weight Matrix Compression:** Decomposes the weight matrices of large Transformer AI models using SVD (Truncated SVD) to optimize memory usage and computational speed while maintaining model precision.
