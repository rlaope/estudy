# One-Hot Encoding, Multicollinearity

### One-Hot Encoding

It is a technique that converts categorical data into vector values of 0s and 1s so that machine learning/deep learning models can handle them.

For example, if we assign values like apple-1 and banana-2 to data such as apples and bananas, the model might incorrectly interpret the relationship as banana > apple (2 > 1) during analysis. This arises from a misinterpretation of whether the input data's features have regularity or classification properties, which can cause problems in linear regression.

The simplest and most intuitive way to quantify non-numerical data like text and categories.

| Class | One-Hot Vector     |
| ----- | ------------------ |
| Apple | `[1, 0, 0, 0]`     |
| Banana | `[0, 1, 0, 0]`     |
| Grape | `[0, 0, 1, 0]`     |
| Strawberry | `[0, 0, 0, 1]` |

| Class | Apple | Banana | Grape | Strawberry |
| ----- | ----- | ------ | ----- | ---------- |
| Apple | 1     | 0      | 0     | 0          |
| Banana | 0     | 1      | 0     | 0          |
| Grape | 0     | 0      | 1     | 0          |
| Strawberry | 0     | 0      | 0     | 1          |

<br>

## Multicollinearity Problem

It is a phenomenon where strong correlations exist among independent variables (input features).

This problem arises from the question of whether each independent variable is truly independent, or if they share related meanings.

When there are independent variables like blood alcohol content and frequency of drinking alcohol over a week, and a dependent variable like lower test scores, a question might arise: can the frequency of drinking alcohol over a week be high while blood alcohol content is low?

Naturally, if the frequency of drinking alcohol over a week increases, blood alcohol content will also increase, indicating a strong correlation between the two independent variables.

This can also occur in one-hot encoding, arising from the characteristic that if all 'n' categories are one-hot encoded, their sum will always be 1.

While it might sound a bit complex, to explain it simply, let's say we categorize apples, bananas, and grapes as 100, 010, and 001. Do we really need to represent them with all three vector values? No, we can classify them with n-1 values.

Can't we separate apples, bananas, and grapes as 10, 01, and 00? Therefore, including all three leads to unnecessary redundancy.

### Resulting Problems

- Regression coefficients become unstable, and specific coefficient values can become abnormally large.
- Model interpretation becomes difficult -> It becomes hard to know which variable truly had an impact.
- As the number of variables increases, performance also becomes inefficient.

### Solutions

The simplest approach is to remove the last column after one-hot encoding.

Alternatively, one can stabilize coefficients by introducing regularization techniques like Ridge/Lasso, or reduce correlations and represent data efficiently using dimensionality reduction methods like PCA.

> **PCA (Principal Component Analysis)**
> Definition: A dimensionality reduction technique that reduces high-dimensional data to fewer dimensions while preserving as much of the data's variance (information) as possible.
> Core Idea:
> Transforms multiple variables in the original data into new orthogonal axes.
> Selects the axes (Principal Components) that best explain the data's variance.
