use std::cmp;
use std::convert::TryFrom;
use crate::checkers;

const CENTER_SQUARE_IDS: [i8; 4] = [14, 15, 18, 19];

pub fn recommended_move(game_state: checkers::state::game_state::GameState, depth: i8) -> Option<checkers::state::mov::Move> {
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

pub fn evaluate(game_state: &mut checkers::state::game_state::GameState, depth: i8, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> Result<i32, &'static str> {
    let moves = game_state.possible_moves();
    if depth == 0 || moves.len() == 0 {
        return Ok(static_evaluation(&game_state));
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
pub fn static_evaluation(game_state: &checkers::state::game_state::GameState) -> i32 {
    let player_one_pieces_count = player_pieces_count(game_state, 1);
    let player_two_pieces_count = player_pieces_count(game_state, 2);
    let pieces_count_value = u_to_i32(player_one_pieces_count) - u_to_i32(player_two_pieces_count);

    let player_one_kings_count = player_kings_count(game_state, 1);
    let player_two_kings_count = player_kings_count(game_state, 2);
    let kings_count_value = u_to_i32(player_one_kings_count) - u_to_i32(player_two_kings_count);

    let player_one_center_squares_count = center_squares_count(game_state, 1);
    let player_two_center_squares_count = center_squares_count(game_state, 2);
    let center_squares_count_value = u_to_i32(player_one_center_squares_count) - u_to_i32(player_two_center_squares_count);

    2*pieces_count_value + 4*kings_count_value + 1*center_squares_count_value + 256*lose_value(game_state)
}

fn lose_value(game_state: &checkers::state::game_state::GameState) -> i32 {
    if game_state.possible_moves().len() == 0 {
        match game_state.current_player_number {
            1 => -1,
            2 => 1,
            _ => 0
        }
    } else {
        0
    }
}

fn center_squares_count(game_state: &checkers::state::game_state::GameState, player_number: i8) -> usize {
    let mut counter = 0;
    for row in game_state.squares.iter() {
        for square in row {
            if square.player_number == player_number && CENTER_SQUARE_IDS.iter().any(|id| square.id == *id ) {
                counter += 1;
            }
        }
    }
    counter
}

fn player_pieces_count(game_state: &checkers::state::game_state::GameState, player_number: i8) -> usize {
    let mut counter = 0;
    for row in game_state.squares.iter() {
        for square in row {
            if square.player_number == player_number {
                counter += 1;
            }
        }
    }
    counter
}

fn player_kings_count(game_state: &checkers::state::game_state::GameState, player_number: i8) -> usize {
    let mut counter = 0;
    for row in game_state.squares.iter() {
        for square in row {
            if square.king && square.player_number == player_number {
                counter += 1;
            }
        }
    }
    counter
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
        let encoded = String::from("W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15");
        let mut game_state = checkers::state::game_state::parse(&encoded).unwrap();

        match evaluate(&mut game_state, 4, std::i32::MIN, std::i32::MAX, false) {
            Ok(result) => assert_eq!(result, 0),
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn recommended_move_test() {
       let encoded = String::from("W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15");
       let game_state = checkers::state::game_state::parse(&encoded).unwrap();
       let mov = recommended_move(game_state, 5);

       match mov {
           Some(m) => {
               assert_eq!(m.from, 23);
               assert_eq!(m.to, vec![19]);
           },
           None => assert!(false, "expected move"),
       }
    }
}
