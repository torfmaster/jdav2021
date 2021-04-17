use chrono::prelude::*;
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
    pub timestamp: DateTime<Utc>,
}

impl Default for KilometerEntry {
    fn default() -> Self {
        Self {
            id: Id { id: Uuid::nil() },
            kilometers: Kilometer { kilometers: 0.0 },
            kind: Kind::Biking,
            timestamp: Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Kind {
    Running,
    Biking,
    Climbing,
    Skating,
    Hiking,
    Swimming,
}

impl Kind {
    pub fn get_kind_multiplier(&self) -> f32 {
        match self {
            Kind::Running => 1.0,
            Kind::Biking => 0.25,
            Kind::Climbing => 100.0,
            Kind::Skating => 0.75,
            Kind::Hiking => 2.0,
            Kind::Swimming => 10.0,
        }
    }

    pub fn get_path(&self) -> String {
        match self {
            Kind::Running => "laufen",
            Kind::Biking => "radfahren",
            Kind::Climbing => "klettern",
            Kind::Skating => "skaten",
            Kind::Hiking => "wandern",
            Kind::Swimming => "schwimmen",
        }
        .to_owned()
    }

    pub fn from_string(input: &str) -> Option<Self> {
        match input {
            "laufen" => Some(Kind::Running),
            "radfahren" => Some(Kind::Biking),
            "klettern" => Some(Kind::Climbing),
            "skaten" => Some(Kind::Skating),
            "schwimmen" => Some(Kind::Swimming),
            "wandern" => Some(Kind::Hiking),
            _ => None,
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Kind::Biking => write!(f, "Radeln"),
            Kind::Climbing => write!(f, "Klettern"),
            Kind::Running => write!(f, "Laufen"),
            Kind::Skating => write!(f, "Skaten"),
            Kind::Hiking => write!(f, "Wandern"),
            Kind::Swimming => write!(f, "Schwimmen"),
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

#[cfg(test)]
mod test {
    use super::Kind;

    #[test]
    pub fn does_conversion() {
        assert_eq!(
            Some(Kind::Climbing),
            Kind::from_string(&Kind::Climbing.get_path()),
        );
        assert_eq!(
            Some(Kind::Hiking),
            Kind::from_string(&Kind::Hiking.get_path()),
        );
        assert_eq!(
            Some(Kind::Running),
            Kind::from_string(&Kind::Running.get_path()),
        );
        assert_eq!(
            Some(Kind::Swimming),
            Kind::from_string(&Kind::Swimming.get_path()),
        );
        assert_eq!(
            Some(Kind::Biking),
            Kind::from_string(&Kind::Biking.get_path()),
        );
        assert_eq!(
            Some(Kind::Skating),
            Kind::from_string(&Kind::Skating.get_path()),
        );
    }

    #[test]
    pub fn rejects_unknown() {
        assert_eq!(None, Kind::from_string("wurst"),);
    }
}
