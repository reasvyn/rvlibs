# File I/O

Reading and writing files — `std::fs` and `std::io`.

## Prerequisites

- [Error Handling](../basics/error-handling.md) — `Result`, `?`

## Reading Files

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // Read entire file at once
    let content = fs::read_to_string("hello.txt")?;
    println!("{content}");

    // Read as bytes
    let bytes = fs::read("image.png")?;

    // Read with BufReader (efficient for large files)
    use std::io::{BufRead, BufReader};
    let file = fs::File::open("lines.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
```

## Writing Files

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // Write (overwrites)
    fs::write("output.txt", b"hello world")?;

    // Write with File
    use std::io::Write;
    let mut file = fs::File::create("output.txt")?;
    file.write_all(b"hello world")?;

    // Append
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("log.txt")?;
    writeln!(file, "line {}", 42)?;

    Ok(())
}
```

## Directories

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    fs::create_dir("new_dir")?;
    fs::create_dir_all("nested/a/b/c")?;  // recursive

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        println!("{}", entry.path().display());
    }

    fs::remove_file("temp.txt")?;
    fs::remove_dir("empty_dir")?;
    fs::remove_dir_all("full_dir")?;      // recursive

    Ok(())
}
```

## Glossarium

| Term | Definition |
|------|------------|
| `BufReader` | A buffered reader that reduces system calls for sequential reads. |
| `OpenOptions` | A builder for configuring how files are opened. |
| `read_dir` | Returns an iterator over entries in a directory. |

## Next Steps

- [Standard Library Overview](std-library.md) — other useful std modules
- [Rust Book: File I/O](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
