use log::info;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;

use crate::APPLICATION_CONTEXT;

use super::config::ApplicationConfig;

pub async fn init_database() {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    info!("init rbatis pool, url:{}", config.database().url());

    let rbatis = RBatis::new();
    rbatis
        .link(MysqlDriver {}, config.database().url())
        .await
        .expect("rbatis link fail");

    APPLICATION_CONTEXT.set::<RBatis>(rbatis);
    print!("init rbatis pool success");
}
