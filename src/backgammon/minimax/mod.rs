use std::cmp;
use std::convert::TryFrom;
use crate::backgammon;

// 21 combinations
const ALL_ROLLS: [(i8,i8); 21] = [
    (1,1), (1,2), (1,3), (1,4), (1,5), (1,6),
    (2,2), (2,3), (2,4), (2,5), (2,6),
    (3,3), (3,4), (3,5), (3,6),
    (4,4), (4,5), (4,6),
    (5,5), (5,6), 
    (6,6)
];

pub fn recommended_move(game_state: backgammon::state::game_state::GameState, depth: i8) -> Option<backgammon::state::mov::Move> {
    let mut new_game_state = game_state.clone();
    let moves = new_game_state.possible_moves();
    match moves.len() {
        0 => None,
        1 => match moves.first() {
            Some(s) => Some(s.clone()),
            None => None
        },
        _ => {
            // Do something
            let moves_with_value = moves.iter().map(|mov| {
                let mut newer_game_state = game_state.clone();
                match newer_game_state.perform_move(mov) {
                    Ok(_) => (),
                    Err(_) => return (mov, 0),
                };

                let maximizing_player = match newer_game_state.current_player_number {
                    1 => true,
                    2 => false,
                    _ => true,
                };

                let value = match evaluate_roll_phase(&mut newer_game_state, depth, std::i32::MIN, std::i32::MAX, maximizing_player) {
                    Ok(v) => v,
                    Err(_) => 0,
                };

                (mov, value)
            });

            let maximizing_player = match new_game_state.current_player_number {
                1 => true,
                2 => false,
                _ => true,
            };

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

// evaluate game state without roll. i.e. roll_phase
pub fn evaluate_roll_phase(game_state: &mut backgammon::state::game_state::GameState, depth: i8, alpha: i32, beta: i32, maximizing_player: bool) -> Result<i32, &'static str> {
    let results: Result<Vec<i32>, _> = ALL_ROLLS.iter().map(|roll| {
        let mut new_game_state = game_state.clone();
        new_game_state.perform_set_roll(roll.0, roll.1);
        evaluate_move_phase(&mut new_game_state, depth - 1, alpha, beta, maximizing_player)
    }).collect();
    
    match results {
        Ok(numbers) => {
            let total: i32 = numbers.iter().sum();
            let length = u_to_i32(numbers.len());
            let eval = total / length;
            Ok(eval)
        },
        Err(e) => Err(e)
    }
}

// evaluate game state with roll, i.e. move_phase
pub fn evaluate_move_phase(game_state: &mut backgammon::state::game_state::GameState, depth: i8, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> Result<i32, &'static str> {
    let moves = game_state.possible_moves(); 
    if depth == 0 || moves.len() == 0 {
        return Ok(static_evaluation(&game_state));
    }

    if maximizing_player {
        let mut max_eval = std::i32::MIN;
        for mov in moves {
            let mut new_game_state = game_state.clone();
            match new_game_state.perform_move(&mov) {
                Ok(_) => {
                    match evaluate_roll_phase(&mut new_game_state, depth, alpha, beta, false) {
                        Ok(eval) => {
                            max_eval = cmp::max(max_eval, eval);
                            alpha = cmp::max(alpha, eval);
                        },
                        Err(e) => return Err(e)
                    };
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
            let mut new_game_state = game_state.clone();
            match new_game_state.perform_move(&mov) {
                Ok(_) => {
                    match evaluate_roll_phase(&mut new_game_state, depth, alpha, beta, true) {
                        Ok(eval) => {
                            min_eval = cmp::min(min_eval, eval);
                            beta = cmp::min(beta, eval);
                        },
                        Err(e) => return Err(e)
                    }
                },
                Err(e) => return Err(e)
            };
            if beta <= alpha {
                break;
            }
        }
        Ok(min_eval)
    }
}

// positive -> player 1
// negative -> player 2
fn static_evaluation(game_state: &backgammon::state::game_state::GameState) -> i32 {
    let player_one_prime_count = player_prime_count(game_state, 1);
    let player_two_prime_count = player_prime_count(game_state, 2);
    let prime_count_value = u_to_i32(player_one_prime_count) - u_to_i32(player_two_prime_count);

    let player_one_blot_count = player_blot_count(game_state, 1);
    let player_two_blot_count = player_blot_count(game_state, 2);
    let blot_count_value = u_to_i32(player_two_blot_count) - u_to_i32(player_one_blot_count);

    let player_one_bar_count = player_bar_count(game_state, 1);
    let player_two_bar_count = player_bar_count(game_state, 2);
    let bar_count_value = i32::from(player_two_bar_count - player_one_bar_count);
    
    let player_one_home_count = player_home_count(game_state, 1);
    let player_two_home_count = player_home_count(game_state, 2);
    let home_count_value = i32::from(player_one_home_count - player_two_home_count);

    let player_one_off_board_count = player_off_board_count(game_state, 1); 
    let player_two_off_board_count = player_off_board_count(game_state, 2); 
    let off_board_count_value = i32::from(player_one_off_board_count - player_two_off_board_count);

    8*prime_count_value + 16*blot_count_value + 32*bar_count_value + 64*home_count_value + 128*off_board_count_value + 256*win_value(game_state) 
}

fn player_prime_count(game_state: &backgammon::state::game_state::GameState, player_number: i8) -> usize {
    game_state.points.iter().filter(|point| {
        match player_number {
            1 => point.player_one_piece_count >= 1,
            2 => point.player_two_piece_count >= 1,
            _ => false 
        }
    }).count()
}

fn player_blot_count(game_state: &backgammon::state::game_state::GameState, player_number: i8) -> usize {
    game_state.points.iter().filter(|point| {
        match player_number {
            1 => point.player_one_piece_count == 1,
            2 => point.player_two_piece_count == 1,
            _ => false
        }
    }).count()
}

fn player_home_count(game_state: &backgammon::state::game_state::GameState, player_number: i8) -> i8 {
    game_state.points.iter().filter(|point| {
        match player_number {
            1 => point.number >= 19,
            2 => point.number <= 6,
            _ => false
        }
    }).map(|point| {
        point.player_one_piece_count + point.player_two_piece_count
    }).sum()
}

fn player_bar_count(game_state: &backgammon::state::game_state::GameState, player_number: i8) -> i8 {
    match player_number {
        1 => game_state.bar.player_one_piece_count,
        2 => game_state.bar.player_two_piece_count,
        _ => 0
    }
}

fn player_off_board_count(game_state: &backgammon::state::game_state::GameState, player_number: i8) -> i8 {
    match player_number {
        1 => game_state.off_board.player_one_piece_count,
        2 => game_state.off_board.player_two_piece_count,
        _ => 0
    }
}

fn win_value(game_state: &backgammon::state::game_state::GameState) -> i32 {
   match game_state.winner() {
        Some(w) => match w {
           1 => 1,
           2 => -1, 
           _ => 0    
        },
        None => 0 
   }
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
    use crate::backgammon::state::mov::PointKind;

    #[test]
    fn evaluate_move_phase_test() {
        let encoded = String::from("0020000000000500300000005005000000030050000000000200121");
        let mut game_state = backgammon::state::game_state::parse(&encoded).unwrap();

        match evaluate_move_phase(&mut game_state, 0, std::i32::MIN, std::i32::MAX, true) {
            Ok(result) => assert_eq!(result, 0),            
            Err(e) => assert!(false, e)
        }
    }

    #[test]
    fn recommended_move_test() {
        let encoded = String::from("0020000000000500300000005005000000030050000000000200121");
        let game_state = backgammon::state::game_state::parse(&encoded).unwrap();
        let mov = recommended_move(game_state, 1);

        match mov {
            Some(m) => {
                let move_step_a = &m.list[0];
                assert_eq!(move_step_a.from.kind, PointKind::Point);
                assert_eq!(move_step_a.from.number, Some(19));
                assert_eq!(move_step_a.to.kind, PointKind::Point);
                assert_eq!(move_step_a.to.number, Some(21));
                assert_eq!(move_step_a.die_number, 2);

                let move_step_b = &m.list[1];
                assert_eq!(move_step_b.from.kind, PointKind::Point);
                assert_eq!(move_step_b.from.number, Some(21));
                assert_eq!(move_step_b.to.kind, PointKind::Point);
                assert_eq!(move_step_b.to.number, Some(22));
                assert_eq!(move_step_b.die_number, 1);
            },
            None => assert!(false, "expected move"),
        }
    }
}
