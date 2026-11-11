# neSia Docs

## Pipeline overview

The current project is organized as a classic compiler pipeline, with each crate responsible for one stage of the pipeline.

### 1. Source file input
The entry point is the CLI in [src/bin/nesiac.rs](../src/bin/nesiac.rs). It reads a file from disk and passes the file contents into the runtime entry point in [src/lib.rs](../src/lib.rs).

### 2. Lexing
The first real stage in the pipeline is `ns_lexer`.

- crate: [crates/ns_lexer](../crates/ns_lexer)
- responsibility: turn raw source text into tokens
- current status: implemented and working for the language surface area in use

The public API is exposed in [crates/ns_lexer/src/lib.rs](../crates/ns_lexer/src/lib.rs):
- `lexerize(source: String) -> Vec<Token>` for the simple call path
- `lexerize_result(...) -> Result<Vec<Token>, LexerError>` for the stricter path

The scanner in [crates/ns_lexer/src/lexer.rs](../crates/ns_lexer/src/lexer.rs) handles:
- keywords and identifiers
- integers and floats
- strings and char literals
- arithmetic, comparison, logical, and assignment operators
- delimiters like `() { } [ ] , : ;`
- comments and whitespace skipping
- token line/column tracking

At the moment, the lexer is the stage that is actually wired into the interpreter flow.

### 3. Parsing
The next expected stage is `ns_parser`.

- crate: [crates/ns_parser](../crates/ns_parser)
- responsibility: consume tokens and build a parse tree / AST
- current status: scaffolded, not yet implementing the language grammar in detail

This stage is meant to sit immediately after lexing and before semantic lowering.

### 4. AST and syntax tree
The AST layer is represented by `ns_ast`.

- crate: [crates/ns_ast](../crates/ns_ast)
- responsibility: define the in-memory syntax tree for parsed code
- current status: structural scaffold for the compiler pipeline

The AST is the tree that represents declarations, expressions, statements, function definitions, and control flow.

### 5. High-level intermediate representation
The HIR stage is represented by `ns_hir`.

- crate: [crates/ns_hir](../crates/ns_hir)
- responsibility: lower the AST into a more semantic, compiler-friendly form
- current status: placeholder stage in the current repo layout

This is where name resolution, type hints, and semantic checks should begin to matter.

### 6. Type system and runtime definitions
Several crates support the middle and later pipeline:

- `ns_types`: core type definitions
- `ns_std`: standard library bindings and helpers
- `ns_runtime`: runtime support and execution primitives

These sit alongside the compiler pipeline and provide the foundation used after semantic analysis.

### 7. Code generation and bytecode
The compiler and bytecode crates form the lowering target for execution.

- `ns_compiler`: generate executable bytecode from lowered IR
- `ns_bytecode`: define bytecode instructions / VM representation

This is the bridge between source code and the runtime engine.

### 8. Virtual machine and execution
The final execution stage is handled by:

- `ns_vm`: the virtual machine that executes bytecode
- `ns_runtime`: runtime utilities invoked by the VM

This is where compiled code actually executes.

## Current reality in this repo

The repository is in a staged buildout:

- lexer: implemented
- parser/AST/HIR/compiler/VM: present as crates, but not fully connected into a complete end-to-end execution pipeline yet

So the pipeline is conceptually:

`source file -> lexer -> parser -> AST -> HIR -> compiler -> bytecode -> VM/runtime`

but in the current codebase, the stage that is actually active today is only the first step: `source -> tokens`.

## Example flow

A file like the example in [docs/design/syntax.md](design/syntax.md) is intended to go through the pipeline roughly as follows:

1. read the file contents
2. tokenize the program
3. parse statements and function declarations
4. build AST nodes for `let`, `fn`, `return`, and calls
5. lower the AST to a semantic representation
6. compile to bytecode
7. execute in the VM

The current implementation has reached the tokenization step and confirms the language syntax is valid enough for the current examples.
