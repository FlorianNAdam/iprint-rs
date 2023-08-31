use std::cell::RefCell;

#[cfg(feature="log")]
pub use log::{trace, debug, info, warn, error};

thread_local!(pub static STACK: RefCell<Vec<usize>> = RefCell::new(vec![]));

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

#[macro_export]
macro_rules! indent {
    ($($t:tt)*) => {{
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
            let indent = 4 * (stack.len() - 1);
            let text = format!($($t)*);
            let indented_text: String = text
                .lines()
                .map(|line| format!("{:indent$}{}", "", line, indent=indent))
                .collect::<Vec<_>>()
                .join("\n");
            indented_text
        })
    }}
}

#[macro_export]
macro_rules! iprintln {
    ($($t:tt)*) => {
        println!("{}", $crate::indent!($($t)*))
    }
}

#[cfg(feature="log")]
pub mod ilog {
    #[macro_export]
    macro_rules! itrace {
        ($($t:tt)*) => {
            $crate::trace!("{}", $crate::indent!($($t)*))
        }
    }

    #[macro_export]
    macro_rules! idebug {
        ($($t:tt)*) => {
            $crate::debug!("{}", $crate::indent!($($t)*))
        }
    }

    #[macro_export]
    macro_rules! iinfo {
        ($($t:tt)*) => {
            $crate::info!("{}", $crate::indent!($($t)*))
        }
    }

    #[macro_export]
    macro_rules! iwarn {
        ($($t:tt)*) => {
            $crate::warn!("{}", $crate::indent!($($t)*))
        }
    }

    #[macro_export]
    macro_rules! ierror {
        ($($t:tt)*) => {
            $crate::warn!("{}", $crate::indent!($($t)*))
        }
    }
}