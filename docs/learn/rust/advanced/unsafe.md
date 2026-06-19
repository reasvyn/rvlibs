# Unsafe Rust

Raw pointers, `unsafe` blocks, and when to use them.

## Prerequisites

- [Ownership](../basics/ownership.md) — ownership rules
- [Borrowing](../basics/borrowing.md) — reference rules

## What Unsafe Means

Unsafe code can:
- Dereference raw pointers (`*const T`, `*mut T`)
- Call `unsafe` functions (e.g., `std::mem::transmute`)
- Implement `unsafe` traits (`Send`, `Sync`)
- Access and modify mutable statics
- Access union fields

```rust
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1: {}", *r1);
    println!("r2: {}", *r2);
}
```

## Unsafe Does Not Mean Wrong

Unsafe code puts the responsibility on YOU to uphold Rust's guarantees:

- No null pointer dereferences
- No dangling pointers
- No data races
- Valid alignment and initialisation

## FFI Example

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("abs(-3): {}", abs(-3));
    }
}
```

## Glossarium

| Term | Definition |
|------|------------|
| Unsafe | A keyword that allows operations the compiler cannot verify. |
| Raw Pointer | `*const T` or `*mut T` — no ownership or borrowing guarantees. |
| FFI | Foreign Function Interface — calling C code from Rust. |

## Next Steps

- [Macros](macros.md) — declarative and procedural macros
- [Rust Book: Unsafe](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
