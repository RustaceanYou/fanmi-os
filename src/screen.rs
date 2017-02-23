use color::Color;
use unsafe_func::write_memory;
use size::Size;

pub unsafe fn clear(size: Size, background: Color) {
    let base_addr = 0xb8000;
    let width = size.width;
    let height = size.height;
    for i in 0..width * height {
        write_memory::<u32>(base_addr + i as u32 * 2, (background as u32) << 12);
    }
}