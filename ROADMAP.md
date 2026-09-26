# Roadmap: book to biometrics

The destination: build face and fingerprint recognition systems from scratch in
Rust, plus other low-level AI work. This file is the long arc; `PROGRESS.md` is
the current position.

Rules for this roadmap:

- Build the core of each phase by hand before using a crate. Crates are allowed
  once you understand what they replace.
- Every phase ends with a project you can run and show, not just reading.
- Prefer `f32`/`f64` correctness first, performance later.

## Phase 0 — Rust fundamentals (the book)

Chapters 1-21 of *The Rust Programming Language* (3rd edition). Work through
each with a project under `books/the-rust-programming-language/`, finishing with
the Ch 21 multithreaded web server as the capstone.

Exit criteria: comfortable with ownership, borrowing, lifetimes, structs, enums,
pattern matching, traits, generics, collections, error handling, closures,
iterators, smart pointers, concurrency, async/await and futures, trait objects,
and macros.

## Parallel track — Data structures and algorithms

Runs alongside Phases 0-2 and feeds Phases 3-7. Implement every structure from
scratch in Rust first; use a `std` collection or crate only after you can build
the equivalent yourself. State time and space complexity before you code.

Projects live in `dsa/<name>/` as library crates with tests.

### DSA tier 1 — Linear structures and complexity

- Arrays, `Vec`, slices, strings; amortized growth; two pointers; sliding window
- Hash maps and sets from scratch (open addressing, chaining); hashing
- Stacks, queues, deques; ring buffers

### DSA tier 2 — Linked structures (the Rust test)

- Singly linked list with `Box`
- Doubly linked list: why it fights the borrow checker; `Rc<RefCell>` vs an
  index/arena design vs `unsafe`
- Skip list

### DSA tier 3 — Trees and heaps

- Binary search tree, traversals, AVL and red-black balancing
- Binary heap / priority queue, heapsort
- Trie, segment tree, Fenwick tree (BIT)
- k-d tree, ball tree, R-tree (spatial indexes — direct CV use)

### DSA tier 4 — Graphs

- Representation: adjacency list/matrix, CSR
- BFS, DFS, connected components, topological sort
- Union-find with path compression and union by rank
- Dijkstra, Bellman-Ford, A*, MST (Kruskal, Prim)
- Max-flow basics

### DSA tier 5 — Algorithm paradigms

- Sorting: insertion, merge, quick, heap, counting, radix; stability
- Binary search and its variants
- Divide and conquer, greedy, backtracking
- Dynamic programming: memoization vs tabulation; classic problems
- Randomized algorithms, reservoir sampling

### DSA tier 6 — Goal-aligned structures

- Spatial hashing, image pyramids, integral images
- Bloom filter, count-min sketch, HyperLogLog
- LRU / LFU caches
- String algorithms: KMP, Rabin-Karp, Boyer-Moore

### DSA milestone projects

- `dynamic_array` — growable array with amortized O(1) push
- `hashmap_from_scratch`
- `linked_list_singly`, `linked_list_doubly`
- `bst`, then `avl_tree`
- `binary_heap`
- `graph_algorithms` — BFS/DFS/Dijkstra/union-find
- `kdtree` — build and nearest-neighbor query, benchmarked against brute force
- `dp_kit` — classic DP problems with tests

## Phase 1 — Idiomatic Rust and tooling

- `cargo` workspaces, features, profiles
- Error handling: `Result`, `?`, custom error enums, `thiserror` vs `anyhow`
- Testing: unit, integration, doc tests, `criterion` benchmarks
- `clippy`, `rustfmt`, `cargo doc`
- Trait design: `From`/`Into`, `Deref`, `AsRef`, `Iterator` (implement one)

Project: a small image-processing CLI that reads, transforms, and writes images
with no CV crate.

## Phase 2 — Systems and memory foundations

- Stack vs heap, `size_of`/`align_of`, `repr(C)`, padding
- Raw pointers, `unsafe`, `MaybeUninit`, `unsafe` soundness
- Slices, `Vec` internals, `Cow`, arenas/allocators
- FFI basics, `no_std`, `core` vs `std`
- SIMD: `core::arch` intrinsics, `std::simd`, auto-vectorization
- Threads, `Send`/`Sync`, scoped threads, `rayon`, `crossbeam`

Project: a hand-written matrix type with a blocked, cache-friendly matmul, then a
SIMD version, benchmarked against a naive one.

## Phase 3 — Numerical computing from scratch

- Image representation: buffers, channels, strides, ROIs
- Convolution, separable filters, Gaussian blur, edge detection (Sobel, Canny)
- Linear algebra: dot, matmul, transpose, LU, eigen/SVD, PCA
- FFT, filtering, resampling, interpolation
- Probability and distance metrics (Euclidean, cosine, Mahalanobis)

Project: implement Gaussian blur, Sobel, and Canny by hand and compare against a
reference.

## Phase 4 — Classical computer vision from scratch

- Feature detection: Harris, FAST, ORB
- Feature matching: brute force, ratio test, RANSAC
- Geometry: homography, camera model, calibration, stereo
- Image pyramids, optical flow

Project: a feature-matching pipeline that stitches two images.

## Phase 5 — Machine learning foundations from scratch

- A `Tensor` type with shapes, strides, broadcasting
- Autograd: computation graph, reverse-mode backprop
- Layers: linear, conv2d, pooling, activations, softmax/cross-entropy
- Optimizers: SGD, momentum, Adam
- A training loop on a small dataset

Project: train an MLP and a small CNN from scratch on MNIST.

## Phase 6 — Face recognition from scratch

- Detection: Viola-Jones, then a CNN detector
- Alignment: landmarks, similarity transform
- Embeddings: triplet loss, FaceNet-style, then ArcFace margin
- Matching: embedding distance, thresholds, ROC/DET curves

Project: detect, align, embed, and match faces on a small dataset.

## Phase 7 — Fingerprint recognition from scratch

- Preprocessing: normalization, segmentation, orientation field
- Enhancement: Gabor filters, ridge frequency
- Minutiae extraction and filtering
- Matching: alignment, minutiae pairing, score normalization

Project: extract minutiae and match two fingerprints.

## Phase 8 — Performance and scale

- Profiling: `perf`, `criterion`, flamegraphs
- SIMD and memory-layout tuning
- Multithreading and parallelism
- GPU: `wgpu` compute or CUDA via `cust`
- Serving and packaging

Project: take a Phase 6 or 7 pipeline and make it real-time.
