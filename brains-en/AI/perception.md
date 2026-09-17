# Neural Network Perception

### Weights

LLMs are said to be massive matrix calculators composed of trillions of weights.

What exactly are these "weights"?

**Deep learning models repeatedly multiply an input `x` by an output `W` and add another number `b`.**
- **Weight W:** The importance value that determines which input data is significant.
- **Bias b:** The sensitivity that controls how easily the output value is activated.

$$y = Wx + b$$

### Activation Function ReLU

If you simply continue with `Wx + b`, no matter how many layers you stack, it will ultimately just be a single linear operation. To learn complex patterns, non-linearity is required.

This is where ReLU (Rectified Linear Unit) comes in.

$$f(x) = \max(0, x)$$

It's a very simple switch: if it's less than 0, it outputs 0 (off); if it's greater than 0, it outputs the value as is (on).

This is important because it's incredibly fast to compute and, to some extent, prevents the chronic deep learning problem known as vanishing gradients.

So, what exactly are vanishing gradients?

### Gradient Vanishing

It's the evaporation of information. When training a model, the error of the output value is propagated backward to adjust the weights (backpropagation).

- **Problem:** If the layers are too deep, the gradients that need to be adjusted become closer to zero as they propagate backward, leading to a phenomenon where the earlier layers don't learn at all.

You can think of it as a similar concept to being in a line and telling the person behind you, "Don't push!" but the person at the very back can't hear you and might still push.

### Normalization

If numbers become too large or too small during computation, the computer cannot perform the calculations.

Normalization is the process of forcibly scaling and refining numbers so that they neatly fall within a specific range, typically 0 to 1, each time they pass through a layer.

**RMSNorm**: This is a method used in recent LLMs like Llama. It's an efficient normalization technique that speeds things up by using the root mean square instead of calculating the mean.

### Code Example

```py
import torch

# 1. Input data (x) and target value (y_target)
x = torch.tensor([2.0, 3.0])
y_target = torch.tensor([10.0]) # Our desired target value

# 2. Initialize weights (W) and bias (b) (random values initially)
# requires_grad=True indicates that these numbers are 'settings' that will change through learning
W = torch.randn(1, 2, requires_grad=True) 
b = torch.randn(1, requires_grad=True)

# 3. Model operation (y = Wx + b)
y_pred = torch.matmul(W, x) + b

# 4. Apply activation function (ReLU)
y_activated = torch.relu(y_pred)

# 5. Calculate error (Loss)
loss = (y_activated - y_target)**2

# 6. Backpropagation -> Calculates the 'gradients' here
loss.backward()

# 7. Update weights (learning)
with torch.no_grad():
    W -= 0.01 * W.grad # Adjust W slightly in the direction that reduces the error
    b -= 0.01 * b.grad
```
