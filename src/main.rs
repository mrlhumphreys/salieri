use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use std::env;

mod checkers;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("200 OK\n")
}

async fn checkers_move(info: web::Path<String>) -> impl Responder {
    match checkers::openings::recommended_move(&info.clone()) {
        Some(m) => HttpResponse::Ok().body(m),
        None => checkers_mcts(&info.into_inner()) 
    }
}

async fn checkers_move_algorithm(info: web::Path<(String, String)>) -> impl Responder {
    let game_data = &info.0;
    let algorithm = &info.1;

    match algorithm.as_str() {
        "openings_db" => checkers_opening(game_data), 
        "minimax" => checkers_minimax(game_data), 
        "mcts" => checkers_mcts(game_data),
        _ => HttpResponse::NotFound().body("404 Not Found\n"),
    }
}

fn checkers_opening(game_data: &String) -> HttpResponse {
    match checkers::openings::recommended_move(game_data) {
        Some(m) => HttpResponse::Ok().body(m),
        None => HttpResponse::NotFound().body("404 Not Found\n")
    }
}

fn checkers_minimax(game_data: &String) -> HttpResponse {
    let game_state = match checkers::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::NotFound().body("404 Not Found\n"),
    };

    let minimax_depth: i8 = env::var("MINIMAX_DEPTH")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .expect("MINIMAX_DEPTH must be a number");

    let recommended_move = checkers::minimax::recommended_move(game_state, minimax_depth);

    match recommended_move {
        Some(m) => HttpResponse::Ok().body(format!("{}\n", m.format())),
        None => HttpResponse::NotFound().body("404 Not Found\n"),
    }
}

fn checkers_mcts(game_data: &String) -> HttpResponse {
    let game_state = match checkers::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::NotFound().body("404 Not Found\n"),
    };

    let mcts_simulation_count: i16 = env::var("MCTS_SIMULATION_COUNT")
        .unwrap_or_else(|_| "120".to_string())
        .parse()
        .expect("MCTS_SIMULATION_COUNT must be a number");

    let mcts_simulation_depth: i16 = env::var("MCTS_SIMULATION_DEPTH")
        .unwrap_or_else(|_| "40".to_string())
        .parse()
        .expect("MCTS_SIMULATION_DEPTH must be a number");

    let recommended_move = checkers::mcts::recommended_move(game_state, mcts_simulation_count, mcts_simulation_depth);

    match recommended_move {
        Ok(m) => HttpResponse::Ok().body(format!("{}\n", m.format())),
        Err(e) => {
            println!("{}", e);
            HttpResponse::NotFound().body("404 Not Found\n")
        }
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
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string());
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
            .service(
                web::resource("/api/v0/checkers/{state}/{algorithm}")
                    .route(web::get().to(checkers_move_algorithm))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
            ) 
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use bytes::Bytes;

    #[actix_rt::test]
    async fn test_index_status() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_index_body() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();

        let result = test::read_response(&mut app, req).await;

        assert_eq!(result, Bytes::from_static(b"200 OK\n"));
    }

    #[actix_rt::test]
    async fn test_checkers_status_with_valid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/checkers/{state}", web::get().to(checkers_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_checkers_body_with_valid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/checkers/{state}", web::get().to(checkers_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww").to_request();

        let result = test::read_response(&mut app, req).await;

        assert_eq!(result, Bytes::from_static(b"24-20\n"));
    }

    #[actix_rt::test]
    async fn test_checkers_status_with_invalid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/checkers/{state}", web::get().to(checkers_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/checkers/-bbbbbbbbb-bb--b-----wwwwwwwwwwwww").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_checkers_body_with_invalid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/checkers/{state}", web::get().to(checkers_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/checkers/-bbbbbbbbb-bb--b-----wwwwwwwwwwwww").to_request();

        let result = test::read_response(&mut app, req).await;

        assert_eq!(result, Bytes::from_static(b"404 Not Found\n"));
    }
}
