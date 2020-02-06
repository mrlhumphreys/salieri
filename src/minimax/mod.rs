use std::cmp;
use std::convert::TryFrom;
use crate::checkers;

const DEPTH: i8 = 4;
const MIDDLE_IDS: [i8; 8] = [10, 11, 14, 15, 18, 19, 22, 23];
const PLAYER_ONE_HOME_ROW: i8 = 0;
const PLAYER_TWO_HOME_ROW: i8 = 7;

pub fn recommended_move(game_state: checkers::game_state::GameState) -> Option<checkers::mov::Move> {
    let moves = game_state.possible_moves();
    let moves_with_value = moves.iter().map(|mov| {
        let new_game_state = match game_state.perform_move(mov) {
            Ok(n) => n,
            Err(_) => return (mov, 0),
        };

        let maximizing_player = match game_state.current_player_number {
            1 => true,
            2 => false,
            _ => true,
        };

        let value = match evaluate(&new_game_state, DEPTH, maximizing_player) {
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

pub fn evaluate(game_state: &checkers::game_state::GameState, depth: i8, maximizing_player: bool) -> Result<i32, &'static str> {
    let moves = game_state.possible_moves();
    if depth == 0 || moves.len() == 0 {
        return Ok(static_evaluation(&game_state));
    }
    
    if maximizing_player {
        let mut max_eval = std::i32::MIN;
        for mov in moves {
            match game_state.perform_move(&mov) {
                Ok(new_game_state) => {
                    match evaluate(&new_game_state, depth - 1, false) {
                        Ok(eval) => {
                            max_eval = cmp::max(max_eval, eval);
                        },
                        Err(e) => return Err(e),
                    }
                },
                Err(e) => return Err(e), 
            }; 
        }
        Ok(max_eval)
    } else {
        let mut min_eval = std::i32::MAX;
        for mov in moves {
            match game_state.perform_move(&mov) {
                Ok(new_game_state) => {
                    match evaluate(&new_game_state, depth - 1, true) {
                        Ok(eval) => {
                            min_eval = cmp::min(min_eval, eval)
                        },
                        Err(e) => return Err(e),
                    }
                },
                Err(e) => {
                    return Err(e);
                },
            };
        }
        Ok(min_eval)
    }
}

// positive -> w
// negative -> b
pub fn static_evaluation(game_state: &checkers::game_state::GameState) -> i32 {
    let player_one_pieces_count = match i32::try_from(game_state.squares.occupied_by_player(1).len()) {
        Ok(v) =>  v,
        Err(_) => 0,
    };
    let player_two_pieces_count = match i32::try_from(game_state.squares.occupied_by_player(2).len()) {
        Ok(v) =>  v,
        Err(_) => 0,
    };
    let pieces_count_value = player_one_pieces_count - player_two_pieces_count;
   
    let player_one_kings_count = match i32::try_from(game_state.squares.occupied_by_player(1).kings().len()) {
        Ok(v) =>  v,
        Err(_) => 0,
    };
    let player_two_kings_count = match i32::try_from(game_state.squares.occupied_by_player(2).kings().len()) {
        Ok(v) =>  v,
        Err(_) => 0,
    };
    let kings_count_value = player_one_kings_count - player_two_kings_count;
    
    let player_one_middle_count = match i32::try_from(game_state.squares.occupied_by_player(1).where_id(&MIDDLE_IDS.to_vec()).len()) {
        Ok(v) => v,
        Err(_) => 0,
    };
    let player_two_middle_count = match i32::try_from(game_state.squares.occupied_by_player(2).where_id(&MIDDLE_IDS.to_vec()).len()) {
        Ok(v) => v,
        Err(_) => 0,
    };
    let middle_count_value = player_one_middle_count - player_two_middle_count;
   
    // number of w pieces on home row - number of b pieces on home row
    let player_one_home_count = match i32::try_from(game_state.squares.occupied_by_player(1).where_y(PLAYER_ONE_HOME_ROW).len()) {
        Ok(v) => v,
        Err(_) => 0,
    };
    let player_two_home_count = match i32::try_from(game_state.squares.occupied_by_player(2).where_y(PLAYER_TWO_HOME_ROW).len()) {
        Ok(v) => v,
        Err(_) => 0,
    };
    let player_home_value = player_one_home_count - player_two_home_count;
   
    2*pieces_count_value + 10*kings_count_value + 7*middle_count_value + 5*player_home_value 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_test() {
        let encoded = String::from("bbbbbbbbbb-b--b-----wwwwwwwwwwwww");
        let game_state = checkers::game_state::parse(&encoded).unwrap();

        match evaluate(&game_state, 3, false) {
            Ok(result) => assert_eq!(result, -14),
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
                assert_eq!(m.from, 23);
                assert_eq!(m.to, vec![18]);
            },
            None => assert!(false, "expected move"), 
        }
    }
}
