use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Default)]
pub struct Kilometer {
    pub kilometers: f32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct UserAuth {
    pub name: String,
    pub pass: String,
}

impl Default for UserAuth {
    fn default() -> Self {
        UserAuth {
            name: "".to_string(),
            pass: "".to_string(),
        }
    }
}
