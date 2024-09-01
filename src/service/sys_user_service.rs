use axum::http::header;
use axum::http::Response;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::info;
use rbatis::rbdc::DateTime;
use rbatis::RBatis;

use crate::error::Error;
use crate::model::dto::SignInDTO;
use crate::model::entity::SysUser;
use crate::model::vo::jwt::Claims;
use crate::model::vo::user_info::UserInfoVO;
use crate::model::vo::RespVO;
use crate::utils::crypto_utils::md5;
use crate::APPLICATION_CONTEXT;

pub struct SysUserService {}

impl Default for SysUserService {
    fn default() -> Self {
        SysUserService {}
    }
}

impl SysUserService {
    pub async fn do_login(&self, arg: SignInDTO) -> impl IntoResponse {
        info!("login:{:?}", arg);
        let rbatis = APPLICATION_CONTEXT.get::<RBatis>();
        let user = SysUser::select_by_column(rbatis, "username", &arg.username)
            .await
            .unwrap()
            .into_iter()
            .next();

        // check password
        let user = user
            .ok_or_else(|| Error::from(String::from("用户不存在！！！")))
            .unwrap();
        if !user.password.eq(&md5(&arg.password)) {
            info!("输入的密码md5为:{}", md5(&arg.password));
            // return Err(Error::from(String::from("用户名或密码错误！！！")));
            // let error_resp = RespVO::from_error(String::from("用户名或密码错误！！！")).json()::<String>();

            return Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(String::from("用户名或密码不存在"))
                .unwrap();
        }

        let claims = Claims {
            id: user.id.clone(),
            username: user.username.clone(),
            roles: vec![],
            permissions: vec![],
            exp: DateTime::now().unix_timestamp() as usize + 30000,
        };
        let token = claims.create_token("secret");

        let user_info = UserInfoVO {
            id: user.id.clone(),
            username: user.username.clone(),
            permissions: vec![],
            roles: vec![],
        };
        let result = Ok(user_info);
        let resp = RespVO::from_result(result).json();
        let res = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", token.unwrap())
            .body(resp.to_string())
            .unwrap();
        return res;
        // return result;
    }
}
