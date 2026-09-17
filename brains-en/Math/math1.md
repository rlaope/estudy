# Systems of Linear Equations and Matrices

## Systems of Linear Equations

In our daily lives, we unknowingly calculate countless unknowns.

For example, imagine ordering two Americanos and one Cafe Latte at a cafe and paying 11,000 won.

The next day, at the same cafe, you buy one Americano and two Cafe Lattes and pay 13,000 won.

What is the unit price of an Americano and a Cafe Latte, respectively?

Recalling the method learned in middle school math, we can set the price of an Americano as $x$ and a Cafe Latte as $y$, and form the following two equations.

$$2x + y = 11000$$

$$x + 2y = 13000$$

Expressing unknown values (variables) as first-degree equations, and when multiple such equations come together to find a common solution, this structure is called a System of Linear Equations.

The reason it's called 'linear' is that when an equation with two variables, like $2x + y = 11,000$, is plotted on a coordinate plane, it appears as an unbent line.

### Linear Equations from an AI Perspective

The example just given only had two variables, $x$ and $y$, but the world that AI deals with is much broader.

For instance, let's say we're building an AI model to predict house prices. Factors influencing house prices, such as house area, number of rooms, distance from a subway station, and year of construction, number in the tens or hundreds.

AI predicts the final house price by multiplying these features by appropriate weights.

$$\text{집값} = (w_1 \times \text{면적}) + (w_2 \times \text{방 개수}) + (w_3 \times \text{역 거리}) + \dots + b$$

The process of collecting house price data for thousands of households and finding the appropriate values for $w_1, w_2, w_3 \dots$ is essentially identical to solving a massive system of linear equations involving thousands of unknowns and thousands of equations.

#### Why Linearity is Powerful in AI

While many phenomena in the world are non-linear and curved, computers are far better at repeating simple **linear operations** hundreds of billions of times than complex curves.

The core of AI engineering lies in transforming the complex real world into a **collection of linear equations** so that computers can process them at the fastest possible speed.

<br>

## Definition of a Matrix

### Matrices Starting from an Excel Table

If you were to write out a system of 100 equations with 100 unknowns, like $x_1, x_2, \dots, x_{100}$, on paper, with all the symbols and addition signs, it would be cumbersome and hard to read. So, mathematicians conceived of a table that neatly arranges only the numbers in a grid, stripping away the symbols and addition signs. This is a Matrix.

Let's revisit the cafe order equations we set up earlier.

- $2x + 1y = 11000$
- $1x + 2y = 13000$

Here, we take only the numbers (coefficients) in front of the variables and gather them into a rectangular shape within square brackets.

$$A = \begin{bmatrix} 2 & 1 \\ 1 & 2 \end{bmatrix}$$

A collection of numbers arranged horizontally and vertically like this is called a **matrix**.

- **Row**: Refers to a horizontal line. In the matrix above, the first row is $[2 \quad 1]$.
- **Column**: Refers to a vertical line. In the matrix above, the first column is $\begin{bmatrix} 2 \ 1 \end{bmatrix}$.

A matrix with $m$ rows and $n$ columns is called an $m \times n$ matrix (m by n matrix), and individual numbers within the matrix are denoted as $a_{ij}$ (element at row i, column j), indicating their row and column position.

```
1열   2열
1행 [   2     1   ]   ->  a_11 = 2,  a_12 = 1
2행 [   1     2   ]   ->  a_21 = 1,  a_22 = 2
```

### What Matrices Mean in AI: Data Bundles

When dealing with computer vision data processing or AI DL frameworks like PyTorch and TensorFlow, you constantly encounter matrices.

- **Image Data:** A 28 x 28 grayscale image is a 28x28 matrix where each pixel's intensity value is represented by a number between 0 and 255.
- **Batch Data:** When inputting data for 100 users into an AI at once, if each user has 5 feature data points, this forms a 100 x 5 matrix.

In other words, a matrix is more than just a mathematical tool; it is a **standard-sized grid box that computers use to store data for batch processing large amounts of information**.

<br>

## Matrix Operations: Regular and Structured Addition and Multiplication of Data

Just as numbers can be added and multiplied, operations can also be defined for matrices, which are containers for numbers.

However, matrix operations have their own unique rules.

### Addition and Subtraction: Pairing Elements at the Same Position

To add or subtract two matrices, **their dimensions (number of rows and columns) must be exactly the same.**

The calculation involves adding or subtracting elements at the corresponding positions.

$$\begin{bmatrix} 1 & 3 \\ 2 & 4 \end{bmatrix} + \begin{bmatrix} 5 & 1 \\ 0 & 2 \end{bmatrix} = \begin{bmatrix} 1+5 & 3+1 \\ 2+0 & 4+2 \end{bmatrix} = \begin{bmatrix} 6 & 4 \\ 2 & 6 \end{bmatrix}$$

It's like having sales reports for January and February, and combining them into a single two-month total sales report by matching items at their respective positions.

### Scalar Multiplication: Applying a Multiplier to All Elements

This operation involves multiplying the entire matrix by a single number (which is called a **scalar** in mathematics).

You simply multiply every element inside the matrix by that number.

$$3 \times \begin{bmatrix} 2 & 1 \\ 0 & 4 \end{bmatrix} = \begin{bmatrix} 3 \times 2 & 3 \times 1 \\ 3 \times 0 & 3 \times 4 \end{bmatrix} = \begin{bmatrix} 6 & 3 \\ 0 & 12 \end{bmatrix}$$

This is like a situation where you have a product price matrix and uniformly increase all product prices by a factor of 3.

### Matrix Multiplication: Dot Product of Rows and Columns

The most important and often initially confusing part of matrix operations for beginners is **matrix multiplication**.

Matrix multiplication is not simply multiplying elements at the same position; instead, it proceeds by pairing a row from the first matrix with a column from the second matrix, multiplying their corresponding elements, and then summing the results.

When $A = \begin{bmatrix} 1 & 2 \\ 3 & 4 \end{bmatrix}$ and $B = \begin{bmatrix} 5 & 6 \\ 7 & 8 \end{bmatrix}$, the process to find the first element (row 1, column 1) of $A \times B$ is as follows.

1. Take row 1 of $A$, $[1 \quad 2]$, and column 1 of $B$, $\begin{bmatrix} 5 \ 7 \end{bmatrix}$.
2. Multiply their corresponding components and sum them: $(1 \times 5) + (2 \times 7) = 5 + 14 = 19$.
3. Place this value in the (row 1, column 1) position of the resulting matrix.

Repeat this process for all combinations of rows and columns.

$$A \times B = \begin{bmatrix} (1 \times 5 + 2 \times 7) & (1 \times 6 + 2 \times 8) \\ (3 \times 5 + 4 \times 7) & (3 \times 6 + 4 \times 8) \end{bmatrix} = \begin{bmatrix} 19 & 22 \\ 43 & 50 \end{bmatrix}$$

For this multiplication to be valid, the **number of columns in the first matrix must match the number of rows in the second matrix.**
. (Multiplying an $m \times \mathbf{k}$ matrix by a $\mathbf{k} \times n$ matrix results in an $m \times n$ matrix.

#### Why is Matrix Multiplication Defined This Way?

Matrix multiplication is not just a mathematical game; it is the most concise mathematical tool for expressing the **Linear Transformation process where input data is converted into new features through multiple layers of weights** in deep learning neural networks.

With a single line of matrix multiplication code, $Y = XW$, tens of thousands of input data points can be processed simultaneously with thousands of artificial neural network nodes ($W$).

<br>

## Relationship Between Matrices and Systems of Linear Equations: $Ax = b$

Now, let's get to the core of this chapter.

Let's see how systems of linear equations and matrices come together.

Looking back at the Americano and Cafe Latte problem we set up earlier,

- $2x + 1y = 11000$
- $1x + 2y = 13000$

If we apply the definition of matrix multiplication in reverse, these equations can be perfectly rewritten as **a single matrix multiplication equation** as follows.

$$\begin{bmatrix} 2 & 1 \\ 1 & 2 \end{bmatrix} \begin{bmatrix} x \\ y \end{bmatrix} = \begin{bmatrix} 11000 \\ 13000 \end{bmatrix}$$

Shall we check if they are truly the same?

Multiplying row 1 and the column: $2x + 1y = 11000$

Multiplying row 2 and the column: $1x + 2y = 13000$

The original complex system of equations is restored. Let's abbreviate it simply with symbols.

$$A \mathbf{x} = \mathbf{b}$$

- $A$: Coefficient Matrix — The relationship between data or the rules of the system
- $\mathbf{x}$: Unknown Vector — The solution we are looking for
- $\mathbf{b}$: Result Vector — The final observed outcome

### $Ax = b$

This representation is revolutionary because it compresses the context, which previously stretched out with numerous symbols and additions like $x, y, z \dots$, into three characters: $A \mathbf{x} = \mathbf{b}$.

The advantages this abbreviation offers are overwhelming:

1.  **Dimensional Scalability:** Regardless of whether there are 2 unknowns or 100 million, it can be expressed in a single line: $A\mathbf{x} = \mathbf{b}$.
2.  **Simplified Code Implementation:** Instead of using a for loop to calculate 100 equations one by one, the parallel processing capabilities of a GPU can be used to vectorize matrices $A$ and $\mathbf{x}$ simultaneously, improving processing speed by thousands of times.

Thus, transforming and solving systems of linear equations in the matrix equation form $A\mathbf{x} = \mathbf{b}$ is the pillar supporting all modern artificial intelligence and data science computations.
