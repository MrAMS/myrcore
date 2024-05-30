#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;
pub mod foodbuffer;
pub use foodbuffer::*;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
pub fn yield_call() -> isize {
    sys_yield()
}
pub fn get_time() -> isize {
    sys_get_time()
}

pub fn halt() -> isize {
    sys_halt()
}

pub fn wait_for_ms(time_ms: isize) -> isize{
    let current_timer = get_time();
    let target_time = current_timer + time_ms;
    while get_time() < target_time {
        yield_call();
    }
    target_time
}