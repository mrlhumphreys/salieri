use std::env;
use actix_web::HttpResponse;
use super::go;

pub fn minimax(game_data: &String) -> HttpResponse {
    let mut game_state = match go::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    };

    let minimax_depth: i8 = env::var("GO_MINIMAX_DEPTH")
        .unwrap_or_else(|_| "0".to_string())
        .parse()
        .expect("GO_MINIMAX_DEPTH must be a number");

    let recommended_move = go::minimax::recommended_move(&mut game_state, minimax_depth);

    match recommended_move {
        Some(m) => {
            HttpResponse::Ok().body(format!("{}\n", m.format()))
        },
        None => HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::MessageBody;

    #[test]
    fn minimax_valid_test() {
        let game_state = String::from("PL[B]XB[0]XW[0]");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 200);
        match result.into_body().try_into_bytes() {
            Ok(bytes) => assert_eq!(bytes, "ss\n"),
            Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn minimax_invalid_game_state_test() {
        let game_state = String::from("asdf");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 422);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }
}
