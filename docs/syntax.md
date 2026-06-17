# Srishti Syntax

## Agent

```srishti
agent SupportAgent {
}
```

## Memory

```srishti
memory ticket_history
```

## Tool

```srishti
tool refund(amount: Float)
```

## Guardrail

```srishti
guardrail refund_limit(amount: Float) {
    assert amount <= 100
}
```

## Intent

```srishti
intent resolve_ticket {
    achieve "Find best resolution"
}
```
