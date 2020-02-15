use std::cmp;
use std::convert::TryFrom;
use crate::checkers;

const DEPTH: i8 = 10;

pub fn recommended_move(game_state: checkers::game_state::GameState) -> Option<checkers::mov::Move> {
    let moves = game_state.possible_moves();
    match moves.len() {
        0 => None,
        1 => match moves.first() {
            Some(s) => Some(s.clone()),
            None => None
        },
        _ => {
            let moves_with_value = moves.iter().map(|mov| {
                let new_game_state = match game_state.perform_move(mov) {
                    Ok(n) => n,
                    Err(_) => return (mov, 0),
                };

                let maximizing_player = match new_game_state.current_player_number {
                    1 => true,
                    2 => false,
                    _ => true,
                };

                let value = match evaluate(&new_game_state, DEPTH, std::i32::MIN, std::i32::MAX, maximizing_player) {
                    Ok(v) => v,
                    Err(_) => 0,
                };

                (mov, value)
            });

            let highest_value_move = moves_with_value.max_by(|a,b| (a.1).cmp(&b.1) );

            match highest_value_move {
                Some(h) => Some((h.0).clone()),
                None => None,
            }
        }
    }
}

pub fn evaluate(game_state: &checkers::game_state::GameState, depth: i8, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> Result<i32, &'static str> {
    let moves = game_state.possible_moves();
    if depth == 0 || moves.len() == 0 {
        return Ok(static_evaluation(&game_state));
    }
    
    if maximizing_player {
        let mut max_eval = std::i32::MIN;
        for mov in moves {
            match game_state.perform_move(&mov) {
                Ok(new_game_state) => {
                    match evaluate(&new_game_state, depth - 1, alpha, beta, false) {
                        Ok(eval) => {
                            max_eval = cmp::max(max_eval, eval);
                            alpha = cmp::max(alpha, eval);
                        },
                        Err(e) => return Err(e),
                    }
                },
                Err(e) => return Err(e), 
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
                Ok(new_game_state) => {
                    match evaluate(&new_game_state, depth - 1, alpha, beta, true) {
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
            if beta <= alpha {
                break;
            }
        }
        Ok(min_eval)
    }
}

// positive -> w
// negative -> b
pub fn static_evaluation(game_state: &checkers::game_state::GameState) -> i32 {
    let player_one_pieces_count = player_pieces_count(game_state, 1);
    let player_two_pieces_count = player_pieces_count(game_state, 2);
    let pieces_count_value = u_to_i32(player_one_pieces_count) - u_to_i32(player_two_pieces_count);
   
    let player_one_kings_count = player_kings_count(game_state, 1);
    let player_two_kings_count = player_kings_count(game_state, 2);
    let kings_count_value = u_to_i32(player_one_kings_count) - u_to_i32(player_two_kings_count);
   
    10*pieces_count_value + 40*kings_count_value
}

fn player_pieces_count(game_state: &checkers::game_state::GameState, player_number: i8) -> usize {
    game_state.squares.squares.iter().filter(|s| {
        match s.piece {
            Some(p) => p.player_number == player_number,
            None => false,
        }
    }).count()
}

fn player_kings_count(game_state: &checkers::game_state::GameState, player_number: i8) -> usize {
    game_state.squares.squares.iter().filter(|s| {
        match s.piece {
            Some(p) => p.king && p.player_number == player_number,
            None => false,
        }
    }).count()
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
        let encoded = String::from("bbbbbbbbbb-b--b-----wwwwwwwwwwwww");
        let game_state = checkers::game_state::parse(&encoded).unwrap();

        match evaluate(&game_state, 4, std::i32::MIN, std::i32::MAX, false) {
            Ok(result) => assert_eq!(result, 0),
            Err(e) => assert!(false, e),
        }
    }

    #[test]
    fn recommended_move_test() {
        let encoded = String::from("bbbbbbbbbb-b--b-----wwwwwwwwwwwww");
        let game_state = checkers::game_state::parse(&encoded).unwrap();
        let mov = recommended_move(game_state); 

        match mov {
            Some(m) => {
                assert_eq!(m.from, 24);
                assert_eq!(m.to, vec![20]);
            },
            None => assert!(false, "expected move"), 
        }
    }
}
