# Long-Term Memory Research

Various open-source projects are tackling the problem of long-term memory for agents using diverse approaches.

Today, we'll explore each of these open-source projects in the ecosystem to understand their characteristics.

The classification axes are threefold:

- Original Search (stores session text as chunks for BM25, semantic similarity search - original utterances are preserved but grouped into fixed top-k sets)
- LLM Processed Memory (LLM extracts and restructures during the collection phase)
- File-Based Agents (tool calls access a persistent Markdown repository)

<br>

## Mem0

> LLM extracts and stores only facts from conversations.

It has a two-stage structure. First, the LLM extracts core facts from the conversation, and then uses Tool Calling to

execute one of ADD / UPDATE / DELETE / NOOP to maintain memory consistency.

The extractor and update modules use small models capable of function calling.

The reason for this implementation is that most of the original conversation text is noise.

Putting things like "Hello" or "Thanks" into a vector DB only pollutes the search. Therefore, the choice is to filter them at write time, leaving only atomic facts.

### Trade Off

In fact, this design clearly exposes its core strengths and weaknesses.

The advantage is compressed representation, and the disadvantage is information loss.

This is because the LLM decides at write time what will be needed at read time, but can it accurately manage this

100% according to human intent? Even humans couldn't do this perfectly. Inevitably, I know my own intentions best,

which is similar to the **lossy compression problem** in MLA. What seems unimportant now might become crucial later, and the inability to recover it is the biggest drawback.

### **Graph Variation (Mem0ᵍ)**

It computes entity embeddings to find similar existing nodes and performs conflict detection and updates with additional LLM calls.

During queries, it combines graph neighbor traversal with triple-based semantic matching.

However, while the graph layer roughly doubles token usage, it provides benefits in scenarios where relational structures are useful, such as temporal queries and open-ended questions.

It's good, but expensive.

<br>

## Letta (MemGPT)

> A method where the model directly decides what to remember.

If a pipeline filters content beforehand, like Mem0, it doesn't know the context at the filtering stage. The model in conversation actually knows the context, but it has no decision-making power.

### Solution

So, rather than the LLM summarizing and storing at that moment,

memory management was made into a tool call for the model, and the storage structure is three-tiered:

- **Core Memory:** Always present within the context; the model reads and writes to it.
- **Recall Memory:** Exists outside the context; directly retrieved via search.
- **Archival Memory:** Stored in external storage; accessed via tool calls.

Core Memory is a small block that fits within the context window.

Recall Memory is a searchable conversation history stored outside the context.

Archival Memory is a long-term storage queried via tool calls.

The unit of editing is a labeled string called a **memory block**, which the model modifies using two functions: `core_memory_append` for appending and `core_memory_replace` for replacing.

- **sleep time compute**: In the initial design, memory cleanup during conversations slowed down responses. This was changed so that a sleep-time agent handles memory management asynchronously, improving response time and memory quality.

A separate sleep-time agent runs asynchronously to edit the main agent's core memory.

It abstracts patterns from specific experiences, resolves contradictions between stored facts, and pre-computes associations to speed up future inference.

#### Strengths

- Decisions made by an entity aware of the context can be more accurate.
- Tool call logs remain, showing what was stored and why.
- Letta Code supports git-based memory directories, preserving version history of what the agent has learned.

#### Weaknesses

The scope of adoption is large, which is a constraint.

Letta isn't just a memory layer on an existing stack; it's the stack itself.

If evaluated as a memory solution, adopting it means adopting the entire agent platform.

If you're already using another agent framework, try just running `./goal` to use its mechanisms. LOL.

Ultimately, the conclusion is that it's a long-term memory management technique that uses memory blocks and layers to add more meaning, managing history and organizing context through tool calls.

Improvements over Mem0 include separation of layers, accuracy, ability to retrieve past history, and management in a defined protocol format via tool calls.

<br>

## Zep / Graphiti

> Let's manage with expiration dates.

The previous two assume that stored facts remain true indefinitely.

However, user information changes: "I use Python" -> "In 3 months, this project will be TypeScript" -> "In 6 months, I won't use any language (?)".

If such information is put into a vector DB, the vector store will hold both and return both when searched.

In other words, it's impossible to know which one is current.

### Solution

Actually, the solution is simple: attach time information to all facts. There are two types of timestamps, totaling four variables.

One pair indicates when the fact was created and expired in the system, and the other pair indicates the period when the fact was true in reality.

```
valid_at    / invalid_at     ← Period when true in reality
created_at  / expired_at     ← Period known by the system
```

It can handle both absolute times, like "Alan Turing born June 23, 1912," and relative times, like "started two weeks ago."

### Core Operation

Instead of deletion, it performs invalidation.

If new information contradicts an existing fact, instead of deleting the edge, it writes an `invalid` timestamp. The graph can then answer what it believed at any given time, without presenting old facts as current.

**Original Preservation** Episodes (original messages) and the semantic edges extracted from them are linked by a bidirectional index, allowing tracing back from extracted facts to their sources. Related facts can also be directly retrieved from the original.

#### Strengths

- Temporal queries are possible: "this person's address information in March".
- Auditing and compliance responses are possible (due to history preservation).
- Incorrect updates can be rolled back.
- All facts are traceable to their origin.

#### Weaknesses

**Heavy** Each time an episode is added, the input text goes through an LLM, is decomposed into entities and edges, and stored in Neo4j along with its temporal validity period.

- Graph DB operational burden.
- Additional LLM call cost per write.
- Requires separate models for embeddings and rerankers.

<br>

## Honcho

> Stores who a person is, rather than what they said.

The previous three all store facts. However, **what's needed for personalization is not a list of facts, but dispositions and preferences.**

To answer preference-based questions like "What does this person prefer?" rather than factual ones, the entire conversation would need to be re-read and evaluated.

Deterministic storage is not the answer.

### How it Solves the Problem

When a message comes in, it creates a representation.

Upon message arrival, a small fine-tuned model extracts latent information—preferences, beliefs, facts, contradictions—and writes it to the speaker's structured representation.

#### Peer Structure: Treating Humans and AI as the Same Type

Both human and AI agents are integrated as Peers, and the internal vector store uses (observer, observed) peer pairs as keys.

It supports self-representation (observer == observed) and mutual modeling simultaneously with the same mechanism.

That is, the storage key is `who, about whom`. My view of myself and my view of others are processed with the same structure.

User peers inherently have `observeOther` set to false. Modeling is based only on what the user says, not on what the agent replies.

This prevents agent conjectures from flowing back into the user profile.

### Architecture

It separates the API server and workers. The API enqueues requests and returns immediately, after which a "deriver" worker processes them.

It does not block HTTP requests in front of LLM operations.

Memory formation occurs with a single structured output LLM call per batch, not within the agent's rule loop.

Costs are predictable and latency is low. The only true agent using tools is Dialectic.

The Dialectic API is a natural language endpoint where the agent engages in backchannel conversations with Honcho to receive all actionable insights about the user in real-time. It's like asking the agent, not querying a DB.

#### Strengths

- Can quickly answer high-level questions.
- In exchange for write latency, it can answer queries like "What does Alice prefer?" without re-reading the entire conversation history.
- Predictable write costs (fixed number of calls).
- Supports perspective-based modeling in multi-party conversations.

#### Weaknesses

- Write latency (triggers background message processing).
- Search itself is not the primary goal; unsuitable if exact original text usage is required.
- Licensed under AGPL-3.0, requiring review before embedding in commercial products.

<br>

## Cognee

> Memory as a Data Pipeline

The systems above target conversations. However, to turn thousands of company documents into memory, formats vary, and a way to verify the processing steps is needed.

An ECL pipeline exists (Extract, Cognify, Load), which is an extract-cognify-load approach.

The public APIs are `add()`, `cognify()`, `search()`, `memify()`, and agent interfaces include `remember`/`recall`/`forget`/`improve`.

The Cognify stage is key: instead of simple chunking and embedding, it creates a network of entities and typed relationships.

The point of this project is that each stage is a reusable task that engineers can inspect or redefine, meaning it's not a black box.

#### Strengths

- Handles various source formats.
- Each stage of the pipeline can be verified and replaced.
- Strong in bulk document processing. Bayer reportedly processed 10,000 scientific papers into research memory, and the University of Wyoming created an evidence graph with page-level citations from scattered policy documents.

#### Weaknesses

- Overkill for conversational memory.
- Learning cost for pipeline configuration.

<br>

## File Based (Hermes, Opneclaw)

> Just Markdown

A common weakness of the systems above is the difficulty in answering "Why was this remembered?" and the inability for humans to directly correct incorrectly stored information.

And all of them require additional infrastructure.

Therefore, hermes and openclaw are classified as a separate category even in research literature.

File-based agents are implemented by having an LLM access a Markdown repository, managed across sessions, via tool calls.

No special technology, just an agent that reads and writes files, that's it.

#### Strengths

- Humans can open, read, and directly modify it.
- History is managed with Git.
- No infrastructure is required.
- Debugging is actually possible.

#### Weaknesses

- Simple search (becomes dumb as files grow; the mechanisms above are indeed needed).
- No automatic deduplication or contradiction resolution.
- Performance degrades at scale.

LettCode also introduced a git-based memory directory called MemFS. This is a case of moving back from sophisticated approaches to simpler ones.

| System | Storage Target | Decision Maker | Conflict Resolution | Deployment Type |
|---|---|---|---|---|
| Mem0 | Extracted Facts | Pipeline | Overwritten by UPDATE | Library |
| Letta | Editable Blocks | Model Itself | Model self-edits | Entire Platform |
| Zep/Graphiti | Time-stamped Edges | Pipeline | Invalidation (history preserved) | Service + GraphDB |
| Honcho | Person Representation | Pipeline (write) / Agent (read) | Representation Update | Service |
| Cognee | Knowledge Graph | Pipeline | Graph Reconstruction | Library |
| File | Markdown Text | Model + Human | Human edits | None |

In oh-my-hermes, these points are addressed by incorporating memory labeling and version management from Letta Code,

applying memory blocks for in-context and out-of-context management, periodically managing and "dreaming" (async LLM data pipeline) agent memory,

and managing it through an enhanced Hermes version memory system that also provides skillsets involving direct user pruning (trimming).

https://github.com/rlaope/oh-my-hermes
