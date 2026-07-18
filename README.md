# Srishti

## The Operating System for AI Agents

Srishti is an **AI Operating System (AI-OS)** designed from the ground up to manage, execute, and coordinate AI agents as native system processes. 

Instead of treating agents as flimsy abstractions running inside heavy Python scripts or API chains, Srishti makes **Agents first-class OS primitives**. It provides the kernel, memory management, process supervision, and networking layers needed to build reliable, auditable, and scalable AI systems.

Just as Linux abstracts hardware for traditional software, **Srishti abstracts the complexities of AI execution** so developers can focus purely on writing intelligent agent logic.

---

## Why an OS for AI?

Modern AI applications are typically built using fragile prompt chains, external orchestration frameworks (like LangChain), and disconnected databases. 

Srishti replaces this chaos with a robust, platform-first approach. By providing its own compiler, runtime execution engine, memory allocation, and process supervisor, Srishti acts as the foundational operating layer. 

### Core OS Architecture

Srishti is built on the exact same architectural principles as traditional operating systems, but engineered for AI:

* 🧠 **Kernel & Process Supervisor:** Agents run as supervised processes tracked within a dedicated `Process Table`. The Kernel handles their lifecycles, execution states, and resource allocations.
* 💾 **Native Memory Management:** No more manual vector DB wiring. Memory is allocated, tracked, and managed natively by the OS runtime.
* 🌐 **Inter-Process Communication (IPC):** Agents communicate securely through a highly optimized Event Bus, gRPC networking, and a centralized `Agent Registry`.
* 🛡️ **Governance & Security Ring:** A built-in `Policy Engine` and `Approval Store` act as the system's security and permissions layer, ensuring agents never execute unauthorized, hallucinated, or dangerous actions.
* 🖥️ **Distributed Clustering:** Native `Cluster Management` and `State Replication` allow your AI OS to scale across multiple physical nodes seamlessly.
* 📜 **AOPL (Agent-Oriented Programming Language):** Srishti comes with its own compiled language and toolchain tailored specifically for writing semantic and deterministic agent behaviors.

---

## Current Status

**Version:** Srishti v0.3.0 – Prarambh  
*"The Beginning of Reliable AI Systems"*

Prarambh represents the transition of Srishti from an experimental compiler into a fully executable AI Operating System. 

Current OS capabilities include:
* Full AOPL Compiler & Interpreter
* Srishti Kernel & Runtime Execution Engine
* Agent Lifecycle Management & Process Table
* Event-Driven IPC & RPC Networking
* Built-in Semantic Engines (OpenAI, Ollama foundations)
* Multi-Agent Workflows & Clustering

*Srishti is currently an experimental research platform under active development.*

---

## Installation

1. Ensure you have [Rust and Cargo](https://rustup.rs/) installed.
2. Install `protoc` (Protocol Buffers Compiler), as it is required for the OS networking layer. You can download it from [the official repository](https://github.com/protocolbuffers/protobuf/releases) or use a package manager. (Make sure it is in your system `PATH` or set the `PROTOC` environment variable).
3. Clone the repository and compile the Srishti OS:
   ```bash
   git clone https://github.com/arenyishu/srishti.git
   cd srishti
   cargo build --release
   ```

## Usage

You can interact with the Srishti OS using its built-in CLI tool.

```bash
# Check the status of your Srishti project code
cargo run --bin srishti -- check

# (Other CLI commands for deployment, process monitoring, and execution will be documented here as the OS evolves)
```
