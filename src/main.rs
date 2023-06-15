use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;

fn is_sukui() -> String {
    let mut rng = rand::thread_rng();
    let i: f32 = rng.gen();
    if i < 0.25 {
        "救いはあります".to_string()
    } else if i < 0.5 {
        "救いはありました".to_string()
    } else if i < 0.75 {
        "救いはありません".to_string()
    } else {
        "救いはないです".to_string()
    }
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
