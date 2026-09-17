# Tool Calling and MCP-based Tool Design

Giving tools to LLMs is very powerful, but also risky.

The core topic of this chapter is resolving the Impedance Mismatch between LLMs, which are probabilistic models predicting text, and software APIs, which demand strict types and rules.

## How to defend against LLMs incorrectly selecting tools or generating incorrect arguments?

LLMs are inherently non-deterministic, so even with well-crafted prompts, hallucinations like the following can occur:

1.  **Calling a non-existent tool:** Given only a `search_web` tool, the LLM attempts to call a `delete_database` tool on its own.
2.  **Type mismatch**: The `limit` argument requires an integer 10, but the LLM sends the string `ten` or `10개`.
3.  **Exceeding permissions and scope:** Requesting `limit: 100000` for an API that should only retrieve 1 to 100 items, causing a database overload.

To defend against these issues, engineers must design systems with a Zero Trust approach.

This means performing strong Schema Validation at the application level as a primary defense.
Instead of crashing the system upon failure, **package the error message as text and return it to the LLM, encouraging self-correction.**

<br>

## MCP

In the past, the JSON format for defining and calling tools differed across OpenAI, Anthropic, and OSS models.

MCP (Model Context Protocol) emerged to solve this problem.

MCP is an open-source protocol that standardizes how AI models (clients) communicate with data sources/tools (servers). It's like a USB-C port for AI, connecting computers to peripherals.

It follows a **Client-Server architecture**:

-   **MCP Host (Client)**: LLM agents (e.g., Claude Desktop, LangChain, Cursor).
-   **MCP Server**: A Tool Provider running locally or remotely. It handles database integration, file system access, internal API communication, etc.
-   **Transport:** Operates over standard I/O (stdio) or HTTP, performing a role similar to gRPC for microservice communication within the LLM ecosystem.

**Advantages:** Once a tool is built as an MCP Server, it can be immediately connected and used with any LLM or agent framework without modifying the code.

<br>

## Four Principles of Defensive Tool Design

To build a secure Tool Server, the following four elements must be enforced at the schema level:

1.  **Explicit Input/Output Schema (Pydantic):** Strictly define types, default values, and enumerations.
2.  **Helpful Description (Comments for LLM):** Instead of just variable names, the description field should provide clear guidelines that the LLM can understand (e.g., "Date must be in YYYY-MM-DD format.").
3.  **Graceful Error Return (Error Encapsulation):** Instead of throwing an `HTTP 500 Error` on validation failure, return a message like `Error: limit cannot exceed 100. Please reduce the value and try again` to be reflected in the Agent's State.
4.  **Timeout and Sandboxing:** Set strict timeouts for all tool executions to prevent external API calls from being indefinitely delayed.

<br>

## MCP Tool Server Implementation and Deployment

This is an example of implementing a tool to securely query an internal database using the latest FastMCP (MCP Python SDK) and Pydantic.

```python
from mcp.server.fastmcp import FastMCP
from pydantic import BaseModel, Field, field_validator
import asyncio

# 1. Create MCP Server instance (assign a name)
mcp = FastMCP("EnterpriseDataServer")

# 2. Strong defense line: Define Pydantic Schema
class QueryUserArgs(BaseModel):
    # Restrict allowed values via Enum
    department: str = Field(
        ...,
        description="Department name to query. Allowed values: 'engineering', 'sales', 'hr'"
    )
    # Range Bound (min/max limit)
    limit: int = Field(
        default=10,
        ge=1,
        le=100,
        description="Maximum number of users to fetch. Only integers between 1 and 100 are allowed."
    )
    # Additional custom validation logic
    @field_validator('department')
    def check_department(cls, v):
        allowed = ['engineering', 'sales', 'hr']
        if v.lower() not in allowed:
            raise ValueError(f"'{v}' is an unknown department. Choose from {allowed}.")
        return v.lower()

# 3. Register Tool and configure Timeout/Error Handling
@mcp.tool(description="Queries the internal database for a list of users by department.")
async def query_users(args: QueryUserArgs) -> str:
    try:
        # Enforce timeout (e.g., 5 seconds)
        async with asyncio.timeout(5.0):
            # Placeholder for actual DB query logic (simulated)
            await asyncio.sleep(1) # Simulate network latency

            return f"Success: Retrieved {args.limit} user details from the '{args.department}' department."

    except asyncio.TimeoutError:
        # Return error to prompt LLM to retry
        return "Error: Database response timed out. Please simplify your query conditions and try again."
    except Exception as e:
        # Pass system errors to LLM's context without hiding them
        return f"Error: An issue occurred during tool execution. {str(e)}"

# 4. Run server (runs in stdio mode for pipe communication with LLM agent process)
if __name__ == "__main__":
    mcp.run()
```

### Expected Results and LLM's Self-Correction Loop

Suppose the LLM receives an ambiguous request from the user, "Find all engineering team members," and generates the following arguments:

#### First Attempt - Failure

-   **LLM Generated JSON:** `{"department": "engineering", "limit": 9999}`
-   **MCP Server Processing Result:** Pydantic Validator immediately intervenes and blocks the request.
-   **Text Returned to LLM:** `"ValidationError: limit value cannot exceed 100. (Current value: 9999). Please modify the value and call the tool again."`

### Second Attempt - Success

-   **State Transition:** The returned error message is added to the Agent State (`messages`), and the LLM reads it, recognizing its mistake.
-   **LLM Generated JSON:** `{"department": "engineering", "limit": 100}`
-   **MCP Server Processing Result:** Validation passed.
-   **Final Return:** `"Success: Retrieved 100 user details from the 'engineering' department."`

<br>

## Deliverable: Guide to MCP Tool Server with Schema Validation Applied

This is a checklist of security architecture items to review when deploying an MCP Tool Server in a production environment.

| Validation Step | Applied Technology / Stack | Architecture Design Guideline |
|-----------------|----------------------------|-------------------------------|
| Input Argument Validation | Pydantic Field, Validator | LLMs often ignore types. Utilize Pydantic's strong Type Coercion to parse strings where numbers are expected, and always set boundary values like `ge`, `le`. |
| Authorization (AuthZ) | JWT, Context Injection | When calling a tool, build an access control model by passing through the **'current End-User's permissions (Token)'** using the LLM, not the LLM's own permissions, to prevent data leakage. |
| Exception Handling Strategy | Try-Except Wrapping | Prevent HTTP 404, 500 exceptions within the tool from halting the entire system process. Catch all exceptions and convert them into plain text error guidance that the LLM can read, then return it. |
| Communication Protocol | MCP stdio / SSE | For local agents (development environment), use process pipes (stdio). In a distributed cloud environment, build an SSE (Server-Sent Events)-based MCP server behind an API Gateway to establish a fast, streamable communication network similar to gRPC. |
