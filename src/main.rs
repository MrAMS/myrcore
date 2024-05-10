#![no_std]
#![no_main]
mod lang_items;
mod sbi;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle] // 避免编译器对名字进行混淆
pub fn rust_main() -> ! {
    clear_bss();
    sbi::console_putchar('H' as usize);
    sbi::shutdown(false);
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
