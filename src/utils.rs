use std::{io::Read, path::Path };

use surrealdb::{Surreal, engine::remote::ws::Client};
use tracing::{error, debug};
use serde::{Serialize, Deserialize};

use crate::{data::common::{MKRepositoryError, MKInternalServiceError, MKError, MKErrorKind}, MiraculousKingdomDatabaseConfig};

#[axum::async_trait]
pub trait FromRawModel<T> {
    async fn resolve(
        &self,
        config: &MiraculousKingdomDatabaseConfig,
    ) -> Result<T, MKRepositoryError>;
} 

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseResult<T>
{
    pub result: Vec<T>,
    pub status: String,
    pub time: String,
}


pub async fn migrate(config: &MiraculousKingdomDatabaseConfig) 
-> Result<(), MKRepositoryError> {
    tracing::debug!("Starting Migration");

    // load ../mk_surrealdb/migrations.surql as a String
    let mut file = std::fs::File::open(Path::new("mk_surrealdb/migrations.surql")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    reqwest::Client::new()
        .post("http://localhost:3100/sql")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Ns", config.ns.clone())
        .header("Db", config.db.clone())
        .header("UserAgent", "miraculous_kingdom")
        .basic_auth(
            config.username.clone(),
            Some(config.password.clone()
        ))
        .body(contents)
        .send()
        .await
        .map_err(|e| MKRepositoryError::from(e.to_string().as_str()))?;    

    Ok(())
}

pub async fn get_all<T>(
    config: &MiraculousKingdomDatabaseConfig,
    table: &str,
) -> Result<Vec<T>, MKInternalServiceError>
where
    T: serde::Serialize +
    for<'de> serde::Deserialize<'de> +
    Clone,

{
    let sql = format!("SELECT * FROM {}", table);

    let response = reqwest::Client::new()
        .post("http://localhost:3100/sql")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Ns", config.ns.clone())
        .header("Db", config.db.clone())
        .header("UserAgent", "miraculous_kingdom")
        .basic_auth(
            config.username.clone(),
            Some(config.password.clone()
        ))
        .body(sql)
        .send()
        .await
        .map_err(|e| MKInternalServiceError::from(e.to_string().as_str()))?;
    
    let json_obj = response.text().await.unwrap();
    
    let res: Vec<DatabaseResult<T>> = match serde_json::from_str(&json_obj) {
        Ok(r) => r,
        Err(e) => return Err(MKInternalServiceError::from(e.to_string().as_str()))
    };

    let res = &res.get(0).unwrap().result;

    Ok(res.clone())
}

pub async fn get_by_id<T>(
    config: &MiraculousKingdomDatabaseConfig,
    table: &str,
    id: &str,
) -> Result<T, MKRepositoryError>
where
    T: serde::Serialize +
    for<'de> serde::Deserialize<'de> +
    Send +
    Clone,
{

    let sql = format!("SELECT * FROM {}:{})", table, id);

    let response = reqwest::Client::new()
        .post("http://localhost:3100/sql")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Ns", config.ns.clone())
        .header("Db", config.db.clone())
        .header("UserAgent", "miraculous_kingdom")
        .basic_auth(
            config.username.clone(),
            Some(config.password.clone()
        ))
        .body(sql)
        .send()
        .await
        .map_err(|e| MKRepositoryError::from(e.to_string().as_str()))?;

    let selected = response
        .json::<Vec<DatabaseResult<T>>>()
        .await
        .map_err(|e| MKRepositoryError::from(e.to_string().as_str()))?;
    
    if selected.get(0).unwrap().result.is_empty() {
        return Err(MKRepositoryError::from("Did not expect empty result from SurrealDB."));
    } else if selected.get(0).unwrap().result.len() != 1 {
        return Err(MKRepositoryError::from(
                format!("Recieved {} results from SurrealDB.",
                        selected.get(0).unwrap().result.len()
                ).as_str()));
    } 
    
    let result = selected.get(0).unwrap().result.get(0).unwrap();

    Ok(result.clone())
}

pub async fn get_many_by_ids<'a, T>(
    config: &MiraculousKingdomDatabaseConfig,
    table: &str,
    ids: &[&str],
) -> Result<Vec<T>, MKRepositoryError>
where
    T: serde::Serialize +
    for<'de> serde::Deserialize<'de> +
    Send +
    Clone +
    'static,
{
    // Create a vector to collect the results of each task
    let mut results = Vec::new();

    // Iterate over the IDs and spawn a Tokio task for each one
    for id in ids {
        let config = config.clone(); // Clone the config for each task
        let table = table.to_owned(); // Clone the table for each task
        let id = id.clone().to_owned(); // Clone the ID for each task

        // Spawn a Tokio task to fetch data by ID
        let task = tokio::spawn(async move {
            get_by_id(&config, &table, &id).await
        });

        results.push(task);
    }

    // Collect the results of all tasks
    let mut final_results = Vec::new();
    for result in results {
        let item = match result.await {
            Ok(i) => i?,
            Err(e) => {
                error!("{}", e);
                return Err(MKRepositoryError::from(e.to_string().as_str()))
                
            }
        };
        final_results.push(item);
    }

    Ok(final_results)
}
