mod api;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web, HttpRequest, Error, HttpResponse};
use crate::api::api::{get_all_criminals, add_criminal, delete_criminal, update_criminal};
use crate::api::account::login;
use crate::api::socket;
use crate::api::logger::get_logs;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, rsa_private_keys};
use std::io::BufReader;
use std::{fs, fs::File};

fn index(req: &HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/plain").body("Welcome!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   // fs::write("./data/logs.json", "[]").expect("Unable to write to file");

   let config = load_rustls_config();

    HttpServer::new(|| {        
        App::new()
            .route("/ws/", web::get().to(socket::index))
            .wrap(Cors::permissive())
            .service(get_all_criminals)
            .service(add_criminal)
            .service(delete_criminal)
            .service(update_criminal)
            .service(login)
            .service(get_logs)
    })
    .bind_rustls("127.0.0.1:8080", config)?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = rsa_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

