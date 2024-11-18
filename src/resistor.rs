#[derive(Debug)]
pub struct Resistor {
    pub value: f64,
    pub tolerance: f64,
}

impl Resistor {
    pub fn new(value: f64, tolerance: f64) -> Self {
        Self { value, tolerance }
    }
}
