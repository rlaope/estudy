# sklearn

sklearn (scikit-learn) provides many useful functions to conveniently perform data preprocessing, model training, and evaluation in machine learning.

I will summarize some functions by their main features.

## 1. Data Splitting

`train_test_split`

```py
from sklearn.model_selection from train_test_split
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size = 0.2, random_state = 42)
```

`test_size 0.2` means an 8:2 ratio for the training set and test set.

`random_state` fixes the seed of the random value to facilitate testing with the same random values.

42 has no real meaning; it's just a meme among engineering students. Roughly,

https://brunch.co.kr/@smarter/97 You can find it in the book **The Hitchhiker's Guide to the Galaxy**, where the supercomputer DEEP THOUGHT says the answer to life, the universe, and everything is 42. It's just a meme.

Since it's a custom in that field, let's confirm and use 42. Haha...

<br>

## 2. Data Preprocessing

- `StandardScaler`: Standardizes to mean 0, variance 1
- `MinMaxScaler`: Transforms values to be between 0 and 1
- `RobustScaler`: Scaling robust to outliers
- `Normalizer`: Adjusts vector length to 1

```py
from sklearn.preprocessing import StandardScaler
scaler = StandardScaler()
X_scaled = scaler.fit_transform(X)
```

Scaling robust to outliers means that in standard deviation-based scaling, if extremely large or small values exist, the mean and standard deviation can be significantly distorted. Therefore, `RobustScaler` uses the median and interquartile range (IQR = Q3 - Q1) instead of the mean and standard deviation. This is why it's called robust.

$$
x' = \frac{x - \text{median}(X)}{\text{IQR}(X)}
$$

$$
\text{IQR}(X) = Q_3 - Q_1
$$


Normalizing to a vector length of 1 means that if a data vector is x1, x2, x3, its length (norm) using Euclidean distance (2-norm) is:

$$
\|\mathbf{x}\| = \sqrt{x_1^2 + x_2^2 + \cdots + x_n^2}
$$

$$
\mathbf{x'} = \frac{\mathbf{x}}{\|\mathbf{x}\|}
$$

Normalization divides each element by this length to make the vector length 1.

Example:

Original vector: $\mathbf{x} = [3, 4]$

Vector length: $\|\mathbf{x}\| = \sqrt{3^2 + 4^2} = 5$

After normalization: $\mathbf{x'} = \left[\frac{3}{5}, \frac{4}{5}\right] = [0.6, 0.8]$

<br>

## 3. Feature Selection / Dimensionality Reduction

- `PCA`: Principal Component Analysis
- `SelectKBest`: Selects the top k most important features
- `VarianceThreshold`: Removes features with low variance

```py
from sklearn.decomposition import PCA
pca = PCA(n_components=2)
X_pca = pca.fit_transform(X)
```

<br>

## 4. Model Related

- Various classifiers such as LogisticRegression, SVC, KNeighborsClassifier, RandomForestClassifier
- Regression models such as LinearRegression, Ridge, Lasso

```py
from sklearn.linear_model import LogisticRegression
model = LogisticRegression()
model.fit(X_train, y_train)
```

<br>

## 5. Performance Evaluation
- accuracy_score, precision_score, recall_score, f1_score
- confusion_matrix: Confusion matrix
- classification_report: Comprehensive report on precision, recall, F1-score

```py
from sklearn.metrics import accuracy_score, classification_report
print(accuracy_score(y_test, y_pred))
print(classification_report(y_test, y_pred))
```

<br>

## 6. Cross-Validation

- cross_val_score: Cross-validation score
- GridSearchCV, RandomizedSearchCV: Hyperparameter tuning

```py
from sklearn.model_selection import cross_val_score
scores = cross_val_score(model, X, y, cv=5)
```

<br>

## 7. Dataset Provision

- Built-in datasets such as load_iris, load_digits, load_wine
- Virtual data generation such as make_classification, make_regression

```py
from sklearn.datasets import load_iris
iris = load_iris()
X, y = iris.data, iris.target
```
