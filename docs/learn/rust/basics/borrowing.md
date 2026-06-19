# Borrowing

Access a value without taking ownership — references and slices.

## Prerequisites

- [Ownership](ownership.md) — ownership rules, moves, the borrow checker


## The Borrowing Rules

1. At any given time, you can have **either** one mutable reference **or** any number of immutable references.
2. References must always be valid (never dangle).

Immutable references — read-only, many allowed:

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &s; // ✅ multiple immutable refs
println!("{r1} and {r2}");
```

Mutable reference — exclusive write access:

```rust
let mut s = String::from("hello");
let r = &mut s;
r.push_str(", world");
// println!("{s}"); // ❌ ERROR: cannot borrow s as immutable because it is also borrowed as mutable
println!("{r}");
```

## The Borrow Checker in Action

```rust
let mut v = vec![1, 2, 3];

let first = &v[0];   // immutable borrow starts
v.push(4);           // ❌ ERROR: cannot borrow v as mutable because it is also borrowed as immutable
println!("{first}"); // immutable borrow used here
```

Why is this an error? `v.push(4)` might reallocate the backing buffer, invalidating `first`. The borrow checker catches this at compile time.

## Slices

A slice is a "view" into a contiguous sequence — it borrows elements without owning them:

```rust
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..4]; // &[i32] — borrows elements 1, 2, 3
```

String slices are the most common:

```rust
let s = String::from("hello world");
let hello = &s[0..5]; // &str — borrows "hello"
let world = &s[6..];  // &str — borrows "world"
```

## The Rules Visualised

```
┌─────────────────────────────────────────────┐
│                 Value Owner                  │
│                let mut data                  │
└──┬──────────────────────┬───────────────────┘
   │                      │
   ▼                      ▼
┌─────────┐          ┌──────────┐
│  &data  │          │ &mut data│
│ read    │          │ read     │
│ many ✅ │          │ write    │
└─────────┘          │ only one │
                     │ at a time│
                     └──────────┘
```

## Glossarium

| Term | Definition |
|------|------------|
| Reference | A non-owning pointer to a value (`&T` or `&mut T`). |
| Borrowing | Creating a reference to a value. The reference "borrows" the value without taking ownership. |
| Mutable Reference | `&mut T` — allows reading and modifying the borrowed value. Exclusive: only one mutable reference at a time. |
| Immutable Reference | `&T` — allows reading the borrowed value. Multiple immutable references are allowed concurrently. |
| Dangling Reference | A reference that points to memory that has been freed. Rust prevents these at compile time. |
| Slice | A dynamically-sized view into a contiguous sequence (`&[T]`, `&str`). |


## Next Steps

- [Lifetimes](lifetimes.md) — ensuring references stay valid across function boundaries
- The Rust Book: [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
