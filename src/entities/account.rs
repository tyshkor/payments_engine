use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Account {
    pub id: u16,
    pub available: f32,   // using f32 here and later for non-integer values as it is 6-digit precise, and the smaller in-built float type
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}

impl Account {
    pub fn new(id: u16) -> Self {
        Account {
            id: id,
            available: 0.0,   // using f32 here and later for non-integer values as it is 6-digit precise, and the smaller in-built float type
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}