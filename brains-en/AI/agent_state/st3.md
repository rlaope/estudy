# Checkpointing Application

When an application server restarts or when multiple server instances operate in a distributed environment, how can the state of a specific user's agent be shared among servers and persistently preserved?

Specifically, what kind of hierarchical design is needed if the state needs to be managed in a human-readable JSON format rather than binary data?

LangGraph's persistence architecture is implemented through an abstraction layer called Checkpointer. RedisSaver, for example, is a high-performance checkpointing component that uses Redis, an in-memory data structure store, as its backend to save and restore agent states in real-time.

- **RedisSaver**: An object that saves the current State schema data to a specific key in Redis every time each node of the LangGraph graph finishes execution. It serves as an external persistence layer that enables session sharing in distributed environments.
- **JSON Serialization**: The process of converting state data in Python object form into JSON format, which can be transmitted over a network and stored. While many checkpointing solutions primarily use Pickle, which is Python-specific, JSON format is often adopted for better compatibility with other languages and readability.
- **Checkpoint**: A snapshot of the agent's state at a specific point in time (immediately after node execution). This includes message history, global variable values, and information about the next node to be executed.
- **Thread Configuration**: A configuration object that includes a `thread_id`. It acts as an index to distinguish data within Redis, allowing stateless servers to accurately retrieve a specific user's previous context.

<br>

## Problem Definition

Using the basic MemorySaver or integrating an external store without an appropriate serialization strategy leads to the following technical debt and operational constraints:

- **Volatile State Data and Lack of Scalability**: `MemorySaver` stores data in the server's RAM, so all conversation history is lost when the process terminates. In a multi-node server environment, if session stickiness is not guaranteed, the context is broken when a user's request goes to a different server.
- **Security and Compatibility Issues with Pickle Serialization**: Python's default Pickle method has security vulnerabilities where arbitrary code can be executed during deserialization. Additionally, it's difficult for administrators to directly query data stored in Redis or for dashboards written in other languages to parse it.
- **State Loss Due to Lack of Atomic Writes**: When an agent performs complex parallel operations, multiple node results might attempt to write to Redis simultaneously, leading to data contention and a risk of corrupting the final state object. Therefore, checkpointing must ensure transactional atomicity.

### Solution

- **Inject Redis-based External Persistence**: Utilize the `langgraph-checkpoint-redis` library to create a `RedisSaver` object. This physically isolates and stores the state in an external Redis instance instead of application memory, ensuring persistence independent of server availability.
- **Apply Custom JSON Serializer**: Inject serialization logic that converts complex LangChain message objects into a structured JSON format, leveraging Pydantic models or `langchain_core`'s `load/dump` utilities. This maintains Redis internal data in text form, ensuring high readability and interoperability.
- **Compile-Time Checkpointer Binding**: Inject the checkpointer at graph build time using the format `compile(checkpointer=redis_saver)`. This allows the framework to automatically intercept node execution completion events and record checkpoints to Redis, without developers needing to manually issue save commands within nodes.

<br>

## Detailed Operation Principles and Structure

```mermaid
sequenceDiagram
    participant Client as Client
    participant Router as FastAPI
    participant Graph as LangGraph Engine
    participant Node as Agent Node
    participant Saver as RedisSaver
    participant Redis as Redis Server

    Client->>Router: 1. POST /invoke (thread_id: 123)
    Router->>Graph: 2. graph.invoke(config={"thread_id": "123"})
    
    Note over Graph,Saver: [State Recovery Phase]
    Graph->>Saver: 3. Request previous checkpoint
    Saver->>Redis: 4. GET checkpoint:123
    Redis-->>Saver: 5. Return serialized JSON
    Saver->>Graph: 6. Deserialize JSON and restore State object
    
    Note over Graph,Node: [Node Execution Phase]
    Graph->>Node: 7. Inject restored State and execute node
    Node-->>Graph: 8. Return State update dictionary
    Graph->>Graph: 9. Merge latest State via internal reducer
    
    Note over Graph,Redis: [Persistence (Checkpointing) Phase]
    Graph->>Saver: 10. Node execution complete and save trigger
    Saver->>
```

We analyze how data is structured and stored within Redis and what lifecycle it undergoes with each node execution, at the physical memory and network levels.

1.  **Connection Pool Initialization**: When the application starts, a Redis connection pool is created using `redis-py`, and `RedisSaver` acquires this pool and waits.
2.  **Graph Execution and `thread_id` Identification**: When a client request arrives, a specific session is designated using `config={"configurable": {"thread_id": "..."}}`.
3.  **Node Execution Completion and State Interception**: When the operation of a specific node, `agent_node`, finishes, the LangGraph runtime checks the returned State update value.
4.  **Object Serialization Pipeline**: `BaseMessage` objects or custom variables within the State are passed to the serializer, where complex Python classes are converted into JSON strings.
5.  **Redis HSET/SET**: The `thread_id` is used as part of the key to record data in Redis. Data is stored as binary-safe string using a naming convention like `checkpoint:<threadid>`.
6.  **Version Control and Checkpoint Tree**: LangGraph supports not only overwriting but also branching execution paths. It can manage timestamps or checkpoint IDs as subkeys, allowing for time travel to revert the state to a specific past point.
7.  **Resource Release After Response**: After recording is complete, the Redis connection is returned to the pool, and the final response is sent.

### Example

This example demonstrates the core flow of logic for directly handling Redis to save and restore state as JSON.

```py
import json
import redis
from typing import Dict, Any

# 1. Redis Connection Setup
r = redis.Redis(host='localhost', port=6379, db=0, decode_responses=True)

# 2. Hypothetical Agent State (mimicking LangGraph State)
current_state = {
    "messages": [
        {"role": "user", "content": "오늘 날씨 어때?"},
        {"role": "assistant", "content": "서울은 맑음입니다."}
    ],
    "next_step": "weather_api_call"
}

# 3. JSON Serialization and Saving Logic
def save_checkpoint(thread_id: str, state: Dict[str, Any]):
    checkpoint_key = f"checkpoint:{thread_id}"
    # Convert object to human-readable JSON string
    serialized_data = json.dumps(state, ensure_ascii=False)
    # Atomically save to Redis
    r.set(checkpoint_key, serialized_data)
    print(f"[Save] Thread {thread_id} state saved.")

# 4. Data Loading and Deserialization
def load_checkpoint(thread_id: str) -> Dict[str, Any]:
    checkpoint_key = f"checkpoint:{thread_id}"
    raw_data = r.get(checkpoint_key)
    if raw_data:
        return json.loads(raw_data)
    return {}

# Execution Test
tid = "user_1234"
save_checkpoint(tid, current_state)
restored_state = load_checkpoint(tid)
print(f"Restored data: {restored_state['messages'][-1]['content']}")
```

Let's look at production-level code that uses the `langgraph-checkpoint-redis` library to inject `RedisSaver` into an actual graph and operates in an asynchronous environment.

```py
import os
from typing import Annotated, TypedDict
from langchain_openai import ChatOpenAI
from langgraph.graph import StateGraph, START, END
from langgraph.checkpoint.redis import RedisSaver
from redis.asyncio import ConnectionPool

# 1. Define State Schema
class AgentState(TypedDict):
    input: str
    history: Annotated[list, lambda x, y: x + y]

# 2. Define Node
def call_model(state: AgentState):
    # In a real environment, LLM call logic would go here
    return {"history": [f"AI response to: {state['input']}"]}

# 3. Redis Integration and Checkpointer Initialization
# In production, Redis URL is managed via environment variables.
REDIS_URL = os.getenv("REDIS_URL", "redis://localhost:6379")
pool = ConnectionPool.from_url(REDIS_URL)

# RedisSaver internally handles serialization,
# and a custom serializer can be injected into the constructor if needed.
checkpointer = RedisSaver(pool)

# 4. Build Graph and Inject Checkpointer
builder = StateGraph(AgentState)
builder.add_node("agent", call_model)
builder.add_edge(START, "agent")
builder.add_edge("agent", END)

# Inject the persistence layer into the framework at compile time.
# Now, all node execution results are automatically recorded to Redis.
graph = builder.compile(checkpointer=checkpointer)

# 5. Mimic Asynchronous Execution Endpoint
async def run_session(thread_id: str, user_input: str):
    config = {"configurable": {"thread_id": thread_id}}
    
    # Automatically loads previous state from Redis with each execution
    async for event in graph.astream(
        {"input": user_input, "history": []}, 
        config, 
        stream_mode="values"
    ):
        print(f"Current State in Thread {thread_id}: {event}")

# Usage Example: Calling twice with the same thread_id accumulates data in Redis
# await run_session("session_001", "First question")
# await run_session("session_001", "Second question (maintaining previous context)")
```
