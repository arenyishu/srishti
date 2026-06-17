# Srishti

### Agent-Oriented Programming Language (AOPL)

Srishti is an experimental Agent-Oriented Programming Language (AOPL) for building reliable, auditable, and scalable AI systems.

Instead of treating agents as runtime abstractions inside frameworks, Srishti makes **Agents first-class language primitives** and provides a complete toolchain including a compiler, interpreter, runtime, memory system, guardrails, workflows, and semantic execution engines.

---

## Why Srishti?

Modern AI applications are typically built using:

* Prompt Chains
* Agent Frameworks
* Workflow Graphs
* Orchestration Layers

While powerful, these approaches often require large amounts of framework-specific code and often lack:

* Compile-time validation
* Strong typing
* Deterministic execution
* Built-in memory abstractions
* Policy enforcement
* Safety guardrails
* Native multi-agent workflows

Srishti explores a language-first approach where agents, memory, workflows, and guardrails are built directly into the language.

---

## Core Principles

### Agents as First-Class Citizens

Agents are language primitives rather than framework objects.

### Deterministic + Semantic Execution

Business logic remains deterministic while semantic reasoning is delegated to LLM-powered engines.

### Built-in Memory

Memory is declared directly in source code.

### Guardrails

Safety constraints are enforced as part of the language.

### Workflows

Agent collaboration is a native language feature.

### Compile-Time Validation

Errors should be detected before runtime whenever possible.

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

## Customer Support System Demo

Srishti includes an end-to-end customer support automation example demonstrating:

* Multi-Agent Workflows
* Memory
* Guardrails
* Event-Driven Communication
* Semantic Reasoning
* Agent Lifecycle Management

Example workflow:

```srishti
workflow CustomerSupport {
    RouterAgent -> RefundAgent
    RouterAgent -> BillingAgent
    RouterAgent -> TechnicalAgent

    RefundAgent -> EscalationAgent
}
```

Run the demo:

```bash
srishti run examples/customer_support_system.srishti
```

The runtime demonstrates:

* Agent boot sequence
* Workflow execution
* Event routing
* Guardrail validation
* Memory operations
* Semantic engine execution
* Escalation handling

---

## Toolchain

```bash
srishti init my-project
srishti check src/main.srishti
srishti run src/main.srishti
srishti build
srishti install
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
* Messages
* Events
* Imports

### Compiler

* Lexer
* Parser
* AST Generation
* Type Checking Foundation
* Semantic Validation
* Diagnostics
* Rust Code Generation

### Runtime

* Tree-Walking Interpreter
* Agent Lifecycle Management
* Event Bus
* Memory System
* OpenAI Provider Foundation
* Ollama Provider Foundation
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
└── LICENSE
```

---

## Examples

```bash
srishti run examples/support_agent.srishti
srishti run examples/travel_agent.srishti
srishti run examples/chatbot.srishti
srishti run examples/multi_agent_workflow.srishti
srishti run examples/customer_support_system.srishti
```

---

## Roadmap

### v0.3.0 Alpha

* Advanced Type System
* Policy Engine
* Structured Outputs
* Enhanced Diagnostics

### v0.4.0 Alpha

* Persistent Memory
* Retrieval APIs
* Vector Storage

### v0.5.0 Alpha

* Multi-Agent Networking
* Distributed Execution
* Workflow Engine

### v1.0.0 Stable

* Production Runtime
* Enterprise Policy System
* WASM Target
* Stable Language Specification

---

## Current Status

**Version:** v0.2.0 Alpha

Srishti is an experimental research language under active development.

Current capabilities include:

* Compiler
* Interpreter
* Runtime Foundation
* Multi-Agent Workflows
* Event System
* Semantic Engine Foundation
* Customer Support System Demonstration

---

## License

Apache License 2.0
