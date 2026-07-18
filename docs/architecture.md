# Srishti OS Architecture

## Core Philosophy

Srishti OS is built on a few unshakeable pillars:
1. **Compilation over Interpretation:** Agents are compiled via `srishti-compiler` into an AST, and strongly typechecked.
2. **Actor Model Concurrency:** Agents are isolated processes managed by `srishti-runtime`.
3. **Decentralized Event Bus:** No direct function calls. Agents communicate purely via pub/sub `EventBus` payloads.
4. **First-class Governance:** LLMs cannot be trusted. Srishti uses an immutable `PolicyEngine` to intercept violations and pause agents via the `ApprovalStore`.

## Agent Explorer

![Agent Explorer Introspection](C:/Users/UserPC/.gemini/antigravity/brain/907a7b91-ac01-4e5f-90ce-5cd643ab3b1e/srishti_agent_explorer_1781950520315.png)

When debugging a system, developers can utilize the Agent Explorer to peer inside an agent's runtime envelope. This exposes:
- **Permissions:** Which IO tools an agent holds (e.g. `payment.read`, `network.connect`).
- **Memory Partitions:** The sizes and counts of isolated Vector Store segments.
- **Current Intent:** The exact semantic goal currently evaluated by the `SemanticEngine`.

## Workflow Visualizer

![Workflow Visualizer Flow](C:/Users/UserPC/.gemini/antigravity/brain/907a7b91-ac01-4e5f-90ce-5cd643ab3b1e/srishti_workflow_visualizer_1781950530782.png)

The `srishti dashboard` maps out dynamic EventBus propagations as visual DAG nodes. If a `RefundAgent` triggers a `FinancialTransactionLimit` policy, the workflow visualizer immediately flags the node as **Suspended**, awaiting a Human-in-the-Loop approval to resume execution.
