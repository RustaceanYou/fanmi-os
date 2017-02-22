use color::Color;
use unsafe_func;

pub fn clear(background: Color) {
    let base_addr = 0xb8000;
    for i in 0..80 * 25 {
        unsafe_func::write_memory::<u32>(base_addr + i * 2, (background as u32) << 12);
    }
}