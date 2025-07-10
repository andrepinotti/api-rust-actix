use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use super::models::{AllUsers, RegisterUser, UpdateUser};
use crate::AppState;
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::{Pool, Postgres};

#[get("/users")]
async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as::<_, AllUsers>(
        "SELECT id, name, email, password FROM users"
    )
    .fetch_all(&app_state.postgres_client)
    .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao trazer todos os usuários do banco")
    }
}

