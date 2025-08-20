mod atlas_cli;

use anyhow::{Context, Result};
use tokio::fs::read_to_string;

use crate::atlas_cli::{config::Config, path::cli_config_file_path};

#[tokio::main]
async fn main() -> Result<()> {
    // Get the path to the config file
    let config_file_path = cli_config_file_path().context("get config file path")?;

    // Read the config file
    // We're using the tokio library to read the config file asynchronously
    // .await is used to wait for the file to be read
    let config_file_contents = read_to_string(config_file_path)
        .await
        .context("read config file")?;

    // Parse the config file
    // The `parse` method is defined on the `FromStr` trait, which is implemented for `Config`
    // This allows us to use the `parse` method to convert the config file contents into a `Config` struct
    let config: Config = config_file_contents.parse().context("parse config")?;

    // Print the config using the pretty print macro
    println!("config: {:#?}", config);

    Ok(())
}
