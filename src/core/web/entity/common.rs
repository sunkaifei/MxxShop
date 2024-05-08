use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BathIdRequest {
    pub ids: Option<Vec<Option<String>>>,
}

#[derive(Deserialize)]
pub struct InfoId {
    pub id: Option<String>,
}

#[derive(Deserialize)]
pub struct QueryUrl {
    pub short_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteIntIdRequest {
    pub ids: Vec<i32>,
}

#[derive(serde::Serialize)]
pub struct Pagination {
    ///当前页码
    pub current: u64,
    ///总页数
    pub total: u64,
    ///每页数量
    pub page_size: u64,
}