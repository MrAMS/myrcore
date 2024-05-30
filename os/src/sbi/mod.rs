//! SBI 

/// SBI Methods
pub trait SBIMethod {
    /// use sbi call to putchar in console (qemu uart handler)
    fn console_putchar(ch: usize);
    /// use sbi call to set timer
    fn set_timer(timer: usize);
    /// use sbi call to shutdown the kernel
    fn shutdown(failure: bool) -> !;
}

mod rustsbi;
pub use rustsbi::SBI;