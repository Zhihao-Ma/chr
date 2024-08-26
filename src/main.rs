use axum::{
    routing::{get, post},
    Router,
};
use chr_rs::{
    config::{
        config::{init_config, ApplicationConfig},
        db::init_database,
        log::init_log,
    },
    controller::login::do_login,
    model::Myorder,
    service::init_service,
    APPLICATION_CONTEXT,
};
use log::info;
use rbatis::RBatis;

#[tokio::main]
async fn main() {
    // 初始化配置
    init_config().await;
    // 初始化日志
    init_log();
    // 初始化数据库
    init_database().await;
    // 初始化服务
    init_service().await;

    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/auth-service/login/userLogin", post(do_login))
        .route("/myorder", get(get_all_myorder));

    let server = format!("{}:{}", config.server().host(), config.server().port());
    let listener = tokio::net::TcpListener::bind(server).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn get_all_myorder() -> String {
    format!("get all myorder");
    let rbatis = APPLICATION_CONTEXT.get::<RBatis>();
    let data = Myorder::select_all(rbatis).await.unwrap();
    info!("get all myorder :{:?}", data);
    format!("{:?}", data)
}
