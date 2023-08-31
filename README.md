# iprint-rs

iprint-rs is a Rust utility library aimed at providing indented text printing based on function call depth. This is particularly useful for debugging, logging, or tracing the flow of code execution. The library offers an `indent` macro for custom indented text and an `iprintln` macro for simplified indented printing. Additionally, feature-gated logging functions `itrace`, `idebug`, `iinfo`, `iwarn`, `ierror` are available when the `log` feature is enabled.

## Features

- **indent macro**: Automatically indents text based on the depth of the function call stack.
- **iprintln macro**: A simplified version of `println!` that includes automatic indentation.
- **indented logging**: Five different levels of logging (`itrace`, `idebug`, `iinfo`, `iwarn`, `ierror`), feature-gated by the `log` feature flag.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
iprintln = "0.1.0"  # Use the latest version
```

To enable the logging features, add this instead:

```toml
[dependencies]
iprintln = { version = "0.1.0", features = ["log"] }
```

## Usage

### indent macro

```rust
use iprint::indent;

fn my_function() {
    indent!("This is a debug message.");
    another_function();
}

fn another_function() {
    indent!("This message will be more indented.");
}
```

### iprintln macro

```rust
use iprint::iprintln;

fn my_function() {
    iprintln!("This is similar to a println! but indented.");
}
```

### Logging Functions (feature-gated)

To enable, make sure you have the `log` feature enabled in your `Cargo.toml`.

```rust
use iprint::iinfo;

fn my_function() {
    iinfo!("This is an info message with indentation.");
}
```

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.

