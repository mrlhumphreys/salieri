use std::env;

use actix_web::HttpResponse;

use super::checkers;

pub fn opening(game_data: &String) -> HttpResponse {
    match checkers::openings::recommended_move(game_data) {
        Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
        None => HttpResponse::NotFound().body("404 Not Found\n")
    }
}

pub fn minimax(game_data: &String) -> HttpResponse {
    let game_state = match checkers::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::NotFound().body("404 Not Found\n"),
    };

    let minimax_depth: i8 = env::var("CHECKERS_MINIMAX_DEPTH")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .expect("CHECKERS_MINIMAX_DEPTH must be a number");

    let recommended_move = checkers::minimax::recommended_move(game_state, minimax_depth);

    match recommended_move {
        Some(m) => HttpResponse::Ok().body(format!("{}\n", m.format())),
        None => HttpResponse::NotFound().body("404 Not Found\n"),
    }
}

pub fn mcts(game_data: &String) -> HttpResponse {
    let game_state = match checkers::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::NotFound().body("404 Not Found\n"),
    };

    let mcts_simulation_count: i16 = env::var("CHECKERS_MCTS_SIMULATION_COUNT")
        .unwrap_or_else(|_| "120".to_string())
        .parse()
        .expect("CHECKERS_MCTS_SIMULATION_COUNT must be a number");

    let mcts_simulation_depth: i16 = env::var("CHECKERS_MCTS_SIMULATION_DEPTH")
        .unwrap_or_else(|_| "40".to_string())
        .parse()
        .expect("CHECKERS_MCTS_SIMULATION_DEPTH must be a number");

    let recommended_move = checkers::mcts::recommended_move(game_state, mcts_simulation_count, mcts_simulation_depth);

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
        let game_state = String::from("bbbbbbb-bbbb--b---w-ww-wwwwwwwwww");
        let result = opening(&game_state); 

        assert_eq!(result.status(), 200); 
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "22-17\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn opening_no_moves_test() {
        let game_state = String::from("----bbb-bbbb--b---w-ww-wwwwwwwwww");
        let result = opening(&game_state); 

        assert_eq!(result.status(), 404); 
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "404 Not Found\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn minimax_valid_test() {
        let game_state = String::from("bbbbbbb-bbbb--b---w-ww-wwwwwwwwww");
        let result = minimax(&game_state); 

        assert_eq!(result.status(), 200); 
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "21-17\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn minimax_invalid_game_state_test() {
        let game_state = String::from("bbbbbbb-bbbb--b---w-ww-wwwwwwwwwn");
        let result = minimax(&game_state); 

        assert_eq!(result.status(), 404); 
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "404 Not Found\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn minimax_no_moves_test() {
        let game_state = String::from("bbbbbbb-bbbb--b-----------------w");
        let result = minimax(&game_state); 

        assert_eq!(result.status(), 404); 
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "404 Not Found\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn mcts_valid_test() {
        let game_state = String::from("bbbbbbb-bbbb--b---w-ww-wwwwwwwwww");
        let result = mcts(&game_state);

        assert_eq!(result.status(), 200);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "27-23\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn mcts_invalid_game_state_test() {
        let game_state = String::from("bbbbbbb-bbbb--b---w-ww-wwwwwwwwwn");
        let result = mcts(&game_state);

        assert_eq!(result.status(), 404);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "404 Not Found\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }
}

