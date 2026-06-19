# Entry API

Safe and efficient map access patterns — the `Entry` enum for `HashMap` and friends.

## Prerequisites

- [Collections](../../rust/collections/collections.md) — `HashMap`, `Vec`

## The Problem

Without the Entry API, inserting or updating a map value requires multiple lookups:

```rust
let mut scores = std::collections::HashMap::new();
let team = "Blue".to_string();

if let Some(score) = scores.get_mut(&team) {
    *score += 1;              // first lookup
} else {
    scores.insert(team, 1);   // second lookup
}
```

## The Entry Solution

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.entry("Blue".to_string())
    .and_modify(|score| *score += 1)
    .or_insert(1);
```

## Entry Methods

| Method | Behaviour |
|--------|-----------|
| `or_insert(v)` | Insert `v` if vacant, return a mutable reference |
| `or_default()` | Insert `T::default()` if vacant |
| `and_modify(f)` | Apply `f` to the value if occupied |
| `or_insert_with(f)` | Insert the result of `f()` if vacant |
| `key()` | Return a reference to the entry's key |

## Glossarium

| Term | Definition |
|------|------------|
| Entry | A view into a single entry in a map — either `Occupied` or `Vacant`. |
| Vacant | An entry that does not yet exist in the map. |
| Occupied | An entry that already exists in the map. |

## Next Steps

- [Builder](builder.md) — constructing complex objects
- [Rust Book: Entry API](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)
