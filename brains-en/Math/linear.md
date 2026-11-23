# Minimum Linear Algebra Set for Deep Learning

The objects dealt with in linear algebra are actually quite similar to those in Excel.

| Linear Algebra | In Excel terms | In Deep Learning |
|---|---|---|
| Scalar | A single cell | A single number like learning rate, decay α |
| Vector | A single row | Embedding of a single token, q / k / v |
| Matrix | A single sheet (table) | Weight W, attention score table, GDN state |
| Tensor | A file bundling multiple sheets | (Batch, head, token, dimension) data |

And the operations learned in this note can ultimately be summarized into three types:

```
내적   : 줄 두 개  → 숫자 하나     ("얼마나 비슷한가")
행렬곱 : 표 × 줄   → 새로운 줄     ("변환하기")
외적   : 줄 두 개  → 표 하나       ("메모 한 장 만들기")
```

Concepts like attention or GDN are merely combinations of these three.

<br>

## Scalar Vector Matrix Tensor

> All data has a shape, and the shape is read in the order of (number of rows, number of columns).

A scalar is a single number like 3 or 0.9. A vector is a sequence of numbers in a single line, like [1, 2, 3] (a vector with 3 numbers, shape (3,)).

Each number in a vector is called an **element** or **entry**, and the number of entries is called the **dimension**. A 128-dimensional vector is 128 numbers in a single line.

A matrix is numbers arranged in a table.

```
A = [[1, 2, 3],    ← 1행
     [4, 5, 6]]    ← 2행
      ↑  ↑  ↑
     1열 2열 3열
```

Horizontal lines are called rows, and vertical lines are called columns.

This matrix has 2 rows and 3 columns, so its shape is (2, 3) and it's read as a 2x3 matrix.

> When confused: remember that '행' (row) is horizontal (like the horizontal stroke in the character), and '열' (column) is vertical (like the vertical stroke in the character). The order is always row, column.

A tensor is something with 3 or more dimensions, which can be thought of as multiple tables stacked on top of each other.

### Why arrange numbers in lines?

Computers don't understand the word "cat". So, words are converted into lists of numbers.

```
고양이 → [0.2, -1.3, 0.7, ..., 0.05]   (4096개)
```

This is called **embedding**, and words with similar meanings are trained to have similar lists of numbers. This allows us to numerically calculate how similar they are, which is the dot product from chapter 3.

### In Deep Learning

- Hidden state of a single token: a vector of length 4096, e.g. -> shape `(4096, )`
- Stack of n tokens: a matrix of shape (n, 4096)
- Divided into B batches and H heads: a tensor of shape (B, H, n, d)

The habit of looking at the **shape** first is important when studying linear algebra. If you get stuck in code or a paper, try writing down the shape next to each variable. In PyTorch, you can print `x.shape`.

<br>

## Vector Basic Operations

> Vector addition and multiplication are performed element-wise.

### Addition, Subtraction

Elements are added/subtracted at the same position, so their lengths must be equal.

```
[1, 2, 3] + [10, 20, 30] = [1+10, 2+20, 3+30] = [11, 22, 33]
[5, 5] - [1, 1] = [4, 4]
```

### Scalar Multiplication

A single number is multiplied by every element.

```
0.9 × [10, 20] = [9, 18]
```

**In deep learning:** The decay α in GDN is exactly this. "Multiplying existing memory by 0.9" means "reducing all elements to 90%", which is to say, "gradually forgetting everything".

### Element-wise Multiplication

Elements are multiplied at the same position. The symbol is ⊙, and in code, it's `a * b`.

```
[1, 2, 3] ⊙ [4, 5, 6] = [1×4, 2×5, 3×6] = [4, 10, 18]
```

As an analogy, imagine there are multiple faucets, and the second vector determines how much to close or open each faucet in the first faucet vector.

```
물      = [10, 10, 10]
꼭지    = [1.0, 0.5, 0.0]    (활짝, 반, 잠금)
통과량  = [10, 5, 0]
```

**In deep learning:** Most gates use this operation. A vector containing numbers between 0 and 1 is multiplied element-wise to determine the pass-through amount for each element. This is seen in GDN output gates and KDA's channel-wise decay. The difference between scalar multiplication and element-wise multiplication is that scalar multiplication adjusts the whole with one 'faucet,' while element-wise multiplication has a separate 'faucet' for each element.

<br>

## Dot Product

The dot product is a single number obtained by multiplying corresponding elements and summing them up, and it indicates how similar in direction two vectors are.

### Calculation Method

```
a = [1, 2, 3]
b = [4, 5, 6]

a · b = 1×4 + 2×5 + 3×6
      = 4 + 10 + 18
      = 32
```

It's the sum of element-wise products.

In academic papers, it's denoted as `a · b`, `aᵀb`, `a^\top b`, `<a, b>`, all meaning the same thing. The reason for writing `aᵀb` comes from transposition, which is discussed below.

### Analogy: Movie Recommendation Score

Let's say a movie is represented by numbers for [Action, Romance, Comedy].

```
영화 특성  = [0.9, 0.1, 0.3]    (액션 위주 영화)

사용자 A 취향 = [5, 1, 0]       (액션 좋아함)
사용자 B 취향 = [0, 4, 5]       (로맨스, 코미디 좋아함)

A의 점수 = 5×0.9 + 1×0.1 + 0×0.3 = 4.5 + 0.1 + 0   = 4.6
B의 점수 = 0×0.9 + 4×0.1 + 5×0.3 = 0 + 0.4 + 1.5   = 1.9
```

The score increases as both the preference and movie characteristic values are higher in the same category. The dot product is a tool to measure how well two lists align.

### Viewing as Direction

If you think of vectors as arrows, the dot product indicates how much two arrows point in the same direction.

```
[1, 0] · [1, 0]  =  1    같은 방향      → 큼
[1, 0] · [0, 1]  =  0    직각(90도)     → 0
[1, 0] · [-1, 0] = -1    반대 방향      → 음수
```

If the dot product is 0, the two vectors are said to be orthogonal, meaning they are completely unrelated in direction, and this concept is used very importantly later on.

### In Deep Learning

The attention score is precisely the dot product of the query and key.

```
score = q · k
```

- **query(q):** what the current token is looking for
- **key(k):** the label each token carries, saying "I am this kind of information"
- **value(v):** the actual content to be retrieved

In the same structure as movie recommendations, q is the preference, k is the movie characteristic, and a larger amount of v is retrieved from tokens with high scores.

<br>

## Norm and Normalization

> If you normalize a vector to length 1, you can compare only its direction, excluding its magnitude.

### Norm

The length of a vector is like the Pythagorean theorem. You sum the squares of each component and take the square root. It's written as ‖a‖ and called the L2 norm.

```
a = [3, 4]
‖a‖ = √(3² + 4²) = √(9 + 16) = √25 = 5
```

It's like the hypotenuse of a right triangle with sides of 3 and 4 being 5./

### Why Normalize the Length?

The dot product is affected not only by direction but also by length.

```
[1, 1]   · [1, 0] = 1
[10, 10] · [1, 0] = 10
```

[1,1] and [10,10] have the same direction, but their dot products differ by a factor of 10. This creates a problem where vectors with larger magnitudes are always considered more similar. Therefore, when you only want to compare directions, you normalize the length to 1.

### Normalization: L2 Normalize

Divide the vector by its own length.

```
[3, 4] / 5 = [0.6, 0.8]

길이 확인: √(0.6² + 0.8²) = √(0.36 + 0.64) = √1 = 1  ✓
```

The direction remains the same, and only the length becomes 1. The dot product of two vectors with length 1 is always between -1 and 1, and this value is called **cosine similarity**.

### In Deep Learning

This is why L2 normalization is applied to q and k in GDN. Fixing the length to 1 ensures that:
- When read with the same k, the stored value comes out exactly as is.
- Erasing the k direction precisely erases only that direction.
- Values do not explode during iterative calculations.

<br>

## Matrix x Vector

> Matrix x vector multiplication is a dot product for each row and a transformation that converts one vector into another.

### Analogy: Multiple Judges

Let's say an applicant's score is [singing, dancing, acting]. Each judge prioritizes different items.

```
지원자 점수 s = [80, 90, 70]   (singing, dancing, acting)

심사위원 1 가중치 = [0.5, 0.3, 0.2]   (prioritizes singing)
심사위원 2 가중치 = [0.1, 0.1, 0.8]   (prioritizes acting)
```

If you stack the judges' weights row by row, you get a matrix.

```
W = [[0.5, 0.3, 0.2],    ← Judge 1
     [0.1, 0.1, 0.8]]    ← Judge 2
```

Each judge's final score is the **dot product of their weight row and the applicant's score**.

```
Judge 1: 0.5×80 + 0.3×90 + 0.2×70 = 40 + 27 + 14 = 81
Judge 2: 0.1×80 + 0.1×90 + 0.8×70 =  8 +  9 + 56 = 73

W s = [81, 73]
```

Matrix x vector = **dot product for each row, with the results collected vertically**. That's all there is to it.

### Viewing by Shape

```
W     : (2, 3)    2 judges, 3 items
s     : (3,)      3 items
W s   : (2,)      Scores of 2 judges
```

A vector of length 3 has been transformed into a vector of length 2. The number of columns in the matrix equals the input length, and the number of rows equals the output length.

### In Deep Learning

This is called a linear transformation, and PyTorch's `nn.Linear` is exactly this (with the addition of a bias `b`).

```
nn.Linear(4096, 1024)
  → Internally, there is a (1024, 4096) matrix W
  → Transforms input (4096,) to output (1024,)
```

You can think of it as 1024 judges scoring 4096 input items with different weights. Learning is the process of adjusting these weights.

q, k, v projections are all this too. The same hidden state `h` is passed through three different matrices to create q, k, and v.

```
q = W_q h      ("transforms to what this token is looking for")
k = W_k h      ("transforms to what label this token is")
v = W_v h      ("transforms to what content this token conveys")
```

<br>

## Transpose

> Transposition is swapping rows and columns, primarily used to match shapes for matrix multiplication. The symbol ᵀ is different from 't' for 't-th'.

### Meaning

It's like flipping a table along its top-left to bottom-right diagonal. The 1st row becomes the 1st column, and the 2nd row becomes the 2nd column.

```
A = [[1, 2, 3],        Aᵀ = [[1, 4],
     [4, 5, 6]]              [2, 5],
                             [3, 6]]

shape: (2, 3)  →  (3, 2)
```

The notation is `Aᵀ`, `A^\top`, `A^T`, and in code, it's `A.T` or `A.transpose(-1, -2)`.

### Why is it needed? To match shapes

Matrix multiplication requires matching shapes. In attention, Q and K are both `n, d`, but they cannot be multiplied as is.

- n: number of tokens, sentence length. e.g., if there are 1,000 tokens, n=1000
- d: length of the vector representing a single token (number of dimensions). e.g., 128

```
Q (n, d) × K (n, d)     → Doesn't work
Q (n, d) × Kᵀ (d, n)    → Works! Result (n, n)
```

- Reason it doesn't work: The inner dimensions, d and n, are different. A row from Q (length d) and a row from K (length n) have different lengths, so their dot product cannot be computed.
- Reason it works: Transposing makes the inner dimensions d and d, which are the same. This allows each token in q to be paired with each token in k for a dot product, resulting in an (n, n) score matrix.

We transpose k to match the shapes. This is why `QKᵀ` appears in the attention formula.

### Vector Transposition

In mathematics, vectors are fundamentally considered as **vertically oriented** (column vectors, shape `(d, 1)`). Transposing them makes them horizontally oriented (row vectors, shape `(1, d)`).

```
k  = [[3],          kᵀ = [[3, 4, 5]]
      [4],
      [5]]
shape (3, 1)             shape (1, 3)
```

Therefore, the dot product is written as `aᵀb`. This is because a horizontal vector (1, d) multiplied by a vertical vector (d, 1) results in (1, 1), which is a single scalar value.

### Common Confusions

```
k_t    subscript t  →  "t-th token"   (index)
kᵀ     superscript ᵀ    →  "transpose"         (horizontal orientation)
k_tᵀ   both          →  "the k of the t-th token, transposed"
```

<br>

## Matrix x Matrix

> (a,b)x(b,c) = (a,c) The inner numbers must be the same to multiply, and the outer numbers determine the shape of the result.

### Shape Rules

```
(a, b) × (b, c) = (a, c)
     ↑    ↑
     안쪽이 같아야 함

 ↑            ↑
 바깥쪽이 결과
```

```
(2, 3) × (3, 4) = (2, 4)    가능
(2, 3) × (2, 4)             불가능 (3 ≠ 2)
(n, d) × (d, n) = (n, n)    attention 점수표
```

With this one rule, you can verify most equations in papers. If you get stuck while reading an equation, write the shape under each symbol and check it with this rule.

### Calculation Method

**Matrix x Matrix = Matrix x (Vector)** repeated for each column of the right matrix.

Cell at row i, column j of the result = i-th row of the left matrix x j-th column of the right matrix (dot product)

```
A = [[1, 2],      B = [[5, 6],
     [3, 4]]           [7, 8]]

                 B의 1열   B의 2열
                 [5, 7]    [6, 8]
A의 1행 [1, 2]     19        22
A의 2행 [3, 4]     43        50

계산:
(1,1) = [1, 2] · [5, 7] = 1×5 + 2×7 =  5 + 14 = 19
(1,2) = [1, 2] · [6, 8] = 1×6 + 2×8 =  6 + 16 = 22
(2,1) = [3, 4] · [5, 7] = 3×5 + 4×7 = 15 + 28 = 43
(2,2) = [3, 4] · [6, 8] = 3×6 + 4×8 = 18 + 32 = 50

AB = [[19, 22],
      [43, 50]]
```

You can remember it as taking the rows from the left matrix and the columns from the right matrix, pairing them up, and performing a dot product, similar to the table in the middle.

### Changing the Order Yields Different Results

Generally, AB != BA.

While 2x3 = 3x2 for numbers, this is not true for matrices. Try calculating BA yourself using the example above (the answer is below, but...). If the shapes are different, it's often only possible to multiply in one order.

### In Deep Learning, Multiple Tokens at Once

When transforming a single token (vector), if you stack n tokens into a matrix X (n, 4096), all of them can be transformed with a single matrix multiplication.

```
X      : (n, 4096)       토큰 n개
Wᵀ     : (4096, 1024)
X Wᵀ   : (n, 1024)       토큰 n개 각각 변환된 결과
```

Since GPUs perform matrix multiplication incredibly fast, grouping deep learning operations into the largest possible matrix multiplications is key to performance. Algorithms like GDN's chunkwise approach aim to convert calculations that were done token by token into matrix multiplications.

### Cost Calculation

For `(a, b)` x `(b, c)`, the result has a x c cells, and it requires b multiplications + approximately b additions per cell.

```
연산량 ≈ 2 × a × b × c  FLOPs
```

Applying `nn.Linear(4096, 4096)` to a single token results in 2 x 4096 x 4096 ≈ 33.55 million FLOPs.

Since there are 4096 x 4096 parameters (weight cells), the rough estimate for serving, **number of parameters x 2 = FLOPs per token**, comes from this.

<br>

## Outer product

An outer product creates a table from two vectors, like a single memo where `v` is written at address `k`.

| | Inner Product | Outer Product |
|---|---|---|
| Notation | `aᵀb` | `a bᵀ` |
| Shape | `(1, d) × (d, 1)` | `(m, 1) × (1, n)` |
| Result | A single scalar `(1, 1)` | A table `(m, n)` |
| Meaning | How similar they are | A memo linking the two |

### Calculation Method

It involves multiplying each element of the first vector by the entire second vector, stacking the results row by row. It's like creating a multiplication table.

```
v = [1, 2]
k = [3, 4, 5]

         k:  3    4    5
v: 1  →  [  3,   4,   5 ]
v: 2  →  [  6,   8,  10 ]

v kᵀ = [[3, 4,  5],
        [6, 8, 10]]

shape: (2, 3)
```

A matrix created from two vectors in this way is called a **rank-1** matrix. This means that although there are 6 elements, the actual information content is only equivalent to two vectors.

### Outer Product as a Memo: `v` written at address `k`

```
k = [0.6, 0.8]   (address, length 1)
v = [1, 2]       (content to write)

M = v kᵀ = [[0.6, 0.8],
            [1.2, 1.6]]
```

Reading `k` with the same address (Multiplying `M` by `k`)

```
M k = [0.6×0.6 + 0.8×0.8,   1.2×0.6 + 1.6×0.8]
    = [0.36 + 0.64,          0.72 + 1.28]
    = [1, 2]
```

The `v` that was written appears as is.

Reading with a different address `q` = [-0.8, 0.6] (Orthogonal to `k`, inner product is 0):
```
M q = [0.6×(-0.8) + 0.8×0.6,   1.2×(-0.8) + 1.6×0.6]
    = [-0.48 + 0.48,            -0.96 + 0.96]
    = [0, 0]
```

Nothing comes out. If the address is different, this memo cannot be read.

### Why this happens

It becomes immediately clear if you change the parenthesis placement, though I'll mention the associative property below.

```
(v kᵀ) q  =  v (kᵀ q)  =  v × (inner product of k and q)
```

- If `k` and `q` are the same, the inner product is 1 -> `v` x 1 = `v`
- If they are orthogonal, the inner product is 0 -> `v` x 0 = 0
- If it's in between -> `v` comes out scaled down proportionally

The statement that attention brings values from similar keys can be expressed in matrix form as follows: an outer product creates the memo, and an inner product determines how much to retrieve from it.

<br>

## Unit Matrix, Diagonal Matrix, and Erasing

> I is a matrix that does nothing, a diagonal matrix scales each component, I - k kᵀ "erases only the k direction"

### Unit Matrix I

It's a matrix where only the diagonal elements are 1 and the rest are 0. Multiplying by it changes nothing. It plays the same role as the number 1.

```
I = [[1, 0],
     [0, 1]]

I [3, 4] = [1×3 + 0×4,  0×3 + 1×4] = [3, 4]   ← unchanged
```

When I appears in an equation, you can read it as "the original unchanged." `I - something` means "something subtracted from the original."

### Diagonal Matrix

It's a matrix with numbers only on the diagonal. When multiplied, it scales each component by a different factor, either increasing or decreasing it. It's like an element-wise multiplication faucet.

```
D = [[0.5, 0  ],
     [0,   0.9]]

D [10, 10] = [5, 9]    ← First component 50%, second component 90%
```

In papers, if you see `diag(β)`, you can understand it as a diagonal matrix with the vector β laid out on its diagonal.

### k kᵀ: Matrix that erases only the k direction (no need to understand if you don't know GDN)

In GDN, I'll treat `(I - β k kᵀ)` as a number. For simplicity, let β = 1 and k = [1, 0].

```
k kᵀ = [[1, 0],        (outer product, Chapter 8)
        [0, 0]]

I - k kᵀ = [[1-1, 0-0],    = [[0, 0],
            [0-0, 1-0]]       [0, 1]]
```

If you multiply this matrix by any vector:

```
(I - k kᵀ) [3, 4] = [0×3 + 0×4,  0×3 + 1×4] = [0, 4]
```

Only the k-direction component (the first component) disappears, and the rest remains unchanged.

As an analogy, it's like erasing only the shadow (that directional component) created when you shine a flashlight in the k-direction. In mathematical terms, it's a projection that subtracts the k-direction component.

If β = 0.5, it erases only half.

```
I - 0.5 k kᵀ = [[0.5, 0],
                [0,   1]]

[3, 4]  →  [1.5, 4]
```

### Eigenvalue Sneak Peek

If there's a vector that, when multiplied by a matrix, **doesn't change direction but only scales in length by some factor**, that scaling factor is called an eigenvalue. `I - 0.5 k kᵀ` scales the k-direction by 0.5 and its orthogonal direction by 1, so its eigenvalues are 0.5 and 1.

In GDN notes, it's mentioned that if an eigenvalue exceeds 1, the state diverges. This can be understood as follows:

```
0.9를 1000번 곱하면  → 거의 0       (gradually forgotten, stable)
1.0을 1000번 곱하면  → 1           (remains unchanged)
1.1을 1000번 곱하면  → 엄청 큰 수   (explodes, unstable)
```

GDN multiplies this matrix once per token, so after thousands of tokens, the scaling factor accumulates thousands of times. Therefore, k is normalized and the range of β is limited to prevent the scaling factor from exceeding 1.

<br>

## Hand-tracing linear attention and the delta rule

> Write externally, read as matrix x vector, rewrite by the difference – that's the delta rule.

With what we've learned so far, we can directly calculate the core equations from the GDN notes. Let's try it with a small 2D example.

### Writing two memories (linear attention)

We write the values for two addresses, one by one. The two addresses are orthogonal.

```
k1 = [1, 0],  v1 = [5, 5]     ("drawer 1" contains [5, 5])
k2 = [0, 1],  v2 = [7, 0]     ("drawer 2" contains [7, 0])
```

We create two memory entries (outer product) and add them.

```
v1 k1ᵀ = [[5, 0],      v2 k2ᵀ = [[0, 7],
          [5, 0]]                [0, 0]]

S = v1 k1ᵀ + v2 k2ᵀ = [[5, 7],
                       [5, 0]]
```

This S is the **state (memory)**. The two memory entries are combined into one table.

Reading (matrix x vector)

```
S k1 = [5×1 + 7×0,  5×1 + 0×0] = [5, 5]   ← v1 as is
S k2 = [5×0 + 7×1,  5×0 + 0×1] = [7, 0]   ← v2 as is
```

Since the addresses are orthogonal, they are stored cleanly without mixing.

### Reading with non-orthogonal addresses

```
If we read with k3 = [0.6, 0.8]

S k3 = [5×0.6 + 7×0.8,  5×0.6 + 0×0.8]
     = [3 + 5.6,  3]
     = [8.6, 3]
```

We get a value that is a mix of v1 and v2, not purely one or the other.

The problem here is that in a 2D space, there are only two orthogonal directions. Therefore, from the third address onwards, it must overlap with existing addresses. In actual models, if `d_k = 128`, there are only 128 orthogonal directions, but there are tens of thousands of tokens. So, every time memory is stacked and read, other memories get mixed in. This is the problem of **memory contamination when only adding in GDN**.

### Overwriting with the delta rule

Now, we want to change the value in drawer 1 from [5, 5] to [1, 1]. Let β = 1.

What happens if we just add new memory like in linear attention?

```
If we read S + [1,1] k1ᵀ → [5, 5] + [1, 1] = [6, 6]    ← Mixed with old value. Failed.
```

The delta rule **writes only the difference from the current value**.

```
1) Read current value:   v_old = S k1 = [5, 5]
2) Difference from desired value:    delta = [1, 1] - [5, 5] = [-4, -4]
3) Write the difference to memory:  S_new = S + delta k1ᵀ

delta k1ᵀ = [[-4, 0],
             [-4, 0]]

S_new = [[5 + (-4), 7 + 0],    = [[1, 7],
         [5 + (-4), 0 + 0]]       [1, 0]]
```

```
S_new k1 = [1, 1]   ← Changed ✓
S_new k2 = [7, 0]   ← Unchanged ✓
```

**Only drawer 1 was accurately changed, and drawer 2 was untouched.** This is the core of the delta rule: selective overwriting.

### Viewing the same result as erase and write

Calculating with the other notation from the GDN notes, `S(I - k kᵀ) + v kᵀ`, yields the same answer. This notation reads as "first erase, then write new."

```
① Erase: S (I - k1 k1ᵀ)

   [[5, 7],  ×  [[0, 0],   =  [[0, 7],
    [5, 0]]      [0, 1]]       [0, 0]]
                                ↑ Drawer 1 is cleared

② Write: + v k1ᵀ

   [[1, 0],
    [1, 0]]

③ Result:

   [[1, 7],
    [1, 0]]     ← Same as 10.3
```

### Moving towards GDN

Adding the GDN decay α simply adds one more step.

```
0) Forget:   S = α × S          (Scalar multiplication by 2.2, slightly blurring everything)
1) Read:   v_old = S k        (Chapter 5, matrix × vector)
2) Difference:   delta = β × (v - v_old)
3) Write:   S = S + delta kᵀ   (Chapter 8, outer product)
4) Output:   o = S q            (Chapter 5, matrix × vector)
```

## Associative Property: Changing the Order of Multiplication

> AB)C = A(BC). The result is the same, but the cost can differ, and this is why linear attention is fast.

### Rule

```
(AB)C = A(BC)
```
When multiplying three matrices, **the result is the same regardless of which two are multiplied first**.

### Different from AB ≠ BA

```
AB ≠ BA          Changing the left-right order → Not allowed
(AB)C = A(BC)    Left-right order remains, only parenthesis position changes → Allowed
```

### Same Result, Different Cost

Let's get a feel for it with a simple example: a calculation involving three vectors.

```
a: (1000, 1)   bᵀ: (1, 1000)   c: (1000, 1)

방법 1: (a bᵀ) c
   a bᵀ   = (1000, 1000)  ← First creates a table with 1 million cells
   × c    = (1000, 1)
   
방법 2: a (bᵀ c)
   bᵀ c   = (1, 1)        ← A single number (dot product)
   a × 숫자 = (1000, 1)
```
The answer is the same, but Method 1 creates a table with 1 million cells, while Method 2 creates only a single number. **A single change in parenthesis position can lead to a thousand-fold difference in cost.**

### Applying to Attention

Let's say there are n tokens and dimension d. Many resources use T for sequence length, but to avoid confusion with the transpose symbol ᵀ, we'll use n here.

```
Q: (n, d)    K: (n, d)    V: (n, d)
```
Standard attention first calculates Q Kᵀ.

```
Q Kᵀ       : (n, d) × (d, n) = (n, n)   ← One score for each token pair
softmax    : (n, n)                      ← Sum of each row becomes 1 (Chapter 12)
× V        : (n, n) × (n, d) = (n, d)

Largest intermediate result: (n, n) table
Cost ≈ n × n × d   → Proportional to the square of the number of tokens
```
Linear attention: If softmax is removed, parentheses can be moved.

```
(Q Kᵀ) V  =  Q (Kᵀ V)

Kᵀ V       : (d, n) × (n, d) = (d, d)   ← This is state S
Q × (Kᵀ V) : (n, d) × (d, d) = (n, d)

Largest intermediate result: (d, d) table (independent of the number of tokens!)
Cost ≈ n × d × d   → Proportional to the number of tokens
```
If n = 100,000 and d = 128, then n²d is n/d ≈ 780 times larger than nd².

### Why Softmax Cannot Be Present

Softmax changes the numbers in the (n,n) score table by **looking at each row entirely** (Chapter 12). For this to happen, the (n,n) table must be completed beforehand. Therefore, Q Kᵀ must be calculated first, and the parentheses cannot be moved.

If softmax is removed, all operations are just matrix multiplications, allowing the associative property to be used. **Removing softmax allows parentheses to be moved** — that's the entire starting point of linear attention.

Instead, the ability to precisely select, which softmax provided, is weakened. Gating and the delta rule were introduced to compensate for this.

For reference, `Kᵀ V` has a `d_k, d_v` shape, and `S = Σ v kᵀ` mentioned in GDN has a `(d_v, d_k)` shape. They are just transpositions of each other and contain the same information. Since the orientation differs across papers, it's best to confirm by shape.

<br>

## Softmax (What's needed to understand attention)

> Softmax converts a list of scores into proportions that sum to 1. Larger scores become even larger, smaller scores become even smaller.

Although it's not linear algebra, it always appears with the attention equation, so let's look at it together.

### Calculation Method

1. Apply exp (exponential function) to each score. exp(x) is the value of e (approx. 2.718) multiplied by itself x times, and it is always positive.
2. Sum all values.
3. Divide each value by the sum.

```
점수 = [1, 2, 3]

1) exp:  [e¹, e², e³] ≈ [2.72, 7.39, 20.09]
2) 합:   2.72 + 7.39 + 20.09 ≈ 30.19
3) 나누기: [2.72/30.19, 7.39/30.19, 20.09/30.19]
         ≈ [0.09, 0.24, 0.67]

합 확인: 0.09 + 0.24 + 0.67 = 1.00 ✓
```

- The results are all positive and sum to 1, so they can be used as proportions or probabilities.
- Even though the score differences are only 1, 1, the proportions spread out to 0.09 / 0.24 / 0.67. Due to exp, larger scores take a disproportionately larger share. This is the source of attention's ability to pinpoint the most relevant tokens.

### Reading the entire attention mechanism in words

```
attention(Q, K, V) = softmax(Q Kᵀ / √d) V
```

1. Q Kᵀ: Dot product of all queries and all keys → similarity score table (n,n)
2. / √d: Normalizes the scale because the dot product increases with higher dimensions (if d = 128, divide by √128 ≈ 11.3)
3. softmax: Converts scores into proportions for each row → "how much percentage of each token to refer to"
4. × V: Mixes and retrieves values according to those proportions.

<br>

## Batch, Head, and Reading einsum

> Front dimensions (batch, head) are just repetitions; in einsum, characters that disappear are axes multiplied and summed.

### 13.1 Front Dimensions Are Just Repetitions

When performing matrix multiplication @ between `(B, H, n, d)` tensors, only the last two dimensions are multiplied, and the preceding B and H dimensions are repeated independently for their respective counts.

```
Q:      (B, H, n, d)
Kᵀ:     (B, H, d, n)
Q @ Kᵀ: (B, H, n, n)

→ This is equivalent to performing B × H instances of (n, d) × (d, n) matrix multiplications all at once.
```

It's like applying the same formula to all sheets in a multi-sheet Excel file.

### einsum

When looking at GDN, you might see expressions like `torch.einsum('bhkv,bhk->bhv', S, kt)` in the code. While it might look like a cipher at first, there are three rules:

1.  The comma-separated groups of characters are the axis names for each input tensor.
2.  The right side of the arrow indicates the output axes.
3.  **Characters present in the input but absent in the output are multiplied and summed along that axis (=dot product).**

Let's break down an example.

```
torch.einsum('bhkv,bhk->bhv', S, kt)

S axes: b h k v
kt axes: b h k
Output axes: b h v

Disappearing character: k
→ "For each batch (b), for each head (h), take the dot product of S and kt along the k axis"
→ "Read k address with S" (M k from Chapter 8)
```

Here's a summary of common patterns:

| einsum | Disappearing Character | Meaning |
|---|---|---|
| `'d,d->'` | d | Vector dot product → single number |
| `'k,v->kv'` | None, new axes created | Outer product → create table |
| `'kv,k->v'` | k | Read table by k |
| `'nd,md->nm'` | d | Q Kᵀ (attention score table) |
| `'bhk,bhv->bhkv'` | None | Batch/head-wise outer product (creating memory in GDN) |

If it disappears, it's a dot product; if it appears, it's an outer product. Remembering just this will allow you to understand most cases.

<br>

## Serving and Connection

Serving memory calculation ultimately comes down to the number of elements in the shape x number of bytes.

| Data Type | Per Cell |
|---|---|
| fp32 | 4 byte |
| bf16, fp16 | 2 byte |
| fp8, int8 | 1 byte |
 

### KV Cache

```
KV cache bytes per token, per layer
= 2 (K and V) × number of KV heads × head dim × number of bytes
```

### GDN State

```
State bytes per layer, per sequence
= number of value heads × d_k × d_v × number of bytes
```

Qwen3-Next's GDN layer: 32 × 128 × 128 × 4 byte(fp32) = 2,097,152 byte = 2 MiB
