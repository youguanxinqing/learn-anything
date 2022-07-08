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

## Chapter 2. Test for Echo

The lack of any return type for main implies that the function returns what Rust calls the **unit** type.
- `unit type` is signified with a set of empty parentheses: ().
- `unit type` is not like a null pointer or undefined value in other language.
```rust
fn main() {
}

// equal

fn main() -> () {

}
```

`std::process` crate handles external processes. `std::env`  crate interacts with the environment.
- std::env::args gets arguments of program.
- `cargo run -- -n help` —— `Args { inner: ["target/debug/echor", "-n", "hello", "world"] }`.
- clap crate: command-lien argument parser.

Cargo places the downloaded source code into .cargo in home directory.

Disk usage command: `du -shc`.

clap basic usage:
```rust
use std::clap::App;

fn main() {
    let _matches = App.new("name of progam")
        .version("1.1.1")
        .author("who")
        .about("...")
        .get_matches();
}
```
- `_matches`, the leading undersocre in the variable name, tells Rust compiler that **I do not intend to use it right now**. Without the underscore, the compiler would warn about an unused variable.
- Automatically handles the flags -V and --version.

Clap positional argument:
```rust
Arg::with_name("xxxx")
    .value_name("XXXX")
    .help("pls ...")
    .required(true)
    .min_values(1)  // aprear at least once and can repeated
```
Clap flag argument:
```rust
Arg::with_name("xxx")
    .short("n")  // flag that has only the short name
    .help("pls ...")
    .take_value(false)  // take no value from command line.
```

`{:#?}` —— pretty printing can only print formated struct or others.