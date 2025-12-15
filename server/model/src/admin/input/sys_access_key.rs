use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;
use validator::Validate;

use crate::admin::entities::sea_orm_active_enums::Status;
use utoipa::ToSchema;
#[derive(Debug, Serialize, Deserialize,ToSchema)]
#[schema(description="分页请求")]
pub struct AccessKeyPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}

#[derive(Deserialize, Validate,ToSchema)]
#[schema(description="AccessKey")]
pub struct AccessKeyInput {
    #[schema(example="域名")]
    pub domain: String,
    #[schema(example="状态")]
    pub status: Status,
    #[schema(example="描述")]
    #[validate(length(max = 200, message = "Description must not exceed 200 characters"))]
    pub description: Option<String>,
}

pub type CreateAccessKeyInput = AccessKeyInput;
