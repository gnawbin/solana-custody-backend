use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize,ToSchema)]
#[schema(description="分页请求")]
pub struct PageRequest {
    #[serde(
        default = "default_current",
        deserialize_with = "deserialize_u64_from_string"
    )]
    #[schema(example="当前页")]
    pub current: u64,
    #[serde(
        default = "default_size",
        deserialize_with = "deserialize_u64_from_string"
    )]
    #[schema(example="当前页的行数")]
    pub size: u64,
}

fn deserialize_u64_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(DeError::custom)
}

fn default_current() -> u64 {
    1
}

fn default_size() -> u64 {
    10
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            current: default_current(),
            size: default_size(),
        }
    }
}

#[derive(Debug, Serialize, Default,ToSchema)]
#[schema(description="分页数据")]
pub struct PaginatedData<T> {
    #[schema(example="当前页数")]
    pub current: u64,
    #[schema(example="当前页的行数")]
    pub size: u64,
    #[schema(example="总数")]
    pub total: u64,
    #[schema(example="记录")]
    pub records: Vec<T>,
}
