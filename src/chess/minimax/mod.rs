use std::cmp;
use std::convert::TryFrom;
use crate::chess;
use crate::chess::state::piece::PieceKind;

const CENTER_SQUARE_COORDINATES: [[i8; 2]; 4] = [
    [3,3],
    [3,4],
    [4,3],
    [4,4]
];

pub fn recommended_move(game_state: &mut chess::state::game_state::GameState, depth: i8) -> Option<chess::state::mov::Move> {
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
                1 => true,
                2 => false,
                _ => true,
            };

            let moves_with_value = moves.iter().map(|mov| {
                match new_game_state.perform_move(mov) {
                    Ok(_) => (),
                    Err(_) => return (mov, 0),
                };

                let value = match evaluate(&mut new_game_state, depth, std::i32::MIN, std::i32::MAX, maximizing_player) {
                    Ok(v) => v,
                    Err(_) => 0,
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

pub fn evaluate(game_state: &mut chess::state::game_state::GameState, depth: i8, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> Result<i32, &'static str> {
    let moves = game_state.possible_moves();
    if depth == 0 || moves.len() == 0 {
        return Ok(static_evaluation(game_state));
    }

    if maximizing_player {
        let mut max_eval = std::i32::MIN;
        for mov in moves {
            match game_state.perform_move(&mov) {
                Ok(()) => {
                    match evaluate(game_state, depth - 1, alpha, beta, false) {
                        Ok(eval) => {
                            max_eval = cmp::max(max_eval, eval);
                            alpha = cmp::max(alpha, eval);
                        },
                        Err(e) => return Err(e),
                    }
                },
                Err(e) => return Err(e),
            };

            match game_state.undo_move(&mov) {
                Ok(()) => (),
                Err(e) => return Err(e)
            };

            if beta <= alpha {
                break;
            }
        }
        Ok(max_eval)
    } else {
        let mut min_eval = std::i32::MAX;
        for mov in moves {
            match game_state.perform_move(&mov) {
                Ok(()) => {
                    match evaluate(game_state, depth - 1, alpha, beta, true) {
                        Ok(eval) => {
                            min_eval = cmp::min(min_eval, eval);
                            beta = cmp::min(beta, eval);
                        },
                        Err(e) => return Err(e),
                    }
                },
                Err(e) => {
                    return Err(e);
                },
            };

            match game_state.undo_move(&mov) {
                Ok(()) => (),
                Err(e) => return Err(e)
            };

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
// p: 1, n: 3, b: 3, r: 5, q: 9, k: 200
pub fn static_evaluation(game_state: &mut chess::state::game_state::GameState) -> i32 {
    let player_one_pieces_count = player_pieces_count(game_state, 1);
    let player_two_pieces_count = player_pieces_count(game_state, 2);
    let pieces_count_value = u_to_i32(player_one_pieces_count) - u_to_i32(player_two_pieces_count);

    let player_one_center_squares_count = center_squares_count(game_state, 1);
    let player_two_center_squares_count = center_squares_count(game_state, 2);
    let center_squares_count_value = u_to_i32(player_one_center_squares_count) - u_to_i32(player_two_center_squares_count);

    let player_one_possible_moves_count = game_state.possible_moves_for_player(1).len();
    let player_two_possible_moves_count = game_state.possible_moves_for_player(2).len();
    let possible_moves_value = u_to_i32(player_one_possible_moves_count) - u_to_i32(player_two_possible_moves_count);

    2*pieces_count_value + 1*center_squares_count_value + 4*possible_moves_value
}

fn center_squares_count(game_state: &chess::state::game_state::GameState, player_number: i8) -> usize {
    game_state.squares.iter().filter(|s| {
        match &s.piece {
            Some(p) => {
                p.player_number == player_number && CENTER_SQUARE_COORDINATES.iter().any(|c| s.x == c[0] && s.y == c[1] )
            },
            None => false
        }
    }).count()
}

// p: 1, n: 3, b: 3, r: 5, q: 9, k: 200
fn player_pieces_count(game_state: &chess::state::game_state::GameState, player_number: i8) -> usize {
    game_state.squares.iter().filter(|s| {
        match &s.piece {
            Some(p) => p.player_number == player_number,
            None => false
        }
    }).map(|s| {
        match s.piece {
            Some(p) => {
                match p.kind {
                    PieceKind::King => 200,
                    PieceKind::Queen => 9,
                    PieceKind::Rook => 5,
                    PieceKind::Bishop => 3,
                    PieceKind::Knight => 3,
                    PieceKind::Pawn => 1
                }
            },
            None => 0
        }
    }).sum()
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
    use crate::chess::state::point::Point;

    #[test]
    fn evaluate_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut game_state = chess::state::game_state::parse(&encoded).unwrap();

        match evaluate(&mut game_state, 2, std::i32::MIN, std::i32::MAX, false) {
            Ok(result) => assert_eq!(result, -1033),
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn recommended_move_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut game_state = chess::state::game_state::parse(&encoded).unwrap();

        let mov = recommended_move(&mut game_state, 2);

        match mov {
            Some(m) => {
                assert_eq!(m.from, Point { x: 1, y: 7 });
                assert_eq!(m.to, Point { x: 2, y: 5 });
                assert_eq!(m.moving_piece_kind, PieceKind::Knight);
                assert_eq!(m.capture_piece_kind, None);
                assert_eq!(m.promote_piece_kind, None);
                assert_eq!(m.en_passant_point, None);
                assert_eq!(m.en_passant_target, None);
                assert_eq!(m.castle_move, None);
            },
            None => assert!(false, "expected move"),
        }
    }
}
