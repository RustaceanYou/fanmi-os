#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Size {
        Size {
            width: width,
            height: height,
        }
    }

    pub fn zero() -> Size {
        Size::new(0, 0)
    }
}