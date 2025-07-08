use actix_web::{get, App, web, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let port = 8080;
        
    HttpServer::new(|| {
        App::new().service(index)
    }).bind(("localhost", port))?.run().await


}
