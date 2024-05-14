#![no_std]
#![no_main]

#[macro_use]
extern crate ulib;

use ulib::halt;
use ulib::{FoodBuffer, BUF_ADDR, BUF_SIZE, wait_for_ms};

#[no_mangle]
fn main() -> i32 {
    FoodBuffer::<u32, BUF_ADDR, BUF_SIZE>::init(0);
    println!("Monitor start");
    let buf = FoodBuffer::<u32, BUF_ADDR, BUF_SIZE>::get();
    for _ in 1..10{
        wait_for_ms(1000);
        buf.lock();
        println!("[Monitor] Buf feed at {}, eat at {}", buf.p_feed, buf.p_eat);
        buf.unlock();
    }
    println!("[Monitor] Time to exit");
    halt();
    0
}
