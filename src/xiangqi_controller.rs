use std::env;

use actix_web::HttpResponse;

use super::xiangqi;

// pub fn opening(game_data: &String) -> HttpResponse {
//     match xiangqi::openings::recommended_move(game_data) {
//         Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
//         None => HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
//     }
// }

pub fn minimax(game_data: &String) -> HttpResponse {
    let mut game_state = match xiangqi::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    };

    let minimax_depth: i8 = env::var("XIANGQI_MINIMAX_DEPTH")
        .unwrap_or_else(|_| "2".to_string())
        .parse()
        .expect("XIANGQI_MINIMAX_DEPTH must be a number");

    let recommended_move = xiangqi::minimax::recommended_move(&mut game_state, minimax_depth);

    match recommended_move {
        Some(m) => {
            let external_move = build_external_move(&game_state, m);
            HttpResponse::Ok().body(format!("{}\n", external_move.format()))
        },
        None => HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    }
}

// pub fn mcts(game_data: &String) -> HttpResponse {
//     let mut game_state = match xiangqi::state::game_state::parse(game_data) {
//         Ok(gs) => gs,
//         Err(_) => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
//     };
//
//     let mcts_simulation_count: i16 = env::var("XIANGQI_MCTS_SIMULATION_COUNT")
//         .unwrap_or_else(|_| "100".to_string())
//         .parse()
//         .expect("XIANGQI_MCTS_SIMULATION_COUNT must be a number");
//
//     let mcts_simulation_depth: i16 = env::var("XIANGQI_MCTS_SIMULATION_DEPTH")
//         .unwrap_or_else(|_| "50".to_string())
//         .parse()
//         .expect("XIANGQI_MCTS_SIMULATION_DEPTH must be a number");
//
//     let recommended_move = xiangqi::mcts::recommended_move(&mut game_state, mcts_simulation_count, mcts_simulation_depth);
//
//     match recommended_move {
//         Ok(m) => {
//             let external_move = build_external_move(&game_state, m);
//             HttpResponse::Ok().body(format!("{}\n", external_move.format()))
//         },
//         Err(e) => {
//             println!("{}", e);
//             HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
//         }
//     }
// }

fn build_external_move(game_state: &xiangqi::state::game_state::GameState, mov: xiangqi::state::mov::Move) -> xiangqi::state::external_mov::ExternalMove {
    let mut new_state = game_state.clone();
    let _result = new_state.perform_move(&mov);

    let mut rank_disambiguation_count: i8 = 0;
    let mut ranks_on_file = vec![];

    for (y, row) in game_state.squares.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            let s_player_number = s.player_number;
            let s_kind = s.kind;
            if s_kind == mov.moving_piece_kind && x as i8 == mov.from.0 && s_player_number == game_state.current_player_number {
                ranks_on_file.push(y);
                rank_disambiguation_count += 1;
            }
        }
    }

    let rank_disambiguation = if rank_disambiguation_count == 2 {
        if game_state.current_player_number == 1 {
            if Some(&(mov.from.1 as usize)) == ranks_on_file.iter().min() {
                Some('+')
            } else {
                Some('-')
            }
        } else {
            if Some(&(mov.from.1 as usize)) == ranks_on_file.iter().max() {
                Some('+')
            } else {
                Some('-')
            }
        }
    } else {
        None
    };

    let pawn_disambiguation = if rank_disambiguation_count > 2 && mov.moving_piece_kind == xiangqi::state::square::PieceKind::Soldier {
        if game_state.current_player_number == 1 {
            // 0, 1, 2 => 3, 2, 1 -len
            // 0, 1, 2, 3 => 4, 3, 2, 1
            let mut rank = 0;
            for (i, x) in ranks_on_file.iter().enumerate() {
                if x == &(mov.from.0 as usize) {
                    rank = ranks_on_file.len() - i;
                }
            }
            Some(rank)
        } else {
            // 0, 1, 2 => 1, 2, 3 +1
            // 0, 1, 2, 3 => 1, 2, 3, 4
            let mut rank = 0;
            for (i, x) in ranks_on_file.iter().enumerate() {
                if x == &(mov.from.0 as usize) {
                    rank = i + 1;
                }
            }
            Some(rank)
        }
    } else {
        None
    };

    let external_mov = xiangqi::state::external_mov::ExternalMove {
        from: mov.from,
        to: mov.to,
        moving_piece_kind: mov.moving_piece_kind,
        player_number: game_state.current_player_number,
        capture_piece_kind: mov.capture_piece_kind,
        rank_disambiguation,
        pawn_disambiguation
    };

    external_mov
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::MessageBody;

    // #[test]
    // fn opening_valid_test() {
    //     let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
    //     let result = opening(&game_state);

    //     assert_eq!(result.status(), 200);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "P-84\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }

    // #[test]
    // fn opening_no_moves_test() {
    //     let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
    //     let result = opening(&game_state);

    //     assert_eq!(result.status(), 422);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }

    #[test]
    fn minimax_valid_test() {
        let game_state = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 200);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "C2+7\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn minimax_invalid_game_state_test() {
        let game_state = String::from("xheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let result = minimax(&game_state);

        assert_eq!(result.status(), 422);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    #[test]
    fn minimax_no_moves_test() {
        let game_state = String::from("1R1k1a3/R3a4/9/9/9/9/9/9/4P4/4K4 b - - 0 1");
        let result = minimax(&game_state);

        // assert_eq!(result.status(), 422);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

    // #[test]
    // fn mcts_valid_test() {
    //     let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
    //     let result = mcts(&game_state);

    //     assert_eq!(result.status(), 200);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "B*69\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }

    // #[test]
    // fn mcts_invalid_game_state_test() {
    //     let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
    //     let result = mcts(&game_state);

    //     assert_eq!(result.status(), 422);
    //     match result.into_body().try_into_bytes() {
    //        Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
    //        Err(_) => assert!(false, "unexpected body")
    //     };
    // }
}

