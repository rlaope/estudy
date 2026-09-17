# Overfitting, Underfitting

Overfitting and underfitting are problems that can occur during the machine learning training process, and they are somewhat different from issues caused by data volume, features, or samples.

Rather than being problems caused by data, they can be classified as problems arising from the training algorithm.

## Overfitting

This refers to a phenomenon where a model becomes too specialized to the training set, leading to poor generalization performance. One might mistakenly think that the results are good because it fits the training set well, but that is not the case at all.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FLLGQ3%2Fbtq3mcEvOEY%2FAAAAAAAAAAAAAAAAAAAAAPyZ-NEA6AOqPLOC9akeo5oBJrrSNL2GM4ED9qfDmRFd%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1759244399%26allow_ip%3D%26allow_referer%3D%26signature%3DuyzgaVb2s0u5sxyN8cuMorU6D5g%253D)

The dots in the image represent data, while the lines (straight or parabolic) show predicted data.

The black lines, while having some error, are represented as lines that can predict new data based on the given data. However, the blue line shows large variability for each data point, which reduces the model's stability.

What's an example? Let's assume there's a machine learning model trained on data about balls. It's learning about soccer balls, basketballs, and baseballs.

But what if the model overfits too much, learning not just that a round shape is a ball, but also details like stitching, weight, density, and leather? Then, if new data like a ping-pong ball comes in, the model might not classify it as a ball.

**Methods to resolve overfitting**
1. Use a sufficient amount of data (simple)
2. Reduce overfitting by applying regularization.

An example of regularization is to reduce models composed of high-order functions, like the blue and green lines in the image above, to first or second-order functions, expressing them as a black straight line. This can be adjusted through hyperparameters.

Let's explore hyperparameters next time.

<br>

## Underfitting

Underfitting, conversely to overfitting, is when a model is too simple and fails to learn the training set well.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FbL11gd%2Fbtq3mYZWvZ1%2FAAAAAAAAAAAAAAAAAAAAAKPC0KJ3bey_xldh8QQRLHu12Hqubs8qO1inMac7HicD%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1759244399%26allow_ip%3D%26allow_referer%3D%26signature%3DdgRQqTDBxLkl5OQ%252FWVoFED13BPw%253D)

When data is distributed as shown, we can predict that the distribution of points is similar to the green line, but what about the blue line? It doesn't represent the data well.

In such a case, the blue line represents an underfit model, which has low accuracy and is difficult to evaluate as a good model.

To solve this, the blue line should be changed to a 2D parabola.

If there's a model trained on ball characteristics, with too little data, it might classify anything round as a ball. For example, if you put in bald Mr. Kim Kye-ran's head, it might be classified as a ball. Even if Mr. Kim Kye-ran's head has very high accuracy, it might still be classified as a ball.

**Methods to resolve underfitting**
1. Apply a model that uses more model parameters.
2. Utilize better features.
3. Reduce underfitting by lowering the regularization strength.
