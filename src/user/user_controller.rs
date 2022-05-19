
use actix_web::{web::{Json, self}, post, Responder, HttpResponse, get};

use crate::user::model::{AddUserReqVO, DelUserByIdSReqVO, DelUserReqVO, GetByPageReqVO, LoginReqVO, SysUser, UpdateUserReqVO};
use crate::user::user_service::UserService;


pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(login)
            .service(add_user)
            .service(del_userById)
            .service(del_userByIds)
            .service(get_by_page)
            .service(update_user)
    );
}


#[post("/login")]
pub async fn login(login:Json<LoginReqVO>) -> impl Responder {
    HttpResponse::Ok().json(login.into_inner())
}

#[post("/add_user")]
pub async fn add_user(add_user:Json<AddUserReqVO>) -> impl Responder {

    let mut user = SysUser{
        id: None,
        user_name: add_user.user_name.clone(),
        password: add_user.password.clone(),
        create_time: None,
    };
    UserService::add_user(user).await;
    return  HttpResponse::Ok();
}

#[post("/del_userById")]
pub async fn del_userById(delUserById:Json<DelUserReqVO>) -> impl Responder {
    let id = delUserById.id.as_ref().unwrap();
    UserService::del_userById(&id).await.expect("删除失败");
    return HttpResponse::Ok();
}

#[post("/del_userByIds")]
pub async fn del_userByIds(delUserById:Json<DelUserByIdSReqVO>) -> impl Responder {
    let ids = delUserById.ids.as_ref().unwrap();
    UserService::del_userByIds(&ids).await.expect("删除失败");
    return HttpResponse::Ok();
}

#[post("/get_by_page")]
pub async fn get_by_page(reqVO:Json<GetByPageReqVO>) -> impl Responder {

    let respVO = UserService::get_by_page(reqVO.into_inner()).await.expect("获取失败");
    return HttpResponse::Ok().json(respVO);
}

#[post("/update_user")]
pub async fn update_user(reqVO:Json<UpdateUserReqVO>) -> impl Responder {
    let respVO = UserService::update_user(reqVO.into_inner()).await.expect("修改失败");
    return HttpResponse::Ok();
}