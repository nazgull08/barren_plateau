
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files as fs;
use std::path::PathBuf;

// Функция для обработки wasm файлов
async fn wasm_file(path: web::Path<PathBuf>) -> actix_web::Result<HttpResponse> {
    let path = path.into_inner();
    let full_path = PathBuf::from("./static/pkg").join(path); // Убедитесь, что путь соответствует вашей структуре
    Ok(HttpResponse::Ok()
        .content_type("application/wasm")
        .body(actix_web::web::block(move || std::fs::read(full_path)).await??))
}

async fn script_file(path: web::Path<PathBuf>) -> actix_web::Result<HttpResponse> {
    let path = path.into_inner();
    let full_path = PathBuf::from("./static/pkg").join(path);
    Ok(HttpResponse::Ok()
        .content_type("application/javascript")
        .body(actix_web::web::block(move || std::fs::read(full_path)).await??))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .route("/pkg/{filename:.*}.wasm", web::get().to(wasm_file)) // Обработчик для .wasm файлов
            .route("/pkg/{filename:.*}.js", web::get().to(script_file)) // Обработчик для .js файлов
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

