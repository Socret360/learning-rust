# Progress

Last updated: 2026-09-27
Book: *The Rust Programming Language*, 3rd edition (Rust 1.90+, edition 2024).

## Current position

- Reading: **Ch 5 — Using Structs to Structure Related Data** (Ch 4 complete).
- Ch 4 code-alongs are done: ownership, references, and slices (`first_word` in
  `ownership`), plus `string_type` and `variable_scope`.
- DSA: not started; ready for Tier 1.

## Book checklist

Projects live in `books/the-rust-programming-language/` and are noted
in parentheses.

- [x] Ch 1 — Getting Started (`hello_world`, `hello_cargo`)
  - [x] Installation
  - [x] Hello, World!
  - [x] Hello, Cargo!
- [x] Ch 2 — Programming a Guessing Game (`guessing_game`)
- [x] Ch 3 — Common Programming Concepts
  - [x] Variables and Mutability (`variables`)
  - [x] Data Types (`no_type_annotations`)
  - [x] Functions (`functions`)
  - [x] Comments
  - [x] Control Flow (`branches`, `loops`, `temperatures`, `fibonacci`)
- [x] Ch 4 — Understanding Ownership (`ownership`, `string_type`,
      `variable_scope`)
  - [x] What is Ownership?
  - [x] References and Borrowing
  - [x] The Slice Type
- [ ] Ch 5 — Using Structs to Structure Related Data
  - [ ] Defining and Instantiating Structs
  - [ ] An Example Program Using Structs
  - [ ] Methods
- [ ] Ch 6 — Enums and Pattern Matching
  - [ ] Defining an Enum
  - [ ] The `match` Control Flow Construct
  - [ ] Concise Control Flow with `if let` and `let...else`
- [ ] Ch 7 — Packages, Crates, and Modules
  - [ ] Packages and Crates
  - [ ] Control Scope and Privacy with Modules
  - [ ] Paths for Referring to an Item in the Module Tree
  - [ ] Bringing Paths Into Scope with the `use` Keyword
  - [ ] Separating Modules into Different Files
- [ ] Ch 8 — Common Collections
  - [ ] Storing Lists of Values with Vectors
  - [ ] Storing UTF-8 Encoded Text with Strings
  - [ ] Storing Keys with Associated Values in Hash Maps
- [ ] Ch 9 — Error Handling
  - [ ] Unrecoverable Errors with `panic!`
  - [ ] Recoverable Errors with `Result`
  - [ ] To `panic!` or Not to `panic!`
- [ ] Ch 10 — Generic Types, Traits, and Lifetimes
  - [ ] Generic Data Types
  - [ ] Defining Shared Behavior with Traits
  - [ ] Validating References with Lifetimes
- [ ] Ch 11 — Writing Automated Tests
  - [ ] How to Write Tests
  - [ ] Controlling How Tests Are Run
  - [ ] Test Organization
- [ ] Ch 12 — An I/O Project: Building a Command Line Program
  - [ ] Accepting Command Line Arguments
  - [ ] Reading a File
  - [ ] Refactoring to Improve Modularity and Error Handling
  - [ ] Adding Functionality with Test Driven Development
  - [ ] Working with Environment Variables
  - [ ] Redirecting Errors to Standard Error
- [ ] Ch 13 — Functional Language Features: Iterators and Closures
  - [ ] Closures
  - [ ] Processing a Series of Items with Iterators
  - [ ] Improving Our I/O Project
  - [ ] Performance in Loops vs. Iterators
- [ ] Ch 14 — More about Cargo and Crates.io
  - [ ] Customizing Builds with Release Profiles
  - [ ] Publishing a Crate to Crates.io
  - [ ] Cargo Workspaces
  - [ ] Installing Binaries with `cargo install`
  - [ ] Extending Cargo with Custom Commands
- [ ] Ch 15 — Smart Pointers
  - [ ] Using `Box<T>` to Point to Data on the Heap
  - [ ] Treating Smart Pointers Like Regular References
  - [ ] Running Code on Cleanup with the `Drop` Trait
  - [ ] `Rc<T>`, the Reference Counted Smart Pointer
  - [ ] `RefCell<T>` and the Interior Mutability Pattern
  - [ ] Reference Cycles Can Leak Memory
- [ ] Ch 16 — Fearless Concurrency
  - [ ] Using Threads to Run Code Simultaneously
  - [ ] Transfer Data Between Threads with Message Passing
  - [ ] Shared-State Concurrency
  - [ ] Extensible Concurrency with `Send` and `Sync`
- [ ] Ch 17 — Fundamentals of Asynchronous Programming: Async, Await, Futures,
      and Streams
  - [ ] Futures and the Async Syntax
  - [ ] Applying Concurrency with Async
  - [ ] Working With Any Number of Futures
  - [ ] Streams: Futures in Sequence
  - [ ] A Closer Look at the Traits for Async
  - [ ] Futures, Tasks, and Threads
- [ ] Ch 18 — Object Oriented Programming Features
  - [ ] Characteristics of Object-Oriented Languages
  - [ ] Using Trait Objects to Abstract over Shared Behavior
  - [ ] Implementing an Object-Oriented Design Pattern
- [ ] Ch 19 — Patterns and Matching
  - [ ] All the Places Patterns Can Be Used
  - [ ] Refutability: Whether a Pattern Might Fail to Match
  - [ ] Pattern Syntax
- [ ] Ch 20 — Advanced Features
  - [ ] Unsafe Rust
  - [ ] Advanced Traits
  - [ ] Advanced Types
  - [ ] Advanced Functions and Closures
  - [ ] Macros
- [ ] Ch 21 — Final Project: Building a Multithreaded Web Server
  - [ ] Building a Single-Threaded Web Server
  - [ ] From Single-Threaded to Multithreaded Server
  - [ ] Graceful Shutdown and Cleanup
- [ ] Appendices
  - [ ] A — Keywords
  - [ ] B — Operators and Symbols
  - [ ] C — Derivable Traits
  - [ ] D — Useful Development Tools
  - [ ] E — Editions
  - [ ] F — Translations of the Book
  - [ ] G — How Rust is Made and "Nightly Rust"

## DSA track checklist

- [ ] Tier 1 — Linear structures and complexity (dynamic array, hash map,
      stack, queue, ring buffer)
- [ ] Tier 2 — Linked structures (singly list, doubly list, skip list)
- [ ] Tier 3 — Trees and heaps (BST, AVL, heap, trie, segment tree, k-d tree)
- [ ] Tier 4 — Graphs (BFS/DFS, topological sort, union-find, Dijkstra, A*, MST)
- [ ] Tier 5 — Algorithm paradigms (sorting, binary search, greedy,
      backtracking, DP)
- [ ] Tier 6 — Goal-aligned structures (spatial hashing, integral images, LRU,
      bloom filter)

## Concepts to review

- Borrowing rules and mutable references (`&mut` exclusivity).
- Lifetimes (not yet covered; will matter for image buffers later).

## Next steps

1. Continue to Ch 5 (structs): build a `structs` project for the chapter's
   code-alongs.
2. Start DSA Tier 1 with `/dsa dynamic array` — a growable array with amortized
   O(1) push, built from scratch.
3. Start Phase 1 of `ROADMAP.md` (idiomatic Rust and tooling) once the book
   reaches Chapter 10.
