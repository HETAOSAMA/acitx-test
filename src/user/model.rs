use rbatis::crud_table;
use rbatis::DateTimeNative;
use rbson::DateTime;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginReqVO{
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddUserReqVO{
    pub user_name: Option<String>,
    pub password: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DelUserReqVO{
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelUserByIdSReqVO{
    pub ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetByPageReqVO{
    pub user_name: Option<String>,
    pub page_index: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserReqVO{
    pub id: Option<String>,
    pub user_name: Option<String>,
    pub password: Option<String>,
}

#[crud_table(table_name:sys_user)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SysUser{
    pub id: Option<String>,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub create_time: Option<DateTimeNative>,
}