use crate::middlewares::auth::TokenAuth;
use crate::models::student::{NewStudent, UpdStudent};
use crate::repository::common::Repository;
use crate::repository::student::StudentRepository;
use actix_web::{web, HttpResponse, ResponseError};

pub async fn find_all(auth: TokenAuth, student_repo: web::Data<StudentRepository>) -> HttpResponse {
    find_all!(student_repo)
}

pub async fn create(
    auth: TokenAuth,
    student_repo: web::Data<StudentRepository>,
    obj: web::Json<NewStudent>,
) -> HttpResponse {
    create!(student_repo, obj.into_inner())
}

pub async fn update(
    auth: TokenAuth,
    student_repo: web::Data<StudentRepository>,
    id: web::Path<(i32,)>,
    obj: web::Json<UpdStudent>,
) -> HttpResponse {
    update!(student_repo, id.into_inner().0, obj.into_inner())
}

pub async fn delete(
    auth: TokenAuth,
    student_repo: web::Data<StudentRepository>,
    id: web::Path<(i32,)>,
) -> HttpResponse {
    delete!(student_repo, id.into_inner().0)
}
