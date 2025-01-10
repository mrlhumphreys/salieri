use std::cmp;
use std::convert::TryFrom;
use crate::shogi::state::square::PieceKind;
use crate::shogi;

pub fn recommended_move(game_state: &mut shogi::state::game_state::GameState, depth: i8) -> Option<shogi::state::mov::Move> {
    let mut new_game_state = game_state.clone();
    let moves = game_state.possible_moves();
    match moves.len() {
        0 => None,
        1 => match moves.first() {
            Some(s) => Some(s.clone()),
            None => None
        },
        _ => {
            let maximizing_player = match new_game_state.current_player_number {
                2 => false,
                _ => true
            };

            let moves_with_value = moves.iter().map(|mov| {
                match new_game_state.perform_move(mov) {
                    Ok(_) => (),
                    Err(_) => return (mov, 0),
                };

                let value = match evaluate(&mut new_game_state, depth, std::i32::MIN, std::i32::MAX, maximizing_player) {
                    Ok(v) => v,
                    Err(_) => {
                        // TODO: pass error up instead of ignoring
                        0
                    }
                };

                match new_game_state.undo_move(mov) {
                    Ok(_) => (),
                    Err(_) => return (mov, 0)
                }

                (mov, value)
            });

            let best_move = match maximizing_player {
                true => moves_with_value.max_by(|a,b| (a.1).cmp(&b.1) ),
                false => moves_with_value.min_by(|a,b| (a.1).cmp(&b.1) ),
            };

            match best_move {
                Some(h) => Some((h.0).clone()),
                None => None,
            }
        }
    }
}

pub fn evaluate(game_state: &mut shogi::state::game_state::GameState, depth: i8, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> Result<i32, &'static str> {
    let moves = game_state.possible_moves();

    if depth == 0 || moves.len() == 0 {
        return Ok(static_evaluation(game_state));
    }

    if maximizing_player {
        let mut max_eval = std::i32::MIN;
        for mov in moves {
            game_state.perform_move(&mov)?;

            match evaluate(game_state, depth - 1, alpha, beta, false) {
                Ok(eval) => {
                    max_eval = cmp::max(max_eval, eval);
                    alpha = cmp::max(alpha, eval);
                },
                Err(e) => return Err(e),
            }

            game_state.undo_move(&mov)?;

            if beta <= alpha {
                break;
            }
        }
        Ok(max_eval)
    } else {
        let mut min_eval = std::i32::MAX;
        for mov in moves {
            game_state.perform_move(&mov)?;

            match evaluate(game_state, depth - 1, alpha, beta, true) {
                Ok(eval) => {
                    min_eval = cmp::min(min_eval, eval);
                    beta = cmp::min(beta, eval);
                },
                Err(e) => return Err(e),
            }

            game_state.undo_move(&mov)?;

            if beta <= alpha {
                break;
            }
        }
        Ok(min_eval)
    }
}

// positive -> w
// negative -> b
// Piece Scores
pub fn static_evaluation(game_state: &mut shogi::state::game_state::GameState) -> i32 {
    let player_one_pieces_count = player_pieces_count(game_state, 1);
    let player_two_pieces_count = player_pieces_count(game_state, 2);
    let pieces_count_value = u_to_i32(player_one_pieces_count) - u_to_i32(player_two_pieces_count);

    let player_one_possible_moves_count = game_state.possible_moves_for_player(1).len();
    let player_two_possible_moves_count = game_state.possible_moves_for_player(2).len();
    let possible_moves_value = u_to_i32(player_one_possible_moves_count) - u_to_i32(player_two_possible_moves_count);

    10*pieces_count_value + 1*possible_moves_value
}

fn player_pieces_count(game_state: &shogi::state::game_state::GameState, player_number: i8) -> usize {
    let mut score: usize = 0;
    for row in game_state.squares.iter() {
        for s in row.iter() {
            if s.player_number == player_number {
                score += match s.kind {
                    PieceKind::Oushou => 200,
                    PieceKind::Gyokushou => 200,
                    PieceKind::Hisha => 9,
                    PieceKind::Kakugyou => 8,
                    PieceKind::Kinshou => 6,
                    PieceKind::Ginshou => 5,
                    PieceKind::Keima => 4,
                    PieceKind::Kyousha => 3,
                    PieceKind::Fuhyou => 1,
                    PieceKind::Ryuuou => 11,
                    PieceKind::Ryuuma => 10,
                    PieceKind::Narigin => 6,
                    PieceKind::Narikei => 6,
                    PieceKind::Narikyou => 6,
                    PieceKind::Tokin => 7,
                    PieceKind::Empty => 0
                };
            }
        }
    }
    score
}

fn u_to_i32(value: usize) -> i32 {
    match i32::try_from(value) {
        Ok(v) =>  v,
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
        let mut game_state = shogi::state::game_state::parse(&encoded).unwrap();

        match evaluate(&mut game_state, 0, std::i32::MIN, std::i32::MAX, false) {
            Ok(result) => assert_eq!(result, 6),
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn recommended_move_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
        let mut game_state = shogi::state::game_state::parse(&encoded).unwrap();

        let mov = recommended_move(&mut game_state, 0);

        match mov {
            Some(m) => {
                assert_eq!(m.from, None);
                assert_eq!(m.to, (5, 0));
                assert_eq!(m.moving_piece_kind, PieceKind::Kakugyou);
                assert_eq!(m.capture_piece_kind, None);
                assert_eq!(m.promote, false);
            },
            None => assert!(false, "expected move"),
        }
    }
}
