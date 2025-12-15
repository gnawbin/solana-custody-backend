use serde::Serialize;

use super::MenuRoute;
use utoipa::ToSchema;
#[derive(Clone, Debug, Serialize,ToSchema)]
#[schema(description="授权信息响应")]
pub struct AuthOutput {
    #[schema(example="token")]
    pub token: String,
    // 为了复用soybean-admin-nestjs前端,暂时弃用
    // pub access_token: String,
    #[schema(example="刷新token")]
    pub refresh_token: String,
}

#[derive(Debug, Serialize,ToSchema)]
#[schema(description="用户信息")]
pub struct UserInfoOutput {
    #[schema(example="用户ID")]
    #[serde(rename = "userId")]
    pub user_id: String,
    #[schema(example="用户名")]
    #[serde(rename = "userName")]
    pub user_name: String,
    #[schema(example="角色列表")]
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize,ToSchema)]
#[schema(description="用户路由")]
pub struct UserRoute {

    pub routes: Vec<MenuRoute>,
    pub home: String,
}
