# Ownership

Rust's most distinctive feature: every value has exactly one owner at a time.

## Prerequisites

- Basic Rust syntax (functions, variables, `let`)
- Understanding of stack vs heap memory


## Ownership Rules

1. Each value in Rust has exactly one **owner**.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is **dropped** (memory is freed).

```rust
{
    let s = String::from("hello"); // s owns the string
}                                  // scope ends → s is dropped, memory freed
```

## Moves

When you assign a value to another variable, ownership is **moved**:

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved into s2

// println!("{s1}"); // ❌ ERROR: s1 is no longer valid
println!("{s2}");     // ✅ s2 now owns the string
```

Stack-only types (integers, bools, floats, chars, tuples of Copy types) implement `Copy` and are copied instead of moved:

```rust
let x = 42;
let y = x;     // x is copied, not moved
println!("{x}"); // ✅ still valid
```

## Functions and Ownership

Passing a value to a function moves ownership to the function parameter:

```rust
fn take_ownership(s: String) {
    println!("{s}");
} // s is dropped here

let s = String::from("hello");
take_ownership(s);
// println!("{s}"); // ❌ ERROR: s was moved
```

Returning a value transfers ownership back:

```rust
fn give_ownership() -> String {
    String::from("hello")
}

let s = give_ownership(); // s owns the returned String
```

## Glossarium

| Term | Definition |
|------|------------|
| Owner | The variable that holds a value. Each value has exactly one owner. |
| Move | Transferring ownership from one variable to another. The original is invalidated. |
| Copy | Types that implement `Copy` are duplicated instead of moved on assignment. |
| Clone | Explicit deep-copy via the `Clone` trait. |
| Borrow Checker | The compiler component that enforces ownership, borrowing, and lifetime rules at compile time. |
| Scope | The range within a program where a variable is valid (from `let` to closing `}`). |


## Next Steps

- [Borrowing](borrowing.md) — references without taking ownership
- [Lifetimes](lifetimes.md) — how Rust ensures references are always valid
- The Rust Book: [What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
