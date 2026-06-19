# Control Flow

Conditionals, loops, and pattern matching — controlling program execution.

## Prerequisites

- [Functions](functions.md) — function syntax

## if / else

```rust
let number = 6;

if number % 4 == 0 {
    println!("divisible by 4");
} else if number % 3 == 0 {
    println!("divisible by 3");
} else {
    println!("not divisible by 4 or 3");
}
```

`if` is an expression — can be used in `let`:

```rust
let condition = true;
let result = if condition { 5 } else { 6 };
```

## loop

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // returns value from loop
    }
};
```

## while

```rust
let mut n = 3;
while n > 0 {
    n -= 1;
}
```

## for

```rust
let arr = [10, 20, 30];

for element in arr {
    println!("{element}");
}

// Range
for n in 1..4 {       // 1, 2, 3
    println!("{n}");
}
for n in 1..=4 {      // 1, 2, 3, 4
    println!("{n}");
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Loop | An infinite loop, exited with `break`. |
| Range | A sequence of numbers created with `..` or `..=`. |
| Expression | `if`, `loop`, and `match` can all return values. |

## Next Steps

- [Strings](strings.md) — String and &str
- [Rust Book: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
