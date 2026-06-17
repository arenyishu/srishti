# Srishti Language Specification

## Overview

Srishti is an Agent-Oriented Programming Language (AOPL).

Core primitives:

- agent
- memory
- tool
- guardrail
- intent
- achieve

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
