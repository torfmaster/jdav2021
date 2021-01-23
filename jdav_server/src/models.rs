use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub struct Kilometer {
    pub kilometers: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct Id {
    pub id: Uuid,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KilometerEntry {
    pub id: Id,
    pub kilometers: Kilometer,
}

pub type UserKey = String;
pub type DatabaseModel = HashMap<UserKey, Vec<KilometerEntry>>;
