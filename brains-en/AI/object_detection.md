# Object Detection

Object detection is a Localization technique that identifies (classifies) specific objects (people, cars, chairs) present in an image and simultaneously finds their coordinates within the image using a **bounding box**.

### Problem to be solved

Previous topics like CNNs (ResNet) are excellent at classifying an entire image, for example, determining 'This is a dog picture!'

However, positional information, such as 'The dog is in the bottom-left of the picture,' is lost.

For instance, in a real-world furniture search service, a user might upload a picture of an entire room.

If the AI searches the entire photo indiscriminately, it might yield irrelevant search results due to background wallpaper or floor patterns.

Therefore, preprocessing that accurately finds and crops only the furniture the user wants using a bounding box is essential.

### Evolution of Problem-Solving Methods: 2-Stage vs 1-Stage

Initially, deep learning models were too heavy, so problems were solved in two stages.

However, as real-time processing requirements grew, the approach evolved to solve problems in a single pass.

A representative **2-Stage model is Faster R-CNN**,

1. Its operation involves first finding thousands of potential object regions (region proposals) in an image in the first stage,
2. And then, in the second stage, cropping only those regions and feeding them into a CNN to identify what object they are.

While its accuracy is very high, its slow speed makes it unsuitable for real-time video processing.

A representative **1-Stage model is YOLO - You Only Look Once**,

1. Its operation involves dividing the image into a grid. The CNN model quickly scans the image just once (Only Look Once) and simultaneously calculates for each grid cell: Is there an object here? What is its size? What kind of object is it?
2. Its characteristic is that while accuracy is slightly lower than 2-Stage models, its speed is tens of times faster, making it a standard for applications like autonomous driving, CCTV, and mobile apps.

<br>

## YOLO Operation Principle

When a YOLO model analyzes an image, it often outputs dozens of messy bounding boxes for a single object because the model lacks certainty.

There are two essential post-processing algorithms to clean this up.

#### 1. **Confidence Threshold**

Each box is assigned a Confidence Score between 0 and 1, which is the product of `the probability that an object is inside this box x the probability that the object is a dog`.

By setting a rule to discard boxes with a score of 0.5 (50%) or less without even looking at them, false boxes are eliminated in the first pass.

#### 2. NMS (Non-Maximum Suppression)

Even after the above process, multiple overlapping boxes, such as 0.9 and 0.85, might remain over a single dog.

NMS is an algorithm that keeps only the maximum scoring box among overlapping boxes and suppresses (deletes) the rest.

At this point, how much two boxes overlap is measured by a metric called IoU (Intersection over Union), which is the ratio of their intersection to their union.

### Exmaple

This is code that infers an image using a pre-trained Faster R-CNN model with the `pytorch` and `torchvision` libraries and organizes the results using a confidence threshold.

```py
import torch
import torchvision
from torchvision.models.detection import fasterrcnn_resnet50_fpn, FasterRCNN_ResNet50_FPN_Weights
from PIL import Image
import torchvision.transforms.functional as F

def detect_objects(image_path, confidence_threshold=0.8):
    # 1. Load a pre-trained Faster R-CNN model (one of the most powerful and versatile models)
    # Specify weights to load weights trained on ImageNet data.
    weights = FasterRCNN_ResNet50_FPN_Weights.DEFAULT
    model = fasterrcnn_resnet50_fpn(weights=weights)
    
    # Switch the model to evaluation (inference) mode
    model.eval()

    # 2. Load and preprocess the image (convert to a tensor between 0 and 1)
    image = Image.open(image_path).convert("RGB")
    image_tensor = F.to_tensor(image).unsqueeze(0) # Add batch dimension

    # 3. Perform model inference
    with torch.no_grad():
        # The model's output is a list of dictionaries in the form of 'boxes' (coordinates), 'labels' (class number), and 'scores' (confidence).
        predictions = model(image_tensor)

    # 4. Post-processing: Apply Confidence Threshold
    pred = predictions[0] # Since it's a single image, use only the first batch result
    
    # Extract only the indices of objects whose confidence (score) is higher than the threshold
    keep_indices = pred['scores'] > confidence_threshold
    
    # Filter only the final data that passed the threshold
    final_boxes = pred['boxes'][keep_indices]
    final_scores = pred['scores'][keep_indices]
    final_labels = pred['labels'][keep_indices]
    
    # (Note: The Faster R-CNN model in torchvision already includes NMS logic,
    # so it is applied automatically, and the user does not need to call a separate NMS function.)

    print(f"Number of valid objects found: {len(final_boxes)}")
    
    # Print results
    for i in range(len(final_boxes)):
        # Can be mapped to class labels from the COCO dataset (1=person, 2=bicycle, etc.)
        label_idx = final_labels[i].item()
        score = final_scores[i].item()
        box = final_boxes[i].tolist() # [x_min, y_min, x_max, y_max] format
        
        print(f"[{i+1}] Class ID: {label_idx} | Confidence: {score:.2f} | Coordinates: {box}")

    return final_boxes, final_labels, final_scores

# Usage example:
# boxes, labels, scores = detect_objects("living_room.jpg", confidence_threshold=0.7)
```
