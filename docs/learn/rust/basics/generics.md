# Generics

Generic types, functions, and constraints — writing code that works with many types.

## Prerequisites

- [Functions](functions.md) — function parameters and return types

## Generic Functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

let numbers = vec![34, 50, 25, 100, 65];
let result = largest(&numbers);  // &i32

let chars = vec!['y', 'm', 'a', 'q'];
let result = largest(&chars);   // &char
```

## Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
```

Multiple type parameters:

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}
```

## Generic impl

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {   // specialised impl for f64
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

## Const Generics

```rust
struct Array<T, const N: usize> {
    data: [T; N],
}

let arr: Array<i32, 3> = Array { data: [1, 2, 3] };
```

## Glossarium

| Term | Definition |
|------|------------|
| Generic | A type or function parameterised over types. |
| Monomorphisation | The compiler generates concrete code for each type used. |
| Trait Bound | A constraint specifying what traits a generic type must implement. |

## Next Steps

- [Closures](closures.md) — anonymous functions and capturing environments
- [Rust Book: Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
