use anyhow::{bail, Result};
use dotenv::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppEnv {
    Development,
    Production,
    Staging,
    Test,
}

impl AppEnv {
    pub fn current_env() -> Result<AppEnv> {
        let app_environment = var("APP_ENVIRONMENT")?;

        match app_environment.as_str() {
            "development" => Ok(AppEnv::Development),
            "production" => Ok(AppEnv::Production),
            "staging" => Ok(AppEnv::Staging),
            "test" => Ok(AppEnv::Test),
            _ => {
                bail!("No Application Environment Found. At least one Required")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::env_helper::AppEnv;
    use dotenv::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::{env, io};
    use tempfile::{tempdir, TempDir};

    pub fn tempdir_with_dotenv(dotenv_text: &str) -> io::Result<TempDir> {
        let dir = tempdir()?;
        env::set_current_dir(dir.path())?;
        let dotenv_path = dir.path().join(".env");
        let mut dotenv_file = File::create(dotenv_path)?;
        dotenv_file.write_all(dotenv_text.as_bytes())?;
        dotenv_file.sync_all()?;
        Ok(dir)
    }

    pub fn make_test_dotenv() -> io::Result<TempDir> {
        tempdir_with_dotenv("APP_ENVIRONMENT=development")
    }

    #[test]
    fn current_env_should_be_working() {
        let dir = make_test_dotenv().unwrap();
        assert_eq!(var("APP_ENVIRONMENT").unwrap(), "development");
        env::set_current_dir(dir.path().parent().unwrap()).unwrap();
        let app_environment = AppEnv::current_env().unwrap();
        assert_eq!(app_environment, AppEnv::Development);
        dir.close().unwrap();
    }
}
