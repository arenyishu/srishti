# Getting Started with Srishti

Welcome to Srishti, the Agent-Oriented Programming Language!

## Installation

Ensure you have Rust and Cargo installed, then run:

```bash
cargo install --path cli
```

Verify the installation:
```bash
srishti --version
```

## Creating Your First Project

Create a new project using the `init` command:

```bash
srishti init my-agent
cd my-agent
```

This generates a `srishti.toml` file and a basic `src/main.srishti` file.

## Writing an Agent

Open `src/main.srishti` and define your first agent:

```srishti
import IO from "std/io"

agent HelloAgent {
    intent say_hello {
        achieve "Say a friendly hello"
    }

    on start {
        IO.print("Starting up...")
        let response = say_hello()
        IO.print(response)
    }
}
```

## Running Your Agent

Execute the agent directly using the interpreter:

```bash
srishti run src/main.srishti
```

## Building for Production

To compile your agent into a standalone, production-ready Rust binary:

```bash
srishti build
cargo run --manifest-path build/Cargo.toml
```

You are now ready to build complex, multi-agent systems with Srishti!
