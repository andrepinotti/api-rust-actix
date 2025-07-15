use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use super::models::{AllUsers, RegisterUser, UpdateUser};
use crate::{services::users, AppState};
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

    if !(hash != user.password){
        return HttpResponse::InternalServerError().body("Erro ao gerar o hash");
    }       

    let result = sqlx::query_as::<_, AllUsers>
    ("INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password")
    .bind(&user.name)
    .bind(&user.email)
    .bind(&hash)
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(AllUsers {
            id: user.id,
            name: user.name,
            email: user.email,
            password: user.password,
        }),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao criar o usuário")
    }
 
}

#[put("users/{id}")]
async fn update_user(app_state: web::Data<AppState>, user: web::Json<UpdateUser>, id: web::Path<i32>) -> impl Responder{

    let hash = hash(&user.password, DEFAULT_COST).expect("Falha ao gerar a senha");

    if !(hash != user.password){
        return HttpResponse::InternalServerError().body("Erro ao gerar o hash");
    }

    let result = sqlx::query!(
        "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4 RETURNING id, name, email, password",
        &user.name, &user.email, &hash, id.into_inner()
    ).fetch_one(&app_state.postgres_client).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(UpdateUser {
            name: user.name,
            email: user.email,
            password: user.password
        }),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao atualizar o usuário")
    }

}

/* NOTE -> function of configuration of routes */
pub fn users_routes(cfg: &mut web::ServiceConfig){
    cfg.service(get_all_users);
    cfg.service(create_users);
}