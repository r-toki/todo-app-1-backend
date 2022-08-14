use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    web, Responder, Result,
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::model::{NewTask, Task};

#[derive(Deserialize)]
pub struct TaskPath {
    pub task_id: i64,
}

pub async fn index(pool: web::Data<PgPool>) -> Result<impl Responder> {
    let tasks = Task::all(&pool).await.map_err(ErrorInternalServerError)?;
    Ok(web::Json(tasks))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateForm {
    #[validate(length(min = 1))]
    description: String,
}

pub async fn create(
    pool: web::Data<PgPool>,
    form: web::Json<CreateForm>,
) -> Result<impl Responder> {
    form.validate().map_err(ErrorBadRequest)?;
    let new_task = NewTask {
        description: form.description.clone(),
    };
    let task = Task::insert(&pool, new_task)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(web::Json(task))
}

pub async fn destroy(pool: web::Data<PgPool>, path: web::Path<TaskPath>) -> Result<impl Responder> {
    let task = Task::delete(&pool, path.task_id)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(web::Json(task))
}
