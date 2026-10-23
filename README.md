# 🦀 Learning Rust

My progress in learning Rust: lab assignments from the *Rust Programming* course (Warsaw University of Technology, Faculty of Mathematics and Information Science, 5th semester).

## Labs

| # | Topic | What I learned |
|---|-------|----------------|
| [Lab 1](lab1) | Introduction & language basics | `loop` with values, `match`, arrays, functions, nested & labeled loops, tuples, file I/O, external crates |

---

### Lab 1: Introduction & language basics

A console program that reads numbers in a loop, adds a random offset (`rand` crate), computes the first 10 powers of each number and checks the [Collatz conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture) for them, saving the results to `xyz.txt`. Invalid input or `0` ends the loop.

**Own feature:** finds `a, b ∈ [1, 10]` such that `a * b = x`, using a labeled `break 'outer` and a function returning a `(bool, u32, u32)` tuple.

**Concepts:** `loop` with `break value`, `match`, arrays, tuples, labeled breaks, file I/O, `cargo clippy`.

---

## Running

```bash
cd lab1
cargo run
```

## Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
