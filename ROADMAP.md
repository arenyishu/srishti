# Srishti Roadmap

## Vision

Srishti is an **AI Operating System (AI-OS)** designed from the ground up to manage, execute, and coordinate AI agents as native system processes.

The long-term vision is to become the foundational runtime kernel for AI-native software systems — enabling developers to build, govern, deploy, and scale AI agents as naturally and securely as traditional software.

---

## The Foundation (Completed) ✅

### v0.1.0 to v0.3.0 – The Kernel & Runtime Foundation
Srishti has successfully transitioned from an experimental compiler into a functional executable OS layer.

* **Compiler & Toolchain:** Full AOPL Compiler, Lexer, Parser, and CLI (`srishti run`, `check`, `build`).
* **Kernel & Process Supervisor:** Agent Lifecycle Management and Process Table tracking.
* **Inter-Process Communication (IPC):** Event Bus Architecture and dynamic event routing.
* **Networking:** Basic RPC Networking and Agent Registry for service discovery.
* **Basic Memory:** Foundation for memory allocation and state management.
* **Telemetry:** Real-time Dashboard server and internal telemetry APIs.

---

## The Future: OS Architecture Roadmap

### v0.4.0 – Security Ring & Governance (Sankalp) 🚧 *In Progress*
*Transforming the basic policy engine into a robust OS security layer.*

* **Kernel-Level Sandboxing:** Hard limits and quotas on what agents can execute.
* **Approval Store:** Human-in-the-loop intercepts and workflow halting for privileged actions.
* **Audit Subsystem:** Immutable, append-only logging of all OS-level state changes and agent executions.
* **Access Control:** Strict Agent-to-Agent permission scopes and Identity management.

### v0.5.0 – Semantic CPU & Execution (Shakti)
*Making the OS natively intelligent.*

* **Pluggable LLM Drivers:** Native OS support for OpenAI, Ollama, Anthropic, and Gemini.
* **Hardware Abstraction:** Switch between cloud AI APIs and local on-device models seamlessly.
* **Semantic Scheduler:** OS-level planning, reasoning, and context management.
* **Structured Generation:** Guaranteed JSON outputs and typed schema enforcement.

### v0.6.0 – Advanced Memory Management (Pragati)
*Evolving from basic state arrays to true OS-level memory architecture.*

* **Memory Paging & Swap:** Archiving old context to disk (SQLite/PostgreSQL) when RAM is full.
* **Vector Memory Integration:** Built-in embeddings and similarity search without manual DB wiring.
* **Shared Memory Spaces:** Multiple agents reading and writing from the same allocated context block safely.
* **Memory Encryption:** Securing sensitive agent states and isolated memory per process.

### v0.7.0 – Distributed Computing (Udaan)
*Scaling the OS across multiple machines and clusters.*

* **Advanced Remote Procedure Calls (RPC):** Agents talking seamlessly across different physical servers.
* **Cluster Load Balancing:** Intelligently distributing agent workloads across the node cluster.
* **Fault Tolerance & State Replication:** If a node dies, the OS Supervisor restarts the agent on a healthy node with intact state.

### v0.8.0 – Developer Tooling & Ecosystem (Siddhi)
*Building out the User Space tools for developers.*

* **Package Manager:** `srishti install` for third-party agents and dependency resolution.
* **Language Server (LSP):** Official VS Code extensions, syntax highlighting, and inline diagnostics.
* **System Monitoring:** Top-like CLI commands (`srishti top`) to monitor CPU/Token/Memory usage in real time.
* **Developer Playground:** Web-based simulator and formatting tools (`srishti fmt`).

### v0.9.0 – Production Hardening (Moksha)
*Preparing the OS for enterprise workloads.*

* **Performance Tuning:** Deep Rust compiler optimizations and lock-free concurrency.
* **Security Auditing:** Penetration testing and vulnerability patches.
* **Deployment Tooling:** Official Docker/Kubernetes containerization support for the Srishti Kernel.

### v1.0.0 – Srishti The AI Kernel
*The golden release.*

* **Production Runtime:** Long-Term Support (LTS) release of the Srishti OS.
* **Stable Specification:** Guaranteed backward compatibility for AOPL.
* **Enterprise Features:** Multi-organization agent management and compliance packs.

---

## Long-Term Vision

Create the definitive operating system layer for AI, ending the era of fragmented python scripts and API chains. Srishti will allow developers and organizations to govern, deploy, and scale intelligent systems on a unified, high-performance runtime kernel.
