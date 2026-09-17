# Image Feature Extraction Feature Extraction

Image feature extraction is the process of finding meaningful information (Features) from original image data (2D array pixels) where geometric shapes or pixel value changes are prominent, and transforming it into a quantitative, multi-dimensional vector array (Descriptor) that computers can compute and compare. It is a core pre-processing step for various computer vision tasks such as image retrieval, object recognition, and panorama stitching.

- **Feature**: Refers to a unique point within an image that is distinct from the background or flat areas. It primarily denotes corners, edges, and blobs. (A Blob refers to a mass of bright or dark regions)
- **Extraction**: Instead of using all millions of pixel data, it extracts only the data that captures the essential characteristics of the image.

#### Problems Feature Extraction Aims to Solve

**Curse of Dimensionality and Computational Limits**: A 1920 x 1080 resolution image has approximately 2 million pixels. Comparing two images pixel by pixel is extremely computationally expensive and impossible for real-time processing.

**Vulnerability of Pixel Data**: Even for photos of the same object, the grid values of pixels change completely depending on lighting, camera viewpoint, image size, and rotation. Using raw pixel values for search significantly degrades accuracy.

### Problem-Solving Approach

Instead of absolute pixel color values, the problem is solved by focusing on the **relative rate of change (Gradient) or local structure** around pixels.

Flat regions are discarded, and only information-rich points (e.g., corners where two edges intersect) are identified. Then, the directionality and brightness change patterns of pixels around these points are modeled using mathematical histograms. This ensures **Invariance**, allowing consistent vector values even if the image rotates or changes size.

### How It Works

The feature extraction pipeline is broadly divided into two low-level components.

### Feature Detector

It finds the x, y coordinates of points within an image that are likely to be features.

It slides a small window, or pixel block, across the entire image, searching for the point where the pixel value changes most significantly when the window is moved up, down, left, and right.

Let the pixel intensity function be $I(x, y)$. The sum of squared differences $E(u, v)$ when the window is moved by $(u, v)$ is calculated as:

$$E(u, v) = \sum_{x,y} w(x, y) [I(x+u, y+v) - I(x, y)]^2$$

Through Taylor series expansion and Eigenvalue analysis, points where the first derivative changes rapidly in both the x and y directions are defined as corners (features).

### Feature Descriptor

It analyzes the characteristics of the surrounding pixel region, centered on the x,y coordinates found by the detector, and encodes them into a multi-dimensional vector.

It divides the region around the feature point (Patch), calculates the magnitude and direction of the Gradient for pixels within each sub-region, and generates an orientation histogram.

The bin values of the generated histogram are arranged in a sequence to create a numerical vector.

If feature points from two images are extracted from the same object, the Euclidean distance or Hamming distance between these vectors will be very short.

<br>

### Example Feature Detection & Description using OpenCV

The following demonstrates the process of detecting features in an image and transforming them into vectors (description).

It details the operation of the classic Harris Corner methodology and the widely used ORB (Oriented FAST and Rotated BRIEF) algorithm, which complements SIFT, with comments.

```py
import cv2
import numpy as np

def extract_features_example(image_path):
    # 1. Image Loading and Preprocessing
    # Feature extraction is primarily based on the rate of pixel brightness change (Gradient),
    # so it is converted to Grayscale to reduce computation and minimize noise.
    img = cv2.imread(image_path)
    if img is None:
        raise ValueError("Could not find the image.")
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

    # ==========================================
    # [Component 1] Low-level Corner Detection (Harris Corner)
    # Finds corners by mathematically calculating the eigenvalues of pixel change rates.
    # ==========================================
    # float32 type casting: Precision in decimal points is required for derivative calculations.
    gray_float = np.float32(gray)
    
    # cv2.cornerHarris parameters:
    # 1. gray_float: Input image
    # 2. blockSize=2: Size of the neighboring pixel window to consider for corner detection (2x2)
    # 3. ksize=3: Kernel size of the Sobel derivative operator (for X, Y direction Gradient calculation)
    # 4. k=0.04: Empirical constant for the Harris Corner equation (usually 0.04 ~ 0.06)
    dst = cv2.cornerHarris(gray_float, blockSize=2, ksize=3, k=0.04)
    
    # Morphological dilation operation to remove noise and make shapes clearer
    dst = cv2.dilate(dst, None)
    
    # Thresholding: 
    # Only points where the result of the corner response function (R) is greater than 1% of the local maximum are considered valid feature points.
    harris_corners = np.argwhere(dst > 0.01 * dst.max())
    print(f"Number of Harris corners detected: {len(harris_corners)}")


    # ==========================================
    # [Component 2] Feature Detection + Description (ORB)
    # ORB finds corners (Detector) using the FAST algorithm,
    # and generates binary vectors (Descriptor) using the BRIEF algorithm, forming an integrated pipeline.
    # ==========================================
    # Create ORB object (limiting the maximum number of detected features to 500)
    orb = cv2.ORB_create(nfeatures=500)
    
    # The detectAndCompute function executes both Detector and Descriptor simultaneously.
    # keypoints: A list of objects containing information about the feature point's location, size, and angle.
    # descriptors: A 2D Numpy array (N x 32) encoding information around the feature points.
    keypoints, descriptors = orb.detectAndCompute(gray, None)
    
    print(f"Number of ORB extracted keypoints: {len(keypoints)}")
    if descriptors is not None:
        print(f"Descriptor matrix shape: {descriptors.shape}")
        # Example: If the shape is (500, 32), it means 500 feature points are each described by a 32-byte (256-bit) hash vector.
        # These vectors become input values for LSH hashing or Elasticsearch vector search.
        
        # Check the descriptor structure of the first feature point (Low-level array)
        print("32-byte descriptor of the first feature point (Binary Descriptor):")
        print(descriptors[0])
        
    return keypoints, descriptors

# Usage example (replace with local image path when actually running)
# kp, desc = extract_features_example("sample_image.jpg")
```
