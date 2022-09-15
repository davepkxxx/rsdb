#[derive(Debug)]
pub struct SyntaxError {
    pub msg: String,
}

impl SyntaxError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: format!("SyntaxError: {}", msg),
        }
    }

    pub fn new_excpeted(excpeted: &str) -> Self {
        Self::new(&format!("excpeted '{}'", excpeted))
    }

    pub fn new_missing(name: &str) -> Self {
        Self::new(&format!("missing {}", name))
    }
}
