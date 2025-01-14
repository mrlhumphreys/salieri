use std::env;
use actix_web::HttpResponse;
use super::chess;

pub fn opening(game_data: &String) -> HttpResponse {
   match chess::openings::recommended_move(game_data) {
        Some(m) => HttpResponse::Ok().body(format!("{}\n", m)),
        None => HttpResponse::NotFound().body("404 Not Found\n")
    }
}

pub fn minimax(game_data: &String) -> HttpResponse {
    let mut game_state = match chess::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    };

    let minimax_depth: i8 = env::var("CHESS_MINIMAX_DEPTH")
        .unwrap_or_else(|_| "3".to_string())
        .parse()
        .expect("CHESS_MINIMAX_DEPTH must be a number");

    let recommended_move = chess::minimax::recommended_move(&mut game_state, minimax_depth);

    match recommended_move {
        Some(m) => {
            let external_move = build_external_move(&game_state, m);
            HttpResponse::Ok().body(format!("{}\n", external_move.format()))
        },
        None => HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n")
    }
}

pub fn mcts(game_data: &String) -> HttpResponse {
    let mut game_state = match chess::state::game_state::parse(game_data) {
        Ok(gs) => gs,
        Err(_) => return HttpResponse::UnprocessableEntity().body("422 Unprocessable Entity\n"),
    };

    let mcts_simulation_count: i16 = env::var("CHESS_MCTS_SIMULATION_COUNT")
        .unwrap_or_else(|_| "120".to_string())
        .parse()
        .expect("CHESS_MCTS_SIMULATION_COUNT must be a number");

    let mcts_simulation_depth: i16 = env::var("CHESS_MCTS_SIMULATION_DEPTH")
        .unwrap_or_else(|_| "60".to_string())
        .parse()
        .expect("CHESS_MCTS_SIMULATION_DEPTH must be a number");

    let recommended_move = chess::mcts::recommended_move(&mut game_state, mcts_simulation_count, mcts_simulation_depth);

    match recommended_move {
        Ok(m) => {
            let external_move = build_external_move(&game_state, m);
            HttpResponse::Ok().body(format!("{}\n", external_move.format()))
        },
        Err(e) => {
            println!("{}", e);
            HttpResponse::NotFound().body("404 Not Found\n")
        }
    }
}

fn build_external_move(game_state: &chess::state::game_state::GameState, mov: chess::state::mov::Move) -> chess::state::external_mov::ExternalMove {
    let mut new_state = game_state.clone();
    let _result = new_state.perform_move(&mov);

    let file_disambiguation = match mov.moving_piece_kind {
        chess::state::square::PieceKind::Pawn => mov.capture_piece_kind.is_some(),
        _ => {
            let mut count: i8 = 0;
            for (y, row) in game_state.squares.iter().enumerate() {
                for s in row.iter() {
                    let s_player_number = s.player_number;
                    let s_kind = s.kind;
                    if s_kind == mov.moving_piece_kind && y as i8 == mov.from.1 && s_player_number == game_state.current_player_number {
                        count += 1;
                    }
                }
            }
            count > 1
        }
    };

    let rank_disambiguation = match mov.moving_piece_kind {
        chess::state::square::PieceKind::Pawn => false,
        _ => {
            let mut count: i8 = 0;
            for row in game_state.squares.iter() {
                for (x, s) in row.iter().enumerate() {
                    let s_player_number = s.player_number;
                    let s_kind = s.kind;
                    if s_kind == mov.moving_piece_kind && x as i8 == mov.from.0 && s_player_number == game_state.current_player_number {
                        count += 1;
                    }
                }
            }
            count > 1
        }
    };

    let in_check = new_state.in_check(new_state.current_player_number);
    let in_checkmate = new_state.in_checkmate(new_state.current_player_number);

    let external_mov = chess::state::external_mov::ExternalMove {
        from: mov.from,
        to: mov.to,
        moving_piece_kind: mov.moving_piece_kind,
        capture_piece_kind: mov.capture_piece_kind,
        promote_piece_kind: mov.promote_piece_kind,
        en_passant_point: mov.en_passant_point,
        en_passant_target: mov.en_passant_target,
        castle_move: mov.castle_move,
        file_disambiguation,
        rank_disambiguation,
        in_check,
        in_checkmate
    };

    external_mov
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::MessageBody;

    #[test]
    fn opening_valid_test() {
        let game_state = String::from("rnbqkbnr/ppp1pppp/3p4/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");
        let result = opening(&game_state);

        assert_eq!(result.status(), 200);
        match result.into_body().try_into_bytes() {
           Ok(bytes) => assert_eq!(bytes, "d4\n"),
           Err(_) => assert!(false, "unexpected body")
        };
    }

     #[test]
     fn opening_no_moves_test() {
         let game_state = String::from("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 16");
         let result = opening(&game_state);

         assert_eq!(result.status(), 404);
         match result.into_body().try_into_bytes() {
            Ok(bytes) => assert_eq!(bytes, "404 Not Found\n"),
            Err(_) => assert!(false, "unexpected body")
         };
     }

     #[test]
     fn minimax_valid_test() {
         let game_state = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
         let result = minimax(&game_state);

         assert_eq!(result.status(), 200);
         match result.into_body().try_into_bytes() {
            Ok(bytes) => assert_eq!(bytes, "e3\n"),
            Err(_) => assert!(false, "unexpected body")
         };
     }

     #[test]
     fn minimax_invalid_game_state_test() {
         let game_state = String::from("znbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
         let result = minimax(&game_state);

         assert_eq!(result.status(), 422);
         match result.into_body().try_into_bytes() {
            Ok(bytes) => assert_eq!(bytes, "422 Unprocessable Entity\n"),
            Err(_) => assert!(false, "unexpected body")
         };
     }

//     #[test]
//     fn minimax_no_moves_test() {
//         let game_state = String::from("bbbbbbb-bbbb--b-----------------w");
//         let result = minimax(&game_state);
//
//         assert_eq!(result.status(), 404);
//         match result.into_body().try_into_bytes() {
//            Ok(bytes) => assert_eq!(bytes, "404 Not Found\n"),
//            Err(_) => assert!(false, "unexpected body")
//         };
//     }

//    #[test]
//    fn mcts_valid_test() {
//        let game_state = String::from("bbbbbbb-bbbb--b---w-ww-wwwwwwwwww");
//        let result = mcts(&game_state);
//
//        assert_eq!(result.status(), 200);
//        match result.into_body().try_into_bytes() {
//           Ok(bytes) => assert_eq!(bytes, "27-23\n"),
//           Err(_) => assert!(false, "unexpected body")
//        };
//    }
//
//    #[test]
//    fn mcts_invalid_game_state_test() {
//        let game_state = String::from("bbbbbbb-bbbb--b---w-ww-wwwwwwwwwn");
//        let result = mcts(&game_state);
//
//        assert_eq!(result.status(), 404);
//        match result.into_body().try_into_bytes() {
//           Ok(bytes) => assert_eq!(bytes, "404 Not Found\n"),
//           Err(_) => assert!(false, "unexpected body")
//        };
//    }

     #[test]
     fn build_external_move_file_disambiguation_test() {
        let encoded = String::from("4kr1r/8/8/8/8/8/8/4K3 b - - 0 1");
        let state = chess::state::game_state::parse(&encoded).unwrap();
        let mov = chess::state::mov::Move {
            from: (5, 0),
            to: (6, 0),
            moving_piece_kind: chess::state::square::PieceKind::Rook,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };
        let result = build_external_move(&state, mov);

        assert_eq!(result.file_disambiguation, true);
     }

     #[test]
     fn build_external_move_rank_disambiguation_test() {
        let encoded = String::from("4k3/8/8/R7/8/8/8/R3K3 w - - 0 1");
        let state = chess::state::game_state::parse(&encoded).unwrap();
        let mov = chess::state::mov::Move {
            from: (0, 3),
            to: (0, 4),
            moving_piece_kind: chess::state::square::PieceKind::Rook,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = build_external_move(&state, mov);

        assert_eq!(result.rank_disambiguation, true);
     }

     #[test]
     fn build_external_move_file_disambiguation_pawn_capture_test() {
        let encoded = String::from("4k3/p7/1P6/8/8/8/8/4K3 b - - 0 1");
        let state = chess::state::game_state::parse(&encoded).unwrap();
        let mov = chess::state::mov::Move {
            from: (0, 1),
            to: (1, 2),
            moving_piece_kind: chess::state::square::PieceKind::Pawn,
            capture_piece_kind: Some(chess::state::square::PieceKind::Pawn),
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = build_external_move(&state, mov);

        assert_eq!(result.file_disambiguation, true);
     }

     #[test]
     fn build_external_move_file_and_rank_disambiguation_test() {
        let encoded = String::from("5k2/8/8/8/4Q2Q/8/8/5K1Q w - - 0 1");
        let state = chess::state::game_state::parse(&encoded).unwrap();
        let mov = chess::state::mov::Move {
            from: (7, 4),
            to: (4, 7),
            moving_piece_kind: chess::state::square::PieceKind::Queen,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = build_external_move(&state, mov);

        assert_eq!(result.file_disambiguation, true);
        assert_eq!(result.rank_disambiguation, true);
     }

     #[test]
     fn build_external_move_in_check_test() {
        let encoded = String::from("4k3/7R/8/8/8/8/8/4K3 w - - 0 1");
        let state = chess::state::game_state::parse(&encoded).unwrap();
        let mov = chess::state::mov::Move {
            from: (7, 1),
            to: (7, 0),
            moving_piece_kind: chess::state::square::PieceKind::Rook,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = build_external_move(&state, mov);
        assert_eq!(result.in_check, true);
     }

     #[test]
     fn build_external_move_in_checkmate_test() {
        let encoded = String::from("4k3/R6R/8/8/8/8/8/4K3 w - - 0 1");
        let state = chess::state::game_state::parse(&encoded).unwrap();
        let mov = chess::state::mov::Move {
            from: (0, 1),
            to: (0, 0),
            moving_piece_kind: chess::state::square::PieceKind::Rook,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = build_external_move(&state, mov);
        assert_eq!(result.in_checkmate, true);
     }
}

