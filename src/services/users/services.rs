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

// ever as parameter he must receive the conexion
#[post("/users")]
async fn create_users(app_state: web::Data<AppState>, user: web::Json<RegisterUser>) -> impl Responder {

    let hash = hash(&user.password, DEFAULT_COST).expect("Falha ao gerar a senha");

    let result = sqlx::query_as::<_, user>(
        ""
    );

   match result {
    Ok() => HttpResponse::Ok().json(value),
    Err(_) => HttpResponse::InternalServerError().body("Erro ao cadastrar o usuário")
   } 
}

/* NOTE -> function of configuration of routes */
pub fn users_routes(cfg: &mut web::ServiceConfig){
    cfg.service(get_all_users);
}