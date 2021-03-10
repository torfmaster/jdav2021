use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Default)]
pub struct Kilometer {
    pub kilometers: f32,
}
