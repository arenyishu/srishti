# Srishti

**Agent-Oriented Programming Language (AOPL)**

Srishti is an experimental programming language designed specifically for AI agents.

Instead of treating agents as runtime abstractions inside frameworks, Srishti makes **Agents** a first-class language primitive and compiles them into deterministic Rust systems.

---

## Vision

Modern AI applications are typically built using:

* Prompt Chains
* Agent Frameworks
* Workflow Graphs
* Orchestration Layers

These approaches often lack:

* Compile-time validation
* Strong typing
* Deterministic execution
* Native memory abstractions
* Built-in safety mechanisms

Srishti aims to provide a language-level solution for building reliable AI systems.

---

## Core Principles

### Agents as First-Class Citizens

Agents are defined directly in the language.

### Deterministic Execution

Generated Rust code provides predictable execution.

### Semantic Reasoning

Future runtime integration will enable LLM-powered reasoning.

### Built-in Memory

Memory becomes a language primitive rather than an external component.

### Guardrails

Safety constraints are defined directly in source code.

### Compile-Time Validation

Errors should be detected before runtime whenever possible.

---

# Example

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

# Compiler Pipeline

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
Validation
      │
      ▼
Rust Code Generation
      │
      ▼
Runtime
```

---

# Architecture

```text
+-------------------+
|   Srishti Source  |
+-------------------+
          |
          v
+-------------------+
|     Compiler      |
+-------------------+
          |
          v
+-------------------+
|   Generated Rust  |
+-------------------+
          |
          v
+-------------------+
|      Runtime      |
+-------------------+
| Deterministic     |
| Semantic Engine   |
| Memory System     |
| Guardrails        |
+-------------------+
```

---

# Language Concepts

## Agent

```srishti
agent TravelAgent {
}
```

## Memory

```srishti
memory user_preferences
```

## Tool

```srishti
tool search_flights(destination: String)
```

## Guardrail

```srishti
guardrail budget_limit(amount: Float) {
    assert amount <= 500
}
```

## Intent

```srishti
intent book_trip {
    achieve "Find best flight under budget"
}
```

---

# Current Features (v0.2.0 Alpha)

Implemented:

* Lexer
* Parser
* AST Generation
* Rust Code Generation
* CLI-based Compilation
* Agent Definitions
* Memory Declarations
* Tool Definitions
* Guardrails
* Intents
* Example Programs
* Documentation

---

# Usage

Compile a Srishti source file:

```bash
cargo run -- examples/support_agent.srishti
```

Example output:

```text
Compiling Srishti file...
AST successfully parsed.
Generated Rust code.
```

---

# Project Structure

```text
srishti/
├── compiler/
├── runtime/
├── docs/
├── examples/
├── README.md
├── ROADMAP.md
└── LICENSE
```

---

# Examples

### Support Agent

```text
examples/support_agent.srishti
```

### Travel Agent

```text
examples/travel_agent.srishti
```

---

# Documentation

```text
docs/
├── language-spec.md
├── syntax.md
└── compiler-architecture.md
```

---

# Roadmap

## v0.3.0 Alpha

* Type System
* Semantic Validation
* Symbol Resolution
* Compiler Diagnostics

## v0.4.0 Alpha

* Runtime Engine
* Structured Outputs
* Runtime Configuration

## v0.5.0 Alpha

* Semantic Engine
* LLM Integration
* Planning Framework

## v0.6.0 Alpha

* Memory System
* Persistent Memory
* Retrieval APIs

## v0.7.0 Alpha

* Multi-Agent Workflows
* Agent Communication
* Event System

## v1.0.0 Stable

* Production Runtime
* Package Manager
* WASM Target
* Optimization Passes
* Stable Language Specification

---

# Status

Current Version:

**Srishti v0.2.0 Alpha**

Status:

**Experimental / Research Project**

The compiler can successfully parse agent definitions and generate Rust code.

---

# Contributing

Contributions, feedback, issues, and design discussions are welcome.

Please open an issue before proposing major language changes.

---

# License

Apache License 2.0
