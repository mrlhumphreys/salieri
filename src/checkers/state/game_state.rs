use crate::checkers::state::square_set::SquareSet;
use crate::checkers::state::square_set::parse_square_set;
use crate::checkers::state::mov::Move;

#[derive(PartialEq, Debug)]
pub struct GameState {
    pub current_player_number: i8,
    pub squares: SquareSet,
}

impl Clone for GameState {
    fn clone(&self) -> GameState {
        GameState {
            current_player_number: self.current_player_number,
            squares: self.squares.clone(),
        }
    }
}

impl GameState {
    pub fn winner(&self) -> Option<i8> {
        if self.possible_moves_for_player(1).len() == 0 {
            Some(2) 
        } else if self.possible_moves_for_player(2).len() == 0 {
            Some(1)
        } else {
            None
        }
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        self.possible_moves_for_player(self.current_player_number)
    }

    pub fn possible_moves_for_player(&self, player_number: i8) -> Vec<Move> {
        let jumps = self.squares.jumps_for_player(player_number, &self.squares);
        if jumps.len() == 0 {
            self.squares.moves_for_player(player_number, &self.squares)
        } else {
            jumps
        }
    }

    pub fn perform_move(&self, mov: &Move) -> Result<GameState, &'static str> {
        let legs = mov.legs();

        let mut squares = self.squares.clone();

        for (origin, destination) in legs {
           match squares.perform_move(origin, destination) {
                Ok(s) => squares = s,
                Err(e) => {
                    return Err(e);
                },
           }
        }

        let (other_player_number, promotion_row) = match self.current_player_number {
            1 => (2, 7),
            2 => (1, 0),
            _ => return Err("invalid player number"),
        };
        
        match mov.to.last() {
            Some(last_id) => {
                match squares.squares.clone().into_iter().find(|s| s.id == *last_id) {
                    Some(s) => {
                        if promotion_row == s.y {
                            match squares.promote(*last_id) {
                                Ok(ss) => squares = ss,
                                Err(e) => return Err(e),
                            }
                        }
                    },
                    None => return Err("invalid square id"),
                }
            },
            None => return Err("no square id"),
        };

        let game_state = GameState {
            current_player_number: other_player_number,
            squares: squares,
        };

        Ok(game_state)
    }
}

pub fn parse(encoded: &String) -> Result<GameState, &'static str> {
    if encoded.len() != 33 {
        return Err("Invalid State");
    }

    let squares_component = &encoded[0..32]; 
    let player_component = &encoded[32..33];
    
    let current_player_number = match player_component {
      "b" => 1,
      "w" => 2,
      _ => return Err("Invalid State"),
    };

    let squares = match parse_square_set(squares_component) {
        Ok(s) => s,
        Err(e) => return Err(e),
    }; 

    let game_state = GameState { current_player_number, squares };

    Ok(game_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::state::mov::MoveKind;

    #[test]
    fn parsing() {
        let encoded = String::from("bbbbbbbbbbbb--------wwwwwwwwwwwwb");
        let result = parse(&encoded).unwrap();
        assert_eq!(result.current_player_number, 1);
        assert_eq!(result.squares.squares.len(), 32);
    }

    #[test]
    fn parsing_example_b() {
        let encoded = String::from("bbbbbbbbb-bb--b-----wwwwwwwwwwwww");
        let result = parse(&encoded);
        match result {
            Err(e) => assert!(false, e),
            Ok(_) => assert!(true, "success"),
        }
    }

    #[test]
    fn parsing_invalid() {
        let encoded = String::from("bbbbbbbbbbbb--------wwwwwwwwwwww");
        let result = parse(&encoded);
        match result {
            Ok(_) => assert!(false, "Expected Error"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn winner_none_test() {
        let encoded = String::from("bbbbbbbbbbbb--------wwwwwwwwwwwwb");
        let game_state = parse(&encoded).unwrap();
        assert_eq!(None, game_state.winner());
    }

    #[test]
    fn winner_some_test() {
        let encoded = String::from("bbbbbbbbbbbb--------------------b");
        let game_state = parse(&encoded).unwrap();
        assert_eq!(Some(1), game_state.winner());
    }

    #[test]
    fn possible_moves_test() {
        let encoded = String::from("bbbbbbbbbbbb--------wwwwwwwwwwwwb");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves();
        assert_eq!(result.len(), 7);

        assert_eq!((&result[0]).from, 9);
        assert_eq!((&result[0]).to, vec![13]);

        assert_eq!((&result[1]).from, 9);
        assert_eq!((&result[1]).to, vec![14]);

        assert_eq!((&result[2]).from, 10);
        assert_eq!((&result[2]).to, vec![14]);

        assert_eq!((&result[3]).from, 10);
        assert_eq!((&result[3]).to, vec![15]);

        assert_eq!((&result[4]).from, 11);
        assert_eq!((&result[4]).to, vec![15]);

        assert_eq!((&result[5]).from, 11);
        assert_eq!((&result[5]).to, vec![16]);

        assert_eq!((&result[6]).from, 12);
        assert_eq!((&result[6]).to, vec![16]);
    }

    #[test]
    fn possible_moves_for_player_test() {
        let encoded = String::from("bbbbbbbbbbbb--------wwwwwwwwwwwwb");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(2);
        assert_eq!(result.len(), 7);

        assert_eq!((&result[0]).from, 21);
        assert_eq!((&result[0]).to, vec![17]);

        assert_eq!((&result[1]).from, 22);
        assert_eq!((&result[1]).to, vec![17]);

        assert_eq!((&result[2]).from, 22);
        assert_eq!((&result[2]).to, vec![18]);

        assert_eq!((&result[3]).from, 23);
        assert_eq!((&result[3]).to, vec![18]);

        assert_eq!((&result[4]).from, 23);
        assert_eq!((&result[4]).to, vec![19]);

        assert_eq!((&result[5]).from, 24);
        assert_eq!((&result[5]).to, vec![19]);

        assert_eq!((&result[6]).from, 24);
        assert_eq!((&result[6]).to, vec![20]);
    }

    #[test]
    fn possible_moves_b_test() {
        let encoded = String::from("bbbbb-b--bb-b--ww-bwww-wwww-w--ww");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(2);
        assert_eq!(result.len(), 2);

        assert_eq!((&result[0]).from, 24);
        assert_eq!((&result[0]).to, vec![15, 6]);

        assert_eq!((&result[1]).from, 24);
        assert_eq!((&result[1]).to, vec![15, 8]);
    }

    #[test]
    fn perform_move_test() {
        let encoded = String::from("wwwwwwwwwwww--------bbbbbbbbbbbbw");
        let game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: 9,
            to: vec![13],
        };

        match game_state.perform_move(&mov) {
            Ok(n) => {
                match n.squares.squares.clone().into_iter().find(|s| s.id == 9) {
                    Some(s) => assert_eq!(s.occupied(), false),
                    None => assert!(false, "square not found"),
                }

                match n.squares.squares.clone().into_iter().find(|s| s.id == 13) {
                    Some(s) => assert_eq!(s.occupied(), true),
                    None => assert!(false, "square not found"),
                }
            },
            Err(e) => assert!(false, e),
        }
    }

    #[test]
    fn perform_move_with_promote() {
        let encoded = String::from("bbbbbbbbbbb---------------wb----b");
        let game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: 28,
            to: vec![32],
        };
        match game_state.perform_move(&mov) {
            Ok(n) => {
                match n.squares.squares.clone().into_iter().find(|s| s.id == 32) {
                    Some(s) => assert_eq!(s.king, true),
                    None => assert!(false, "square not found"),
                }
            },
            Err(e) => assert!(false, e),
        } 
    }
}
