#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

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

use core::sync::atomic::{AtomicBool, Ordering};

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

pub const BUF_ADDR : usize = 0x84000000;
pub const BUF_SIZE : usize = 16;

pub struct FoodBuffer<T: Copy, const BUF_ADDR: usize, const BUF_SIZE: usize> {
    pub p_feed: usize,
    pub p_eat: usize,
    pub lock: AtomicBool,
    pub lock_feed: AtomicBool,
    pub lock_eat: AtomicBool,
    pub foods : [T; BUF_SIZE],
}

impl<T:Copy, const BUF_ADDR: usize, const BUF_SIZE: usize> FoodBuffer<T, BUF_ADDR, BUF_SIZE> {
    pub fn init(init_val: T){
        let p_buf = BUF_ADDR as *mut FoodBuffer<T, BUF_ADDR, BUF_SIZE>;
        let buf = unsafe{&mut *p_buf};
        buf.lock = AtomicBool::new(false);
        buf.lock_feed = AtomicBool::new(false);
        buf.lock_eat = AtomicBool::new(false);
        buf.p_feed = 0;
        buf.p_eat = 0;
        buf.foods = [init_val; BUF_SIZE];
    }
    pub fn get() -> &'static mut FoodBuffer<T, BUF_ADDR, BUF_SIZE> {
        let p_buf = BUF_ADDR as *mut FoodBuffer<T, BUF_ADDR, BUF_SIZE>;
        return unsafe{&mut *p_buf};
    }
    pub fn lock(self: &mut Self){
        while self.lock.
            compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_err(){
                yield_call();
            }
    }
    fn is_empty(self: &mut Self) -> bool {
        return self.p_feed == self.p_eat;
    }
    fn is_full(self: &mut Self) -> bool {
        return (self.p_feed+1)%BUF_SIZE == self.p_eat;
    }
    pub fn lock_feed(self: &mut Self){
        while self.lock_feed.
            compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_err(){
                yield_call();
        }
        while self.is_full(){
            yield_call();
        }
    }
    pub fn lock_eat(self: &mut Self){
        while self.lock_eat.
            compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_err(){
                yield_call();
        }
        while self.is_empty(){
            yield_call();
        }
    }
    pub fn unlock(self: &mut Self){
        self.lock.store(false, Ordering::Relaxed);
    }
    pub fn unlock_eat(self: &mut Self){
        self.lock_eat.store(false, Ordering::Relaxed);

    }
    pub fn unlock_feed(self: &mut Self){
        self.lock_feed.store(false, Ordering::Relaxed);
    }
    pub fn feed(self: &mut Self, new_food: T){
        self.foods[self.p_feed] = new_food;
        self.p_feed = (self.p_feed+1)%BUF_SIZE;
    }
    pub fn eat(self: &mut Self) -> T{
        let food = self.foods[self.p_eat];
        self.p_eat = (self.p_eat+1)%BUF_SIZE;
        return food;
    }
    pub fn cat(self: &mut Self, idx: usize) -> T{
        return self.foods[idx];
    }
}
