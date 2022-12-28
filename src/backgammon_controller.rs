use std::env;

use actix_web::HttpResponse;

use super::backgammon;

pub fn opening(game_data: &String) -> HttpResponse {
    match backgammon::openings::recommended_move(game_data) {
        Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
        None => HttpResponse::NotFound().body("404 Not Found\n")
    }
}

pub fn minimax(game_data: &String) -> HttpResponse {
    let game_state = match backgammon::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::NotFound().body("404 Not Found\n"),
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

pub fn mcts(game_data: &String) -> HttpResponse {
    let game_state = match backgammon::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::NotFound().body("404 Not Found\n"),
    };

    let mcts_simulation_count: i16 = env::var("BACKGAMMON_MCTS_SIMULATION_COUNT")
        .unwrap_or_else(|_| "120".to_string())
        .parse()
        .expect("BACKGAMMON_MCTS_SIMULATION_COUNT must be a number");

    let mcts_simulation_depth: i16 = env::var("BACKGAMMON_MCTS_SIMULATION_DEPTH")
        .unwrap_or_else(|_| "40".to_string())
        .parse()
        .expect("BACKGAMMON_MCTS_SIMULATION_DEPTH must be a number");

    let recommended_move = backgammon::mcts::recommended_move(game_state, mcts_simulation_count, mcts_simulation_depth);

    match recommended_move {
        Ok(m) => HttpResponse::Ok().body(format!("{}\n", m.format())),
        Err(e) => {
            println!("{}", e);
            HttpResponse::NotFound().body("404 Not Found\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::MessageBody;

    #[test]
    fn opening_valid_test() {
        let game_state = String::from("0020000000000500030000005005000000300050000000000200461");
        let result = opening(&game_state);

        assert_eq!(result.status(), 200);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "4-6: 24/18 13/9\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn opening_no_moves_test() {
        let game_state = String::from("0020000000000500030000005005000000300050000000000200661");
        let result = opening(&game_state);

        assert_eq!(result.status(), 404);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "404 Not Found\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn minimax_valid_test() {
        let game_state = String::from("0020000000000500300000005005000000030050000000000200121");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 200);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "2-1: 19/21 21/22\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn minimax_invalid_test() {
        let game_state = String::from("002000000000050030000000500500000003005000000000020012n");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 404);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "404 Not Found\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn mcts_valid_test() {
        let game_state = String::from("0020000000000500300000005002000000005000300000000500121");
        let result = mcts(&game_state);

        assert_eq!(result.status(), 200);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "2-1: 20/22 22/23\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn mcts_invalid_test() {
        let game_state = String::from("x020000000000500300000005002000000005000300000000500121");
        let result = mcts(&game_state);

        assert_eq!(result.status(), 404);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "404 Not Found\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }
}

