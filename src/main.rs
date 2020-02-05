use actix_web::{web, App, HttpResponse, HttpServer, Responder};

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
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/checkers_move/{state}", web::get().to(checkers_move))
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
