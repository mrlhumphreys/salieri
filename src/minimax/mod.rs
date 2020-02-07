use std::cmp;
use std::convert::TryFrom;
use crate::checkers;

const DEPTH: i8 = 6;
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

    let player_one_offense = player_offense(game_state, 1);
    let player_two_offense = player_offense(game_state, 2);
    let player_offense_value = i32::from(player_one_offense) - i32::from(player_two_offense);

    let player_one_defense = player_defense(game_state, 1);
    let player_two_defense = player_defense(game_state, 2);
    let player_defense_value = i32::from(player_one_defense) - i32::from(player_two_defense);
   
    3*pieces_count_value + 10*kings_count_value + 5*player_offense_value + 7*player_defense_value
}

fn defense_score(game_state: &checkers::game_state::GameState, player_number: i8, x: i8, y: i8) -> i8 {
    let square = game_state.squares.squares.iter().find(|s| {
        s.x == x && s.y == y && match s.piece {
            Some(p) => p.player_number == player_number,
            None => false,
        }
    });
    match square {
        Some(_) => 1,
        None => 0,
    }
}

fn player_defense(game_state: &checkers::game_state::GameState, player_number: i8) -> i8 {
    game_state.squares.squares.iter().filter(|s| {
        match s.piece {
            Some(p) => !p.king && p.player_number == player_number,
            None => false,
        }
    }).map(|s| {
        match player_number {
            1 => {
                let left_x = s.x - 1;
                let right_x = s.x + 1;
                let y = s.y - 1;
                let left_score = defense_score(game_state, player_number, left_x, y);
                let right_score = defense_score(game_state, player_number, right_x, y);
                left_score + right_score 
            },
            2 => {
                let left_x = s.x - 1;
                let right_x = s.x + 1;
                let y = s.y + 1;
                let left_score = defense_score(game_state, player_number, left_x, y);
                let right_score = defense_score(game_state, player_number, right_x, y);
                left_score + right_score 
            },
            _ => 0,
        }
    }).sum()
}

fn player_offense(game_state: &checkers::game_state::GameState, player_number: i8) -> i8 {
    game_state.squares.squares.iter().filter(|s| {
        match s.piece {
            Some(p) => !p.king && p.player_number == player_number,
            None => false,
        }
    }).map(|s| {
        match player_number {
            1 => s.y - PLAYER_ONE_HOME_ROW, 
            2 => PLAYER_TWO_HOME_ROW - s.y,
            _ => 0
        }
    }).sum()
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
            Ok(result) => assert_eq!(result, -12),
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
                assert_eq!(m.from, 22);
                assert_eq!(m.to, vec![18]);
            },
            None => assert!(false, "expected move"), 
        }
    }
}
