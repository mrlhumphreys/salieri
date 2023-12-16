use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use std::env;

mod checkers;
mod checkers_controller;

mod backgammon;
mod backgammon_controller;

mod chess;
mod chess_controller;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("200 OK\n")
}

async fn post_game_move(info: web::Path<String>, req_body: String) -> impl Responder {
    let game_type = &info.into_inner();

    match game_type.as_str() {
        "checkers" => {
            match checkers::openings::recommended_move(&req_body) {
                Some(m) => HttpResponse::Ok().body(m),
                None => checkers_controller::mcts(&req_body) 
            }
        },
        "backgammon" => {
            match backgammon::openings::recommended_move(&req_body) {
                Some(m) => HttpResponse::Ok().body(m),
                None => backgammon_controller::mcts(&req_body)
            }
        },
        "chess" => {
            chess_controller::minimax(&req_body)
        },
        _ => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    }
}

async fn post_game_algorithm_move(info: web::Path<(String, String)>, req_body: String) -> impl Responder {
    let (game_type, algorithm) = &info.into_inner();

    match game_type.as_str() {
        "checkers" => {
            match algorithm.as_str() {
                "openings_db" => checkers_controller::opening(&req_body),
                "minimax" => checkers_controller::minimax(&req_body), 
                "mcts" => checkers_controller::mcts(&req_body),
                _ => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
            }
        },
        "backgammon" => {
            match algorithm.as_str() {
                "openings_db" => backgammon_controller::opening(&req_body),
                "minimax" => backgammon_controller::minimax(&req_body),
                "mcts" => backgammon_controller::mcts(&req_body),
                _ => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
            }
        },
        "chess" => {
            match algorithm.as_str() {
                "minimax" => chess_controller::minimax(&req_body),
                _ => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
            }
        },
        _ => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    }
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
        "backgammon" => {
            match backgammon::openings::recommended_move(game_data) {
                Some(m) => HttpResponse::Ok().body(m),
                None => backgammon_controller::mcts(game_data)
            }
        },
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
                "openings_db" => backgammon_controller::opening(game_data),
                "minimax" => backgammon_controller::minimax(game_data),
                "mcts" => backgammon_controller::mcts(game_data),
                _ => HttpResponse::NotFound().body("404 Not Found\n"),
            }
        },
        "chess" => {
            match algorithm.as_str() {
                "minimax" => chess_controller::minimax(game_data),
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
            .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&allowed_origin)
                    .allowed_methods(vec!["GET","POST"])
                    .max_age(3600)
            )
            .service(
                web::resource("/api/v0/{game_type}")
                    .route(web::post().to(post_game_move))
            )
            .service(
                web::resource("/api/v0/{game_type}/{state}")
                    .route(web::get().to(game_move))
                    .route(web::post().to(post_game_algorithm_move)) // temporary path while get
                                                                     // requests are still used
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
    use actix_web::http::header::ContentType;
    use bytes::Bytes;

    // index page
    #[actix_rt::test]
    async fn test_index_status() {
        let app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_index_body() {
        let app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"200 OK\n"));
    }

    // checkers with valid params
    #[actix_rt::test]
    async fn test_checkers_status_with_valid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww")
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_checkers_body_with_valid_params() {
        let app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww")
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"24-20\n"));
    }

    // checkers with invalid params
    #[actix_rt::test]
    async fn test_checkers_status_with_invalid_params() {
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/checkers/-bbbbbbbbb-bb--b-----wwwwwwwwwwwww")
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_checkers_body_with_invalid_params() {
        let app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/checkers/-bbbbbbbbb-bb--b-----wwwwwwwwwwwww")
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"404 Not Found\n"));
    }

    // backgammon with valid params
    #[actix_rt::test]
    async fn test_backgammon_status_with_valid_params() {
        let app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/backgammon/0020000000000500300000005005000000030050000000000200121")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_backgammon_body_with_valid_params() {
        let app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/backgammon/0020000000000500300000005005000000030050000000000200121")
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"2-1: 19/21 21/22\n"));
    }

    // backgammon invalid params
    #[actix_rt::test]
    async fn test_backgammon_status_with_invalid_params() {
        let app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/backgammon/002000000000050030000000500500000003005000000000020012n")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_backgammon_body_with_invalid_params() {
        let app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/backgammon/002000000000050030000000500500000003005000000000020012n")
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"404 Not Found\n"));
    }

    // chess with valid params
    #[actix_rt::test]
    async fn test_chess_status_with_valid_params() {
        let game_state = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/chess")
            .set_payload(game_state)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_chess_body_with_valid_params() {
        let game_state = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/chess")
            .set_payload(game_state)
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"e4\n"));
    }

    // chess with invalid params
    #[actix_rt::test]
    async fn test_chess_status_with_invalid_params() {
        let game_state = String::from("znbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/chess")
            .set_payload(game_state)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_chess_body_with_invalid_params() {
        let game_state = String::from("znbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/chess")
            .set_payload(game_state)
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"422 Unprocessable Entity\n"));
    }

    // invalid game type 
    #[actix_rt::test]
    async fn test_invalid_game_type_status() {
        let app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/mario/-bbbbbbbbb-bb--b-----wwwwwwwwwwwww")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_invalid_game_type_body() {
        let app = test::init_service(App::new().route("/api/v0/{game_type}/{state}", web::get().to(game_move))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/mario/-bbbbbbbbb-bb--b-----wwwwwwwwwwwww")
            .to_request();

        let result = test::call_and_read_body(&app, req).await;

        assert_eq!(result, Bytes::from_static(b"404 Not Found\n"));
    }
}
