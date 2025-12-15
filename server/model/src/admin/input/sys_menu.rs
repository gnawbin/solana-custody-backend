use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;
use validator::Validate;
use crate::admin::entities::sea_orm_active_enums::{MenuType, Status};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MenuPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}

#[derive(Deserialize, Validate,ToSchema)]
#[schema(description="菜单录入")]
pub struct MenuInput {
    #[schema(example="菜单")]
    pub menu_type: MenuType,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Menu name must be between 1 and 100 characters"
    ))]
    #[schema(example="菜单名称")]
    pub menu_name: String,
    #[schema(example="图片类型")]
    pub icon_type: Option<i32>,
    #[schema(example="图片")]
    #[validate(length(max = 100, message = "Icon must not exceed 100 characters"))]
    pub icon: Option<String>,
    #[schema(example="路由名称")]
    #[validate(length(
        min = 1,
        max = 100,
        message = "Route name must be between 1 and 100 characters"
    ))]
    pub route_name: String,
    #[schema(example="路由路径")]
    #[validate(length(
        min = 1,
        max = 200,
        message = "Route path must be between 1 and 200 characters"
    ))]
    pub route_path: String,
    #[schema(example="组件")]
    #[validate(length(
        min = 1,
        max = 200,
        message = "Component must be between 1 and 200 characters"
    ))]
    pub component: String,
    #[schema(example="路径参数")]
    #[validate(length(max = 100, message = "Path param must not exceed 100 characters"))]
    pub path_param: Option<String>,
    #[schema(example="状态")]
    pub status: Status,
    #[validate(length(max = 100, message = "Active menu must not exceed 100 characters"))]
    #[schema(example="激活菜单")]
    pub active_menu: Option<String>,
    #[schema(example="隐藏菜单")]
    pub hide_in_menu: Option<bool>,
    #[validate(length(min = 1, max = 50, message = "PID must be between 1 and 50 characters"))]
    #[schema(example="菜单父ID")]
    pub pid: String,
    #[schema(example="顺序号")]
    pub sequence: i32,
    #[schema(example="国际化按键")]
    #[validate(length(max = 100, message = "i18n key must not exceed 100 characters"))]
    pub i18n_key: Option<String>,
    #[schema(example="是否活跃")]
    pub keep_alive: Option<bool>,
    #[schema(example="常数")]
    pub constant: bool,
    #[schema(example="网址",maximum=200)]
    #[validate(length(max = 200, message = "Href must not exceed 200 characters"))]
    pub href: Option<String>,
    #[schema(example="是否多签")]
    pub multi_tab: Option<bool>,
}

pub type CreateMenuInput = MenuInput;

#[derive(Deserialize, Validate)]
pub struct UpdateMenuInput {
    pub id: i32,
    #[serde(flatten)]
    pub menu: MenuInput,
}
