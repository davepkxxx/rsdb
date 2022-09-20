pub trait Named {
    const NAMED: &'static str;
}

pub trait NamedEnum {
    fn name(&self) -> &'static str;
}

pub trait Values {
    fn values() -> Vec<Self> where Self: Sized;
}

pub trait Is<Rhs: ?Sized = Self> {
    fn is(&self, other: &Rhs) -> bool;

    fn is_not(&self, other: &Rhs) -> bool {
        !self.is(other)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
