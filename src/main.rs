use actix_web::{get, App, web, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod databases {
    pub mod postgres_connection;
}

mod services {
    pub mod users;
}
//para verificar sempre a conexão em todas as rotas criamos a struct com o pool
#[derive(Clone)]
pub struct AppState {
    pub postgres_client: Pool<Postgres>
}

#[get("/")]
async fn index() -> impl Responder {

    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();
    
    let _pool = databases::postgres_connection::start_connection().await;

    //FIXME - The port and others crucial data
    let port = 8080;
        
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(AppState{ 
            postgres_client: _pool.clone()    
         }))
        .service(index)
        .configure(services::users::services::users_routes)
    }).bind(("localhost", port))?.run().await

}
