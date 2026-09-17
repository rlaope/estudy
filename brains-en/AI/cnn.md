# CNN (Convolution Neural Network) Based Feature Extraction

While SIFT, SURF, ORB, etc., are methods where mathematical formulas are handcrafted by humans, CNN is a core technology in modern image search where the model itself learns optimal feature extraction filters from data.

### CNN-Based Feature Extraction

CNN-based feature extraction is a technique that leverages the deep learning architecture of convolutional neural networks to hierarchically identify features in an original image, ranging from low-level geometric shapes (lines, corners) to high-level semantic information such as object parts, and then transforms these into numerical feature vectors of hundreds to thousands of dimensions.

#### Origin of the Name

- **Convolutional**: The term 'convolution' is used because image processing primarily employs a mathematical operation where a filter (kernel) slides over pixels, multiplying the pixel values at each position by the filter's weights and summing them.
- **Neural Network**: This refers to the stacking of artificial neurons in deep layers, mimicking the process by which the brain's visual cortex recognizes objects through step-by-step recognition, from simple lines to complex shapes.

### Problem Definition of Existing Methods

**Lack of semantic information.** SIFT or SURF merely find "sharp points where brightness changes rapidly" but do not understand whether it's a dog's eye or a car's wheel. Consequently, they often misidentify completely different objects that have similar textures or patterns.

**Limitations in complex environments also existed.** When lighting changes drastically, or when non-linear deformations occur, such as an object being partially obscured (occlusion) or distorted, traditional handcrafted algorithms often failed to find any feature points.

### Problem-Solving Approach

**"Instead of humans devising formulas, let's give millions of images to computers and let them learn the formulas, i.e., the filter weights, themselves."**

It started from this premise. Models (VGG, ResNet, etc.) are trained to classify dogs, cats, cars, and so on, using vast datasets like ImageNet.

To get the correct answers, the model undergoes a backpropagation process, autonomously creating filters that are best suited for distinguishing objects.

For image search tasks, the very last layer responsible for classification is removed from such a pre-trained model, and the multi-dimensional array (feature map) output from the layer immediately preceding it is directly extracted and used as the unique feature vector (descriptor) for that image.

This is referred to as Feature Extraction from a transfer learning perspective.

<br>

## Detailed Component Operating Principles and Structuring

A CNN architecture typically has a repetitive structure of the following layers.

### Convolution Layer

Extracts spatial characteristics of an image.

A learnable filter (kernel) of size 3x3 or 5x5 traverses the original image or the output of the previous layer, performing convolution operations.

Regarding hierarchical features, shallow layers (early stages) of the network extract low-level features like lines, curves, and color contrasts, while deeper layers (later stages) synthesize high-level features such as eyes, noses, and tires.

### Activation Function (Mainly ReLU)

Sets all negative values from the convolution operation to 0: $f(x) = \max(0, x)$

The reason is that if only linear convolution operations are repeated, no matter how deep the layers are stacked, it ultimately reduces to a single large linear operation. Non-linearity must be introduced via ReLU for the model to learn complex patterns.

### Pooling Layer (Mainly Max Pooling)

Reduces the resolution of the image feature map by half by keeping only the largest value within a specific 2x2 region and discarding the rest.

Significantly reduces computational load and provides invariance, allowing the model to maintain the same features even if an object shifts slightly within the image.

### Global Average Pooling (GAP) or Fully Connected Layer (FC Layer)

The spatial information of the 3D Feature Map (e.g., 7 x 7 x 2048) extracted through deep layers is averaged (GAP) and compressed into a 1D vector.

The final output is a numerical vector (float32 array) of 512, 1024, or 2048 dimensions. This vector is then hashed by the LSH algorithm and becomes the final result indexed in Elasticsearch.

## Example

PyTorch-based ResNet50 Feature Vector Extraction

This is the logic for extracting a 2048-dimensional vector from an original image using the PyTorch framework, which is most commonly used in practice for building image search engines, and a pre-trained ResNet50 model.

```py
import torch
import torchvision.models as models
import torchvision.transforms as transforms
from PIL import Image

def extract_cnn_features(image_path):
    # 1. Load Pre-trained Model (ResNet50 already trained with ImageNet data)
    # Load the latest weights with weights=models.ResNet50_Weights.DEFAULT
    model = models.resnet50(weights=models.ResNet50_Weights.DEFAULT)
    
    # 2. Modify Architecture for Image Search
    # ResNet's last layer (fc) is a classifier that outputs 1000 class probabilities.
    # We need a 'feature vector' not classification, so we replace the last fc layer
    # with an Identity layer that does nothing (effectively cutting it off).
    import torch.nn as nn
    model.fc = nn.Identity()
    
    # Switch model to evaluation (inference) mode (fixes behavior of Dropout, BatchNorm, etc.)
    model.eval()

    # 3. Image Preprocessing Pipeline (Must match the environment used during ResNet training)
    preprocess = transforms.Compose([
        transforms.Resize(256),             # Resize to 256 based on the shorter side
        transforms.CenterCrop(224),         # Crop central 224x224 region
        transforms.ToTensor(),              # Convert pixel values (0~255) to tensor (0.0~1.0)
        transforms.Normalize(               # Normalize with ImageNet data's mean and standard deviation
            mean=[0.485, 0.456, 0.406], 
            std=[0.229, 0.224, 0.225]
        ),
    ])

    # 4. Load Image and Apply Preprocessing
    img = Image.open(image_path).convert('RGB')
    img_tensor = preprocess(img)
    
    # PyTorch models inherently assume batch-wise processing, so
    # we add one dimension: (C, H, W) -> (B, C, H, W), i.e., (1, 3, 224, 224)
    input_batch = img_tensor.unsqueeze(0)

    # 5. Model Inference (Feature Vector Extraction)
    # torch.no_grad(): Saves memory and computation speed by not updating weights (training)
    with torch.no_grad():
        # Output: Tensor of shape (1, 2048)
        features = model(input_batch)
    
    # Convert to Numpy array and flatten to 1D
    feature_vector = features.numpy().flatten()
    
    print(f"Extracted Feature Vector Shape: {feature_vector.shape}")
    print(f"First 10 values of Feature Vector: {feature_vector[:10]}")
    
    return feature_vector

# Usage Example:
# vector = extract_cnn_features("query_image.jpg")
# This 2048-dimensional vector data will be the input for the next topic, 'LSH Hashing Algorithm'.
```

While methods like SIFT extract multiple keypoints per image, a fundamental difference is that typical CNN approaches compress the entire image into a single high-dimensional vector.
