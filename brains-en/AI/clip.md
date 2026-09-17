# Metric Learning & CLIP

We will explore the process of overhauling an AI model's brain to overcome the limitations of conventional general-purpose image classification models (such as ResNet) and accurately distinguish even subtle differences among products in our service. Furthermore, this becomes the foundation for a search engine that simultaneously understands images and text.

- **Metric Learning**: A technique that forces the model to learn by placing vectors of similar products close together (distance reduction) and vectors of different products far apart (distance expansion) in the high-dimensional vector space extracted by the deep learning model.
- **Fine-tuning**: The process of taking a smart pre-trained model that Google or Meta has trained on large-scale general data and performing additional training with data specific to our company.
- **Triplet Loss**: The most widely used loss function in metric learning, which evaluates the model by grouping three items together: an Anchor (reference), a Positive (correct answer), and a Negative (incorrect answer).
- **CLIP (Contrastive Language-Image Pretraining)**: A model announced by OpenAI that maps text and images not as separate systems but into a single shared vector space, enabling cross-modal search in a multimodal architecture.

### Origin of Terms

- **Metric Learning**: Mathematically, the criterion for measuring the distance between two points in space is called a metric (measure, distance function). The intuitive meaning of learning that optimizes this distance.
- **Triplet Loss**: Named because it uses a triplet of three data points—reference, correct answer, and incorrect answer—as one bundle to calculate loss, going beyond twins.
- **CLIP**: An acronym meaning that pre-training was performed by contrasting whether text and images match each other.

### The Problem

First, there is **absence of domain-specific knowledge**. A general-purpose ResNet model trained on ImageNet distinguishes brilliantly between cats and chairs, but when given a red V-neck knit and a red round cardigan registered in home shopping, from the model's perspective, they're both just red clothes and it outputs identical vectors.

Second, there is **impossibility of cross-modality search**. When a user types "Nike Air Force white" as text in the search box, if that text is not tagged in the database, there is no way to find the product image. This is because the image system and text system are completely disconnected.

### Problem-Solving Approach

**Performing Metric Learning with domain data**, we fine-tune the model's weights using Triplet Loss to pull together the same products in home shopping (photographed from different angles) and push apart different products. Through this, the model evolves to focus on subtle patterns or materials in clothing.

**Text-Image Joint Projection (CLIP)** simultaneously trains an image encoder (ResNet/ViT) and a text encoder (Transformer). It forces the vector of a puppy photo and the text vector of "cute puppy" to have a cosine similarity of 1.0 (perfect match), realizing the magic of searching for images with text.

Let me illustrate with an intuitive analogy:

- **General-purpose model (existing ResNet-50)**: A smart new part-time worker who just came from abroad. They distinguish between clothes and furniture but don't know the subtle difference between loose-fit shirts and oversized shirts, just stuffing them into a drawer as "big shirts."
- **Metric Learning (domain fine-tuning)**: The owner (engineer) sits this part-timer down and trains them Sparta-style (learning). They show two photos (Anchor, Positive) and teach them "these photos are the same loose-fit shirt, just different angles, so put them in the drawer!" and instruct them to move this photo (Negative) to a different drawer because it looks similar but is oversized. The part-timer becomes an expert at distinguishing the details unique to our clothing store.
- **CLIP (multimodal)**: The existing part-timer couldn't hear, so you had to show them an image to find similar clothes. The owner taught the part-timer how to match Korean text with clothing images, and now when you just say "red coat!" the part-timer perfectly imagines it in their head and immediately retrieves the red coat from the warehouse.

<br>

## Detailed Operating Principles and Structure

The mathematical mechanism of Triplet Loss, the core of Metric Learning, is structured as follows:

#### 1. Data Sampling: Extract 3 images from a batch

- $A$ (Anchor): The reference image to use as a query
- $P$ (Positive): An image of the same class (same product) as $A$
- $N$ (Negative): An image of a different class (different product) from $A$

#### 2. Vector Extraction

Pass the three images through a CNN model to obtain embedding vectors.

#### 3. Distance Calculation

Calculate the distance $d(A, P)$ from the reference point ($A$) to the correct answer ($P$) and the distance $d(A, N)$ to the incorrect answer ($N$).

#### 4. Loss Evaluation and Backpropagation

The goal is to make $d(A, P)$ as small as possible, close to 0, and make $d(A, N)$ as large as possible.

- Formula: $Loss = \max(0, d(A, P) - d(A, N) + margin)$
- Here, $margin$ is a safety distance that enforces "the distance between correct and incorrect answers must differ by at least this much (e.g., 1.0)!"

### Example

This is code that calculates cross-modal similarity between text and images using the CLIP model from Hugging Face's `transformers` library to integrate home shopping text search and image search.

```py
import torch
from PIL import Image
from transformers import CLIPProcessor, CLIPModel
import torch.nn.functional as F

def clip_multimodal_search(image_paths, search_query):
    """
    Using the CLIP model, calculate how relevant (similarity) 
    the given images are to the text query.
    """
    
    # 1. Load pre-trained general CLIP model and processor (preprocessing device)
    # "openai/clip-vit-base-patch32" is the most popular base model.
    model_id = "openai/clip-vit-base-patch32"
    model = CLIPModel.from_pretrained(model_id)
    processor = CLIPProcessor.from_pretrained(model_id)
    
    # Evaluation mode & disable gradients (essential optimization learned in Step 1)
    model.eval()
    
    # 2. Load image files as PIL objects
    images = [Image.open(path).convert("RGB") for path in image_paths]
    
    # 3. Data preprocessing (simultaneously perform text tokenization and image resizing/normalization)
    # return_tensors="pt" means return in PyTorch tensor format.
    inputs = processor(
        text=[search_query], 
        images=images, 
        return_tensors="pt", 
        padding=True
    )
    
    with torch.no_grad():
        # 4. Model inference (extract text embedding and image embedding)
        outputs = model(**inputs)
        
        # 5. Calculate similarity scores
        # logits_per_image: similarity matrix of shape (number of images x number of texts)
        # Returns scaled values based on cosine similarity.
        logits_per_image = outputs.logits_per_image 
        
        # Apply Softmax for easier viewing, converting to probabilities (0~1)
        probs = logits_per_image.softmax(dim=0).squeeze()

    # Output results
    print(f"Text query: '{search_query}'")
    for i, path in enumerate(image_paths):
        # Handle indexing according to array shape
        prob_score = probs[i].item() if probs.dim() > 0 else probs.item()
        print(f" - [Image {i+1}] {path} match probability: {prob_score * 100:.2f}%")
        
    return probs

# ==========================================
# Execution Simulation
# ==========================================
# Three hypothetical images: red_dress.jpg, blue_jeans.jpg, black_shoes.jpg
# In a real environment, this function directly matches user 'search terms (Text)' with DB 'product images (Image)'.
# 
# query = "화사한 봄에 어울리는 빨간색 원피스"
# results = clip_multimodal_search(
#     ["red_dress.jpg", "blue_jeans.jpg", "black_shoes.jpg"], 
#     query
# )
# 
# Expected output:
# 텍스트 쿼리: '화사한 봄에 어울리는 빨간색 원피스'
#  - [이미지 1] red_dress.jpg 일치 확률: 98.50%
#  - [이미지 2] blue_jeans.jpg 일치 확률: 1.20%
#  - [이미지 3] black_shoes.jpg 일치 확률: 0.30%
```

<br>

## CLIP

Let's explore CLIP in more detail. As mentioned earlier, CLIP is **a multimodal deep learning architecture that maps text (language) and images (vision) not as separate data but together into a single shared multidimensional vector space (Shared Latent Space).**

This enables searching for images with text (Text-to-Image Search) or classifying what an image is with text without additional training (Zero-shot Classification), serving as the foundation for modern AI search engines.

- **Multi-modal**: AI technology that can simultaneously receive and process different types of data such as vision, audio, and text
- **Shared Latent Space**: A virtual multidimensional space where visual vectors like "puppy photo" and text vectors like "cute puppy" are made to converge at the same coordinates (location) in mathematical space.
- **Zero-shot**: The powerful generalization capability to immediately perform classification or search without a single additional training session (Zero) for a specific domain (like home shopping products) when deploying the model in production.

#### Origin of CLIP's Name

Contrastive Language-Image Pretraining:

Contrastive learning means using a method that pulls correct pairs (truly matching photos and text) together to increase similarity and contrasts incorrect pairs (non-matching photos and text) to push them apart.

Language-Image literally means connecting two different modalities.

Pretraining means having completed massive-scale training in advance using 400 million (image, text) pairs scattered across the internet.

### Problems to Solve

There is a **limitation of supervised learning**. Existing models like ResNet had to be trained on limited datasets (ImageNet, etc.) where humans manually labeled "this is a cat, this is a chair" as class 1, 2. If a new category like "smartphone" appeared, they had to be retrained from scratch.

There is also a **limitation of metadata (tag)-dependent search**. To search for images with text, DB administrators had to manually input text tags like "red," "dress," "summer" for each image. Images with missing tags became isolated data that would never be searched.

### Problem-Solving Approach

**Internet Scraping Natural Language Learning**: Instead of humans labeling, they scraped 400 million (image, description text) pairs naturally bundled together like the `alt` (alternative text) attribute of `<img>` tags on internet web pages and trained directly on them.

**Two-Tower Architecture and Similarity Maximization**: They separated the brain that reads text (text encoder) and the brain that sees images (image encoder), then applied contrastive learning to match the resulting vectors from the same pair to have a cosine similarity of 1.

Let me illustrate with an analogy of a clothing store part-timer:

- **Past part-timer (DB tag search & general classification model)**: When a customer requests "Please find a chic black leather jacket!" the past part-timer couldn't look at the clothes themselves and only searched through barcode text (metadata) that warehouse staff attached to sleeves. If staff mistakenly didn't tag a black jacket, they could never find it even if it was right in front of them.
- **CLIP part-timer (multimodal shared space model)**: This part-timer read 400 million fashion magazines, Instagram blogs, etc. before coming to work (Pretraining), so when a customer says "chic black leather jacket," the **unique atmosphere and texture (specific coordinates in vector space)** that the text signifies comes to mind. The part-timer doesn't look at barcode tags at all. They quickly scan tens of thousands of clothes (images) hanging in the warehouse with their eyes and immediately retrieve the clothes with the feeling (image vector) that visually best matches the feeling (text vector) in their mind. Tags don't matter. This is the principle of CLIP's cross-modal search.

### Detailed Operating Principles and Structure

The NxN contrastive learning mechanism by which CLIP learns 400 million data points is performed through parallel processing as follows:

1. **Batch Construction**: Push N (image, text) pairs into one learning unit (Batch). Typically uses a massive size of N = 32,768.
2. **Independent Embedding:**
   1. N images pass through the image encoder (ViT or ResNet) to become N high-dimensional vectors ($I_1, I_2, ..., I_n$).
   2. N texts pass through the Text Encoder (Transformer) to become N high-dimensional vectors $T_1, T_2, ..., T_n$.
3. **Matrix Multiplication**: Dot product the image vector matrix and text vector matrix to create an N x N similarity matrix.
4. **Diagonal Optimization (Symmetric Cross-Entropy)**
   1. Train diagonal components of the matrix ($I_1 \cdot T_1$, $I_2 \cdot T_2$, etc.) so that true pairs have a maximum dot product value of 1.
   2. Force all remaining off-diagonal components of the matrix (different pairs) to have minimum dot product values of 0 or negative.

This is **Zero-shot Classification code that directly shows low-level operations of how text vectors and image vectors are mathematically dot-producted and how probabilities (Softmax) change**.

```py
import torch
from transformers import CLIPProcessor, CLIPModel
from PIL import Image
import torch.nn.functional as F

def clip_zero_shot_classification(image_path, candidate_texts):
    """
    Even without pre-trained classes, calculate probabilities through mathematical 
    dot products to determine which candidate text the image is closest to.
    """
    model_id = "openai/clip-vit-base-patch32"
    model = CLIPModel.from_pretrained(model_id)
    processor = CLIPProcessor.from_pretrained(model_id)
    model.eval()

    image = Image.open(image_path).convert("RGB")
    
    # 1. Tensor conversion and preprocessing
    inputs = processor(text=candidate_texts, images=image, return_tensors="pt", padding=True)
    
    with torch.no_grad():
        # 2. Call each encoder inside the model to extract 'independent vectors (embeddings)'.
        # Image Feature: [1, 512] dimensions (1 image)
        image_features = model.get_image_features(pixel_values=inputs['pixel_values'])
        # Text Feature: [3, 512] dimensions (3 texts)
        text_features = model.get_text_features(input_ids=inputs['input_ids'], attention_mask=inputs['attention_mask'])
        
        # 3. Vector normalization (L2 Normalization)
        # Adjust length to 1 so that dot product becomes cosine similarity.
        image_features = image_features / image_features.norm(dim=-1, keepdim=True)
        text_features = text_features / text_features.norm(dim=-1, keepdim=True)
        
        # 4. Dot product operation (Matrix Multiplication)
        # [1, 512] @ [512, 3] = [1, 3] dimensional similarity scores (Logits).
        # model.logit_scale.exp() is CLIP's unique temperature parameter that amplifies score magnitude.
        logit_scale = model.logit_scale.exp()
        logits_per_image = logit_scale * image_features @ text_features.t()
        
        # 5. Probability conversion (Softmax)
        # Convert similarity scores to probabilities that sum to 1 (100%).
        probs = logits_per_image.softmax(dim=1).squeeze()

    # Result analysis
    print(f"Target image: {image_path}")
    print("-" * 30)
    for i, text in enumerate(candidate_texts):
        print(f"Candidate text: '{text}' -> Dot product-based match probability: {probs[i].item() * 100:.2f}%")

# ==========================================
# Execution Simulation
# ==========================================
# While existing ResNet would only output predefined classes like "This is home shopping appliance category 3,"
# CLIP compares similarity in real-time with any text query we create on-site.
# 
# candidates = [
#     "A modern wooden dining table", 
#     "A red leather sofa", 
#     "An abstract oil painting"
# ]
# clip_zero_shot_classification("sample_furniture.jpg", candidates)
```

Let's look at a pipeline component class that performs offline image indexing and real-time text encoding -> Elasticsearch delivery in an actual e-commerce service.

```py
import torch
from transformers import CLIPProcessor, CLIPTextModelWithProjection
from typing import List

class HomeShoppingSearchEncoder:
    """
    Real-time text embedding-only class mounted on home shopping search backend.
    Optimizes response speed by running only the text encoder in memory during real-time search,
    without loading the heavy image encoder.
    """
    
    def __init__(self, model_name: str = "openai/clip-vit-base-patch32"):
        self.device = "cuda" if torch.cuda.is_available() else "cpu"
        
        # Memory optimization: Load only Text Encoder and Projection Head, excluding Image Encoder.
        self.text_model = CLIPTextModelWithProjection.from_pretrained(model_name).to(self.device)
        self.processor = CLIPProcessor.from_pretrained(model_name)
        
        self.text_model.eval()

    @torch.no_grad()
    def get_search_query_vector(self, search_query: str) -> List[float]:
        """
        Convert customer's search query into a [512]-dimensional real number array (List)
        that can be inserted into an Elasticsearch k-NN query.
        """
        # 1. Preprocess customer's search query (tokenization)
        inputs = self.processor(text=[search_query], return_tensors="pt", padding=True).to(self.device)
        
        # 2. Extract text embedding vector
        text_outputs = self.text_model(**inputs)
        text_embeds = text_outputs.text_embeds
        
        # 3. L2 normalization (essential for Elasticsearch cosine similarity search)
        normalized_embeds = text_embeds / text_embeds.norm(dim=-1, keepdim=True)
        
        # 4. Convert PyTorch Tensor to pure Python List for JSON serialization
        return normalized_embeds.squeeze().cpu().tolist()

# ==========================================
# Backend API Controller Usage Example
# ==========================================
# encoder = HomeShoppingSearchEncoder()
#
# @app.get("/search")
# def search_products(query: str):
#     # 1. Convert text query to 512-dimensional vector (consumes about 10~20ms)
#     query_vector = encoder.get_search_query_vector(query)
#     
#     # 2. Request vector search query (k-NN) directly to Elasticsearch
#     es_query = {
#         "knn": {
#             "field": "image_vector", # Image vector field indexed offline in advance
#             "query_vector": query_vector,
#             "k": 20,
#             "num_candidates": 200
#         }
#     }
#     # results = es_client.search(index="home_shopping_products", body=es_query)
#     # return parse_results(results)
```
