use actix_web::{get, App, HttpServer, Responder, web};

#[get("/{id}/{name}/index.html")]
async fn index_id_name(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}


// #[tokio::main]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    HttpServer::new(|| App::new().service(index_id_name).service(index))
        .bind("127.0.0.1:8080")?
        .start()
        .await
}