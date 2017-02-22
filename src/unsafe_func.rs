pub fn halt() {
    unsafe {
        asm!("hlt");
        asm!("ret");
    }
}

pub fn write_memory<T>(addr: u32, data: T) {
    unsafe {
        *(addr as *mut T) = data;
    }
}