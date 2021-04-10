use std::env;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;
use serde::Serialize;
use toml;
use toml::de::Error;

#[cfg(test)]
use crate::learn_open_source::config::Config;
use crate::learn_open_source::error::ConfigError;

mod tests;

#[derive(Debug, Serialize, Deserialize)]
struct Configs {
    configs: Vec<Config>,
}

#[test]
fn de_from_str() {
    let _a = 1;
    let toml_str = r#"
        [[configs]]
        environment = "development"
        address = "localhost"
        port = 8000
        workers = 4
        [configs.database]
        adapter = "postgresql"
        db_name = "blog_development"
        pool = 5

        [[configs]]
        environment = "staging"
        address = "localhost"
        port = 800
        workers = 5
        [configs.database]
        adapter = "postgresql"
        db_name = "blog_staging"
        pool = 5

        [[configs]]
        environment = "production"
        address = "localhost"
        port = 800
        workers = 5
        [configs.database]
        adapter = "postgresql"
        db_name = "blog_prodction"
        pool = 5
        "#;
    let res: Result<Configs, Error> = toml::from_str(toml_str);
    match res {
        Ok(config) => println!("config = {:?}", config),
        Err(e) => println!("err = {:?}", e),
    }
    de_from_file();
}

fn de_from_file() -> Result<Configs, ConfigError> {
    let path = env::current_dir().map_err(|_| ConfigError::BadEnv("".to_string()))?;
    let conf_path = path.as_path().join(r"src/learn_open_source/conf/demo.toml");
    let mut conf_file = File::open(&conf_path).map_err(|_| ConfigError::IoError)?;
    let mut txt = String::new();
    let cap = conf_file
        .read_to_string(&mut txt)
        .map_err(|_| ConfigError::IoError)?;
    let res = toml::from_str(txt.as_str()).map_err(|_| {
        ConfigError::ParseError(
            txt.clone(),
            conf_path.clone(),
            "".to_string(),
            Some((cap, cap)),
        )
    })?;
    println!("configs = {:?}", &res);
    println!("cwd = {:?}", path);
    Ok(res)
}

struct Solution {}
