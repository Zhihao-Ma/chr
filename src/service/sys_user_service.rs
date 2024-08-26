use log::info;
use rbatis::RBatis;

use crate::error::Error;
use crate::error::Result;
use crate::model::dto::SignInDTO;
use crate::model::entity::SysUser;
use crate::utils::crypto_utils::md5;
use crate::APPLICATION_CONTEXT;

pub struct SysUserService {}

impl Default for SysUserService {
    fn default() -> Self {
        SysUserService {}
    }
}

impl SysUserService {
    pub async fn do_login(&self, arg: SignInDTO) -> Result<String> {
        info!("login:{:?}", arg);
        let rbatis = APPLICATION_CONTEXT.get::<RBatis>();
        let user = SysUser::select_by_column(rbatis, "username", &arg.username)
            .await?
            .into_iter()
            .next();

        // check password
        let user = user.ok_or_else(|| Error::from(String::from("用户不存在！！！")))?;
        if !user.password.eq(&md5(&arg.password)) {
            info!("输入的密码md5为:{}", md5(&arg.password));
            return Err(Error::from(String::from("用户名或密码错误！！！")));
        }

        let result = Ok(String::from("success"));
        return result;
    }
}
