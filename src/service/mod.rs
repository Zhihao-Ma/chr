pub mod sys_user_service;
use crate::service::sys_user_service::SysUserService;

use crate::APPLICATION_CONTEXT;

pub async fn init_service() {
    APPLICATION_CONTEXT.set::<SysUserService>(SysUserService::default());
}
