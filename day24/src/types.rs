#[derive(Debug, Clone)]
pub enum Instruction {
    Input(String),
    Add(String, String),
    Mul(String, String),
    Div(String, String),
    Mod(String, String),
    Equal(String, String),
}