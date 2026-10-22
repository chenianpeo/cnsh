use std::{env, fs::create_dir_all, path::PathBuf};

use crate::error::Error;

#[derive(Debug)]
pub struct InitData {
    pub env: ShellEnv,
}

impl InitData {
    pub fn initialize() -> Result<Self, Error> {
        let env = ShellEnv::load()?;

        Ok(Self { env })
    }
}

#[derive(Debug)]
pub struct ShellEnv {
    pub current_dir: PathBuf,
    pub home_dir: PathBuf,
    pub data_dir: PathBuf,
}

impl ShellEnv {
    #[cfg(target_os = "linux")]
    pub fn load() -> Result<Self, Error> {
        let current_dir = env::current_dir()?;

        let home_dir = env::home_dir().ok_or(Error::Unfinished)?;

        let data_dir = home_dir.join(".cnsh");
        create_dir_all(&data_dir)?;

        Ok(Self {
            current_dir,
            home_dir,
            data_dir,
        })
    }

    #[cfg(target_os = "windows")]
    fn load() -> Result<Self, Error> {
        let current_dir = env::current_dir()?;

        let home_path = std::env::var("LOCALAPPDATA").map_err(|_| Error::Unfinished)?;
        let home_dir = PathBuf::from(home_path);

        let data_dir = home_dir.join("cnsh");
        create_dir_all(&data_dir)?;

        Ok(Self {
            current_dir,
            home_dir,
            data_dir,
        })
    }
}
