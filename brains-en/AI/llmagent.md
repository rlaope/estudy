# LLM Agent Architecture

### ReAct (Reasoning and Acting) Pattern and Tool Use (Function Calling)

ReAct is a core reasoning framework that enables LLMs to act as a **controller, or brain,** for controlling systems, moving beyond simple text generation.

#### How it Works (Thought -> Action -> Observation)

When an agent receives a user request, it first performs a thought.

It plans what to do and determines if external tools are needed.

After that, it performs an action, which involves calling an external system API.

After observing the API's result values, status codes, JSON responses, etc., it thinks again to derive a final answer or take further action. This loop repeats until the goal is achieved.

#### Tool Use (Function Calling)

In the past, LLM text output had to be parsed with regular expressions (regex) to execute functions.

Today's function calling is a feature that forces LLMs to accurately understand the JSON schema we define (function name, parameter types, required status) and return **deterministic** JSON objects.

It is the most reliable bridge connecting the unstructured natural language world with structured enterprise systems like databases and CRM APIs.

**Hallucination in Arguments**: LLMs frequently invent non-existent parameters or return an integer when a string is expected (type casting errors), making a strict validation layer essential.

**API Failures and Latency**: When a client's API times out and returns a 500 error, meaning the observation failed, the thought process must be designed at the prompt and system architecture level to determine whether the agent should give up (e.g., "API is not responding"), retry, or execute a fallback scenario.

### Single Agent Limitations and Multi-Agent System Design

In the early stages of a project, one might start with a single agent architecture where a massive prompt is given all tools (DB search, email sending, CRM updates, etc.). However, as the business becomes more complex, it encounters the same limitations as monolithic architectures.

**Limitations of a Single Agent:**

1.  **Context Overflow & Attention Dilution**: As prompts get longer and the number of tools increases, the LLM's attention becomes diluted, leading it to call incorrect tools or forget core instructions.
2.  **Security and Access Control**: A single agent possesses all permissions, making it highly vulnerable to prompt injection.
3.  **Bottleneck**: Since all tasks are processed sequentially, latency increases exponentially.

**Multi-Agent System Design**:
This can be solved by breaking down a single agent into smaller, purpose-specific microservices. For example, by dividing it into an intent-recognition router agent, a DB query-specific agent, and an answer review and correction agent.

-   **Topology Design**: Hierarchical: A manager agent divides tasks, instructs worker agents, and aggregates their results.
-   **Network/Collaborative**: Agents communicate with each other and update their states. Frameworks like LangGraph and AutoGen are utilized for this.

**Orchestration and State Management**: Challenges include how agents will exchange data (e.g., LangGraph's State Graph) and how to set depth limits to prevent infinite loops.

### Agent Memory Management Strategies: Short-term vs. Long-term

LLMs are stateless functions. Therefore, to utilize continuous conversations or past information, we must inject memory into them.

-   **Short-term Memory**
    -   The concept is information that fits within the conversation history (context window) of the current session.
    -   The strategy involves using a sliding window approach, where the oldest conversations are discarded when the conversation length exceeds the token limit, or summarizing the previous conversation content itself for storage. In-memory databases like Redis are utilized for this.
-   **Long-term Memory and Vector DB**
    -   **Concept**: Information that must be permanently maintained even after a session ends, such as customer preferences, past purchase history, or internal manuals.
    -   **Role of VectorDB**: It converts text into high-dimensional numerical vectors and stores them. When a user asks a question, the question is also vectorized, and past memories or documents with the closest mathematical distance (e.g., Cosine Similarity) are retrieved and passed to the LLM. This forms the basic backbone of RAG. Pinecone, Milvus, or the pgvector extension for PostgreSQL are commonly used.

**Entity Memory**: Simple semantic-based vector search is insufficient. Clear facts, such as a specific customer's tier or remaining points, require a hybrid strategy of explicitly storing, managing, updating, and deleting them as entities in an RDBMS or graph database, rather than a vector database.

**Cache Invalidation**: When long-term memory changes (e.g., a customer's address is updated), how to invalidate and update previous embedding data within the vector DB determines system reliability.
