# Macros

Declarative macros (`macro_rules!`) and procedural macros.

## Prerequisites

- [Generics](../basics/generics.md) — generic code patterns

## Declarative Macros

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

say_hello!(); // expands to println!("Hello!");
```

## With Parameters

```rust
macro_rules! create_function {
    ($name:ident) => {
        fn $name() {
            println!("function {:?} called", stringify!($name));
        }
    };
}

create_function!(foo);
create_function!(bar);

foo(); // prints: function "foo" called
```

## Repetition

```rust
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

let v = vec![1, 2, 3]; // expands to a Vec<i32>
```

## Procedural Macros

Procedural macros operate on the AST at compile time — they're defined in separate proc-macro crates.

```rust
// In a proc-macro crate (Cargo.toml has proc-macro = true)
#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    // ...
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Declarative Macro | A macro defined with `macro_rules!` that matches patterns. |
| Procedural Macro | A macro defined as Rust code that processes token streams. |
| Token Stream | A sequence of tokens representing Rust source code. |
| `stringify!` | A built-in macro that converts expressions to string literals. |

## Next Steps

- [Unsafe](unsafe.md) — raw pointers and unsafe operations
- [Rust Book: Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
