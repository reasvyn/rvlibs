# Collections

Rust's standard library collection types — growable, heap-allocated data structures.

## Prerequisites

- [Ownership](ownership.md) — ownership rules and moves
- [Traits](traits.md) — `Clone`, `Eq`, `Hash`, trait bounds


## `Vec<T>` — The Workhorse

```rust
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);

// Macro syntax:
let v = vec![1, 2, 3];

// Access:
let first = &v[0];      // panics if index out of bounds
let maybe = v.get(0);    // returns Option<&T>

// Iterate:
for x in &v { println!("{x}") }
for x in &mut v { *x *= 2 }

// Common methods:
v.len();             // number of elements
v.is_empty();        // bool
v.pop();             // Option<T> — remove and return last
v.insert(0, 42);     // insert at index
v.remove(0);         // remove at index — O(n) shift
v.contains(&42);     // bool (requires PartialEq)
v.sort();            // requires Ord
v.dedup();           // remove adjacent duplicates
```

## `HashMap<K, V>`

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Entry API — idempotent insert:
scores.entry(String::from("Blue")).or_insert(0);

// Access:
let score = scores.get("Blue"); // Option<&i32>

// Iterate:
for (key, value) in &scores {
    println!("{key}: {value}");
}
```

## `String`

`String` is an owned, growable UTF-8 string. Different from `&str` (string slice).

```rust
let mut s = String::new();
s.push_str("hello");         // append &str
s.push(' ');                 // append char
s += "world";                // concatenation

// Building:
let s = format!("{}-{}", "hello", "world");

// UTF-8 safe iteration:
for c in "hello".chars() { ... }       // Unicode characters
for b in "hello".bytes() { ... }       // raw bytes

// Slicing (careful with UTF-8 boundaries):
let hello = &s[0..5];                     // panics if not on char boundary
```

## When to Use What

| Collection | When to Use |
|------------|-------------|
| `Vec<T>` | Default — ordered, indexed, iterable. Use 90% of the time. |
| `HashMap<K, V>` | Lookup by key. Fast but unordered. |
| `HashSet<T>` | Checking membership. Unique values. |
| `BTreeMap<K, V>` / `BTreeSet` | When you need sorted order. |
| `VecDeque<T>` | Stack or queue (push/pop both ends). |
| `BinaryHeap<T>` | Priority queue. |
| `LinkedList<T>` | Almost never. Use `Vec` or `VecDeque`. |

## Glossarium

| Term | Definition |
|------|------------|
| `Vec<T>` | Growable array. The most used collection — contiguous, indexed. |
| `HashMap<K, V>` | Key-value store with hash-based lookup. |
| `HashSet<T>` | Set of unique values. Implemented as `HashMap<T, ()>`. |
| `String` | A growable, UTF-8 encoded string (owned, unlike `&str`). |
| `VecDeque<T>` | Double-ended queue. Push/pop from both ends efficiently. |
| `BinaryHeap<T>` | Priority queue. Largest value always at the front. |
| `BTreeMap<K, V>` | Sorted key-value map. |
| `LinkedList<T>` | Doubly-linked list. Rarely needed — almost always use `Vec` instead. |


## Next Steps

- [Concurrency](concurrency.md) — threads, `Arc`, `Mutex`, channels
- The Rust Book: [Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
