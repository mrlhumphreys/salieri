use std::env;
use actix_web::HttpResponse;
use super::go;

pub fn opening(game_data: &String) -> HttpResponse {
   match go::openings::recommended_move(game_data) {
        Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
        None => HttpResponse::NotFound().body("404 Not Found\n")
    }
}

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

pub fn mcts(game_data: &String) -> HttpResponse {
    let mut game_state = match go::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n"),
    };

    let mcts_simulation_count: i16 = env::var("GO_MCTS_SIMULATION_COUNT")
        .unwrap_or_else(|_| "40".to_string())
        .parse()
        .expect("GO_MCTS_SIMULATION_COUNT must be a number");

    let mcts_simulation_depth: i16 = env::var("GO_MCTS_SIMULATION_DEPTH")
        .unwrap_or_else(|_| "20".to_string())
        .parse()
        .expect("GO_MCTS_SIMULATION_DEPTH must be a number");

    let recommended_move = go::mcts::recommended_move(&mut game_state, mcts_simulation_count, mcts_simulation_depth);

    match recommended_move {
        Ok(m) => {
            HttpResponse::Ok().body(format!("{}\n", m.format()))
        },
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
        let game_state = String::from("PL[B]ABAWXB[0]XW[0]XS");
        let result = opening(&game_state);

        assert_eq!(result.status(), 200);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => {
               let option_a = bytes == "dd\n";
               let option_b = bytes == "pd\n";
               let option_c = bytes == "dp\n";
               let option_d = bytes == "pp\n";
               let result = option_a || option_b || option_c || option_d;
               assert!(result);
           },
           Err(_) => assert!(false, "unexpected body")
        };
    }

     #[test]
     fn opening_no_moves_test() {
         let game_state = String::from("PL[W]AB[aa]AWXB[0]XW[0]XS");
         let result = opening(&game_state);

         assert_eq!(result.status(), 404);
         match result.into_body().try_into_bytes() {
            Ok(bytes) => assert_eq!(bytes, "404 Not Found\n"),
            Err(_) => assert!(false, "unexpected body")
         };
     }

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

    #[test]
    fn mcts_valid_test() {
        let game_state = String::from("PL[B]XB[0]XW[0]");
        let result = mcts(&game_state);

        assert_eq!(result.status(), 200);
        match result.into_body().try_into_bytes() {
            Ok(bytes) => assert_eq!(bytes, "tt\n"),
            Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn mcts_invalid_game_state_test() {
        let game_state = String::from("asdf");
        let result = mcts(&game_state);

        assert_eq!(result.status(), 422);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }
}
