use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use std::env;

mod checkers;
mod checkers_controller;

mod backgammon;
mod backgammon_controller;

mod chess;
mod chess_controller;

mod go;
mod go_controller;

mod shogi;
mod shogi_controller;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("200 OK\n")
}

async fn post_game_move(info: web::Path<String>, req_body: String) -> impl Responder {
    let game_type = &info.into_inner();

    match game_type.as_str() {
        "checkers" => {
            match checkers::openings::recommended_move(&req_body) {
                Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
                None => checkers_controller::mcts(&req_body)
            }
        },
        "backgammon" => {
            match backgammon::openings::recommended_move(&req_body) {
                Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
                None => backgammon_controller::mcts(&req_body)
            }
        },
        "chess" => {
            match chess::openings::recommended_move(&req_body) {
                Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
                None => chess_controller::minimax(&req_body)
            }
        },
        "go" => {
            match go::openings::recommended_move(&req_body) {
                Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
                None => go_controller::minimax(&req_body)
            }
        },
        "shogi" => {
            match shogi::openings::recommended_move(&req_body) {
                Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
                None => shogi_controller::minimax(&req_body)
            }
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
                "openings_db" => chess_controller::opening(&req_body),
                "minimax" => chess_controller::minimax(&req_body),
                "mcts" => chess_controller::mcts(&req_body),
                _ => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
            }
        },
        "go" => {
            match algorithm.as_str() {
                "openings_db" => go_controller::opening(&req_body),
                "minimax" => go_controller::minimax(&req_body),
                "mcts" => go_controller::mcts(&req_body),
                _ => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
            }
        },
        "shogi" => {
            match algorithm.as_str() {
                "openings_db" => shogi_controller::opening(&req_body),
                "minimax" => shogi_controller::minimax(&req_body),
                "mcts" => shogi_controller::mcts(&req_body),
                _ => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
            }
        },
        _ => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
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
            .unwrap_or_else(|_| "http://localhost:5173".to_string());
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
                web::resource("/api/v0/{game_type}/{algorithm}")
                    .route(web::post().to(post_game_algorithm_move))
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
        let game_state = String::from("B:W19,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15");
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/checkers")
            .set_payload(game_state)
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_checkers_body_with_valid_params() {
        let game_state = String::from("B:W18,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/checkers")
            .set_payload(game_state)
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"8-11\n"));
    }

    // checkers with invalid params
    #[actix_rt::test]
    async fn test_checkers_status_with_invalid_params() {
        let game_state = String::from("X:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        let mut app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/checkers")
            .set_payload(game_state)
            .to_request();
        let res = test::call_service(&mut app, req).await;
        assert!(res.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_checkers_body_with_invalid_params() {
        let game_state = String::from("X:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/checkers")
            .set_payload(game_state)
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"422 Unprocessable Entity\n"));
    }

    // backgammon with valid params
    #[actix_rt::test]
    async fn test_backgammon_status_with_valid_params() {
        let game_state = String::from("0020000000000500300000005005000000030050000000000200121");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/backgammon")
            .set_payload(game_state)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_backgammon_body_with_valid_params() {
        let game_state = String::from("0020000000000500300000005005000000030050000000000200121");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/backgammon")
            .set_payload(game_state)
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"2-1: 19/21 21/22\n"));
    }

    // backgammon invalid params
    #[actix_rt::test]
    async fn test_backgammon_status_with_invalid_params() {
        let game_state = String::from("002000000000050030000000500500000003005000000000020012n");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/backgammon")
            .set_payload(game_state)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_backgammon_body_with_invalid_params() {
        let game_state = String::from("002000000000050030000000500500000003005000000000020012n");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/backgammon")
            .set_payload(game_state)
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"422 Unprocessable Entity\n"));
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
        let game_state = String::from("rnbqkbnr/ppp1pppp/3p4/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/chess")
            .set_payload(game_state)
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"d4\n"));
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

    // go with valid params
    #[actix_rt::test]
    async fn test_go_status_with_valid_params() {
        let game_state = String::from("PL[B]XB[0]XW[0]");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/go")
            .set_payload(game_state)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_go_body_with_valid_params() {
        let game_state = String::from("PL[B]XB[0]XW[0]");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/go")
            .set_payload(game_state)
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"ss\n"));
    }

    // go with invalid params
    #[actix_rt::test]
    async fn test_go_status_with_invalid_params() {
        let game_state = String::from("asdf");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/go")
            .set_payload(game_state)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_go_body_with_invalid_params() {
        let game_state = String::from("asdf");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/go")
            .set_payload(game_state)
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"422 Unprocessable Entity\n"));
    }

    // shogi with valid params
    #[actix_rt::test]
    async fn test_shogi_status_with_valid_params() {
        let game_state = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/shogi")
            .set_payload(game_state)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
    }

    #[actix_rt::test]
    async fn test_shogi_body_with_valid_params() {
        let game_state = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/shogi")
            .set_payload(game_state)
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"B*41\n"));
    }

    // shogi with invalid params
    #[actix_rt::test]
    async fn test_shogi_status_with_invalid_params() {
        let game_state = String::from("asdf");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/shogi")
            .set_payload(game_state)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_shogi_body_with_invalid_params() {
        let game_state = String::from("asdf");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/shogi")
            .set_payload(game_state)
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from_static(b"422 Unprocessable Entity\n"));
    }

    // invalid game type
    #[actix_rt::test]
    async fn test_invalid_game_type_status() {
        let game_state = String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/mario")
            .set_payload(game_state)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_invalid_game_type_body() {
        let game_state = String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        let app = test::init_service(App::new().route("/api/v0/{game_type}", web::post().to(post_game_move))).await;
        let req = test::TestRequest::post()
            .insert_header(ContentType::plaintext())
            .uri("/api/v0/mario")
            .set_payload(game_state)
            .to_request();

        let result = test::call_and_read_body(&app, req).await;

        assert_eq!(result, Bytes::from_static(b"422 Unprocessable Entity\n"));
    }
}
