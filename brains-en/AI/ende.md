# Positional Encoding, Encoder-Decoder

Through self-attention, we were able to break free from the sequential constraints of RNNs and compute the relationships between words in parallel all at once.

However, a critical side effect emerges here:

**Loss of Order Information: Permutation Invariance**. Self-attention is merely matrix multiplication, and from the model's perspective, if it receives "I love you" and "You love me" as input without being told the position of the words, the mathematical results for both sentences will be identical. This is because words don't enter in a queue; they all come in at once.

The direction for a solution emerged from the idea of "mathematically imprinting positional information (a 'number tag') onto the input word vectors (embeddings), indicating 'you are the first word,' 'you are the second word,' and so on." This is Positional Encoding.

<br>

## Positional Encoding

The simplest learning method would be to add integer indices like 1, 2, 3..., but if sentences become long, the values would grow infinitely large, hindering model training. To solve this, Transformers use Sine and Cosine periodic functions.

Imagine an analog clock: the second hand moves fast, the minute hand at a medium pace, and the hour hand slowly. If we only know the angles (positions) of these three hands, we can tell the exact hour, minute, and second. Similarly, by overlapping waves with different periods, a unique mathematical fingerprint (barcode) is created for each position that never overlaps, and this is added to the word vector.

Mathematically, for a position $pos$ and an index $i$ in the embedding dimension, positional encoding is defined as follows:

$$PE_{(pos, 2i)} = \sin\left(\frac{pos}{10000^{2i/d_{model}}}\right)$$

$$PE_{(pos, 2i+1)} = \cos\left(\frac{pos}{10000^{2i/d_{model}}}\right)$$

The positional vector generated this way is added to the original word embedding vector and fed as input to the model.

<br>

## Overall Architecture: Assembling the Encoder and Decoder

Now, words tagged with PE (Positional Encoding) enter the Transformer's main body.

-   **Encoder**: Deep Understanding
    -   Repeats layers of `Multi-Head Attention -> Feed-Forward Network (FFN)` multiple times.
    -   Each layer undergoes Residual Connection and Layer Normalization. As a result, the encoder outputs a rich contextual map where all word relationships in the input sentence are perfectly computed.
-   **Decoder**: Translating with a Masked Future
    -   **Masked Self-Attention**: When generating words, the decoder must not peek at future words. This would be like solving with the answer key, so it masks out attention scores for words after the current timestep (using infinity values).
    -   **Cross-Attention (Encoder-Decoder Attention)**: This is the core of the decoder. Here, the **Query (Q)** comes from the words the Decoder has generated so far, while Key and Value come from the contextual map provided by the encoder. If I've just translated the word "apple" (Q), this process asks where in the original sentence's K and V I should look to write the next word.

### Example

Let's look at generating a fingerprint (barcode) using periodic functions with PyTorch and adding it to an embedding.

```py
import torch
import torch.nn as nn
import math

class PositionalEncoding(nn.Module):
    def __init__(self, d_model, max_len=5000):
        # d_model: Embedding dimension of the model (e.g., 512)
        # max_len: Maximum sequence length the model can handle
        super().__init__()

        # Create an empty tensor of size [max_len, d_model] (to be filled with positional tags)
        pe = torch.zeros(max_len, d_model)
        
        # Generate position indices pos: [0, 1, 2, ..., max_len-1]
        position = torch.arange(0, max_len, dtype=torch.float).unsqueeze(1)
        
        # Calculate the denominator (div_term) that determines the period (optimized using exp and log)
        # Mathematically equivalent to 10000^(2i/d_model)
        div_term = torch.exp(torch.arange(0, d_model, 2).float() * (-math.log(10000.0) / d_model))
        
        # Apply Sine function to even indices (2i)
        pe[:, 0::2] = torch.sin(position * div_term)
        # Apply Cosine function to odd indices (2i+1)
        pe[:, 1::2] = torch.cos(position * div_term)
        
        # Add a dimension to pe for adding it to [Batch, Seq_len, d_model] shape
        pe = pe.unsqueeze(0)
        
        # Not a model parameter (weight), but needs to be saved as state, so use register_buffer
        self.register_buffer('pe', pe)

    def forward(self, x):
        # x: Word embedding vector [Batch, Seq_len, d_model]
        # Add positional encoding values corresponding to the length of the input vector x
        x = x + self.pe[:, :x.size(1), :]
        return x

# --- Example Usage ---
d_model = 512
seq_len = 50   # Sentence with 50 words
batch_size = 2

# Create a dummy word embedding tensor
word_embeddings = torch.randn(batch_size, seq_len, d_model)

# Pass through the positional encoding module
pos_encoder = PositionalEncoding(d_model=d_model)
encoded_x = pos_encoder(word_embeddings)

print(f"Input embedding shape: {word_embeddings.shape}")
print(f"Output shape with positional information added: {encoded_x.shape}")
```
