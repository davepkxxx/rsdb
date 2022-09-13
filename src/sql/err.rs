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

    pub fn build_excpeted_err(excpeted: &str) -> Self {
        Self::new(&format!("excpeted '{}'", excpeted))
    }

    pub fn build_miss_err(name: &str) -> Self {
        Self::new(&format!("missing {}", name))
    }

    pub fn build_required_found_err(expected_name: &str, actual_name: &str) -> Self {
        Self::new(&format!(
            "{} required, but found {}",
            expected_name, actual_name
        ))
    }
}
