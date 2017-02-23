pub unsafe fn halt() {
    asm!("hlt");
    asm!("ret");
}

pub unsafe fn write_memory<T>(addr: u32, data: T) {
    *(addr as *mut T) = data;
}

pub unsafe fn inb(port: u32) -> u8 {
    let result: u8;
    asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    result
}

pub unsafe fn outb(value: u8, port: u32) {
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}

pub unsafe fn inw(port: u32) -> u16 {
    let result: u16;
    asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile");
    result
}

pub unsafe fn outw(value: u16, port: u32) {
    asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile");
}

pub unsafe fn inl(port: u32) -> u32 {
    let result: u32;
    asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile");
    result
}

pub unsafe fn outl(value: u32, port: u32) {
    asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
}