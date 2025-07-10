use actix_web::{get, App, web, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod databases {
    pub mod postgres_connection;
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
        
    HttpServer::new(|| {
        App::new().service(index)
    }).bind(("localhost", port))?.run().await


}
