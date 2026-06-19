# Closures

Anonymous functions that capture their environment — syntax, capturing modes, and the `Fn` traits.

## Prerequisites

- [Generics](generics.md) — generic type parameters and bounds

## Closure Syntax

```rust
let add_one = |x: i32| -> i32 { x + 1 };
let add_one = |x| x + 1;  // type inference
let add_one = |x| x + 1;  // single expression, no braces

assert_eq!(add_one(5), 6);
```

## Capturing the Environment

```rust
let x = 42;
let print_x = || println!("{x}");
print_x(); // borrows x immutably
```

Closures can capture by:
- **Immutable reference** (`&T`) — multiple closures can read
- **Mutable reference** (`&mut T`) — exclusive write access
- **Move** (`T`) — takes ownership with `move` keyword

## move Closures

```rust
let data = vec![1, 2, 3];
let closure = move || {
    // data is moved into the closure
    println!("{data:?}");
};
```

## Fn Traits

```rust
// FnOnce — can be called once (consumes captured values)
fn call_once<F: FnOnce()>(f: F) {
    f();
}

// FnMut — can mutate captured state
fn call_mut<F: FnMut()>(mut f: F) {
    f();
}

// Fn — can be called multiple times, no mutation
fn call<F: Fn() -> i32>(f: F) -> i32 {
    f()
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Closure | An anonymous function that can capture values from its environment. |
| Capture | A variable from the enclosing scope that is accessible inside the closure. |
| `Fn` | A trait for closures that can be called multiple times without mutation. |
| `FnMut` | A trait for closures that can mutate their captured state. |
| `FnOnce` | A trait for closures that can be called only once. |

## Next Steps

- [Iterators](iterators.md) — iterating over collections and custom sequences
- [Rust Book: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
