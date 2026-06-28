#![no_std]
#![no_main]

#[macro_use]
extern crate app;

use app::yield_;

const WIDTH: usize = 10;
const HEIGHT: usize = 3;

#[unsafe(no_mangle)]
fn main() -> i32 {
    for i in 0..HEIGHT {
        for _ in 0..WIDTH {
            print!("C");
        }
        println!(" [{}/{}]", i + 1, HEIGHT);
        yield_();
    }
    println!("Test write_c OK!");
    0
}