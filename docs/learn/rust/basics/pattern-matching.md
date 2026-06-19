# Pattern Matching

`match`, `if let`, `while let`, and destructuring — Rust's pattern matching in depth.

## Prerequisites

- [Enums](enums.md) — enum variants, `Option`, `Result`

## match

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Matches must be **exhaustive** — cover every possible value.

## Matching with Values

```rust
match x {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    4..=10 => println!("four through ten"),
    _ => println!("other"),
}
```

## Destructuring

```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 0, y: 7 };
let Point { x, y } = p; // x = 0, y = 7

match p {
    Point { x, y: 0 } => println!("on x axis at {x}"),
    Point { x: 0, y } => println!("on y axis at {y}"),
    Point { x, y } => println!("at ({x}, {y})"),
}
```

## if let

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("max is {max}"),
    _ => (),
}

// Simpler:
if let Some(max) = config_max {
    println!("max is {max}");
}
```

## Guards

```rust
match x {
    n if n < 5 => println!("small"),
    n if n < 10 => println!("medium"),
    _ => println!("large"),
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Exhaustive | A match that covers all possible values of a type. |
| Destructuring | Breaking a compound value into its parts. |
| Guard | An additional condition in a match arm with `if`. |

## Next Steps

- [Generics](generics.md) — generic types, functions, and constraints
- [Rust Book: Pattern Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
