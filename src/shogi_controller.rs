use std::env;

use actix_web::HttpResponse;

use super::shogi;

// pub fn opening(game_data: &String) -> HttpResponse {
//     match shogi::openings::recommended_move(game_data) {
//         Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
//         None => HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
//     }
// }

pub fn minimax(game_data: &String) -> HttpResponse {
    let mut game_state = match shogi::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    };

    let minimax_depth: i8 = env::var("SHOGI_MINIMAX_DEPTH")
        .unwrap_or_else(|_| "0".to_string())
        .parse()
        .expect("SHOGI_MINIMAX_DEPTH must be a number");

    let recommended_move = shogi::minimax::recommended_move(&mut game_state, minimax_depth);

    match recommended_move {
        Some(m) => {
            let external_move = build_external_move(&game_state, m);
            HttpResponse::Ok().body(format!("{}\n", external_move.format()))
        },
        None => HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    }
}

// pub fn mcts(game_data: &String) -> HttpResponse {
//     let game_state = match shogi::state::game_state::parse(game_data) {
//         Ok(gs) => gs,
//         Err(_) => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
//     };
//
//     let mcts_simulation_count: i16 = env::var("SHOGI_MCTS_SIMULATION_COUNT")
//         .unwrap_or_else(|_| "1000".to_string())
//         .parse()
//         .expect("SHOGI_MCTS_SIMULATION_COUNT must be a number");
//
//     let mcts_simulation_depth: i16 = env::var("SHOGI_MCTS_SIMULATION_DEPTH")
//         .unwrap_or_else(|_| "50".to_string())
//         .parse()
//         .expect("SHOGI_MCTS_SIMULATION_DEPTH must be a number");
//
//     let recommended_move = shogi::mcts::recommended_move(game_state, mcts_simulation_count, mcts_simulation_depth);
//
//     match recommended_move {
//         Ok(m) => HttpResponse::Ok().body(format!("{}\n", m.format())),
//         Err(e) => {
//             println!("{}", e);
//             HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
//         }
//     }
// }

fn build_external_move(game_state: &shogi::state::game_state::GameState, mov: shogi::state::mov::Move) -> shogi::state::external_mov::ExternalMove {
    let mut new_state = game_state.clone();
    let _result = new_state.perform_move(&mov);

    let file_disambiguation = match mov.from {
        Some(f) => {
            let mut file_disambiguation_count: i8 = 0;
            for (y, row) in game_state.squares.iter().enumerate() {
                for s in row.iter() {
                    let s_player_number = s.player_number;
                    let s_kind = s.kind;
                    if s_kind == mov.moving_piece_kind && y as i8 == f.1 && s_player_number == game_state.current_player_number {
                        file_disambiguation_count += 1;
                    }
                }
            }
            file_disambiguation_count > 1
        },
        None => false
    };

    let rank_disambiguation = match mov.from {
        Some(f) => {
            let mut count: i8 = 0;
            for row in game_state.squares.iter() {
                for (x, s) in row.iter().enumerate() {
                    let s_player_number = s.player_number;
                    let s_kind = s.kind;
                    if s_kind == mov.moving_piece_kind && x as i8 == f.0 && s_player_number == game_state.current_player_number {
                        count += 1;
                    }
                }
            }
            count > 1
        },
        None => false
    };

    let disambiguation = file_disambiguation || rank_disambiguation;

    let promotion_possible = if shogi::state::square::PROMOTABLE_PIECE_KINDS.iter().any(|pk| *pk == mov.moving_piece_kind) {
        if game_state.current_player_number == 1 {
            mov.to.1 == 0 || mov.to.1 == 1 || mov.to.1 == 2
        } else {
            mov.to.1 == 6 || mov.to.1 == 7 || mov.to.1 == 8
        }
    } else {
        false
    };

    let external_mov = shogi::state::external_mov::ExternalMove {
        from: mov.from,
        to: mov.to,
        moving_piece_kind: mov.moving_piece_kind,
        capture_piece_kind: mov.capture_piece_kind,
        promote: mov.promote,
        promotion_possible,
        disambiguation
    };

    external_mov
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::MessageBody;

    // #[test]
    // fn opening_valid_test() {
    //     let game_state = String::from("B:W18,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15");
    //     let result = opening(&game_state);

    //     assert_eq!(result.status(), 200);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "8-11\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }

    // #[test]
    // fn opening_no_moves_test() {
    //     let game_state = String::from("W:W:B1,2,3,4,5,6,7,8,9,10,12,15");
    //     let result = opening(&game_state);

    //     assert_eq!(result.status(), 422);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }

    #[test]
    fn minimax_valid_test() {
       let game_state = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
       let result = minimax(&game_state);

       assert_eq!(result.status(), 200);
       match result.into_body().try_into_bytes() {
          Ok(bytes) => assert_eq!(bytes, "B*41\n"),
          Err(_) => assert!(false, "unexpected body")
       };
    }

    #[test]
    fn minimax_invalid_game_state_test() {
        let game_state = String::from("xnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 422);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn minimax_no_moves_test() {
        let game_state = String::from("k7R/8R/9/9/9/9/9/9/8K w -");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 422);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    // #[test]
    // fn mcts_valid_test() {
    //     let game_state = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
    //     let result = mcts(&game_state);

    //     assert_eq!(result.status(), 200);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "7-11\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }

    // #[test]
    // fn mcts_invalid_game_state_test() {
    //     let game_state = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
    //     let result = mcts(&game_state);

    //     assert_eq!(result.status(), 422);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }
}

