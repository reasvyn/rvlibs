# Traits

Rust's mechanism for defining shared behaviour across types — similar to interfaces in other languages.

## Prerequisites

- [Ownership](ownership.md) — ownership, moves, references
- Basic understanding of generics


## Defining and Implementing Traits

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}...", self.title, &self.content[..50.min(self.content.len())])
    }
}
```

## Default Implementations

```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(read more...)")
    }
}

// Types can use the default:
impl Summary for Article {} // uses default
```

## Traits as Parameters

```rust
// Trait bound syntax:
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// Or with generic bounds:
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

## Common Standard Traits

| Trait | Description | Derivable |
|-------|-------------|-----------|
| `Clone` | Deep-copy a value | ✅ |
| `Copy` | Bitwise copy (stack-only) | ✅ |
| `Debug` | Format with `{:?}` | ✅ |
| `PartialEq` / `Eq` | Equality comparisons | ✅ |
| `PartialOrd` / `Ord` | Ordering comparisons | ✅ |
| `Hash` | Compute a hash | ✅ |
| `Default` | Create a default value | ✅ |
| `Display` | User-facing `{}` output | ❌ (manual) |
| `From` / `Into` | Type conversions | ❌ (manual) |
| `Iterator` | Produce a sequence of values | ❌ (manual) |
| `Deref` | Smart pointer dereference | ❌ (manual) |
| `Drop` | Cleanup on scope exit | ❌ (manual) |

## Trait Bounds with Multiple Traits

```rust
fn debug_compare<T: Debug + PartialOrd>(a: &T, b: &T) {
    println!("{a:?} vs {b:?}");
}
```

Using `where` for readability:

```rust
fn complex<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Clone + Debug,
{ ... }
```

## Trait Objects (`dyn Trait`)

Dynamic dispatch allows runtime polymorphism:

```rust
fn get_printer(format: &str) -> Box<dyn Summary> {
    match format {
        "json" => Box::new(JsonPrinter),
        "text" => Box::new(TextPrinter),
        _ => panic!("unknown format"),
    }
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Trait | A collection of methods that types can implement. Like an interface. |
| Trait Bound | A constraint on a generic type parameter requiring it to implement a trait. |
| Derive | Auto-implementing common traits via `#[derive(...)]`. |
| Trait Object | A type-erased pointer to any type that implements a trait (`dyn Trait`). |
| Associated Type | A type placeholder within a trait, specified by the implementor. |
| Default Implementation | A trait method with a body — implementors can override it or use the default. |


## Next Steps

- [Error Handling](error-handling.md) — `Result`, `Option`, custom error types
- The Rust Book: [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
