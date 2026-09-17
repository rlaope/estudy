# ORB (Oriented FAST and Rotated BRIEF)

ORB is an algorithm considered an industry standard for feature extraction in real-time tasks.

Before diving into ORB, let's understand which category of algorithms it belongs to.

Is it a feature detection algorithm, like Harris, FAST, and GFTT, or a feature descriptor algorithm, like BRIEF and LBP?

The answer is that ORB can do both. It extracts keypoints through feature detection and then computes descriptors.

ORB is an algorithm that produces both keypoints and ORB descriptors as its final output, hence the name Oriented FAST and Rotated BRIEF. It incorporates FAST as a detector and BRIEF as a descriptor.

> - **feature detector:** A feature point detector that finds the x, y coordinates of points that can serve as features within an image.
> - **feature descriptor:** Analyzes the characteristics of the surrounding pixel region centered on the x, y coordinates found by the detector and encodes them into a multi-dimensional vector.

This means FAST will extract keypoints, and BRIEF will compute descriptors.

So, how can ORB be redefined? It can be defined as a Feature Detection and Description or Feature Extraction algorithm.

However, in the paper's abstract, the authors define it as a binary descriptor. In practice, the term "descriptor" is often used to encompass feature detection, which extracts keypoints.

And there's an important point here: ORB's advantages are defined as rotation invariance and resistance to noise.

These are elements not seen in the original FAST and BRIEF. This means ORB is not simply a combination of FAST and BRIEF given a new name, but rather an improvement of the two algorithms, resulting in a new algorithm.

Let's first look at the original FAST and BRIEF, based on version 1.0.

## FAST (Features from Accelerated Segment Test)

FAST was introduced in the 2006 paper "Machine Learning for High-Speed Corner Detection."

Accelerated Segment Test implies that it examines (Test) the pixels of a circular trajectory (Segment) surrounding a central pixel, but accelerates the process by skipping unnecessary computations.

FAST was proposed to perform real-time tasks in environments with limited resources, including robotics.

### Problem to Solve and Solution Method

Existing SIFT, SURF, and Harris Corner algorithms performed well but involved too many mathematical operations. This became a bottleneck, causing frame delays in mobile devices or real-time video streaming environments.

It solved this by completely abandoning differential operations. Instead, it maximized speed by applying an intuitive geometric rule: for a specific pixel `p` to be a corner, **there must be a continuous region of surrounding pixels that are significantly brighter or significantly darker than `p`**.

Therefore, low computational cost and high speed are considered the algorithm's greatest advantages and contributions. FAST assumes that if there is a large difference from adjacent pixels, it's a corner/edge feature point, focusing solely on brightness changes.

1.  **16-Pixel Circle (16-pixel circular search)**: Drawing a circle with a radius of 3 around a target pixel `p` places exactly 16 surrounding pixels on the circumference. Let $I_p$ be the brightness of the central pixel and $t$ be the threshold.
2.  **Continuity Test (Segment Test)**: If `N` (usually 12) or more consecutive pixels out of the 16 are brighter than $I_p + t$ or darker than $I_p - t$, it is identified as a corner.
3.  **High-speed Test Logic (Short-circuit Evaluation)**: Examining all 16 pixels is inefficient. First, only pixels 1, 9, 5, and 13 (in a cross pattern) are checked. If it's a corner, at least 3 of these 4 must satisfy the condition. If not, it immediately moves to the next pixel (early return), significantly reducing memory access frequency.

<br>

## BRIEF (Binary Robust Independent Elementary Features)

BRIEF is an algorithm that analyzes the region around feature points found by FAST, etc., and converts them into a vector descriptor that computers can compare.

The core idea is to generate this vector not as real numbers (floats) but as a binary string composed of 0s and 1s.

-   **Binary**: Generates a bit array instead of a float array.
-   **Independent Elementary Features**: Means that it performs only independent and basic comparison operations by pairing the brightness of pixels around the feature point.

### Problem to Solve and Method

Descriptors like SIFT generate 128-dimensional float32 vectors.

To compare if two feature points are the same, the Euclidean distance ($\sqrt{\sum(x_i - y_i)^2}$) must be calculated for each dimension, which involves square root operations, consuming excessive CPU cycles and occupying significant memory bandwidth.

It randomly selects specific pixels A and B around the feature point and only asks: Is A brighter than B? If yes, it records 1; otherwise, 0. This process is repeated 256 times to create a 256-bit (i.e., 32-byte) string. Comparison operations are replaced by XOR-based Hamming distance, which is the fastest at the hardware level.

### Detailed Component Operation Principle

1.  **Smoothing (Gaussian Smoothing)**: Pixel-level comparisons are highly susceptible to noise, so Gaussian blur is first applied to the patch around the feature point to remove noise.
2.  **Random Pair Sampling**: Within the patch centered on the feature point, 256 pixel pairs (x, y) are selected according to a predefined Gaussian distribution probability. This selection pattern is pre-cached in memory.
3.  **Binary Vector Generation**: For each pair, the brightness function `I` is compared.

$$f(X, Y) = \begin{cases} 1 & \text{if } I(X) < I(Y) \\ 0 & \text{otherwise} \end{cases}$$

As a result, a 256-bit array of the form 1011010... is completed. The similarity between two descriptors is processed at the single CPU clock cycle level using a Popcount instruction, which counts the number of 1s after an XOR operation.

<br>

## ORB Oriented FAST and Rotated BRIEF

Returning to ORB, it is an algorithm published by OpenCV researchers that combines the FAST Detector and BRIEF Descriptor and solves their critical weaknesses (vulnerability to scale changes and rotation), making it an optimized model for practical use.

-   **Oriented FAST:** Adds the ability to measure the rotation angle of an object to the existing FAST algorithm, which lacked orientation information.
-   **Rotated BRIEF:** To solve BRIEF's weakness where pixel pair positions shift when an image rotates, the sampling pattern is rotated according to the detected orientation.

### Problems and Solutions

FAST failed to recognize corners when images were scaled, and BRIEF failed to match when images were rotated by just 10 degrees, as the binary vector would completely change.

Furthermore, to avoid patents, high-performing SIFT and SURF were patented algorithms that required royalties for commercial use (though they have since expired, this was a major issue at the time). OpenCV needed a free, fast, and high-performing alternative.

-   **Scale Invariance**: For scale invariance, an image pyramid is created by reducing the original image size in multiple steps, and FAST is performed at each level.
-   **Rotation Invariance**: The brightness distribution around the feature point is treated as physical mass, and the direction is calculated using a vector connecting the center point and the center of mass. After Intensity Centroid, the BRIEF binary comparison pattern is pre-rotated by that angle before extraction.

### Detailed Component Operation Principle

1.  **Multi-resolution Search**: Feature points are found using FAST at each level within the image pyramid, and only the N highest quality feature points are kept using the Harris corner response function (sorting & filtering).
2.  **Intensity Centroid Calculation**: The moments of the region around the feature point are calculated. Pixel brightness $I(x, y)$ is treated as mass, and the center of mass $(C_x, C_y)$ is found for the x and y axes. The angle $\theta$ of the vector connecting the feature point coordinates and this intensity centroid is calculated using the arctangent (atan2) function to determine the object's orientation.
3.  **Steered BRIEF (Steered Binary Extraction)**: A rotation matrix is generated using the angle found previously. The 256 pixel comparison pair coordinates of BRIEF are multiplied by this matrix to rotate them similarly. This ensures that even if the image is flipped upside down, pixel brightness is always compared based on the object's correct orientation, yielding the same binary vector.

### Example

The code below is a pipeline that creates an ORB object, finds feature points between two images (original and rotated/scaled), and performs high-speed matching using binary bitwise operations.

```py
import cv2
import numpy as np

def orb_feature_matching(img1_path, img2_path):
    # 1. Load Images (Grayscale operations are advantageous)
    img1 = cv2.imread(img1_path, cv2.IMREAD_GRAYSCALE) # Target image for search
    img2 = cv2.imread(img2_path, cv2.IMREAD_GRAYSCALE) # Query image (user-uploaded image)

    # 2. Create ORB object
    # nfeatures: Maximum number of features to extract
    # scaleFactor: Pyramid ratio (Images are scaled down by 1.2x for scale invariance)
    # nlevels: Number of pyramid levels
    orb = cv2.ORB_create(nfeatures=1000, scaleFactor=1.2, nlevels=8)

    # 3. Calculate Keypoints (detected x,y position and rotation angle) and Descriptors (extracted 256-bit binary array)
    kp1, des1 = orb.detectAndCompute(img1, None)
    kp2, des2 = orb.detectAndCompute(img2, None)

    # The shape of des1, des2 is an (N, 32) NumPy array, and its dtype is uint8. (8bit * 32 = 256bit)
    
    # 4. Feature Matching (Brute-Force Matcher)
    # Since ORB uses binary descriptors, NORM_HAMMING (Hamming distance) must be used.
    # Using L2 (Euclidean) would treat the bit string as a regular number, completely breaking the logic.
    # crossCheck=True: Returns only bidirectionally matched results to reduce false positives.
    bf = cv2.BFMatcher(cv2.NORM_HAMMING, crossCheck=True)

    # Perform matching: Find the shortest distance (most similar) pairs between des1 and des2 using XOR bitwise operations.
    matches = bf.match(des1, des2)

    # 5. Sort matching results in ascending order by Distance (shorter distance means higher similarity)
    matches = sorted(matches, key=lambda x: x.distance)

    print(f"Image 1 features: {len(kp1)}")
    print(f"Image 2 features: {len(kp2)}")
    print(f"Successfully matched pairs: {len(matches)}")

    # Visualize only the top 50 matching results with connecting lines
    result_img = cv2.drawMatches(img1, kp1, img2, kp2, matches[:50], None, 
                                 flags=cv2.DrawMatchesFlags_NOT_DRAW_SINGLE_POINTS)
                                 
    return result_img

# Usage example:
# result = orb_feature_matching("original.jpg", "rotated_query.jpg")
# cv2.imshow("ORB Matches", result)
# cv2.waitKey(0)
```

### Hamming Distance

Binary Descriptors have the advantage of being able to use Hamming Distance, which allows for fast descriptor comparison operations.

Here, Hamming Distance simply refers to the number of elements that need to be changed for two string vectors to become identical.

> In information theory, the Hamming distance between two strings of equal length is the number of positions at which the corresponding symbols are different

When comparing two data strings of equal length, typically binary strings composed of 0s and 1s, it refers to the **number of positions where corresponding values are different**.

Intuitively, how many bits do you need to flip to make one string exactly identical to another?

#### Example

Let's assume we have two 7-bit binary arrays:

-   1 0 1 1 1 0 1
-   1 0 0 1 0 0 1
-   - - X - X - -

Looking at the results above, the 3rd and 5th bits are different. In this case, the number of differing positions is 2, so the **Hamming distance** between the two data sets is 2. A shorter distance means the two data sets are very similar.

When computers calculate this Hamming distance, they use the lightweight and fast bitwise XOR operation. Therefore, by simply counting the number of 1s in the result using a Popcount instruction, the Hamming distance is obtained.

This means the CPU's computational cycles are significantly reduced.
