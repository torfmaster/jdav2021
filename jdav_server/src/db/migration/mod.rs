use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use tokio::{fs::File, sync::Mutex};

use crate::models::DatabaseModel;

use super::{
    db::{Database, DATABASE_FILENAME},
    DatabaseWithVersion,
};

pub mod migration_to_v1;

pub async fn migrate() -> Result<Database, ()> {
    let file = File::open(DATABASE_FILENAME).await.map_err(|_| ())?;

    let data = from_reader::<_, DatabaseWithVersion>(file.into_std().await).map_err(|_| ())?;

    match data.database_version {
        Some(super::DatabaseVersion::V1) => {
            let file = File::open(DATABASE_FILENAME).await.map_err(|_| ())?;

            let data = from_reader::<_, DatabaseModel>(file.into_std().await).map_err(|_| ())?;

            Ok(Database {
                database: Arc::new(Mutex::new(data)),
            })
        }
        None => {
            let file = File::open(DATABASE_FILENAME).await.map_err(|_| ())?;
            let data = from_reader::<_, migration_to_v1::DatabaseModel>(file.into_std().await)
                .map_err(|_| ())?;
            Ok(Database {
                database: Arc::new(Mutex::new(data.to_v1())),
            })
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum DatabaseVersion {
    V1,
}
