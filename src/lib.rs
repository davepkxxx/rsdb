#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub value: usize,
}

impl Offset {
    pub fn new(value: usize) -> Self {
        Offset { value }
    }
}
