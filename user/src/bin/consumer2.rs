#![no_std]
#![no_main]

#[macro_use]
extern crate ulib;

use ulib::{FoodBuffer, BUF_ADDR, BUF_SIZE, wait_for_ms};

#[no_mangle]
fn main() -> i32 {
    println!("c2 start");
    // let p_buf = BUF_ADDR as *mut FoodBuffer<T, BUF_ADDR, BUF_SIZE>;
    // let buf = unsafe{&mut *p_buf};
    let buf = FoodBuffer::<u32, BUF_ADDR, BUF_SIZE>::get();
    loop {
        println!("c2 wakeup at {}", wait_for_ms(1000));
        buf.lock_eat();
        buf.lock();
        println!("c2 eat {}", buf.eat());
        buf.unlock();
        buf.unlock_eat();
    }
    // println!("c2 exit");
    // 0
}
