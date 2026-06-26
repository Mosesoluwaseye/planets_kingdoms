# Planets and Kingdoms

## Exercise Description

This project solves the **Planets and Kingdoms** problem from the CSES Problem Set.

Exercise Link:

https://cses.fi/problemset/task/1683

The objective is to determine the strongly connected components (SCCs) of a directed graph. Every planet belongs to exactly one kingdom, where every planet in a kingdom is reachable from every other planet in the same kingdom.

---

## Algorithms Implemented

This project implements two different algorithms for finding Strongly Connected Components (SCCs):

### Algorithm 1: Kosaraju's Algorithm

Kosaraju's algorithm performs two depth-first searches.

Steps:

1. Perform a DFS on the original graph.
2. Store vertices according to their finishing time.
3. Reverse every edge in the graph.
4. Perform DFS on the reversed graph following decreasing finishing order.
5. Each DFS traversal identifies one strongly connected component.

Advantages:

- Easy to understand.
- Straightforward implementation.
- Efficient for sparse graphs.

Disadvantages:

- Requires reversing the graph.
- Performs two complete DFS traversals.

---

### Algorithm 2: Tarjan's Algorithm

Tarjan's algorithm finds SCCs using a single DFS traversal.

Steps:

1. Perform one DFS.
2. Assign every node an index.
3. Maintain a stack of active nodes.
4. Compute low-link values.
5. When a root of an SCC is found, pop the stack to produce one component.

Advantages:

- Only one DFS traversal.
- Does not require reversing the graph.
- Uses less graph traversal.

Disadvantages:

- Slightly more difficult to implement.
- Requires careful handling of stack and low-link values.

---

## Time Complexity

### Kosaraju

- DFS on original graph: O(V + E)
- Reverse graph: O(E)
- DFS on reversed graph: O(V + E)

Overall:

O(V + E)

---

### Tarjan

Single DFS traversal.

Overall:

O(V + E)

---

## Space Complexity

### Kosaraju

- Graph
- Reversed graph
- Visited array
- Stack

Overall:

O(V + E)

---

### Tarjan

- Graph
- Stack
- Index array
- Low-link array

Overall:

O(V + E)

---

## Benchmark Results

Run:

```bash
cargo bench
```

Paste your benchmark results here after running the benchmarks.

Example:

```
Kosaraju
time:
[......]

Tarjan
time:
[......]
```

---

## Benchmark Interpretation

Both algorithms have the same asymptotic complexity of O(V + E), but their practical performance differs.

Kosaraju performs two complete depth-first searches and constructs a reversed graph before computing the strongly connected components. This increases memory usage because an additional graph representation must be stored. The second traversal also increases execution time.

Tarjan performs only one DFS traversal while maintaining a stack and low-link values. Since it avoids building a reversed graph, it generally performs fewer memory allocations and accesses graph data more efficiently.

If benchmark results show Tarjan executing faster, this is expected because:

- only one graph traversal is required
- fewer memory allocations occur
- improved cache locality due to continuous traversal
- no reversed graph needs to be created

If Kosaraju performs similarly, the difference is likely small because both algorithms are linear-time algorithms.

Overall:

- Both algorithms run in O(V + E).
- Tarjan usually has better practical performance.
- Kosaraju is often easier to understand and verify.

---

## Project Structure

```
planets_kingdoms/
│
├── Cargo.toml
├── README.md
│
├── src/
│   ├── main.rs
│   └── lib.rs
│
├── tests/
│   └── planets_kingdoms_tests.rs
│
└── benches/
    └── benchmark.rs
```

---

## Running the Project

Build:

```bash
cargo build
```

Run:

```bash
cargo run
```

Run tests:

```bash
cargo test
```

Run benchmarks:

```bash
cargo bench
```

---

## Deliverables

This project includes:

- Cargo.toml
- src/main.rs
- src/lib.rs
- tests/planets_kingdoms_tests.rs
- benches/benchmark.rs
- README.md

The project satisfies the assignment requirements by implementing two different algorithms, automated tests, Criterion benchmarks, benchmark interpretation, and complexity analysis.
cargo run

Testing

cargo test

Benchmark

cargo bench