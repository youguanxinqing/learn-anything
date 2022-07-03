# Command-Line Rust

## Preface

- rustup update: get the latest version of language
- cargo fmt: format code
- Clippy: a linter for Rust code (`rustup component add clippy` to get, `cargo clippy` to format)

## Chapter 1. Truth or Consequences

Where's name of binary from?
```toml
[package]
name = "helloWorld"  # build binary name
version = "0.1.0"
authors = ["youguanxinqing <youguanxinqing@qq.com>"]
edition = "2018"  # probably introduce breaking changes
```

Rust libraries are called `crates`.

`mkdir tests` creates diretory parallel to the **src** diretory. `Cargo test` runs all test functions.

`tr`: replcae the colons(:) with newlines(\n).
```bash
$ echo $PATH | tr : '\n'
```

Add a development dependency to Cagro.toml.
```toml
[dev-dependencies]
assert_cmd = "1"
```

The standard exit code is 0 to indicate success and any number from 1 to 255 otherwise.
- `man true`
- `man false`

Create **src/bin/true.rs** with the following content. Run `cargo run --bin true`.
```rust
fn main() {
    std::process::exit(0);
}
```

>The tests are **not** necessarily run in the same order they are declared in the code.This is because Rust is a safe language for writing concurrent code, which means code can be **run across multiple threads**.
>... `cargo test -- --test-threads=1` requires a single thread.

Testing the program output:
```rust
cmd.assert().success().stdout("tomorrow is well\n");
```

For `false && ls`, the result is that the first process fails and ls **never** runs.

