#![no_std]
#![no_main]

#[macro_use]
extern crate ulib;

use ulib::{FoodBuffer, BUF_ADDR, BUF_SIZE, wait_for_ms};


#[no_mangle]
fn main() -> i32 {
    println!("p2 start");
    let buf = FoodBuffer::<u32, BUF_ADDR, BUF_SIZE>::get();
    let mut i: u32 = 2000;
    loop {
        println!("p2 wakeup at {}", wait_for_ms(2000));
        buf.lock_feed();
        buf.lock();
        println!("p2 feed {}", i);
        buf.feed(i);
        buf.unlock();
        buf.unlock_feed();
        i += 1;
    }
    // println!("p2 exit");
    // 0
}
