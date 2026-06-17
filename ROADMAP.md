# Srishti Roadmap

## Vision

Srishti is an Agent-Oriented Programming Language (AOPL) that treats AI agents as first-class language primitives and compiles them into deterministic execution systems with semantic reasoning capabilities.

The long-term goal is to provide a complete platform for building reliable, auditable, memory-aware, and multi-agent AI applications.

---

# v0.1.0 Alpha ✅

## Compiler Foundation

* Lexer
* Parser
* AST Generation
* Rust Code Generation
* Initial Agent Syntax
* Basic Compiler Architecture

**Status:** Released

---

# v0.2.0 Alpha ✅

## Language & Toolchain Foundation

### Language Features

* Agent Definitions
* Memory Declarations
* Tool Definitions
* Guardrails
* Intents

### Toolchain

* CLI Commands

  * `srishti run`
  * `srishti build`
  * `srishti check`
  * `srishti init`
  * `srishti install`
  * `srishti fmt`

### Runtime Foundation

* Tree-Walking Interpreter
* Event Bus
* Agent Lifecycle Management
* Mock Semantic Engine
* OpenAI Provider Foundation
* Ollama Provider Foundation

### Project System

* Workspace Support
* Project Manifests (`srishti.toml`)
* Module Resolution Foundation
* Standard Library Foundation

**Status:** Released

---

# v0.3.0 Alpha 🚧

## Language Correctness & Safety

### Type System

* Float
* String
* Boolean
* Structured Types

### Compiler Validation

* Semantic Validation
* Symbol Resolution
* Scope Analysis
* Duplicate Definition Detection

### Diagnostics

* Source Spans
* Error Recovery
* Compiler Warnings
* Rich Diagnostic Messages

### Testing

* Lexer Tests
* Parser Tests
* Typechecker Tests
* Interpreter Tests

**Goal:** Catch language and logic errors before execution.

---

# v0.4.0 Alpha

## Policy Engine & Structured Execution

### Policy System

* Policy Definitions
* Compliance Rules
* Runtime Enforcement
* Audit Logs

### Structured Outputs

* JSON Outputs
* Schema Validation
* Typed Responses

### Runtime Improvements

* Better Tool Execution
* Runtime Configuration
* Execution Tracing

**Goal:** Make AI agents safe, predictable, and auditable.

---

# v0.5.0 Alpha

## Semantic Intelligence

### LLM Integration

* OpenAI Support
* Ollama Support
* Anthropic Support

### Semantic Engine

* Planning
* Reasoning
* Prompt Templates
* Context Management

### Hybrid Execution

* Deterministic Control Flow
* Semantic Decision Making

**Goal:** Enable intelligent agent reasoning.

---

# v0.6.0 Alpha

## Memory Platform

### Memory Types

* Session Memory
* Long-Term Memory
* Vector Memory

### Retrieval

* Embeddings
* Similarity Search
* Knowledge Retrieval

### Persistence

* Local Storage
* External Vector Databases

**Goal:** Enable persistent, context-aware agents.

---

# v0.7.0 Alpha

## Multi-Agent Platform

### Agent Communication

* Messages
* Channels
* Events

### Coordination

* Agent Workflows
* Shared Memory
* Agent Orchestration

### Distributed Execution

* Agent Networking
* Remote Agents
* Service Discovery

**Goal:** Enable collaborative AI systems.

---

# v0.8.0 Beta

## Developer Experience

### Ecosystem

* Package Registry
* Dependency Resolution
* Version Management

### Tooling

* VS Code Extension
* Language Server Protocol (LSP)
* Syntax Highlighting
* Autocomplete

### Developer Tools

* Formatter
* Linter
* Project Templates
* Playground

**Goal:** Improve developer productivity and adoption.

---

# v0.9.0 Release Candidate

## Production Readiness

### Optimization

* Compiler Optimizations
* Runtime Optimizations

### Reliability

* Security Auditing
* Performance Tuning
* Load Testing

### Deployment

* Deployment Tooling
* Docker Support
* CI/CD Integration

**Goal:** Prepare for stable release.

---

# v1.0.0 Stable

## Production Agent Platform

### Language

* Stable Language Specification
* Backward Compatibility

### Runtime

* Production Runtime
* Enterprise Runtime Features

### Deployment

* WASM Target
* Cloud Deployment Support
* Container Deployment

### Enterprise Features

* Policy Engine
* Audit Trails
* Governance Framework
* Compliance Modules

### Support

* Long-Term Support (LTS)

**Goal:** Production-ready Agent-Oriented Programming Language.

---

# Beyond v1.0

### Srishti Cloud

* Hosted Agent Runtime
* Agent Monitoring
* Managed Memory
* Managed Semantic Engine

### Srishti Registry

* Agent Marketplace
* Package Registry
* Shared Policies

### Srishti Enterprise

* Governance Layer
* Compliance Packs
* Enterprise Agent Platform
