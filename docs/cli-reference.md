# Srishti CLI Reference

The Srishti Command Line Interface (CLI) is the primary way to interact with Srishti projects, compile code, and run agents.

## Commands

### `srishti init [name]`
Scaffolds a new Srishti project in the current directory (or a new directory if `[name]` is provided).
Creates:
- `srishti.toml`
- `src/main.srishti`
- `.gitignore`

### `srishti build [file]`
Compiles a `.srishti` file into Rust code.
- If `[file]` is provided, compiles that file.
- Otherwise, reads the `entry` field from `srishti.toml`.
- Outputs generated Rust code to the `build/` directory.

### `srishti run <file>`
Interprets and executes a `.srishti` file directly using the tree-walking interpreter.
- Requires a file path.
- Instantiates agents and runs their lifecycle.

### `srishti check <file>`
Validates a `.srishti` file without running it.
- Performs lexing, parsing, and type-checking.
- Reports any errors or warnings with detailed diagnostics.

### `srishti install [package]`
Installs dependencies.
- If `[package]` is provided, installs that specific package.
- Otherwise, reads `srishti.toml` and installs all listed dependencies into `srishti_modules/`.

### `srishti fmt <file>`
Formats a `.srishti` file to adhere to standard styling conventions.
