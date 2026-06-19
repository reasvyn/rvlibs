# Lifetimes

How Rust ensures references are always valid — lifetime annotations, elision, and `'static`.

## Prerequisites

- [Ownership](ownership.md) — ownership rules and moves
- [Borrowing](borrowing.md) — references, the borrowing rules


## Why Lifetimes Exist

The borrow checker needs to know: **how long does each reference live?** When a function returns a reference, which input's lifetime does it tie to?

```rust
// Without lifetime annotations — this won't compile:
// fn longest(x: &str, y: &str) -> &str { ... }
```

## Lifetime Annotations

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

This says: "The returned reference lives as long as the shorter of `x` and `y`."

The caller must provide inputs that outlive the returned reference:

```rust
let result;
let s1 = String::from("long");
{
    let s2 = String::from("short");
    result = longest(&s1, &s2); // ❌ ERROR: s2 does not live long enough
}
println!("{result}");
```

## Lifetime Elision Rules

Rust can infer lifetimes in most cases. Three elision rules:

1. Each input reference gets its own lifetime parameter.
2. If there is exactly one input lifetime, it is assigned to all output references.
3. If there are multiple input lifetimes but one is `&self` or `&mut self`, the output gets `self`'s lifetime.

```rust
fn first_word(s: &str) -> &str { ... }
// Elided to: fn first_word<'a>(s: &'a str) -> &'a str

fn greet(&self, msg: &str) -> &str { ... }
// Elided to: fn greet<'a, 'b>(&'a self, msg: &'b str) -> &'a str
```

## Lifetimes in Structs

When a struct holds a reference, every reference field needs a lifetime parameter:

```rust
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Long text. Second sentence.");
    let first = novel.split('.').next().expect("no period");
    let excerpt = Excerpt { part: first }; // excerpt cannot outlive novel
}
```

## The `'static` Lifetime

`'static` means the reference is valid for the entire program execution:

```rust
let s: &'static str = "hello"; // string literals are 'static
```

`'static` is often used as a bound in generic code:

```rust
fn print_it<T: Debug + 'static>(item: T) { ... }
```

## Glossarium

| Term | Definition |
|------|------------|
| Lifetime | The period during which a reference is valid. Every reference has a lifetime. |
| Lifetime Annotation | Syntax `'a`, `'b`, etc. that names a lifetime so the compiler can verify relationships. |
| Lifetime Elision | Rust's rules for inferring lifetimes in function signatures when they can be determined unambiguously. |
| `'static` | The special lifetime that lasts the entire program (string literals, const values). |


## Next Steps

- [Traits](traits.md) — defining shared behaviour across types
- The Rust Book: [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
