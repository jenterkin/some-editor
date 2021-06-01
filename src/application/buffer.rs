use ropey::Rope;

pub struct Buffer {
    pub data: Rope,
}

impl Buffer {
    pub fn new(data: String) -> Buffer {
        return Buffer {
            data: Rope::from(data),
        };
    }
}
