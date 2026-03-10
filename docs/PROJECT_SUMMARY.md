# Ocelotter Project Summary

## What this project is

`ocelotter` is an experimental JVM implementation in Rust. It can parse Java `.class` files, load classes into an internal runtime repository, and interpret bytecode for a subset of JVM instructions.

The codebase is explicitly a work in progress: parts of classloading, native method support, object/field handling, and invocation semantics are implemented, while other pieces are still marked TODO/FIXME.

## High-level architecture

The runtime is split into two major areas:

- **Class system (`src/klass`)**: Parses class files, models constant pool / methods / fields, and manages loaded classes in a shared class repository.
- **Interpreter (`src/interpreter`)**: Executes method bytecode against local variables + operand stack, handles object/array operations, and dispatches method calls.

The executable entrypoint is `src/main.rs`, which sets up thread/channel communication between:

- a class repository thread (`SharedKlassRepo`)
- a Java thread that executes the target class method

Communication is channel-based using `OtKlassComms` requests (`class name -> class reply`), which is central to the current design.

## Startup and execution flow

1. CLI options are parsed (`src/klass/options.rs`).
2. A class repository thread starts, bootstraps base classes, and installs native method shims.
3. The target class is loaded (from local `.class` file or from `--classpath` jar/zip).
4. A Java thread starts and looks up `ClassName.main2:([Ljava/lang/String;)I`.
5. The interpreter executes method bytecode and returns a value.

Important detail: this runtime currently expects `main2` (not standard `main`) in the executed class.

## Key modules

- `src/main.rs`: process setup, thread/channel wiring, startup.
- `src/klass/klass_parser.rs`: binary class parsing into runtime structures.
- `src/klass/otklass.rs`: runtime class representation and constant-pool-driven lookups.
- `src/klass/klass_repo.rs`: class repository, bootstrap loading, native method installation, class lookup service loop.
- `src/interpreter/thread.rs`: bytecode interpreter loop and opcode dispatch.
- `src/interpreter/native_methods.rs`: Rust implementations/stubs for selected Java native methods (e.g., parts of `java/lang/Object`, `java/lang/System`, `java/lang/Math`).
- `src/interpreter/simple_heap.rs` + `src/interpreter/object.rs`: heap/object/array model used by field and array opcodes.

## Test assets and tests

- `resources/lib/classes.jar`: bootstrap library jar used during repository bootstrap.
- `resources/test`: small Java examples and compiled `.class` fixtures used by tests.
- `src/tests.rs`: opcode behavior and interpreter-focused tests.
- `src/runtime_tests.rs`: parser/class-repo/runtime tests.

Some tests are intentionally `#[ignore]`, documenting behavior that is incomplete or under development.

## Running locally

- Build: `cargo build`
- Run tests: `cargo test`
- Execute a class from local `.class`: `cargo run -- Main3`
- Execute using a classpath jar/zip: `cargo run -- --classpath resources/test/some.jar YourClassName`

Notes:

- The class name is positional.
- The runtime appends `.class` internally when not using `--classpath`.
- The currently expected entry method is `main2:([Ljava/lang/String;)I`.

## Current limitations (as seen in code)

- Not all JVM opcodes and semantics are complete.
- Virtual dispatch is partially stubbed in places.
- Native support is selective and includes placeholders/NO-OP behavior.
- Several code paths still use panic-driven error handling.
- The project includes debug logging and experimental test fixtures, reflecting active design iteration.
