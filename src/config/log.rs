use fast_log::{consts::LogSize, plugin::{file_split::{KeepType, Rolling, RollingType}, packer::LogPacker}, Config};
use log::info;

use crate::APPLICATION_CONTEXT;

use super::config::ApplicationConfig;

pub fn init_log() {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    std::fs::create_dir_all(&config.log().path()).unwrap();

    let log_cfg = Config::new().level(log::LevelFilter::Info).file_split(
        &config.log().path(),
        Rolling::new(RollingType::BySize(LogSize::MB(10))),
        KeepType::All,
        LogPacker{},
    ).console();
    
    fast_log::init(log_cfg).unwrap();
    info!("log init success");
}
