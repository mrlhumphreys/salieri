use std::env;

use actix_web::HttpResponse;

use super::international_draughts;

// pub fn opening(game_data: &String) -> HttpResponse {
//     match international_draughts::openings::recommended_move(game_data) {
//         Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
//         None => HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
//     }
// }

pub fn minimax(game_data: &String) -> HttpResponse {
    let game_state = match international_draughts::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    };

    let minimax_depth: i8 = env::var("INTERNATIONAL_DRAUGHTS_MINIMAX_DEPTH")
        .unwrap_or_else(|_| "8".to_string())
        .parse()
        .expect("INTERNATIONAL_DRAUGHTS_MINIMAX_DEPTH must be a number");

    let recommended_move = international_draughts::minimax::recommended_move(game_state, minimax_depth);

    match recommended_move {
        Some(m) => HttpResponse::Ok().body(format!("{}\n", m.format())),
        None => HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    }
}

// pub fn mcts(game_data: &String) -> HttpResponse {
//     let game_state = match international_draughts::state::game_state::parse(game_data) {
//         Ok(gs) => gs,
//         Err(_) => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
//     };
//
//     let mcts_simulation_count: i16 = env::var("INTERNATIONAL_DRAUGHTS_MCTS_SIMULATION_COUNT")
//         .unwrap_or_else(|_| "1000".to_string())
//         .parse()
//         .expect("INTERNATIONAL_DRAUGHTS_MCTS_SIMULATION_COUNT must be a number");
//
//     let mcts_simulation_depth: i16 = env::var("INTERNATIONAL_DRAUGHTS_MCTS_SIMULATION_DEPTH")
//         .unwrap_or_else(|_| "50".to_string())
//         .parse()
//         .expect("INTERNATIONAL_DRAUGHTS_MCTS_SIMULATION_DEPTH must be a number");
//
//     let recommended_move = international_draughts::mcts::recommended_move(game_state, mcts_simulation_count, mcts_simulation_depth);
//
//     match recommended_move {
//         Ok(m) => HttpResponse::Ok().body(format!("{}\n", m.format())),
//         Err(e) => {
//             println!("{}", e);
//             HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::MessageBody;

    // #[test]
    // fn opening_valid_test() {
    //     let game_state = String::from("W:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
    //     let result = opening(&game_state);

    //     assert_eq!(result.status(), 200);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "8-11\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }

    // #[test]
    // fn opening_no_moves_test() {
    //     let game_state = String::from("W:W:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
    //     let result = opening(&game_state);

    //     assert_eq!(result.status(), 422);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }

    #[test]
    fn minimax_valid_test() {
       let game_state = String::from("W:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
       let result = minimax(&game_state);

       assert_eq!(result.status(), 200);
       match result.into_body().try_into_bytes() {
          Ok(bytes) => assert_eq!(bytes, "20-24\n"),
          Err(_) => assert!(false, "unexpected body")
       };
    }

    #[test]
    fn minimax_invalid_game_state_test() {
        let game_state = String::from("X:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 422);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn minimax_no_moves_test() {
        let game_state = String::from("W:W:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 422);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    // #[test]
    // fn mcts_valid_test() {
    //     let game_state = String::from("W:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
    //     let result = mcts(&game_state);

    //     assert_eq!(result.status(), 200);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "7-11\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }

    // #[test]
    // fn mcts_invalid_game_state_test() {
    //     let game_state = String::from("X:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
    //     let result = mcts(&game_state);

    //     assert_eq!(result.status(), 422);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }
}

