# Evolution from RNN to Attention

RNNs are neural network architectures designed to process time-series (sequence) data like text.

Their operation involves sequentially processing an input sequence $x_1, x_2, \dots, x_T$ timestep by timestep.

**Hidden State Update**: At each timestep $t$, the current state $h_t$ is computed by taking the previous hidden state $h_{t-1}$ and the current input $x_t$.

As a result, the hidden state $h_T$ at the final timestep accumulates information from the entire sequence, processed sequentially.

## Seq2Seq

Seq2Seq is a model that combines two RNNs (Encoder and Decoder) to map a variable-length input sequence to an output sequence of a different form.

- **Encoder:** Sequentially processes the words of the input sentence and outputs the hidden state $h_T$ of the final timestep. This is used as a fixed-length context vector (C) containing information about the entire sentence.
- **Decoder:** Receives the single vector C from the encoder as its initial hidden state and sequentially generates the output sequence $y_1, y_2, \dots, y_{T'}$.

### Limitations of Sequential Processing and Compression Structure

This RNN-based Seq2Seq structure suffers from two critical mathematical and architectural flaws.

- **Long-term Dependency and Vanishing Gradient**: RNNs multiply the same weight matrix at each timestep. During BPTT (Backpropagation Through Time), if the backpropagation distance becomes long, the gradient for early inputs converges to 0, leading to a Vanishing Gradient problem. As sentences get longer, the model loses information from the beginning.
- **Fixed Size Bottleneck**: Regardless of whether the input sequence length is 10 or 100, all information is overwritten and compressed into a single vector $C$ of fixed dimension. This inevitably leads to a loss of detailed information.

<br>

## Attention Mechanism

Attention eliminates the bottleneck where the Decoder relies solely on a single fixed vector $C$ to generate each output word.

Instead, it directly accesses all timestep hidden states $h_1, \dots, h_T$ of the Encoder to generate a dynamic context vector at each step.

### Mathematical Operations

1.  **Score Calculation:** The dot product of the decoder's current hidden state $s_t$ and each encoder state $h_i$ is computed to find the relevance (similarity) between words.

    $$e_{ti} = s_t^\top h_i$$

2.  **Weight Calculation (Softmax):** The computed scores are converted into probability values between 0 and 1 to determine which encoder state $h_i$ to weigh at the current timestep.

    $$\alpha_{ti} = \frac{\exp(e_{ti})}{\sum_{k=1}^T \exp(e_{tk})}$$

3.  **Dynamic Context Vector Generation:** A new vector $c_t$ is created by summing the encoder states multiplied by their weights (weighted sum), and this vector is used as input (reference) for the decoder.

    $$c_t = \sum_{i=1}^T \alpha_{ti} h_i$$

<br>

## Example

Below is the attention operation code where the decoder generates a dynamic context vector by referencing the encoder's entire hidden states `encoder_outputs` at each step.

```py
import torch
import torch.nn as nn
import torch.nn.functional as F

class AttentionMechanism(nn.Module):
    def __init__(self):
        super().__init__()

    def forward(self, decoder_hidden, encoder_outputs):
        # decoder_hidden: [Batch, 1, Hidden_dim] (current state s_t of the decoder)
        # encoder_outputs: [Batch, Seq_len, Hidden_dim] (all states h_1 ... h_T of the encoder)

        # 1. Calculate attention scores (dot product operation)
        # [Batch, 1, Hidden_dim] x [Batch, Hidden_dim, Seq_len] -> [Batch, 1, Seq_len]
        attn_scores = torch.bmm(decoder_hidden, encoder_outputs.transpose(1, 2))

        # 2. Calculate attention weights (Softmax)
        attn_weights = F.softmax(attn_scores, dim=-1)

        # 3. Derive dynamic context vector (weighted sum)
        # [Batch, 1, Seq_len] x [Batch, Seq_len, Hidden_dim] -> [Batch, 1, Hidden_dim]
        context_vector = torch.bmm(attn_weights, encoder_outputs)

        return context_vector, attn_weights
```
