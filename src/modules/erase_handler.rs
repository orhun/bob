use crate::models::Config;

use super::utils;
use anyhow::{anyhow, Result};
use tokio::fs;
use tracing::info;

pub async fn start(config: Config) -> Result<()> {
    let downloads = utils::get_downloads_folder(&config).await?;
    let installation_dir = utils::get_installation_folder(&config)?;

    if fs::remove_dir_all(&installation_dir).await.is_ok() {
        info!("Successfully removed neovim's installation folder");
    }
    if fs::remove_dir_all(downloads).await.is_ok() {
        // For some weird reason this check doesn't really work for downloads folder
        // as it keeps thinking the folder exists and it runs with no issues even tho the folder
        // doesn't exist damn...
        info!("Successfully removed neovim downloads folder");
    } else {
        return Err(anyhow!("There's nothing to erase"));
    }

    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            use winreg::RegKey;
            use winreg::enums::*;

            let current_usr = RegKey::predef(HKEY_CURRENT_USER);
            let env = current_usr.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;
            let usr_path: String = env.get_value("Path")?;
            if usr_path.contains("neovim") {
                let usr_path = usr_path.replace(&format!("{}\\bin", installation_dir.display()), "");
                env.set_value("Path", &usr_path)?;

                info!("Successfully removed neovim's installation PATH from registry");
            }

        }
    }

    Ok(())
}
