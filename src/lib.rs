pub trait Named {
    const NAMED: &'static str;
}

pub trait NamedEnum {
    fn name(&self) -> &'_ str;
}

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub value: usize,
}

impl Offset {
    pub fn new(value: usize) -> Self {
        Offset { value }
    }

    pub fn increment(&mut self, step: usize) -> &mut Self {
        self.value += step;
        self
    }
}
