//! This task implements creating the open_api doc for
//! generated types in the `frontend/src/domain` directory
//!
//! # Example
//!
//! Run the task with the following command:
//! ```sh
//! cargo run task
//! ```
//!
//! To override existing data and reset the data structure, use the following
//! command with the `refresh:true` argument:
//! ```sh
//! cargo run task gen_api
//! ```

use loco_rs::prelude::*;
use std::{fs::{File, self}, io::{BufWriter, Write}};
use std::process::Command;
use utoipa::OpenApi;

use crate::controllers::OpenAPI;

#[allow(clippy::module_name_repetitions)]
pub struct GenSwagger;
#[async_trait]
impl Task for GenSwagger {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "gen_api".to_string(),
            detail: "Generates an OpenAPI doc and runs the openapi-generator-cli. Check `frontend/src/domain` for the output".to_string(),
        }
    }

    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let file = File::create("open_api.json")?;
        let output_dir = "frontend/src/domain";
        let mut writer = BufWriter::new(file);
        let prett_json = OpenAPI::openapi();
        tracing::info!("{}", prett_json.to_pretty_json()?);
        serde_json::to_writer(&mut writer, &prett_json)?;
        tracing::info!("Open API Doc Written");
        writer.flush()?;

        fs::create_dir_all(output_dir).unwrap_or_else(|_|{
            tracing::warn!("Directory {} does not exist. Creating it...", output_dir);
            fs::create_dir_all(output_dir).unwrap_or_else(|err| tracing::error!("{}",err));
        });

        match Command::new("openapi-generator-cli")
            .arg("generate")
            .arg("-g")
            .arg("typescript-fetch")
            .arg("-o")
            .arg("frontend/src/domain")
            .arg("-i")
            .arg("open_api.json")
            .spawn() {
                Ok(_) =>{
                    Ok(tracing::info!("Generated Client Successfully"))
                },
                Err(err) => {
                    Ok(tracing::error!("{}", err))
                }
            }
    }
}
