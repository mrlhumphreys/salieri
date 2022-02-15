use std::env;

use actix_web::HttpResponse;

use super::backgammon;

pub fn minimax(game_data: &String) -> HttpResponse {
    let game_state = match backgammon::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::NotFound().body("Invalid State\n"),
    };

    let minimax_depth: i8 = env::var("BACKGAMMON_MINIMAX_DEPTH")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .expect("BACKGAMMON_MINIMAX_DEPTH must be a number");

    let recommended_move = backgammon::minimax::recommended_move(game_state, minimax_depth);

    match recommended_move {
        Some(m) => HttpResponse::Ok().body(format!("{}\n", m.format())),
        None => HttpResponse::NotFound().body("No Moves\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::ResponseBody;
    use actix_web::dev::Body;

    #[test]
    fn minimax_valid_test() {
        let game_state = String::from("0020000000000500300000005005000000030050000000000200121");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 200);
        match result.body() {
            ResponseBody::Body(body) => {
                match body {
                    Body::Bytes(bytes) => assert_eq!(bytes, "2-1: 19/21 21/22\n"),
                    _ => assert!(false, "unexepected body")
                }
            },
            ResponseBody::Other(_) => assert!(false, "unexpected body")
        }
    }

    #[test]
    fn minimax_invalid_test() {
        let game_state = String::from("002000000000050030000000500500000003005000000000020012n");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 404);
        match result.body() {
            ResponseBody::Body(body) => {
                match body {
                    Body::Bytes(bytes) => assert_eq!(bytes, "Invalid State\n"),
                    _ => assert!(false, "unexepected body")
                }
            },
            ResponseBody::Other(_) => assert!(false, "unexpected body")
        }
    }
}

