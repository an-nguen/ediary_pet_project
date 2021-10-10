use crate::errors::api_error::ApiError;
use crate::middlewares::auth::TokenAuth;
use crate::models::subject::{Subject, UpdSubject};
use crate::repository::common::Repository;
use crate::repository::subject;
use crate::repository::subject::SubjectRepository;
use actix_web::{web, HttpResponse, ResponseError};

pub async fn find_all(auth: TokenAuth, subject_repo: web::Data<SubjectRepository>) -> HttpResponse {
    find_all!(subject_repo)
}

pub async fn create(
    auth: TokenAuth,
    subject_repo: web::Data<SubjectRepository>,
    obj: web::Json<Subject>,
) -> HttpResponse {
    create!(subject_repo, obj.into_inner())
}

pub async fn update(
    auth: TokenAuth,
    subject_repo: web::Data<SubjectRepository>,
    id: web::Path<(i32,)>,
    obj: web::Json<UpdSubject>,
) -> HttpResponse {
    update!(subject_repo, id.into_inner().0, obj.into_inner())
}

pub async fn delete(
    auth: TokenAuth,
    subject_repo: web::Data<SubjectRepository>,
    id: web::Path<(i32,)>,
) -> HttpResponse {
    delete!(subject_repo, id.into_inner().0)
}
