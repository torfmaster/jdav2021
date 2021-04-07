use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Default)]
pub struct Kilometer {
    pub kilometers: f32,
}

impl fmt::Display for Kilometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} km", self.kilometers)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct Id {
    pub id: Uuid,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct KilometerEntry {
    pub id: Id,
    pub kilometers: Kilometer,
    pub kind: Kind,
}

impl Default for KilometerEntry {
    fn default() -> Self {
        Self {
            id: Id { id: Uuid::nil() },
            kilometers: Kilometer { kilometers: 0.0 },
            kind: Kind::Biking,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Kind {
    Running,
    Biking,
    Climbing,
}

pub fn get_kind_multiplier(k: &Kind) -> f32 {
    match k {
        Kind::Running => 1.0,
        Kind::Biking => 0.1,
        Kind::Climbing => 10.0,
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Kind::Biking => write!(f, "Radeln"),
            Kind::Climbing => write!(f, "Klettern"),
            Kind::Running => write!(f, "Laufen"),
        }
    }
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
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct Entries {
    pub list: Vec<KilometerEntry>,
}
