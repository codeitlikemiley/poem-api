use std::{env, fmt, io::ErrorKind, str::FromStr};

use dotenvy::{EnvLoader, EnvMap, EnvSequence};

#[derive(Debug, Clone, Copy)]
pub enum AppEnv {
    Dev,
    Prod,
}

impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEnv::Dev => write!(f, "Dev"),
            AppEnv::Prod => write!(f, "Prod"),
        }
    }
}

impl FromStr for AppEnv {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            s => Err(format!(
                "APP_ENV can only be `dev` or `prod`: {s} was provided"
            )),
        }
    }
}

impl From<AppEnv> for EnvSequence {
    fn from(v: AppEnv) -> Self {
        match v {
            AppEnv::Dev => Self::InputThenEnv,
            AppEnv::Prod => Self::EnvOnly,
        }
    }
}

pub fn load_env(path: &str) -> Result<EnvMap, std::io::Error> {
    let app_env = env::var("APP_ENV")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(AppEnv::Dev);

    EnvLoader::with_path(path)
        .sequence(app_env.into())
        .load()
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))
}
