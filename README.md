# iprint-rs

`iprint-rs` is a Rust utility library for indented printing and logging, designed to help you easily trace the execution of your code. The library provides a suite of macros for indented formatting (`iformat`), simplified indented printing (`iprintln`), and logging with varying levels (`itrace`, `idebug`, `iinfo`, `iwarn`, `ierror`). All features are geared to improve debugging and code tracing by automatically adjusting indentation based on the function call depth.

## ⚠️ Warning

This library may not function correctly when compiled in release mode due to function inlining. It is recommended to use it in debug mode for accurate results.


## Features

- **iprintln! macro**: This is an enhanced version of `println!`, adding automatic indentation.
- **iformat! macro**: The `iformat` macro allows for custom indented formatting.
- **call_depth! macro**: The `call_depth` macro provides the current depth of the function call stack, useful for custom logging or tracing solutions.
- **indented logging**: Provides five levels of logging (`itrace`, `idebug`, `iinfo`, `iwarn`, `ierror`) that are feature-gated by the `log` feature.

## Installation

To include `iprint-rs` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
iprint = "0.1.4"  # Use the latest version
```

To enable logging functionalities, update your `Cargo.toml` like so:

```toml
[dependencies]
iprint = { version = "0.1.4", features = ["log"] }
```

## Usage

### iprintln! macro

```rust
use iprint::iprintln;

fn my_function() {
    iprintln!("This is like println! but with automatic indentation.");
    another_function();
    iprintln!("This will be indented like the first iprintln!");
}

fn another_function() {
    indent!("This message will be more indented.");
}
```

### iformat! macro

```rust
use iprint::iformat;

fn my_function() {
    let msg = iformat!("This will be indented based on call depth.");
    println!("{}", msg);
}
```

### call_depth! macro

```rust
use iprint::call_depth;

fn custom_logging() {
    let depth = call_depth!();
    println!("Current call depth: {}", depth);
}
```

### logging functions (feature-gated)

To use logging functions, make sure you have the `log` feature enabled.

```rust
use iprint::iinfo;

fn yet_another_function() {
    iinfo!("This is an informational message with automatic indentation.");
}
```

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.
