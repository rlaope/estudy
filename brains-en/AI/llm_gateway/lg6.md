# Processing LLM API Responses

In a multi-tenant B2B AI service, when multiple clients or internal departments call numerous LLM APIs through a single system, how can we accurately re-bill tens of millions of won charged monthly by cloud vendors (OpenAI, vLLM Infra, etc.) to each tenant, proportional to their actual usage, down to the last won? Furthermore, what standardization work is needed to prevent omissions or format errors when this billing data is transferred to a central billing system?

The most crucial backend pipeline responsible for the profitability of LLM services and the control of infrastructure costs is the **Chargeback pipeline**. To implement this, a layer is essential to extract key metrics for billing from unstructured LLM response data and transform them into structured data that the system can reliably process.

- **Chargeback (Internal Billing/Re-billing Model)**: This is a financially engineered process that transparently distributes and settles the total costs borne by a central IT department or platform infrastructure among each tenant (department, client, project) that actually consumed the system, based on their resource utilization. In an LLM environment, tokens become the absolute unit of billing.
- **Metadata Extraction**: The response payload of an LLM API called for text generation includes metadata such as `prompt_tokens`, `completion_tokens`, and `total_tokens` (input, output, and total, respectively) in addition to the text body (`content`) to be shown to the client. The technique of separating and capturing this from the main content is metadata extraction.
- **Schema Normalization**: This is the process of combining fragmented token information with tenant IDs, model IDs, call timestamps, and other server context data, then transforming it into a strictly typed JSON document structure that can be 100% processed without parsing errors by downstream systems (such as data warehouses or Kafka).

<br>

## Problem Definition

When simply extracting token counts from response values and attempting to save them to a database, the following data consistency flaws and exceptional situations arise in real-world architectures.

- **Hidden Metadata in Streaming (Server-Sent Events) Responses**: When LLM responses are configured for streaming mode to reduce perceived client latency, data is fragmented and delivered in chunks. Most LLM providers do not include token usage in intermediate data chunks, but only in a specific field of the final chunk. If this is not captured mid-stream, billing data is permanently lost.
- **Fragmented Response Payloads Across LLM Providers**: Each model provider, such as OpenAI, Anthropic, or internal vLLM servers, uses completely different JSON keys (e.g., `usage`, `token_usage`, `amazon-bedrock-invocationMetrics`) and hierarchical structures to indicate token usage. Failure to map these to a consistent schema will cause failures in the central billing system.
- **Separation of Authentication Context and Response Data**: While token information is present in the LLM's response object, information about which tenant ID made the call resides in the HTTP Request header or `request.state` middleware from the incoming API request. In an asynchronous environment, there are difficulties in state management to accurately combine these two disparate data sources into a single transaction.

### Solution Approach

- **Parsing Abstraction via Adapter Pattern**: Implement an adapter layer that receives raw payloads, which differ by LLM provider, dynamically calls the appropriate parser for the provider type, and returns a standardized token dictionary.
- **Enforcing Strongly-Typed Billing Event Schema with Pydantic**: Instead of simple Python dictionaries, define a `ChargebackEvent` schema using a Pydantic model. If there are missing fields or type mismatches, an error is raised at the instantiation stage, fundamentally preventing corrupted data from entering the billing database.
- **Hooking the Final Chunk using Async Generators**: During the process of proxying streaming responses to the client via `yield`, metadata is extracted the moment the end-of-response (DONE) or the last object is detected. This metadata is then combined with the tenant ID from the Request Context to asynchronously publish a billing event.

<br>

## Detailed Operation Principles and Structuring

This section analyzes the data mapping and processing flow at the application memory and network I/O levels, step by step, without diagrams.

1.  **Context Isolation and Identification**: When a client's API call enters the FastAPI framework, the authentication middleware parses the HTTP header (`Authorization` or `X-API-Key`) to extract the `tenant_id` of the request's principal. This value is loaded into memory in `request.state.tenant_id`, ensuring thread safety for the current request.
2.  **LLM Inference and Raw Metadata Reception**: The controller requests inference from an external LLM API and receives a response. At the very end of this response body, a raw JSON trigger in the form of `{"usage": {"prompt_tokens": 150, "completion_tokens": 50, "total_tokens": 200}}` is included.
3.  **Usage Data Extraction and Adapter Processing**: The business logic separates the actual text message part to be sent to the client from the `usage` metadata node. The separated `usage` node is passed to a predefined parser function and mapped to internal standard system variables (e.g., input_tokens, output_tokens).
4.  **Schema Instantiation**: The `tenant_id` stored in step 1, the `input_tokens` and `output_tokens` obtained in step 3, and the `model_id` (the basis for unit price calculation) retrieved from environment variables are injected into a single Pydantic class constructor. At this point, a creation timestamp and a unique transaction ID are automatically issued and imprinted onto the object.
5.  **JSON Serialization and Message Publishing**: A Pydantic instance that has successfully passed validation is serialized into a byte/string JSON format via the `.model_dump_json()` method. This string is then securely transmitted to the persistence layer via a Kafka topic or Redis stream subscribed to by the billing system, or through asynchronous DB insertion via BackgroundTasks.

### Example

This illustrates the basic principle of assembling various context variables into a structured billing dictionary and converting it into a JSON string.

```py
import json
from datetime import datetime
import uuid

# 1. Request Context stored after extraction by middleware, etc.
current_tenant_id = "tenant-marketing-01"
current_model = "gpt-4-turbo"

# 2. Hypothetical raw response JSON received from an external LLM API (e.g., OpenAI)
llm_raw_response = {
    "id": "chatcmpl-123",
    "choices": [{"message": {"content": "Hello! I am an AI."}}],
    "usage": {
        "prompt_tokens": 10,
        "completion_tokens": 7,
        "total_tokens": 17
    }
}

# 3. Token extraction and Chargeback schema assembly via tenant mapping
def generate_chargeback_event(tenant_id: str, model_id: str, raw_response: dict) -> str:
    # Safe extraction (prevents KeyError)
    usage = raw_response.get("usage", {})
    
    chargeback_data = {
        "event_id": str(uuid.uuid4()),
        "timestamp": datetime.utcnow().isoformat() + "Z",
        "tenant_id": tenant_id,
        "model_id": model_id,
        "metrics": {
            "input_tokens": usage.get("prompt_tokens", 0),
            "output_tokens": usage.get("completion_tokens", 0),
            "total_tokens": usage.get("total_tokens", 0)
        }
    }
    
    # 4. Serialization into JSON format for downstream systems
    return json.dumps(chargeback_data)

# Execution result
chargeback_json = generate_chargeback_event(current_tenant_id, current_model, llm_raw_response)
print(chargeback_json)
```

The above is extraction. Assuming a FastAPI environment, let's look at the structure that retrieves tenant information from Request State and creates an event object through strict schema validation using Pydantic and dependency injection.

```py
import uuid
from datetime import datetime, timezone
from pydantic import BaseModel, Field
from fastapi import FastAPI, Request, BackgroundTasks

app = FastAPI()

# 1. Define a strict Pydantic schema agreed upon with the central billing system (Billing DB/Kafka)
class TokenMetrics(BaseModel):
    input_tokens: int = Field(..., ge=0, description="Number of prompt tokens")
    output_tokens: int = Field(..., ge=0, description="Number of generated tokens")
    total_tokens: int = Field(..., ge=0, description="Total number of tokens")

class ChargebackEvent(BaseModel):
    event_id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    timestamp: str = Field(default_factory=lambda: datetime.now(timezone.utc).isoformat())
    tenant_id: str = Field(..., description="ID of the department or client to be billed")
    model_id: str = Field(..., description="Name of the LLM model used (for unit price mapping)")
    metrics: TokenMetrics

# 2. Hypothetical worker to send billing events to an external system (e.g., Kafka, DB) in an async environment
async def publish_chargeback_event(event_json: str):
    # In a real environment, use aiokafka, boto3 (Kinesis), etc., for transmission
    print(f"[Billing Pipeline] Transmission complete: {event_json}")

# 3. Main endpoint
@app.post("/v1/completions")
async def generate_text(request: Request, bg_tasks: BackgroundTasks):
    # A. Obtain tenant identifier injected into request.state by middleware
    # (In practice, this would be after passing through Header or JWT parsing middleware)
    tenant_id = getattr(request.state, "tenant_id", "default_untracked_tenant")
    target_model = "claude-3-opus-20240229"
    
    # B. Simulate LLM call and response reception (actual is async client call to Anthropic/OpenAI)
    mock_llm_response = {
        "content": "This is billing pipeline data at a practical level.",
        "usage": {"input_tokens": 105, "output_tokens": 45} # Anthropic style
    }
    
    # C. Provider-independent parsing and schema instantiation
    try:
        # Runtime validation via Pydantic model to check for missing fields or type mismatches
        chargeback_event = ChargebackEvent(
            tenant_id=tenant_id,
            model_id=target_model,
            metrics=TokenMetrics(
                input_tokens=mock_llm_response["usage"].get("input_tokens", 0),
                output_tokens=mock_llm_response["usage"].get("output_tokens", 0),
                total_tokens=(
                    mock_llm_response["usage"].get("input_tokens", 0) + 
                    mock_llm_response["usage"].get("output_tokens", 0)
                )
            )
        )
        
        # Serialize the Pydantic object into a final JSON string
        validated_json_payload = chargeback_event.model_dump_json()
        
        # D. Pass to BackgroundTasks for asynchronous publishing to avoid slowing down user response
        bg_tasks.add_task(publish_chargeback_event, validated_json_payload)
        
    except Exception as e:
        # Alarm logic (e.g., Sentry) if chargeback object creation fails (e.g., data omission)
        print(f"[Error] Chargeback creation failed: {str(e)}")

    # E. Return only the text cleanly to the client
    return {"message": mock_llm_response["content"]}
```
