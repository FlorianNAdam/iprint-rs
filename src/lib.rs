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
//! iprint = "0.1.4"  # Use the latest version
//! ```
//!
//! To enable the `log` feature for additional logging functionalities:
//!
//! ```toml
//! [dependencies]
//! iprint = { version = "0.1.4", features = ["log"] }
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
//! #[cfg(feature = "log")]
//! {
//!     use iprint::iinfo;
//!
//!     fn yet_another_function() {
//!         iinfo!("This is an informational message with automatic indentation.");
//!     }
//! }
//! ```

use std::cell::RefCell;

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
/// fn main() {
///     // Call depth here should be 0, since this is the main function.
///     assert_eq!(call_depth!(), 0);
///
///     // Call first_function(), this will increase the call depth.
///     first_function();
///
///     // Call depth here should be 0 again
///     assert_eq!(call_depth!(), 0);
/// }
///
/// fn first_function() {
///     // Call depth here should be 1, since we're one function deep from main.
///     assert_eq!(call_depth!(), 1);
///
///     // Call second_function(), this will increase the call depth.
///     second_function();
///
///     // Call depth here should be 1 again
///     assert_eq!(call_depth!(), 1);
/// }
///
/// fn second_function() {
///     // Call depth here should be 2, since we're two functions deep from main.
///     assert_eq!(call_depth!(), 2);
/// }
/// ```
#[macro_export]
macro_rules! call_depth {
    () => {{
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
            stack.len() - 1
        })
    }};
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
        let indent = 4 * call_depth;
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
    /// Logs a trace message with automatic indentation.
    ///
    /// This macro is an enhanced version of the `trace!` macro from the `log` crate,
    /// adding automatic indentation based on the current call depth.
    ///
    /// # Example
    ///
    /// ```
    /// #[cfg(feature = "log")]
    /// {
    ///     use iprint::itrace;
    ///
    ///     fn my_trace_function() {
    ///         itrace!("This is an informational message with automatic indentation.");
    ///     }
    /// }
    /// ```
    /// This macro is available only if the "log" feature is enabled.
    #[macro_export]
    macro_rules! itrace {
        ($($t:tt)*) => {
            trace!("{}", $crate::iformat!($($t)*))
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
    /// #[cfg(feature = "log")]
    /// {
    ///     use iprint::idebug;
    ///
    ///     fn my_debug_function() {
    ///         idebug!("This is a debug message with automatic indentation.");
    ///     }
    /// }
    /// ```
    ///
    /// This macro is available only if the "log" feature is enabled.
    #[macro_export]
    macro_rules! idebug {
        ($($t:tt)*) => {
            debug!("{}", $crate::iformat!($($t)*))
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
    /// #[cfg(feature = "log")]
    /// {
    ///     use iprint::iinfo;
    ///
    ///     fn my_info_function() {
    ///     iinfo!("This is an informational message with automatic indentation.");
    /// }
    /// }
    /// ```
    ///
    /// This macro is available only if the "log" feature is enabled.
    #[macro_export]
    macro_rules! iinfo {
        ($($t:tt)*) => {
            info!("{}", $crate::iformat!($($t)*))
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
    ///#[cfg(feature = "log")]
    /// {
    ///     use iprint::iwarn;
    ///
    ///     fn my_warn_function() {
    ///         iwarn!("This is a warning message with automatic indentation.");
    ///     }
    /// }
    /// ```
    ///
    /// This macro is available only if the "log" feature is enabled.
    #[macro_export]
    macro_rules! iwarn {
        ($($t:tt)*) => {
            warn!("{}", $crate::iformat!($($t)*))
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
    /// #[cfg(feature = "log")]
    /// {
    ///     use iprint::ierror;
    ///
    ///     fn my_error_function() {
    ///         ierror!("This is an error message with automatic indentation.");
    ///     }
    /// }
    /// ```
    ///
    /// This macro is available only if the "log" feature is enabled.
    #[macro_export]
    macro_rules! ierror {
        ($($t:tt)*) => {
            error!("{}", $crate::iformat!($($t)*))
        }
    }
}
