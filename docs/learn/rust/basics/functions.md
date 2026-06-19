# Functions

Defining and calling functions — parameters, return values, and expressions.

## Prerequisites

- [Data Types](data-types.md) — type annotations

## Defining Functions

```rust
fn greet() {
    println!("Hello!");
}
```

## Parameters

```rust
fn greet(name: &str, age: u32) {
    println!("{name} is {age} years old");
}
```

Every parameter must have a type annotation.

## Return Values

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // no semicolon = return expression
}

fn is_even(n: i32) -> bool {
    return n % 2 == 0;  // explicit return (also works)
}
```

The last expression in a function body is implicitly returned. Use `return` for early returns.

## Expressions vs Statements

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1  // expression — value is 4
    };         // statement — ends with semicolon
    println!("{y}"); // 4
}
```

Expressions produce values. Statements perform actions and end with `;`.

## Diverging Functions

```rust
fn panic(msg: &str) -> ! {
    panic!("{msg}");
}
```

The `!` return type means the function never returns (diverges).

## Glossarium

| Term | Definition |
|------|------------|
| Expression | Code that evaluates to a value. |
| Statement | Code that performs an action but produces no value. |
| Diverging Function | A function that never returns, using the `!` type. |

## Next Steps

- [Control Flow](control-flow.md) — if, loop, while, for, match
- [Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
