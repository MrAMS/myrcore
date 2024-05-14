#![no_std]
#![no_main]

#[macro_use]
extern crate ulib;

use ulib::{FoodBuffer, BUF_ADDR, BUF_SIZE, wait_for_ms};

#[no_mangle]
fn main() -> i32 {
    println!("c1 start");
    let buf = FoodBuffer::<u32, BUF_ADDR, BUF_SIZE>::get();
    loop {
        println!("c1 wakeup at {}", wait_for_ms(5000));
        buf.lock_eat();
        buf.lock();
        println!("c1 eat {}", buf.eat());
        buf.unlock();
        buf.unlock_eat();
    }
    // println!("c1 exit");
    // 0
}
