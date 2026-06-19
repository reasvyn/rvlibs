# Iterators

The iterator pattern — `Iterator` trait, adapter methods, and consuming operations.

## Prerequisites

- [Closures](closures.md) — closures as function arguments

## The Iterator Trait

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

## Creating Iterators

```rust
let v = vec![1, 2, 3];
let iter = v.iter();      // &T — borrows elements
let iter = v.iter_mut();  // &mut T — mutable borrow
let iter = v.into_iter(); // T — consumes the vector
```

## Adapter Methods

```rust
let v = vec![1, 2, 3, 4, 5];

// map — transform each element
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

// filter — keep elements matching a predicate
let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();

// Chain adapters:
let result: Vec<i32> = v.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 10)
    .collect();
```

## Consuming Methods

```rust
let sum: i32 = v.iter().sum();
let count = v.iter().count();
let max = v.iter().max();
let any = v.iter().any(|x| *x > 3);
```

## Custom Iterators

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self { Counter { count: 0 } }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 { Some(self.count) } else { None }
    }
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Iterator | An object that produces a sequence of values, one at a time. |
| Adapter | A method that transforms an iterator into another iterator (lazy). |
| Consumer | A method that drives the iterator and produces a final value (eager). |
| Lazy | Iterators don't compute anything until a consumer is called. |

## Next Steps

- [Closures](closures.md) — functions that capture their environment
- [Rust Book: Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
