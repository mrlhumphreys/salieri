use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

mod checkers;
mod minimax;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("200 OK\n")
}

async fn checkers_move(info: web::Path<String>) -> impl Responder {
    let game_state = match checkers::game_state::parse(&info.into_inner()) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::NotFound().body("404 Not Found\n"),
    };
    let recommended_move = minimax::recommended_move(game_state);
    match recommended_move {
        Some(m) => HttpResponse::Ok().body(format!("{}\n", m.format())),
        None => return HttpResponse::NotFound().body("404 Not Found\n"),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "7878".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/api/v0/checkers/{state}", web::get().to(checkers_move))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
