# The Srishti Control Plane

The Control Plane (`srishti dashboard`) is the nervous system of the Srishti OS.

![Dashboard Main Overview](C:/Users/UserPC/.gemini/antigravity/brain/907a7b91-ac01-4e5f-90ce-5cd643ab3b1e/srishti_dashboard_hero_1781950506292.png)

## Technology Stack
- **Backend:** Axum (Rust) streaming `tokio` channels over HTTP Server-Sent Events (SSE).
- **State Store:** `srishti-runtime` provides `Arc<RwLock<ProcessTable>>` natively bridging the OS to the HTTP layer.
- **Frontend:** React + Vite + Tailwind CSS + Lucide Icons + Recharts.

## Key Features

1. **Agent Status Table:** Monitors the PID, version, and hardware allocation of thousands of distributed agents.
2. **Policy Center:** Aggregates and enforces `Guardrail` directives injected at compile-time.
3. **Approval Center:** Acts as the Human-in-the-Loop gateway. If an agent hits a threshold, it sleeps. A human clicking "Approve" here flips the `ApprovalStore` state, triggering a wakeup broadcast in the OS.
4. **Audit Log:** Irrefutable, append-only logs of every `SemanticEngine` thought, LLM payload, and IO boundary crossing.
