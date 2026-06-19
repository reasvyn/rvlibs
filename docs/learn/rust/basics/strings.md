# Strings

`String` and `&str` — Rust's two string types, UTF-8, and common operations.

## Prerequisites

- [Data Types](data-types.md) — basic types

## String vs &str

```rust
let s: &str = "hello";         // string slice (borrowed, fixed)
let t: String = "hello".to_string(); // owned, growable
let u: String = String::from("hello");
```

| Type | Owned | Mutable | Heap |
|------|-------|---------|------|
| `&str` | No | No | No |
| `String` | Yes | Yes | Yes |

## Creating Strings

```rust
let mut s = String::new();
s.push_str("hello");          // append &str
s.push(' ');                  // append char
s += "world";                 // concatenation

let s = format!("{}-{}", "hello", "world");  // building
```

## Accessing & Iterating

Strings are UTF-8 — bytes don't always equal characters:

```rust
let s = "hello";
for c in s.chars() {      // Unicode characters
    println!("{c}");
}
for b in s.bytes() {      // raw bytes
    println!("{b}");
}

// Slicing — careful with UTF-8 boundaries!
let hello = &s[0..5];     // "hello"
```

## Common Methods

```rust
s.len();                   // byte length
s.is_empty();              // bool
s.contains("world");       // bool
s.replace("hello", "hi"); // String
s.trim();                  // &str
s.to_uppercase();          // String
```

## Glossarium

| Term | Definition |
|------|------------|
| `String` | An owned, heap-allocated, growable UTF-8 string. |
| `&str` | A borrowed string slice pointing to UTF-8 data. |
| UTF-8 | A variable-width encoding for Unicode characters. |

## Next Steps

- [Structs](structs.md) — custom data types
- [Rust Book: Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
