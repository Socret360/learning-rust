# Progress

Last updated: 2026-10-08
Book: *The Rust Programming Language*, 3rd edition (Rust 1.90+, edition 2024).

## Current position

- Reading: **Ch 8 — Common Collections** (Ch 7 complete).
- Ch 7 code-alongs are done in `restaurant` (library crate + binary in one
  package; `mod` tree, `pub` on structs vs enums, `crate::`/`super::` paths,
  `use`, and `front_of_house` split into `front_of_house.rs` +
  `front_of_house/hosting.rs`) and `backyard` (`garden/vegetables.rs`,
  `use crate::garden::vegetables::Asparagus`). Both build.
- Note: `backyard/`, `restaurant/` and their `settings.json` registrations are
  **uncommitted**.
- Ch 6 code-alongs are done in `enums` (variants, `Option`, `match`,
  `if let`, `let...else`). Committed in `rpb: ch6: enums and pattern matching`.
- Ch 5 code-alongs are done: `structs` (definition, instantiation, field
  shorthand, struct update syntax, tuple structs) and `rectangles`
  (`&Rectangle` + `#[derive(Debug)]` + `dbg!`, then `impl`, `&self`,
  `can_hold`, `Rectangle::square`). Committed in `rpb: ch5: structs and
  methods`.
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
- [x] Ch 5 — Using Structs to Structure Related Data (`structs`, `rectangles`)
  - [x] Defining and Instantiating Structs
  - [x] An Example Program Using Structs
  - [x] Methods
- [x] Ch 6 — Enums and Pattern Matching (`enums`)
  - [x] Defining an Enum
  - [x] The `match` Control Flow Construct
  - [x] Concise Control Flow with `if let` and `let...else`
- [x] Ch 7 — Packages, Crates, and Modules (`restaurant`, `backyard`)
  - [x] Packages and Crates
  - [x] Control Scope and Privacy with Modules
  - [x] Paths for Referring to an Item in the Module Tree
  - [x] Bringing Paths Into Scope with the `use` Keyword
  - [x] Separating Modules into Different Files
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
- Move vs borrow: `println!` borrows its arguments, so "borrow of moved value"
  (`E0382`) really means the move already happened.
- Partial moves via struct update syntax (`..user1` moves only the fields not
  spelled out and not `Copy`); the source is then unusable as a whole but
  surviving fields still work.
- `Copy` is opt-in (`#[derive(Copy, Clone)]`), not a property of "primitive":
  `Point(i32, i32, i32)` moves unless derived. Design rule: small plain-data
  types get `Copy`; buffer-owning types (`Frame`, `Image`, `Matrix`) must not.
- `let...else`: the `else` block must diverge (type `!`, the never type) via
  `return`, `break`, `continue`, or `panic!`; the binding outlives the
  statement, so control must never continue past a failed match.
- `match` exhaustiveness is checked by the compiler, but it cannot detect two
  arms that return an identical value — a logic bug found by hand in
  `describe_state_quarter`.
- Privacy is per-item and defaults to private: a `pub struct` still has
  private fields (so `Breakfast` needs a `summer` constructor), but a `pub
  enum` makes every variant public. Design rule for CV: `Image` exposes
  `width()`/`height()` and keeps its pixel `Vec` private so the
  `len == w * h * channels` invariant can't be broken from outside.
- `use` idioms not exercised in code yet: `pub use` re-exports, `as`
  renaming, nested paths (`use std::{cmp, io}`), and glob `*`. Re-exports are
  how a future `vision` crate would present a flat public API over a deep
  module tree.
- `mod foo;` *declares* a module once (in the parent); `use` only creates a
  shortcut. Declaring the same file twice is a common mistake.
- Refutable vs. irrefutable patterns (introduced in Ch 6; deepened in Ch 19).
- Lifetimes (not yet covered; will matter for image buffers later).

## Next steps

1. Commit `restaurant`, `backyard` and the `settings.json` registrations as
   `rpb: ch7: packages, crates, and modules`.
2. **DSA Tier 1 is overdue** (planned since Ch 6, still not started): run
   `/dsa dynamic array` and build `dsa/dynamic_array`, a growable array with
   amortized O(1) push. State the complexity before coding. Do this *before*
   Ch 8.1 so you have built a `Vec` before you use one. It also exercises the
   Ch 7 privacy rules (private `len`/`cap`, public methods).
3. Ch 8 — Common Collections. After 8.1, add a tiny goal-aligned project: a
   grayscale `Image { width, height, pixels: Vec<u8> }` in its own module with
   a private buffer and a `histogram() -> [u32; 256]`.
4. Start Phase 1 of `ROADMAP.md` (idiomatic Rust and tooling) once the book
   reaches Chapter 10.
