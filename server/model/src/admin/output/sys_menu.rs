use chrono::NaiveDateTime;
use serde::Serialize;

use crate::admin::entities::sea_orm_active_enums::{MenuType, Status};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone,ToSchema)]
#[schema(description="授权信息响应")]
pub struct MenuRoute {
    #[schema(example="菜单名称")]
    pub name: String,
    #[schema(example="菜单路径")]
    pub path: String,
    #[schema(example="组件")]
    pub component: String,
    pub meta: RouteMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuRoute>>,
    pub id: i32,
    pub pid: String,
}

#[derive(Debug, Serialize, Clone,ToSchema)]
#[schema(description="路由元数据")]
pub struct RouteMeta {
    #[schema(example="路由元数据")]
    pub title: String,
    #[schema(example="国际化")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "i18nKey")]
    pub i18n_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "keepAlive")]
    #[schema(example="是否活跃")]
    pub keep_alive: Option<bool>,
    #[schema(example="常数")]
    pub constant: bool,
    #[schema(example="常数")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[schema(example="排序")]
    pub order: i32,
    #[schema(example="网址")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[schema(example="隐藏菜单")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "hideInMenu")]
    pub hide_in_menu: Option<bool>,
    #[schema(example="活跃菜单")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeMenu")]
    pub active_menu: Option<String>,
    #[schema(example="是否多签")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "multiTab")]
    pub multi_tab: Option<bool>,
}

#[derive(Debug, Serialize, Clone,ToSchema)]
#[schema(description="菜单树")]
pub struct MenuTree {
    #[schema(example="节点ID")]
    pub id: i32,
    #[schema(example="父节点")]
    pub pid: String,
    #[schema(example="菜单类型")]
    #[serde(rename = "menuType")]
    pub menu_type: MenuType,
    #[schema(example="菜单名称")]
    #[serde(rename = "menuName")]
    pub menu_name: String,
    #[schema(example="icon类型")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "iconType")]
    pub icon_type: Option<i32>,
    #[schema(example="icon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "routeName")]
    pub route_name: String,
    #[serde(rename = "routePath")]
    pub route_path: String,
    #[serde(rename = "组件")]
    pub component: String,
    #[schema(example="路径参数")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "pathParam")]
    pub path_param: Option<String>,
    #[schema(example="状态")]
    pub status: Status,
    #[schema(example="激活菜单")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeMenu")]
    pub active_menu: Option<String>,
    #[schema(example="隐藏菜单")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "hideInMenu")]
    pub hide_in_menu: Option<bool>,
    #[schema(example="序列号")]
    pub sequence: i32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "i18nKey")]
    #[schema(example="国际化")]
    pub i18n_key: Option<String>,
    #[schema(example="是否激活")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "keepAlive")]
    pub keep_alive: Option<bool>,
    #[schema(example="常数")]
    pub constant: bool,
    #[schema(example="网址")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[schema(example="多签")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "multiTab")]
    pub multi_tab: Option<bool>,
    #[schema(example="创建时间")]
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
    #[schema(example="创建人")]
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[schema(example="更新时间")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
    #[schema(example="更新人")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "updatedBy")]
    pub updated_by: Option<String>,
    #[schema(example="菜单树")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuTree>>,
}
