# Route Optimizer

A Rust practice project exploring route optimization algorithms and performance engineering.

This project focuses on the **Traveling Salesperson Problem (TSP)**, a combinatorial optimization problem that aims to find the shortest possible route visiting a set of destinations exactly once and returning to the starting point.

### Algorithms
- **Nearest Neighbor:** Builds an initial route by repeatedly selecting the closest unvisited destination.
- **2-opt:** Improves an existing route by reversing segments to reduce total distance.

### Technologies
- Rust
- Rayon (parallel computing)
