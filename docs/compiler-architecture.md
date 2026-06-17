# Compiler Architecture

## Pipeline

```text
Srishti Source
      ↓
Lexer
      ↓
Parser
      ↓
AST
      ↓
Code Generator
      ↓
Rust Output
```

## Components

### Lexer

Converts source code into tokens.

### Parser

Builds the Abstract Syntax Tree (AST).

### Code Generator

Generates Rust code from the AST.
