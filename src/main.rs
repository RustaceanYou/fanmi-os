#![no_std]
#![feature(custom_attribute, asm)]
#![allow(dead_code, unused_attributes)]

mod color;
mod unsafe_func;
mod console;
mod screen;
mod size;
mod point;

use color::Color;
use size::Size;
use console::Console;
use unsafe_func::halt;

#[no_mangle]
#[no_split_stack]
pub unsafe fn main() {
    let mut console = Console::new(Size::new(80, 25), Color::LightBlue, Color::White);
    for _ in 0..3 {
        console.println("Hello, FanmiOS!");
    }
    loop {
        halt();
    }
}
