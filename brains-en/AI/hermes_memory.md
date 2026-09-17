# Hermes Agent, Agent Memory Layer

> Target: Hermes Agent (NousResearch) native memory + the extension layer added by oh-my-hermes (OMH)
As of: 2026-08-01
>

---

## 0. Why Divide Memory into "Layers"?

The information an agent needs to remember varies greatly in nature. The fact that "this person prefers polite language" is useful only if it's in the prompt every turn, while the fact that "a Redis connection pool issue was fixed this way three Thursdays ago" is only needed when that topic comes up. If you put the former in storage, it will never be retrieved; if you put the latter in the prompt, the context will explode within days.

Therefore, we use two axes:

- **Residency**: Is it always in the prompt, or read only when requested?
- **Volatility**: Does it rarely change, or accumulate every turn?

Dividing along these two axes results in four stages: L0 to L3. The top (L0) is small, unchanging, and always active, while moving down, it gets larger, changes more frequently, and is retrieved only when needed.

---

## 1. What Does the Overall Picture Look Like?

| Layer | Name | Owner | Storage Medium | Size | Prompt Residency | Writer |
| --- | --- | --- | --- | --- | --- | --- |
| **L0** | soul / persona | Human (by convention) | Config file | Very small | Always | Human only |
| **L1** | `MEMORY.md` + `USER.md` | Agent (curated) | 2 Markdown files | Hard cap (default 2200 chars / 1375 chars) | Always (snapshot at session start) | Agent |
| **L2** | memory blocks (OMH) | OMH + Reviewer | `.omh/memory/*.json` | Per-block limit, 2-tier | Labels only resident, body on-demand | OMH capture → Human approval |
| **L3** | session store | Runtime | SQLite FTS5 (`~/.hermes/state.db`) | Unlimited | None | Automatic (raw logs) |

The render pipeline looks like this every turn:

```
[L0 soul]  ─┐
[L1 core]  ─┼→ System Prompt (rendered within a total 6000-char budget)
[L2 labels] ┘        ↑
                  If overflow, display overflow indicator (no silent truncation)

[L2 block body] ← Read when needed, based on labels
[L3 raw session] ← session_search query or provider prefetch
```

The key is that the **budget applies only to the prompt side**. L3 incurs zero prompt cost no matter how large it gets, while L1 incurs a per-turn cost even if it grows by just 1KB.

---

## 2. Part-Timer Analogy

Let's imagine a new part-timer (agent) has joined a cafe.

**L0 — The cafe's character set by the owner on day one**
"We are a quiet neighborhood cafe, and we don't initiate conversation with customers." The part-timer cannot change this, and it's ingrained in their mind throughout their shift. Only the owner (human) can modify it.

**L1 — A single laminated A4 sheet taped next to the cash register**
"Espresso machine group head #2 pressure abnormal", "Regular Mr. Kim prefers less ice". It's visible with a glance and automatically reflected in every customer interaction. However, **it cannot exceed one A4 sheet.** If it's full and they try to add a new note, instead of secretly erasing an old one, it refuses, saying, "No space, please tidy up first." And because this A4 is **ingrained by taking a photo of it at the start of the shift**, any new notes written during the shift remain in the file but don't enter the mind's "photo" for today. They will be reflected in the next shift.

**L2 — A filing cabinet with labels**
Only labels like "Coffee Bean Ordering Procedure", "POS Error Handling", "Last Month's Store Policy Changes" are visible on the drawer fronts. The part-timer always knows the list of drawers and opens only the relevant one when needed. The amount that can be put into a single drawer is also fixed. Newly learned information first goes into an "Unverified Box," and only after the manager reviews it is it moved to a formal drawer.

**L3 — All the boxes of receipts and work logs piled up in the storage room**
Nothing is thrown away. However, it's impossible to read everything at once, so they search by keywords like "milk delivery delayed in June" and retrieve only the relevant sections.

The rules that naturally emerge from this analogy align perfectly with the actual design rules:

- Using an A4 sheet instead of a drawer reduces accuracy. → L1 should not be used as a work log.
- Transcribing warehouse receipts onto an A4 sheet might lead to much lost information. → L3 raw data is not promoted to L1.
- If drawer labels are poor, the correct drawer cannot be opened. → L2's performance depends on label quality (recall performance is crucial).

---

## 3. L0 — soul / persona

**What is it?** The agent's identity, tone of voice, role boundaries, and red lines. In Hermes, this corresponds to the persona/system instruction area.

**Why keep it separate?** If this layer can be modified by the agent itself, its identity will gradually drift over long sessions. Therefore, the OMH stack designates it as **"human owned by convention"** — a layer that, by convention, only humans touch. It's important that this is enforced by operational convention, not by code.

**Engineering points.**

- Since it rarely changes, it's ideal to place it at the very beginning of the prompt cache prefix.
- It is copied identically to sub-agents.
- Keep its size to a few hundred tokens. If this gets larger, the budget for all three lower layers will be reduced.

---

## 4. L1 — `MEMORY.md` + `USER.md` (Core Memory)

**What is it?** This is Hermes' native curated memory. It exists as two Markdown files under `$HERMES_HOME/memories/` per profile.

- `MEMORY.md` — Agent notes: environmental facts, project conventions, tool specifics, stable lessons learned. Default limit 2200 characters (approx. 800 tokens)
- `USER.md` — User profile: preferences, communication style. Default limit 1375 characters (approx. 500 tokens)

Settings are adjusted in `~/.hermes/config.yaml`.

```yaml
memory:
  memory_enabled: true
  user_profile_enabled: true
  memory_char_limit: 2200
  user_char_limit: 1375
  write_approval: false   # if true, requires approval for writing
```

**Operational characteristics (this is important).**

1.  **Frozen snapshot injection.** It is injected entirely into the system prompt at the start of a session. Content written during the session is immediately saved to disk, but the active prompt remains unchanged until the next session or prompt rebuild. This property ensures the prompt prefix remains fixed throughout the session, enabling **prompt caching.**
2.  **No read action.** Memory tools only support add / replace / remove. Since it's already in the prompt, there's no reason to read it.
3.  **Write refusal instead of automatic compression on hard cap exceedance.** This is a point emphasized by the OMH stack. Automatic summarization silently burns information, and no one knows which sentences disappeared. Refusal forces a human or a higher-level workflow to take explicit action, i.e., "cleanup".
4.  **The entry delimiter is `§`**, and before writing to disk, `_scan_memory_content` strictly checks for prompt injection, role hijacking, and leakage patterns.

**What should not be put here.** In-progress task status, ticket numbers, temporary decisions, logs. L1 breaks the moment it becomes a work log.

---

## 5. L2 — memory blocks (Layer added by OMH)

**What is it?** Reviewed long-term context in labeled blocks. It's not present in Hermes native and is filled in by OMH.

**Structure (based on OMH project memory).**

```
.omh/memory/
  candidates/*.json   # captured but not yet reviewed
  records/*.json      # approved formal records
  reviews/*.json      # approval/rejection decision records
  index.json          # local file inventory
```

Record types are fixed at five:

| Type | Meaning | Example |
| --- | --- | --- |
| `fact` | Unchanging fact | "This repo targets Python 3.11+" |
| `decision` | Decision made and its reason | "Authentication will use session cookies, JWT deprecated" |
| `lesson` | Lesson learned from failure | "This migration won't fail if indexes are created first" |
| `procedure` | Reproducible procedure | "Run unittest discovery after workflow contract changes" |
| `episode` | Specific event | "Aug 1st deployment rollback incident" — short default TTL |

**Two Tiers.** Blocks are divided into two stages. One tier is always rendered, and the other exposes only labels, with the body read on-demand. It is rendered within a 6000-character budget per turn, and if the budget is exceeded, **overflow is explicitly indicated.** There is no silent truncation.

**Safety Rules.** Capture does not store the original text. Only hash, length, typed summary, and review metadata are retained. A local safety classifier blocks or forces review for the following:

- Strings resembling credentials
- Raw logs and tracebacks
- Full conversation transcripts
- Short-lived PR/commit identifiers
- Temporary work in progress
- Abnormally long original texts

Blocked candidates cannot be approved at all. They must be re-captured as a safe and bounded summary, or rejected.

**Policy Modes.** `omh setup` writes `project_memory_policy/v1` to `.omh/setup-profile.json`.

```bash
omh setup --memory-mode review-first   # default: recall only after capture and review approval
omh setup --memory-mode auto-safe      # automatically approve safe items, review only risky ones
omh setup --memory-mode off            # disable automatic capture and recall
```

**CLI Flow.**

```bash
omh memory capture --type procedure --tag tests "Run unittest discovery after workflow contract changes"
omh memory review
omh memory approve cand_1234 --approved-by user
omh memory reject  cand_1234 --reason "Temporary work in progress"
omh memory recall  --executor codex "workflow docs verification"
omh memory status
```

---

## 6. L3 — session store (FTS5)

**What is it?** All CLI and messenger sessions are stored verbatim in SQLite (`~/.hermes/state.db`), with an FTS5 full-text index attached. It uses two indexes: `messages_fts` and `messages_fts_trigram`.

**If you use Korean, you must check this.** FTS5 tokenizes words by whitespace by default. Since Korean, Chinese, and Japanese do not, Hermes includes a native CJK tokenizer extension in `native/fts5_cjk/` and configures `messages_fts` to use it. If this extension is not loaded, Korean session searches will silently return empty results. This is the first item to check when building your own L3.

**Access Methods.** The `session_search` tool has three invocation forms:

- **discovery** — Find which session contained a keyword
- **scroll** — Move forwards and backwards within a found session (`session_id` + `around_message_id`)
- **browse** — Browse a session

Search results return the actual messages from the DB verbatim. No LLM summarization, no truncation.

**Use with awareness of limitations.** FTS5 is based on **token matching**. If a past session contains "authentication microservice uses Redis," querying "What was said about the auth service?" might not yield results. If semantic retrieval is needed, L2 blocks or external vector providers must bridge that gap.

Furthermore, **the agent must decide to invoke a search.** To reduce this decision burden, OMH attaches to the memory-provider seam to **prefetch recalls, ensuring they arrive first.** This structure means relevant context is already present, even if the agent doesn't explicitly think, "I should look this up."

---

## 7. Glossary

| Term | Meaning |
| --- | --- |
| **core memory** | Small amount of memory always resident in the prompt. Here, L1. |
| **frozen snapshot** | A prompt fragment fixedly injected at session start and unchanging throughout the session. |
| **prompt caching** | Optimization that skips re-computation when the beginning of the prompt is identical. Requires L0/L1 to be stable. |
| **residency** | Whether resident. Is it always in the prompt, or read on request? |
| **memory provider seam** | An extension point opened by Hermes to plug in external memory implementations (`agent/memory_provider.py`, `MemoryManager` orchestrates, only 1 active at a time). |
| **prefetch** | Retrieving relevant memory in advance, just before the response stage, before the agent requests it. |
| **consolidation** | The process of promoting multiple items from a lower layer into a single summarized block in an upper layer. |
| **eviction** | Removing items when the budget is exceeded. |
| **compaction** | A runtime operation that summarizes and reduces conversation history when context is full. |
| **TTL / staleness** | Record validity period and freshness metadata. Expired items are excluded from recall. |
| **source priority** | The ranking of which source to trust when multiple sources exist for the same topic. |
| **prepared ≠ observed** | The principle of never mixing prepared context with actually observed execution evidence. |
| **recall pack** | A compressed memory bundle attached to coding handoffs (`memory_recall_pack/v1`). |
| **context pack** | Conflict-free, metadata-only context (`handoff_context_pack/v1`). |

---

## 8. Confusion with the "L0~L3" Label

Within the same ecosystem, L numbers are used with **three different meanings.** It is crucial to distinguish them when reading documentation.

**(A) Residency Layers — What this note covers**
L0 Persona → L1 Core → L2 Blocks → L3 Raw Sessions.  
As the number increases, the **volume grows, and residency decreases.**

**(B) Resolution Layers — OpenViking approach**
Stores an item at three resolutions. L0 is a one-sentence summary (approx. 50-100 tokens), L1 is core information and usage scenarios (approx. 500-2k tokens), and L2 is the full original text. The agent reads from L0 and moves up only when necessary. Claims 80-90% token savings compared to loading the entire context every turn, based on published benchmarks.  
As the number increases, the **same item becomes more detailed.**

**(C) Abstraction Ladder — TencentDB Agent Memory approach**
L0 Raw Conversation → (LLM extraction every N turns) L1 Atomic Facts → (Synthesis every 50 items) L2 Scenarios → L3 Persona.  
As the number increases, it becomes **more abstract.** The direction is opposite to (A).

Since all three schemes refer to "L0~L3," it is safer to include the axis name when writing design documents. E.g., `L2 (residency tier)`, `L1 (resolution tier)`.

---

## 9. Representative Stacks

**Implementation Candidates per Layer**

| Layer | Implementation in this stack | Other options filling the same role |
| --- | --- | --- |
| L0 | Config file-based persona | System prompt templates, Letta's persona blocks |
| L1 | 2 Markdown files + character limit | Letta/MemGPT core memory blocks, `CLAUDE.md`-family project instruction files |
| L2 | OMH `.omh/memory/` JSON records | Mem0, Zep/Graphiti (temporal knowledge graph), Cognee, Letta archival memory, ByteRover (Markdown knowledge tree) |
| L3 | SQLite FTS5 | pgvector/Postgres, Qdrant·Weaviate·LanceDB, BM25+vector hybrid + reranker |

**External Providers in the Hermes Ecosystem (as of April 2026, 8 types)**
OpenViking (`viking://` scheme, L0/L1/L2 tier loading, AGPL, self-hosted), Mem0 (platform/self-hosted/OSS 3 modes), Hindsight (LongMemEval 94.6%, local PostgreSQL), Honcho (user modeling), Holographic (pure SQLite, HRR algebraic queries), RetainDB (vector+BM25+reranking), ByteRover, agentmemory, etc.  
Only **one** external provider can be active at a time. The built-in L1 continues to operate independently, and external providers are additional layers.

**OMH's Current Backend Stance**
The v1 policy specifies `local_json` as the current backend, leaving Mem0, Graphiti, Cognee, Letta, etc., as **extension seams that can be attached as optional adapters.** A condition is attached: they are only added when dependency, privacy, and packaging boundaries are explicit.

---

## 10. How Should Layers Interact?

### 10-1. Read Path (Every Turn)

```
Turn Start
 ├ L0 Render (fixed)
 ├ L1 Render (session start snapshot, immutable within session)
 ├ L2 Label List Render + Always-on Tier Block Render
 │    └ 6000-char budget exceedance → overflow marking
 ├ provider prefetch → Pre-pull relevant L2/L3 candidates
 └ Model decides to read L2 block / L3 session_search
```

**Three Design Principles.**

1.  **The higher the layer, the narrower and quieter.** If you're debating what to put in L1, it's generally better not to.
2.  **Labels are the interface.** Since the decision to open an L2 block is based solely on its label, the label should convey "when to open it," not just a "title." "Order to check during deployment rollback" is better than "Deployment Procedure."
3.  **Overflow must be visible.** If the fact of truncation is hidden, the agent will mistake incomplete information for complete information.

### 10-2. Write Path (Promotion)

```
L3 Raw Logs
  │ (Consolidation upon turn count reached or compaction observed)
  ↓
L2 candidate  ──Safety Classifier──→ Block / Review Needed / Pass
  │ (Approved by human or auto-safe policy)
  ↓
L2 record (type + TTL + staleness)
  │ (If repeatedly referenced and project-wide)
  ↓
L1 (Human explicitly moves, after securing space)
```

**Do not use time as a promotion trigger.** The OMH stack triggers consolidation on **turn count** or **observed compaction events.** Since compaction means the original data will disappear, the moment just before it is the last chance for promotion. The problem of early decisions being lost later in a 10-hour long loop is precisely a design issue at this point. If an early `decision` record is promoted to L2 before compaction, it can be re-read by its label in later turns.

### 10-3. Eviction

**We do not use oldest-first.** There's no guarantee that older items are less important, and early project architectural decisions are often the oldest yet most enduringly valid.

Instead, only **provable redundancy** is accepted as a basis for eviction. That is, an item is deleted only when it can be shown that "the content of this item is fully contained within another item." Due to this rule, entries do not have timestamps for ordering.

TTL is a separate axis. Records with inherently short lifespans, like `episode` types, have a TTL attached and are excluded from recall when expired. Since **"deleted due to age" and "expired due to a defined lifespan" are different rules,** it's better to separate them in implementation.

### 10-4. Conflict Resolution (Source Priority)

Conflict review is schematized as a separate surface. This design aims to allow users to check "Is what I remember still valid?" via cards, without directly opening files.

| Schema | Purpose |
| --- | --- |
| `memory_snapshot/v1` | Context sources provided by the wrapper or found locally by OMH |
| `memory_inspection/v1` | Source inventory, conflict detection, review items, preview |
| `memory_review_card/v1` | Review UI rendered by the wrapper (separated from `status_card/v1`) |
| `memory_update_batch/v1` | User-approved keep / forget / update / scope decisions |
| `handoff_context_pack/v1` | Conflict-free, metadata-only context attached to executor handoffs |

If multiple sources exist for the same topic, OMH trusts them in the following order:

1.  Runtime evidence from run-ledger artifacts
2.  Wrapper session state
3.  Runtime state index
4.  Target topology
5.  Setup profile
6.  Approved OMH memory
7.  Wiki / Notes
8.  Catalog hints
9.  Wrapper snapshot candidates

Higher-priority sources can **block** outdated assumptions from lower-priority sources. And if conflicts remain, context is not attached to the handoff; instead, `context_pack_blocked` is recorded along with the conflict list. This mechanism prevents outdated context from mixing into the executor's prompt and becoming false evidence.

### 10-5. Write Permissions in Multi-Agent Systems

**Sub-agents read but do not write.** If parallel sub-agents each start writing to L1/L2, issues like unordered merges, mutual contradictions, and a sub-agent's local misconception solidifying into a global fact will simultaneously arise. Writes are centralized to a single orchestrator.

### 10-6. Prepared ≠ Observed

Having "decided to do this" in memory is a completely different fact from "it was actually executed that way." OMH explicitly states in all review and recall payloads that this is merely prepared context and not evidence from execution, review, CI, merge, or Hermes internal memory.

If this distinction is not enforced at the schema level when building a memory system, within a few weeks, the agent will start reporting planned tasks as completed ones.

---

## 11. Items to Decide During Design (Checklist)

- [ ] What character limit to set for L1, and whether to refuse or compress upon exceedance
- [ ] The limit for a single L2 block and the criteria for splitting between always-on tier / on-demand tier
- [ ] Total render budget per turn (6000 characters here)
- [ ] Consolidation trigger: turn count threshold, compaction observation hook
- [ ] Eviction criteria: method of proving redundancy (hash containment? semantic similarity threshold?)
- [ ] Default TTL: should it vary by type?
- [ ] Approval policy: default among review-first / auto-safe / off
- [ ] List of patterns to be blocked by the safety classifier
- [ ] Source priority order and blocking policy upon conflict
- [ ] Sub-agent write permissions (default is forbidden)
- [ ] Is L3 retrieval keyword-only, or will semantic search be added?

---

## 12. Common Failure Modes

| Symptom | Cause | Mitigation |
| --- | --- | --- |
| L1 full of clutter after a few days | Writing progress to L1 | Enforce types (`fact`/`decision`/`lesson`/`procedure`), block progress at capture stage |
| Contradictory modifications to early decisions in the latter half of a long-running loop | Loss of early decisions due to compaction | Register compaction observation as a consolidation trigger, promote early `decision` to L2 |
| Agent doesn't use memory despite it being present | Poor L2 labels or failed recall invocation decision | Rewrite labels to indicate "when to open," remove decision burden with prefetch |
| Past information not searchable | FTS5 token mismatch | Synonym expansion, include search keywords in L2 summaries, use semantic search provider in parallel |
| Agent reports plans as completed | Prepared / observed not separated | Separate at schema level, explicitly define evidence boundaries in handoffs |
| Prompt cache not hit | Upper layers fluctuate every turn | Fix L1 as a session start snapshot, reflect in-session writes only to disk |
| Unapproved context leaks into executor prompt | Attaching pack without conflict check | Attach only when `blocked_by_conflicts` is empty, otherwise `context_pack_blocked` |

---

## 13. References

- Hermes Agent — Persistent Memory: https://hermes-agent.nousresearch.com/docs/user-guide/features/memory/
- Hermes Agent — Memory Providers: https://hermes-agent.nousresearch.com/docs/user-guide/features/memory-providers
- Hermes Agent — memory.md (소스): https://github.com/NousResearch/hermes-agent/blob/main/website/docs/user-guide/features/memory.md
- DeepWiki — Memory and Sessions: https://deepwiki.com/NousResearch/hermes-agent/4.3-memory-and-sessions
- OMH — Project Memory: https://github.com/rlaope/oh-my-hermes/blob/main/docs/MEMORY.md
- OMH — Memory Context Review: https://github.com/rlaope/oh-my-hermes/blob/main/docs/MEMORY_CONTEXT.md
- Hermes 메모리 provider 비교: https://vectorize.io/articles/hermes-agent-memory-providers-compared
