use std::fmt::Debug;
use rbatis::wrapper::Wrapper;
use crate::user::model::{AddUserReqVO, GetByPageReqVO, SysUser, UpdateUserReqVO};
use rbatis::core::db::DBExecResult;
use anyhow::{anyhow};
use rbatis::crud::CRUD;
use rbatis::{Page, PageRequest, Result};
use rbatis::snowflake::new_snowflake_id;
use crate::rb;
use crate::user::user_controller::del_userById;

pub struct UserService {}

impl UserService {
    pub async fn add_user(mut user:SysUser) -> Result<DBExecResult> {
        log::info!("{:#?}",user);
        user.id = Some(new_snowflake_id().to_string());
        user.create_time = Some(rbatis::DateTimeNative::now());

        return rb.save(&user, &[]).await;
    }

    pub async fn del_userById(id:&str) -> Result<u64> {

        let result = rb.remove_by_column::<SysUser,_>("id",id).await;
        return result
    }

    pub(crate) async fn del_userByIds(p0: &Vec<String>) -> Result<u64> {
        let result = rb.remove_batch_by_column::<SysUser,_>("id",p0).await;
        return result
    }

    pub(crate) async fn get_by_page(p0: GetByPageReqVO) -> Result<Page<SysUser>> {
        let mut wrapper = rb.new_wrapper();

        if !&p0.user_name.is_none() {
            wrapper = wrapper.like("user_name", &p0.user_name.as_ref().unwrap());
        }
        let req = PageRequest::new(*p0.page_index.as_ref().unwrap(), *p0.page_size.as_ref().unwrap());

        let result = rb.fetch_page_by_wrapper(wrapper, &req).await;
        return result
    }


    pub(crate) async fn update_user(p0: UpdateUserReqVO) -> Result<u64>{

        let w = rb.new_wrapper().eq("id", p0.id.as_ref().unwrap());
        let mut user = SysUser{
            id: p0.id.clone(),
            user_name: p0.user_name.clone(),
            password: p0.password.clone(),
            create_time: Some(rbatis::DateTimeNative::now()),
        };

        let resp = rb.update_by_wrapper(&user, w, &[]).await;
        return resp
    }

}