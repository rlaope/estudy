# Weight, Bias cc. Linear Regression

### Weight

In linear regression, weights are values multiplied by input values, adjusting the importance of the input.

When there is y = w * x, w is the weight.

For example, in a model where study time (x) influences y, w indicates that study time has a significant impact.

### Bias

As a constant term added to the result, even if the input is 0, it ensures that the output is above a certain value.

With y = w * x + b, even if you don't study at all (x = 0), if b = 10, it's possible to set it so that you score at least 10 points.

This is bias.

In other words, adjusting the input ratio is the weight, and shifting the graph up and down (acting as an intercept) is the bias.

<br>

## weight bias

**Weight w** determines how important data is. For example, the weight determines whether pixels in a cat's ear are important for the result.

In image classification, if it's a model that classifies based on cat ears, the weight would be set high. This is continuously updated during the learning process,

allowing it to learn data patterns. Intuitively, it can also be called the importance of input features.

**Bias b** is an adjustment value that shifts the output value up or down. In neural networks, before the activation function

(ReLU, sigmoid) is applied, it allows the linear combination result to be shifted more flexibly.

It plays a role in ensuring that some neuron is activated even if some input is zero. Intuitively, it acts like a threshold.

The output of a neuron is usually expressed as: `y=f(w1​x1​+w2​x2​+⋯+wn​xn​+b)`

f is the activation function, wi is the weight for each input xi, and b is the bias.
