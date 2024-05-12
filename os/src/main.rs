#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod logging;

pub mod batch;
mod sync;
pub mod syscall;
pub mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));


#[no_mangle] // 避免编译器对名字进行混淆
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    println!("Hello, world!");
    debug!("Hello, world!");
    error!("Shutdown machine!");
    trap::init();
    batch::init();
    batch::run_next_app();
    // sbi::shutdown(false);
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    // a..b => ranger [a, b)
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
