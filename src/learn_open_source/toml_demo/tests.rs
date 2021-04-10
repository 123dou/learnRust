use super::*;

#[test]
fn parse_toml_array_from_string() {
    let toml_str = r#"[[config]]
address = "localhost"
port = "8000"
workers = 4
enviroment = "development"


[[config]]
address = "localhost"
port = "800"
workers = 5
enviroment = "staging"

[[config]]
address = "localhost"
port = "800"
workers = 5
enviroment = "production""#;
    let _config: Result<Configs, Error> = toml::from_str(toml_str);
    if let Ok(config) = _config {
        println!("config = {:?}", config);
    }
}
