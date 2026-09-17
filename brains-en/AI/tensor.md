# Tensor

Let's explore Tensors, the smallest unit of the vast function that is deep learning, and the vessel for all data.

While a tensor is merely an n-dimensional array, in deep learning, its dimension and shape can be considered the entirety of the logic.

### Tensor Dimensions and Shapes

Let's look at the hierarchical structure of tensors.

As data deepens, its name changes, but ultimately, it's just a collection of numbers.

- **Scalar**: Represents a single int or float variable, a 0-dimensional variable.
- **Vector**: Represents a 1-dimensional Array, List, etc. [1, 2, 3, 4]
- **Matrix**: Represents a 2-dimensional Table, Excel, etc. [[1, 2], [3, 4]]
- **Tensor**: An array of 3 or more dimensions, or nested arrays of 3 or more dimensions, such as image data or video.

### Shape

Why are shape and form important? Just as typeErrors occur during development,

in deep learning, 90% of runtime errors are due to shape mismatches.

A shape is a tuple indicating how many elements are in each dimension. For example, (3, 244, 244) -> means an image with 3 channels (RGB) and a width and height of 244 pixels.

Deep learning models compute tens of thousands of numbers simultaneously. For matrix multiplication to be possible, the shape of the input data and the shape of the model's weights must fit together perfectly, like gears.

### Practical Data Shape Examples

This is the fixed format in which deep learning engines like PyTorch and TensorFlow consume data.

- **Image Example: Batch Size, Channels, Height, Width**
  - 64, 3, 28, 28 -> Means processing 64 color images of 28x28 size as a batch.
- **Text Example: BatchSize, Sequence Length, Embedding Size**
  - 32, 10, 512: Means processing 32 sentences, each consisting of 10 words, where each word is represented by a 512-dimensional number.

```py
import torch

# 1. 텐서 생성 (2x3 행렬)
x = torch.tensor([[1, 2, 3], [4, 5, 6]])
print(f"Shape: {x.shape}") # torch.Size([2, 3])

# 2. View/Reshape (가장 중요: 데이터는 그대로 두고 모양만 변경)
# 2x3을 6x1로 바꿈
y = x.view(6, 1) 
print(f"Reshaped: {y.shape}") # torch.Size([6, 1])

# 3. 차원 늘리기 (Unsqueeze) - 모델 입력을 위해 차원 맞출 때 자주 사용
z = x.unsqueeze(0) 
print(f"Added Dimension: {z.shape}") # torch.Size([1, 2, 3])
```

> If you're looking at deep learning code and don't understand it, always print(data.shape). Tracking how data transforms is the beginning and end of debugging.
