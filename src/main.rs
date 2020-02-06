use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

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
        let allowed_origin = env::var("ALLOWED_ORIGIN")
            .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(&allowed_origin)
                    .allowed_methods(vec!["GET"])
                    .max_age(3600)
                    .finish()
            )
            .service(
                web::resource("/api/v0/checkers/{state}")
                    .route(web::get().to(checkers_move))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
            ) 
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
