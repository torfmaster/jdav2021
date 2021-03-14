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

impl UserAuth {
    pub fn to_basic_auth_header(&self) -> String {
        let composed = format!("{}:{}", self.name, self.pass);
        let encoded = base64::encode(composed);
        format!("Basic {}", encoded)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct Highscore {
    pub list: Vec<HighscoreEntry>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct HighscoreEntry {
    pub user: String,
    pub points: f32,
}
