# BERT, GPT

### Is the Seq2Seq structure always the answer?

The Transformer is a perfect translator, combining an encoder (for understanding) and a decoder (for generation).

However, AI engineers realize that the problems we want to solve are not always translation (converting sequence A to sequence B).

- **Understanding Problems:** For tasks like classifying whether a sentence is positive or negative, or finding answers in a document, there's no real need for a decoder that generates new words.
- **Generation Problems:** For tasks like writing novels or conversing with users, a decoder that simply predicts the next word based on the preceding context, like a word chain game, is sufficient, without necessarily needing an encoder to deeply analyze a complete original sentence.

This led to the concept of "splitting the Transformer in half: taking only the encoder specialized in understanding context and maximizing it, and taking only the decoder specialized in generating text and maximizing it."

<br>

## BERT (Bidirectional Encoder Representations form Transformers)

It perfectly understands the deep meaning and context of a sentence in a bidirectional manner.

It is a structure built by stacking multiple layers of only the Transformer's **Encoder** blocks.

### Masked Language Modeling (MLM)

The encoder can view all words in a sentence simultaneously (self-attention). Using this, BERT takes a fill-in-the-blank test.

15% of the words in the input sentence are randomly `[MASK]`ed and hidden.

- ex. I ate a delicious `[MASK]` today.

The model infers what word should come in the `[MASK]` position by simultaneously referencing both the left context (I ate a delicious) and the right context (today) of the masked word in a bidirectional manner.

By repeating hundreds of millions of sentences in this challenging fill-in-the-blank scenario, the model gains a very deep understanding of the complex grammatical and semantic relationships between words. This makes it excellent for tasks like classification and sentiment analysis.

<br>

## GPT Generative Pre-trained Transformer

It generates sentences by predicting the most natural next word based on the given context.

It is a structure built by stacking multiple layers of only the Transformer's **Decoder** blocks, and since there is no encoder, the encoder-decoder cross-attention layer is removed.

### Autoregressive Language Modeling

GPT does not look at context bidirectionally. Like reading or writing, it only views context unidirectionally, from left to right.

Mathematically, this is the process of maximizing the conditional probability $P(w_t | w_1, w_2, \dots, w_{t-1})$, meaning it's a word-chain training where the model predicts the probability of the next word given all preceding words.

To prevent cheating by looking ahead at future words, it strongly applies the masked self-attention mentioned in step 3. Attention scores with words after the current timestep (future words) are all masked to infinity.

### Why BERT Cannot Be Used to Build Chatbots

While BERT, which understands context more deeply in a bidirectional manner, might seem superior to GPT, BERT fundamentally cannot perform generation.

BERT requires an entire sentence to be given to understand relationships within it, and its architecture lacks the sequential generation logic to utter the first word from a blank slate and then subsequent words.

In contrast, GPT is trained from the outset with a structure that predicts the next word based only on preceding words, making it perfectly suited for conversational AI like ChatGPT and text generation. Although it initially seemed simple, as the model size and number of parameters were exponentially increased, this simple next-word prediction mechanism led to the emergence of human-level reasoning capabilities.

<br>

## Example

The architectural difference between BERT and GPT ultimately boils down to a single mask shape within their self-attention mechanisms.

```py
import torch

seq_len = 5 # Assuming a 5-word sentence

# ---------------------------------------------------------
# 1. BERT's Attention Mask (Primarily Padding Mask)
# ---------------------------------------------------------
# Since BERT must see both directions, there is no mask to hide future words.
# It only masks out meaningless [PAD] tokens added to match the sentence length.
# Example: 3-word sentence + 2 padding -> [Word, Word, Word, PAD, PAD]
bert_mask = torch.tensor([
    [1, 1, 1, 0, 0],  # 1 allows attention, 0 blocks attention (treated as -inf)
    [1, 1, 1, 0, 0],
    [1, 1, 1, 0, 0],
    [1, 1, 1, 0, 0],
    [1, 1, 1, 0, 0]
])
print("BERT Mask (All actual words can refer to each other):\n", bert_mask)

# ---------------------------------------------------------
# 2. GPT's Attention Mask (Causal Mask / Look-ahead Mask)
# ---------------------------------------------------------
# Since GPT should not see future words, it uses a lower triangular matrix form
# to mask so that it only refers to itself and words in its past.
# The torch.tril (triangular lower) function is used.
gpt_mask = torch.tril(torch.ones(seq_len, seq_len))

print("\nGPT Mask (Cannot refer to future words):\n", gpt_mask)
# Interpretation of output:
# The first word (row 1) can only see the first word. [1, 0, 0, 0, 0]
# The second word (row 2) can only see the first and second words. [1, 1, 0, 0, 0]
# ...
```

Thus, a single mask matrix shape, by intervening in the matrix multiplication operation, determines whether the same Transformer's self-attention module becomes BERT, which observes context, or GPT, which continues generating text.
