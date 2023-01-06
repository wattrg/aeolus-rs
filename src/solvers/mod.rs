use serde_derive::{Serialize, Deserialize};

pub mod explicit;

#[derive(Debug, Serialize, Deserialize)]
pub enum Solvers {
    Explicit,
}
