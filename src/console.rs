use color::Color;
use unsafe_func::inb;
use unsafe_func::outb;
use unsafe_func::write_memory;
use size::Size;
use screen;
use point::Point;

pub struct Console {
    width: u16,
    height: u16,
    base_addr: u32,
    background_color: Color,
    foreground_color: Color,
    cursor_pos: Point,
}

impl Console {
    pub fn current_addr(&self) -> u32 {
        match self.cursor_pos {
            Point { x, y } => self.base_addr + (((self.width * y) + x) * 2) as u32,            
        }
    }

    pub unsafe fn new(size: Size, background_color: Color, foreground_color: Color) -> Console {
        let console = Console {
            base_addr: 0xb8000,
            width: size.width,
            height: size.height,
            background_color: background_color,
            foreground_color: foreground_color,
            cursor_pos: Point::zero(),
        };
        screen::clear(size, background_color);
        console.set_cursor(Point::zero());
        console
    }

    unsafe fn update_cursor(&self) {
        self.set_cursor(self.cursor_pos);
    }

    pub unsafe fn putchar(&mut self, chr: u8) {
        let data: u16 = ((self.background_color as u16) << 12) |
                        ((self.foreground_color as u16) << 8) |
                        (chr as u16);
        if chr == '\r' as u8 {
            self.cursor_pos = Point::new(0, self.cursor_pos.y);
        } else if chr == '\n' as u8 {
            self.cursor_pos = Point::new(0, self.cursor_pos.y + 1);
        } else {
            write_memory::<u16>(self.current_addr(), data);
            self.cursor_pos = Point::new(self.cursor_pos.x + 1, self.cursor_pos.y);
            if self.cursor_pos.x >= 80 {
                self.cursor_pos = Point::new(0, self.cursor_pos.y + 1);
            }
        }
        self.update_cursor();
    }

    pub unsafe fn print(&mut self, msg: &str) {
        for chr in msg.as_bytes() {
            self.putchar(*chr);
        }
    }

    pub unsafe fn println(&mut self, msg: &str) {
        for chr in msg.as_bytes() {
            self.putchar(*chr);
        }
        self.putchar('\n' as u8);
    }

    pub unsafe fn set_cursor(&self, pos: Point) {
        let position: u16 = (pos.y * 80) + pos.x;
        // cursor LOW port to vga INDEX register
        outb(0x0F, 0x3D4);
        outb((position & 0xFF) as u8, 0x3D5);
        // cursor HIGH port to vga INDEX register
        outb(0x0E, 0x3D4);
        outb(((position >> 8) & 0xFF) as u8, 0x3D5);
    }

    pub unsafe fn enable_cursor(&self) {
        outb(0x0A, 0x3D4);
        let curstart: u8 = inb(0x3D5) & 0x1F; // get cursor scanline start
        outb(0x0A, 0x3D4);
        outb((curstart | 0x20) as u8, 0x3D5); // set enable bit
    }
}
