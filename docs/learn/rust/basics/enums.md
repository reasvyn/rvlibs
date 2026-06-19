# Enums

Enumerations — defining variants, pattern matching, `Option`, and `Result`.

## Prerequisites

- [Structs](structs.md) — custom types, `impl` blocks

## Defining Enums

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

## Enums with Data

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

More expressive than separate structs — each variant can carry different data.

## Methods on Enums

```rust
impl Message {
    fn call(&self) {
        // method body
    }
}
```

## Option

```rust
enum Option<T> {
    None,
    Some(T),
}

let some_number = Some(5);
let absent: Option<i32> = None;
```

`Option` is built into the prelude — you can use `Some` and `None` without qualification.

## Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Variant | A possible value of an enum. |
| Option | An enum representing a value that may or may not exist. |
| Result | An enum representing success (`Ok`) or failure (`Err`). |

## Next Steps

- [Pattern Matching](pattern-matching.md) — match, if let, destructuring
- [Rust Book: Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
