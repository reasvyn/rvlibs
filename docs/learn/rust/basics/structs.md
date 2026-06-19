# Structs

Custom data types — named fields, tuple structs, unit structs, methods, and associated functions.

## Prerequisites

- [Functions](functions.md) — defining functions

## Defining and Instantiating

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

let user = User {
    active: true,
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    sign_in_count: 1,
};
```

## Field Access and Mutation

```rust
let mut user = User { email: String::from("a@b.com"), /* ... */ };
user.email = String::from("b@c.com"); // requires mut

let email = user.email;  // moves email out of user!
```

## Tuple Structs

```rust
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Colour(0, 0, 0);
let origin = Point(0, 0, 0);
```

## Methods

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

## Associated Functions

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

let sq = Rectangle::square(10); // called with ::
```

## Glossarium

| Term | Definition |
|------|------------|
| Struct | A custom data type with named fields. |
| Method | A function defined within an `impl` block, with `self` receiver. |
| Associated Function | A function within an `impl` block without `self` — called with `::`. |

## Next Steps

- [Enums](enums.md) — enumerations and pattern matching
- [Rust Book: Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
