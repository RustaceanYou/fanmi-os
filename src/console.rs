use color::Color;
use unsafe_func;

pub fn putchar(addr: u32, background_color: Color, foreground_color: Color, chr: char) {
    let data: u16 = ((background_color as u16) << 12) | ((foreground_color as u16) << 8) |
                    (chr as u16);
    unsafe_func::write_memory::<u16>(addr, data);
}

pub fn print(addr: u32, background_color: Color, foreground_color: Color, msg: &[char]) {
    let mut index = 0;
    for chr in msg {
        putchar(addr + index, background_color, foreground_color, *chr);
        index += 2;
    }
}