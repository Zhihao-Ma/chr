use axum::response::IntoResponse;
use axum::Json;

use crate::model::dto::SignInDTO;
use crate::model::vo::RespVO;
use crate::service::sys_user_service::SysUserService;
use crate::APPLICATION_CONTEXT;

pub async fn do_login(Json(arg): Json<SignInDTO>) -> impl IntoResponse {
    let sysUserService = APPLICATION_CONTEXT.get::<SysUserService>();
    let result = sysUserService.do_login(arg).await;
    return RespVO::from_result(result).json();
}
