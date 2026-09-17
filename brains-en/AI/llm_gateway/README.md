# Implementing LLM Gateway and Multi-Tenant Environment Architecture

A space to learn the following:

- Configure a LiteLLM proxy server to provide a single, unified interface between OpenAI API and vLLM, and implement fallback routing rules
- Develop an authorization logic that logically separates tenant (departmental) requests through FastAPI middleware and API Key Header validation
- Implement Token Bucket-based Rate Limiting by writing a Lua script that atomically executes Redis's INCRBY and EXPIRE commands
- Implement defensive logic to block traffic surges (Noisy Neighbor) from specific departments at the API frontend using HTTP 429 status code and Retry-After header
- Implement an asynchronous observability pipeline that logs prompt token usage without user response delay by integrating FastAPI's BackgroundTasks with the Langfuse SDK
- Write logic to extract input/output token amounts from LLM API response metadata, map them to tenant IDs, and process them into a chargeback JSON schema
