use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;

fn is_sukui() -> String {
    let mut rng = rand::thread_rng();
    let i: u32 = rng.gen_range(0..4);
    match i {
        0 => "救いはあります",
        1 => "救いはありました",
        2 => "救いはありません",
        _ => "救いはないです"
    }.to_string()
}

#[get("/")]
async fn main_handler() -> impl Responder {
    is_sukui()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(main_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
