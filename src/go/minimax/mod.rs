use std::cmp;
use crate::go;

pub fn recommended_move(game_state: &mut go::state::game_state::GameState, depth: i8) -> Option<go::state::mov::Move> {
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
                Some(h) => Some(h.0.clone()),
                None => None,
            }
        }
    }
}

pub fn evaluate(game_state: &mut go::state::game_state::GameState, depth: i8, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> Result<i32, &'static str> {
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
pub fn static_evaluation(game_state: &mut go::state::game_state::GameState) -> i32 {
    ((game_state.players_score(1) - game_state.players_score(2)) * 2.0) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_test() {
        let encoded = String::from("PL[B]XB[0]XW[0]");
        let mut game_state = go::state::game_state::parse(&encoded).unwrap();

        match evaluate(&mut game_state, 1, std::i32::MIN, std::i32::MAX, false) {
            Ok(result) => assert_eq!(result, -13),
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn recommended_move_test() {
        let encoded = String::from("PL[B]XB[0]XW[0]");
        let mut game_state = go::state::game_state::parse(&encoded).unwrap();

        let mov = recommended_move(&mut game_state, 0);

        match mov {
            Some(m) => {
                let expected_game_state = vec![
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                    vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0]
                ];
                assert_eq!(m.x, 18);
                assert_eq!(m.y, 18);
                assert_eq!(m.simplified_game_state, expected_game_state);
                assert_eq!(m.captures, vec![]);
            },
            None => assert!(false, "expected move"),
        }
    }
}
