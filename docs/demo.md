# Srishti OS Demonstration

Srishti OS is the Operating System for AI Agents. It replaces fragmented python scripts with a unified, verifiable, compiled runtime built in Rust.

## Try It Now

You can see the Srishti OS in action without any complicated setup. We have built-in demonstrations that showcase the core runtime loop, the distributed clustering, and the governance layer.

```bash
# Run the end-to-end customer support workflow
srishti demo

# Run the human-in-the-loop approval workflow
srishti demo approval

# Run the Paxos/Raft leader election cluster failover
srishti demo cluster
```

## Dashboard Overview

You can also run a visually simulated control plane.

```bash
srishti dashboard --demo
```

![Srishti Dashboard Hero](C:/Users/UserPC/.gemini/antigravity/brain/907a7b91-ac01-4e5f-90ce-5cd643ab3b1e/srishti_dashboard_hero_1781950506292.png)

This will spin up a React-based futuristic Web Dashboard on `http://localhost:3000` populated with realistic telemetry, active AI Agents, and live network metrics flowing through the exact same `srishti-runtime` EventBus utilized in production.

## Verification & Trust

Don't trust marketing. Verify the engine yourself.

```bash
# Verify the entire OS suite (Compiler, Runtime, Governance)
srishti verify --full

# Verify the dashboard APIs
srishti verify --dashboard

# Check the production readiness scorecard
srishti readiness
```
