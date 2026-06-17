# Srishti

### Agent-Oriented Programming Language (AOPL)

Srishti is an experimental Agent-Oriented Programming Language (AOPL) designed to build reliable, auditable, and scalable AI systems.

Instead of treating agents as runtime abstractions inside frameworks, Srishti makes **Agents first-class language primitives** and compiles them into deterministic Rust systems while providing semantic execution capabilities through LLM-powered runtimes.

---

## Vision

Modern AI applications are commonly built using:

* Prompt Chains
* Agent Frameworks
* Workflow Graphs
* Orchestration Layers

While powerful, these approaches often lack:

* Compile-time validation
* Strong typing
* Deterministic execution
* Built-in memory abstractions
* Policy enforcement
* Safety guardrails
* Multi-agent coordination

Srishti aims to provide a language-level solution for building production-grade AI systems.

---

## Core Principles

### Agents as First-Class Citizens

Agents are language primitives, not framework objects.

### Deterministic Execution

Business logic is compiled into predictable Rust code.

### Semantic Reasoning

Agents can invoke LLM-powered semantic engines while preserving deterministic control flow.

### Built-in Memory

Memory is defined directly in the language.

### Guardrails & Safety

Safety constraints are declared as part of the source code.

### Compile-Time Validation

Errors should be detected before runtime whenever possible.

### Multi-Agent Systems

Agents can communicate, coordinate, and participate in workflows.

---

## Example

```srishti
agent SupportAgent {

    memory ticket_history

    tool refund(amount: Float)

    guardrail refund_limit(amount: Float) {
        assert amount <= 100
    }

    intent resolve_ticket {
        achieve "Find best resolution"
    }
}
```

---

## Toolchain

Srishti ships with a dedicated CLI.

```bash
srishti init my-project
srishti check src/main.srishti
srishti run src/main.srishti
srishti build
srishti install
```

---

## Compiler Pipeline

```text
Srishti Source
      │
      ▼
Lexer
      │
      ▼
Parser
      │
      ▼
AST
      │
      ▼
Type Checking
      │
      ▼
Validation
      │
      ▼
Interpreter / Runtime
      │
      ▼
Rust Code Generation
```

---

## Architecture

```text
+---------------------+
|  Srishti Source     |
+---------------------+
           |
           v
+---------------------+
|      Compiler       |
+---------------------+
| Lexer               |
| Parser              |
| AST                 |
| Type Checker        |
| Diagnostics         |
+---------------------+
           |
           v
+---------------------+
|     Interpreter     |
+---------------------+
           |
           v
+---------------------+
|      Runtime        |
+---------------------+
| Semantic Engine     |
| Memory System       |
| Event Bus           |
| Agent Lifecycle     |
| Guardrails          |
+---------------------+
```

---

## Current Features

### Language

* Agents
* Memory
* Tools
* Guardrails
* Intents
* Workflows
* Imports
* Events
* Messages

### Compiler

* Lexer
* Parser
* AST Generation
* Semantic Validation
* Diagnostics
* Rust Code Generation

### Runtime

* Tree-Walking Interpreter
* Agent Lifecycle Management
* Event Bus
* OpenAI Provider
* Ollama Provider
* Mock Provider

### CLI

* `srishti init`
* `srishti run`
* `srishti build`
* `srishti check`
* `srishti install`
* `srishti fmt`

### Standard Library

* std/http
* std/json
* std/logging
* std/io

---

## Project Structure

```text
srishti/
├── cli/
├── compiler/
├── runtime/
├── std/
├── examples/
├── docs/
├── tests/
├── README.md
├── ROADMAP.md
└── LICENSE
```

---

## Examples

### Support Agent

```bash
srishti run examples/support_agent.srishti
```

### Travel Agent

```bash
srishti run examples/travel_agent.srishti
```

### Multi-Agent Workflow

```bash
srishti run examples/multi_agent_workflow.srishti
```

### Chatbot

```bash
srishti run examples/chatbot.srishti
```

---

## Roadmap

### v0.3.0 Alpha

* Advanced Type System
* Policy Engine
* Structured Outputs
* Better Diagnostics

### v0.4.0 Alpha

* Persistent Memory
* Retrieval APIs
* Vector Storage

### v0.5.0 Alpha

* Multi-Agent Networking
* Distributed Execution
* Workflow Engine

### v0.6.0 Alpha

* Package Registry
* Dependency Resolution
* Module Publishing

### v1.0.0 Stable

* Production Runtime
* WASM Target
* Optimizer
* Enterprise Policy System
* Stable Language Specification

---

## Current Status

**Version:** v0.2.0 Alpha

Srishti is an experimental language under active development.

The project currently includes a compiler, interpreter, runtime, CLI toolchain, standard library foundation, and multi-agent execution architecture.

---

## Contributing

Contributions, issues, ideas, and design discussions are welcome.

Please open an issue before proposing large language or runtime changes.

---

## License

Apache License 2.0
