# Support Vector Machine

A Support Vector Machine is a classifier that finds a **decision boundary** as far as possible from two classes.

Its goal is to classify classes while satisfying specific conditions.

It's a powerful supervised learning model that determines which side a data point belongs to via a decision boundary, and can be used for linear or non-linear classification, regression, and outlier detection.

In particular, it's well-suited for complex classification and is appropriate for small to medium-sized datasets.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FdAu6hf%2FbtrcA1Z8wIF%2FAAAAAAAAAAAAAAAAAAAAAFT2-x_gdlbA1kNYkGwMd1SzztTstPGDt1iXwxxITqjf%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3DJIQ1LrJsYcgMKyZony1IusWvH2M%253D)

It fundamentally uses the idea of a margin, which refers to the boundary of a 'road' separating two data classes.

In the image above, the solid line in the middle is the decision boundary separating the two data classes, and the dashed lines are tangent to the data points closest to this boundary.

Here, the distance from the decision boundary to the dashed line is the margin. It also refers to the width of the 'road'.

For an SVM classifier, finding the widest 'road' between classes is equivalent to finding the optimal decision boundary. When the optimal decision boundary is found, the road width can be maximized, leading to the maximum margin, which is called margin classification.

<br>

### Support Vectors

What we can additionally understand through the margin is that the **data points closest to the decision boundary determine the margin**.

These are called **support vectors**. If additional training samples are added outside the 'road', they do not affect the decision boundary at all. However, if additional data enters inside the 'road', it can affect the decision boundary and change the margin.

### Hard Margin Classification, Soft Margin Classification

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FdUJm9s%2Fbtrctx0u5QX%2FAAAAAAAAAAAAAAAAAAAAAB80mCRE0nDSxaCNZ94aMD_jGNEpHDO6dtcXFXwQj_rt%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3Dtsfvoxeng5xC%252BzAvzmTpDN2vkXg%253D)

There are two types of margin classification: hard margin classification and soft margin classification.

Hard margin classification is when **all samples are classified outside the 'road'**, as shown in the left image.

Hard margin classification is only possible when the training set is linearly separable and is highly sensitive to outliers.

For example, if red data points are mixed into the blue class, a hard margin might not be found, or generalization might fail due to data outliers.

To avoid such problems, it's necessary to strike an appropriate balance: keeping the margin as wide as possible while allowing for some margin violations.

This is called soft margin classification, and it's represented as shown on the right.

Soft margin classification, as shown in the figure, aims to keep the 'road' width as wide as possible while tolerating some outliers.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FblTXtv%2FbtrczdAmifH%2FAAAAAAAAAAAAAAAAAAAAACsGMqXSlePVwPcfoBmDmkckfGyQ_OkiO3rzWPbrHzQt%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3DPLTYTcCtEnj814Mo10G9fQZGCMM%253D)

In scikit-learn, you can adjust the hyperparameter C, which controls the tolerance for margin violations. A smaller C allows more margin violations, while a larger C allows fewer.

```py
import numpy as np
from sklearn import datasets
from sklearn.pipeline import Pipeline
from sklearn.preprocessing import StandardScaler
from sklearn.svm import LinearSVC

iris = datasets.load_iris()
X = iris["data"][:, (2, 3)] # 길이, 너비
y = (iris["target"] == 2).astype(np.float64) # 품종

scaler = StandardScaler()
svm_clf1 = LinearSVC(C=1, loss="hinge", random_state=42)
svm_clf2 = LinearSVC(C=2, loss="hinge", random_state=42) 

scaled_svm_clf1 = Pipeline([
        ("scaler", scaler),
        ("linear_svc", svm_clf1),
    ])
scaled_svm_clf2 = Pipeline([
        ("scaler", scaler),
        ("linear_svc", svm_clf2),
    ])

scaled_svm_clf1.fit(X, y)
scaled_svm_clf2.fit(X, y)
```

Here, `iris` is a function provided by scikit-learn to load the Iris dataset.

This is an example that distinguishes petal length and width. It classifies by selecting features 2 and 3 (petal length and petal width out of 4 total features) from the entire input data.

```
[[1.4, 0.2],
 [1.4, 0.2],
 [4.7, 1.4],
 [5.1, 2.4],
 ...]
```

And the `loss="hinge"` part is the loss function used by SVM during training. It penalizes incorrect classifications and has zero loss when there's a margin of error.

Expressed as a formula: `L=max(0,1−yi​⋅(w⋅xi​+b))`
