use actix_web::{get, post, web, App,
     HttpServer, HttpResponse, middleware}; // , Result

use actix_web::http::header::ContentType;

#[get("/")]
async fn info() -> HttpResponse {
    HttpResponse::Ok().body("Documentation: https://xcmsend.github.io/api/index.html")

//    format!("Documentation: https://xcmsend.github.io/api/index.html")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}

// open channels, list open ingoing and outgoing hrmp channels for paraid
#[post("/polkadot/openchannels")]
async fn dot_openchannels() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}

// broadcast input: {chain: 'hydradx', tx: ''}
#[post("/broadcast")]
async fn broadcast_tx() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}

#[post("/saveUrl")]
async fn save_url() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}

#[get("/getUrl/{name}")]
async fn get_url(name: web::Path<String>) -> HttpResponse {
    let fluff = format!("Todo {name}!");

    HttpResponse::Ok().body("Todo!")
}

#[post("/xcm-asset-transfer")]
async fn xcm_asset_transfer() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}




async fn indexme() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .body("data")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("running http server http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(xcm_asset_transfer)
            .service(get_url)
            .service(save_url)
            .service(broadcast_tx)
            .service(dot_openchannels)
            .service(info)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
