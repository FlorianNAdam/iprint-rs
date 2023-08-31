//! # iprint-rs
//!
//! `iprint-rs` is a Rust utility library for indented printing and logging,
//! designed to help you easily trace the execution of your code. The library
//! provides a suite of macros for indented formatting (`iformat`), simplified
//! indented printing (`iprintln`), and logging with varying levels
//! (`itrace`, `idebug`, `iinfo`, `iwarn`, `ierror`). All features are geared
//! to improve debugging and code tracing by automatically adjusting indentation
//! based on the function call depth.
//!
//! ## ⚠️ Warning
//!
//! This library may not function correctly when compiled in release mode due
//! to function inlining. It is recommended to use it in debug mode for accurate results.
//!
//! ## Features
//!
//! - **iprintln! macro**: This is an enhanced version of `println!`, adding automatic indentation.
//! - **iformat! macro**: Allows for custom indented formatting.
//! - **call_depth! macro**: Provides the current depth of the function call stack,
//!   useful for custom logging or tracing solutions.
//! - **indented logging**: Offers five levels of logging (`itrace`, `idebug`, `iinfo`,
//!   `iwarn`, `ierror`) that are feature-gated by the `log` feature.
//!
//! ## Installation
//!
//! To use the library, include it in your `Cargo.toml` like so:
//!
//! ```toml
//! [dependencies]
//! iprint-rs = "0.1.1"  # Use the latest version
//! ```
//!
//! To enable the `log` feature for additional logging functionalities:
//!
//! ```toml
//! [dependencies]
//! iprint-rs = { version = "0.1.1", features = ["log"] }
//! ```
//!
//! ## Usage Examples
//!
//! ### iprintln! macro
//!
//! ```rust
//! use iprint::iprintln;
//!
//! fn my_function() {
//!     iprintln!("This is like println! but with automatic indentation.");
//!     another_function();
//!     iprintln!("This will be indented like the first iprintln!");
//! }
//!
//! fn another_function() {
//!     iprintln!("This message will be more indented.");
//! }
//! ```
//!
//! ### iformat! macro
//!
//! ```rust
//! use iprint::iformat;
//!
//! fn my_function() {
//!     let msg = iformat!("This will be indented based on call depth.");
//!     println!("{}", msg);
//! }
//! ```
//!
//! ### call_depth! macro
//!
//! ```rust
//! use iprint::call_depth;
//!
//! fn custom_logging() {
//!     let depth = call_depth!();
//!     println!("Current call depth: {}", depth);
//! }
//! ```
//!
//! ### Logging Functions (Feature-Gated)
//!
//! To use logging functions, make sure you have the `log` feature enabled.
//!
//! ```rust
//! use iprint::iinfo;
//!
//! fn yet_another_function() {
//!     iinfo!("This is an informational message with automatic indentation.");
//! }
//! ```


#[allow(unused)]
mod tests;

use std::cell::RefCell;

#[cfg(feature = "log")]
pub use log::{trace, debug, info, warn, error};

thread_local!(
    #[doc(hidden)]
    pub static STACK: RefCell<Vec<usize>> = RefCell::new(vec![])
);

#[doc(hidden)]
#[macro_export]
macro_rules! stack_ptr {
    () => ({
        let mut rsp: usize;
        unsafe {
            core::arch::asm!("mov {}, rsp", out(reg) rsp);
        }
        rsp
    })
}

/// Retrieves the current call depth of the function stack.
///
/// This macro returns an integer representing the depth of the function
/// call stack at the point where it is invoked.
///
/// # Example
///
/// ```
/// use iprint::call_depth;
///
/// fn custom_logging() {
///     let depth = call_depth!();
///     println!("Current call depth: {}", depth);
/// }
/// ```
#[macro_export]
macro_rules! call_depth {
    () => ({
        let stack_pointer = $crate::stack_ptr!();
        $crate::STACK.with(|c| {
            let mut stack = c.borrow_mut();
            while let Some(&last) = stack.last() {
                if last < stack_pointer {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.last() != Some(&stack_pointer) {
                stack.push(stack_pointer);
            }
            stack.len()
        })
    })
}

/// Formats a given string with indentation based on the current call depth.
///
/// This macro works similarly to Rust's built-in `format!` macro,
/// but prepends an indentation to the formatted string. The level of
/// indentation is determined by the current call depth in the stack.
///
/// # Example
///
/// ```
/// use iprint::iformat;
///
/// fn my_function() {
///     let msg = iformat!("This will be indented based on call depth.");
///     println!("{}", msg);
/// }
/// ```
#[macro_export]
macro_rules! iformat {
    ($($t:tt)*) => {{
        let call_depth = $crate::call_depth!();
        let indent = 4 * (call_depth - 1);
        let text = format!($($t)*);
        let indented_text: String = text
            .lines()
            .map(|line| format!("{:indent$}{}", "", line, indent=indent))
            .collect::<Vec<_>>()
            .join("\n");
        indented_text
    }}
}

/// Prints a given string with automatic indentation to the console.
///
/// This macro is an enhanced version of Rust's `println!` macro,
/// adding automatic indentation based on the current call depth.
///
/// # Example
///
/// ```
/// use iprint::iprintln;
///
/// fn another_function() {
///     iprintln!("This is like println! but with automatic indentation.");
/// }
/// ```
#[macro_export]
macro_rules! iprintln {
    ($($t:tt)*) => {
        println!("{}", $crate::iformat!($($t)*))
    }
}

#[cfg(feature = "log")]
pub mod ilog {
    /// Logs a message at various levels with automatic indentation.
    ///
    /// These macros are feature-gated and can be enabled by activating the "log" feature.
    /// They work similarly to their non-indented counterparts in the `log` crate but add
    /// automatic indentation based on the call depth.
    ///
    /// # Example
    ///
    /// ```
    /// use iprint::iinfo;
    ///
    /// fn yet_another_function() {
    ///     iinfo!("This is an informational message with automatic indentation.");
    /// }
    /// ```
    ///
    /// This macro is available only if the "log" feature is enabled.
    #[macro_export]
    macro_rules! itrace {
        ($($t:tt)*) => {
            $crate::trace!("{}", $crate::iformat!($($t)*))
        }
    }

    /// Logs a debug message with automatic indentation.
    ///
    /// This macro is an enhanced version of the `debug!` macro from the `log` crate,
    /// adding automatic indentation based on the current call depth.
    ///
    /// # Example
    ///
    /// ```rust
    /// use iprint::idebug;
    ///
    /// fn my_debug_function() {
    ///     idebug!("This is a debug message with automatic indentation.");
    /// }
    /// ```
    ///
    /// This macro is available only if the "log" feature is enabled.
    #[macro_export]
    macro_rules! idebug {
        ($($t:tt)*) => {
            $crate::debug!("{}", $crate::iformat!($($t)*))
        }
    }

    /// Logs an informational message with automatic indentation.
    ///
    /// This macro is an enhanced version of the `info!` macro from the `log` crate,
    /// adding automatic indentation based on the current call depth.
    ///
    /// # Example
    ///
    /// ```rust
    /// use iprint::iinfo;
    ///
    /// fn my_info_function() {
    ///     iinfo!("This is an informational message with automatic indentation.");
    /// }
    /// ```
    ///
    /// This macro is available only if the "log" feature is enabled.
    #[macro_export]
    macro_rules! iinfo {
        ($($t:tt)*) => {
            $crate::info!("{}", $crate::iformat!($($t)*))
        }
    }

    /// Logs a warning message with automatic indentation.
    ///
    /// This macro is an enhanced version of the `warn!` macro from the `log` crate,
    /// adding automatic indentation based on the current call depth.
    ///
    /// # Example
    ///
    /// ```rust
    /// use iprint::iwarn;
    ///
    /// fn my_warn_function() {
    ///     iwarn!("This is a warning message with automatic indentation.");
    /// }
    /// ```
    ///
    /// This macro is available only if the "log" feature is enabled.
    #[macro_export]
    macro_rules! iwarn {
        ($($t:tt)*) => {
            $crate::warn!("{}", $crate::iformat!($($t)*))
        }
    }

    /// Logs an error message with automatic indentation.
    ///
    /// This macro is an enhanced version of the `error!` macro from the `log` crate,
    /// adding automatic indentation based on the current call depth.
    ///
    /// # Example
    ///
    /// ```rust
    /// use iprint::ierror;
    ///
    /// fn my_error_function() {
    ///     ierror!("This is an error message with automatic indentation.");
    /// }
    /// ```
    ///
    /// This macro is available only if the "log" feature is enabled.
    #[macro_export]
    macro_rules! ierror {
        ($($t:tt)*) => {
            $crate::error!("{}", $crate::iformat!($($t)*))
        }
    }
}