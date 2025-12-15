use serde::Deserialize;
use validator::Validate;
use utoipa::ToSchema;
#[derive(Deserialize, Validate,ToSchema)]
#[schema(description="登录")]
pub struct LoginInput {
    #[schema(example="标识")]
    #[validate(length(min = 5, message = "Username cannot be empty"))]
    pub identifier: String,
    #[schema(example="密码")]
    #[validate(length(min = 6, message = "Password cannot be empty"))]
    pub password: String,
}
