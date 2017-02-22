#![feature(custom_attribute)]
#![no_std]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![feature(asm)]

mod color;
mod unsafe_func;
mod console;
mod screen;

use color::Color;

#[no_mangle]
#[no_split_stack]
pub fn main() {
    let background_color = Color::LightBlue;
    let foreground_color = Color::White;

    screen::clear(background_color);

    let msg = ['H', 'e', 'l', 'l', 'o', ',', ' ', 'F', 'a', 'n', 'm', 'i', 'O', 'S'];
    console::print(0xb8000, background_color, foreground_color, &msg);

    loop {
        unsafe_func::halt();
    }
}
