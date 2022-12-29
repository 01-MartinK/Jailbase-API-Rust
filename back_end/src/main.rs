mod api;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use crate::api::api::{get_all_criminals, add_criminal, delete_criminal, update_criminal};
use crate::api::account::login;
use crate::api::socket::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {        
        App::new()
            .route("/ws/", web::get().to(index))
            .wrap(Cors::permissive())
            .service(get_all_criminals)
            .service(add_criminal)
            .service(delete_criminal)
            .service(update_criminal)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

