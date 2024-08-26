use tokio::fs::read_to_string;

use crate::APPLICATION_CONTEXT;

pub async fn init_config(){
    let content = read_to_string("application.yml").await.unwrap();
    let config = match serde_yaml::from_str(&content){
        Ok(config) => config,
        Err(e) => panic!("Failed to parse config: {}", e),
    };
    
    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}

#[derive(Debug, serde::Deserialize,Getters, Setters)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ApplicationConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
}

#[derive(Debug, serde::Deserialize,Getters, Setters)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

#[derive(Debug, serde::Deserialize,Getters, Setters)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct DatabaseConfig {
    pub url: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize,Getters, Setters)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct LogConfig {
    pub level: String,
    pub path: String,
}