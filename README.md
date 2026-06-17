# Srishti

Srishti is an Agent-Oriented Programming Language (AOPL) designed to build reliable AI systems through deterministic execution, semantic reasoning, memory, and guardrails.

Unlike orchestration frameworks that treat agents as runtime abstractions, Srishti makes agents a first-class language primitive and compiles them into executable Rust systems.

---

## Why Srishti?

Modern AI applications are typically built using:

* Prompt chains
* Agent frameworks
* Workflow graphs
* Orchestration layers

These approaches often lack strong typing, validation, and compile-time guarantees.

Srishti introduces a programming language specifically designed for AI agents.

### Key Principles

* Agents are first-class citizens
* Deterministic execution in Rust
* Semantic reasoning through LLMs
* Built-in memory primitives
* Built-in guardrails
* Compile-time validation
* Strongly typed agent interfaces

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
Validation
      │
      ▼
Rust Code Generation
      │
      ▼
Runtime
```

---

## Architecture

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

## Language Concepts

### Agent

```srishti
agent TravelAgent {
}
```

### Memory

```srishti
memory user_preferences
```

### Tool

```srishti
tool search_flights(destination: String)
```

### Guardrail

```srishti
guardrail budget_limit(amount: Float) {
    assert amount <= 500
}
```

### Intent

```srishti
intent book_trip {
    achieve "Find best flight under budget"
}
```

---

## Project Structure

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

## Examples

### Support Agent

```bash
examples/support_agent.srishti
```

### Travel Agent

```bash
examples/travel_agent.srishti
```

---

## Documentation

Documentation can be found in:

```text
docs/
├── language-spec.md
├── syntax.md
└── compiler-architecture.md
```

---

## Roadmap

### v0.1.0 Alpha ✅

* Lexer
* Parser
* AST
* Rust Code Generation
* CLI Prototype

### v0.2.0

* Type System
* Semantic Validation
* Compiler Diagnostics
* Improved Runtime Integration

### v0.3.0

* Memory Engine
* Event System
* Multi-Agent Workflows

### v0.4.0

* OpenAI Integration
* Anthropic Integration
* Structured Outputs

### v1.0.0

* Production Runtime
* Optimizer
* Package Manager
* WASM Target

---

## Current Status

Srishti is currently an experimental language prototype under active development.

The compiler successfully parses agent definitions and generates Rust code.

---

## Contributing

Contributions, feedback, issues, and design discussions are welcome.

Please open an Issue before proposing large language changes.

---

## License

Apache License 2.0
