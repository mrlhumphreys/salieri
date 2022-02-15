use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use std::env;

mod checkers;
mod checkers_controller;

mod backgammon;
mod backgammon_controller;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("200 OK\n")
}

async fn game_move(info: web::Path<(String, String)>) -> impl Responder {
    let game_type = &info.0;
    let game_data = &info.1;

    match game_type.as_str() {
        "checkers" => {
            match checkers::openings::recommended_move(game_data) {
                Some(m) => HttpResponse::Ok().body(m),
                None => checkers_controller::mcts(game_data) 
            }
        },
        "backgammon" => backgammon_controller::minimax(game_data), 
        _ => HttpResponse::NotFound().body("404 Not Found\n")
    }
}

async fn game_move_algorithm(info: web::Path<(String, String, String)>) -> impl Responder {
    let game_type = &info.0;
    let game_data = &info.1;
    let algorithm = &info.2;

    match game_type.as_str() {
        "checkers" => {
            match algorithm.as_str() {
                "openings_db" => checkers_controller::opening(game_data), 
                "minimax" => checkers_controller::minimax(game_data), 
                "mcts" => checkers_controller::mcts(game_data),
                _ => HttpResponse::NotFound().body("404 Not Found\n"),
            }
        },
        "backgammon" => {
            match algorithm.as_str() {
                "minimax" => backgammon_controller::minimax(game_data),
                _ => HttpResponse::NotFound().body("404 Not Found\n"),
            }
        },
        _ => HttpResponse::NotFound().body("404 Not Found\n")
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
                web::resource("/api/v0/{game_type}/{state}")
                    .route(web::get().to(game_move))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
            ) 
            .service(
                web::resource("/api/v0/{game_type}/{state}/{algorithm}")
                    .route(web::get().to(game_move_algorithm))
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

    // index page
    #[actix_rt::test]
    async fn test_index_status() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_index_body() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let res = test::read_response(&mut app, req).await;
        assert_eq!(res, Bytes::from_static(b"200 OK\n"));
    }

    // checkers with valid params
    #[actix_rt::test]
    async fn test_checkers_status_with_valid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww").to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_checkers_body_with_valid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww").to_request();
        let res = test::read_response(&mut app, req).await;
        assert_eq!(res, Bytes::from_static(b"24-20\n"));
    }

    // checkers with invalid params
    #[actix_rt::test]
    async fn test_checkers_status_with_invalid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/checkers/-bbbbbbbbb-bb--b-----wwwwwwwwwwwww").to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_checkers_body_with_invalid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/checkers/-bbbbbbbbb-bb--b-----wwwwwwwwwwwww").to_request();
        let res = test::read_response(&mut app, req).await;
        assert_eq!(res, Bytes::from_static(b"404 Not Found\n"));
    }

    // backgammon with valid params
    #[actix_rt::test]
    async fn test_backgammon_status_with_valid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/backgammon/0020000000000500300000005005000000030050000000000200121").to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_backgammon_body_with_valid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/backgammon/0020000000000500300000005005000000030050000000000200121").to_request();
        let res = test::read_response(&mut app, req).await;
        assert_eq!(res, Bytes::from_static(b"2-1: 19/21 21/22\n"));
    }

    // backgammon invalid params
    #[actix_rt::test]
    async fn test_backgammon_status_with_invalid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/backgammon/002000000000050030000000500500000003005000000000020012n").to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_backgammon_body_with_invalid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/backgammon/002000000000050030000000500500000003005000000000020012n").to_request();
        let res = test::read_response(&mut app, req).await;
        assert_eq!(res, Bytes::from_static(b"Invalid State\n"));
    }

    // invalid game type 
    #[actix_rt::test]
    async fn test_invalid_game_type_status() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/mario/-bbbbbbbbb-bb--b-----wwwwwwwwwwwww").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_invalid_game_type_body() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").uri("/api/v0/mario/-bbbbbbbbb-bb--b-----wwwwwwwwwwwww").to_request();

        let result = test::read_response(&mut app, req).await;

        assert_eq!(result, Bytes::from_static(b"404 Not Found\n"));
    }
}
