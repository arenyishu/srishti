# Srishti

Srishti is an Agent-Oriented Programming Language (AOPL) that compiles AI agents into deterministic Rust code and semantic execution engines.

## Vision

Current AI systems rely on chains, graphs, prompts, and orchestration frameworks.

Srishti treats **Agents as first-class language primitives** and separates execution into:

* Deterministic Engine (Rust)
* Semantic Engine (LLM)

This allows developers to build reliable, type-safe AI systems while maintaining access to semantic reasoning.

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

## Architecture

```text
Srishti Source
      ↓
Compiler
      ↓
Rust Code
      ↓
Runtime
 ├── Deterministic Engine
 ├── Semantic Engine
 ├── Memory System
 └── Guardrails
```

---

## Core Concepts

### Agent

```srishti
agent SupportAgent {
}
```

### Tool

```srishti
tool refund(amount: Float)
```

### Guardrail

```srishti
guardrail refund_limit(amount: Float) {
    assert amount <= 100
}
```

### Intent

```srishti
intent resolve_ticket {
    achieve "Find best resolution"
}
```

### Memory

```srishti
memory ticket_history
```

---

## Roadmap

### v0.1.0

* [x] Lexer
* [x] Parser
* [x] AST
* [x] Rust Code Generation

### v0.2.0

* [ ] Runtime Engine
* [ ] Semantic Engine
* [ ] Structured Outputs
* [ ] Guardrails

### v0.3.0

* [ ] Memory System
* [ ] Event Engine
* [ ] Multi-Agent Workflows

### v1.0.0

* [ ] WASM Target
* [ ] Optimizer
* [ ] Production Runtime

---

## Status

Early prototype under active development.

---

## License

Apache-2.0
