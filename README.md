# Rusky

Rusky is a key-value store implementation in Rust that serves as both a functional database and a learning project. The implementation progresses through three distinct stages, each building upon the previous to demonstrate fundamental database and systems programming concepts.

## Project Philosophy

This project was designed to explore how real databases work under the hood, starting from the simplest possible implementation and gradually adding complexity. Rather than building a production-ready system, Rusky focuses on educational value, especially for learning Rust.

The implementation deliberately uses a single file to keep the entire system comprehensible, allowing readers to understand the complete data flow from command parsing to persistent storage to network responses. Comments are deliberaty verbose and unconventional, as an in-line tutorial for users new to Rust (basically just me at this point).

After that, I decided to also use this project to learn about Kubernetes, deployed on GCP. This is planned for the short-term future.

## Architecture Evolution

We split this simple programme into three main phases, which was generally around one phase per commit. 

### Stage One: In-Memory Foundation

The first stage implements a simple in-memory key-value store using Rust's HashMap. This provides the basic storage abstraction while introducing Rust's ownership system and error handling patterns. The command-line interface uses the clap library to parse user input into structured commands.

At this stage, data exists only in memory and is lost when the program exits. This limitation drives the need for persistence, which leads naturally to the second stage.

### Stage Two: Persistent Storage

The second stage introduces durability through a Write-Ahead Log (WAL) pattern. Every operation is first written to an append-only log file before being applied to the in-memory HashMap. This ensures that operations are never lost, even if the program crashes during execution.

The log uses JSON serialization to store commands, making it human-readable for debugging while remaining machine-parseable. On startup, the system rebuilds its in-memory state by replaying all commands from the log file.

This design choice prioritizes simplicity and crash safety over performance. The log grows indefinitely without compaction, which would be problematic in a production system but serves the educational purpose of demonstrating the core WAL concept.

### Stage Three: Network Interface

The third stage adds a REST API server alongside the existing command-line interface. The same binary can operate in either CLI mode or server mode, sharing the same underlying storage implementation.

The server uses tokio for async execution and axum for HTTP handling. Thread safety is achieved through Arc<Mutex<>> wrapping, which provides exclusive access to the shared store across concurrent HTTP requests.

This architecture demonstrates how different interfaces can be built on top of the same storage engine, a common pattern in database systems where SQL, NoSQL, and other interfaces might share underlying storage infrastructure.

## Key Design Decisions

### Write-Ahead Logging

We chose to log every operation to a file before applying it to memory. This means if the program crashes, we can rebuild everything by replaying the log file. It's slower than keeping everything in memory, but it's much safer.

The log file stores each command as JSON on its own line, which makes it easy to read and debug. JSON isn't the most efficient format, but it's simple and human-readable.

### Command Pattern

Instead of calling functions directly, we represent operations as data structures (Command enums). This lets us use the same command format for CLI operations, HTTP requests, and log entries. It also makes adding new operations easier.

### Concurrency Model

When multiple HTTP requests come in at the same time, we use a simple approach: lock the entire store during each operation. This prevents data races but means operations can't happen in parallel. It's simple and safe, though not the fastest approach possible.

We use Arc<Mutex<>> which is Rust's standard way to share data safely between threads. The type system prevents common concurrency bugs automatically.

## Usage Patterns

### Command Line Interface

The CLI mode provides direct access to the store for interactive use or scripting. Commands follow a simple pattern where the operation type is the first argument, followed by the key and optionally the value.

```bash
cargo run -- set username alice
cargo run -- get username  
cargo run -- delete username
```

### HTTP Server

The server mode exposes the same functionality through a REST API. The HTTP interface maps naturally to the underlying operations, with POST for setting values, GET for retrieval, and DELETE for removal.

```bash
cargo run -- server -p 3000

curl -X POST localhost:3000/username -H "Content-Type: application/json" -d '{"value":"alice"}'
curl localhost:3000/username
curl -X DELETE localhost:3000/username
```

## Learning Outcomes

Building Rusky covers several important concepts in systems programming. The progression from in-memory to persistent to networked storage shows how databases evolve in complexity.

Rust's ownership system becomes important when dealing with shared data across multiple threads. The type system prevents many common bugs automatically, and you don't need garbage collection.

The logging approach shows how databases ensure data isn't lost, while the command pattern demonstrates how to design flexible interfaces. The concurrency model shows the trade-offs between simplicity and performance.

## Future Directions

This implementation could be extended with more advanced features. The log file grows forever, so adding log compaction would be useful. Indexing could make lookups faster than replaying the entire log.

You could add replication by sending log entries between multiple servers, or add transaction support for atomic multi-key operations. The design makes it relatively easy to experiment with different approaches while keeping the same CLI and HTTP interfaces.

## Dependencies and Structure

The project uses a few key dependencies: clap for command-line parsing, serde for JSON serialization, and tokio plus axum for the HTTP server. These are all well-established Rust libraries that don't hide the core concepts.

Everything lives in a single file to keep it simple and readable. The rusky.log file gets created automatically when you first write data, and you can open it in a text editor to see exactly what's been stored.

## Educational Value

Rusky is designed as both a working key-value store and a learning tool. The code prioritizes clarity over performance, with detailed comments explaining each step. It's a good starting point for understanding how databases work or for building more advanced storage systems.

The project shows how persistence, concurrency, and network programming work together in a real system, using Rust's safety features to prevent common bugs along the way.
