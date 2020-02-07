use crate::checkers::square_set::SquareSet;
use crate::checkers::square_set::parse_square_set;
use crate::checkers::mov::Move;
use crate::checkers::mov::MoveKind;

pub struct GameState {
    pub current_player_number: i8,
    pub squares: SquareSet,
}

impl GameState {
    pub fn possible_moves(&self) -> Vec<Move> {
        let occupied_by_player = self.squares.occupied_by_player(self.current_player_number);
        let jumps = occupied_by_player.jumps(&self.squares);
        if jumps.len() == 0 {
            occupied_by_player.moves(&self.squares)
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

        let other_player_number = match self.current_player_number {
            1 => 2,
            2 => 1,
            _ => return Err("invalid player number"),
        };
        
        match mov.to.last() {
            Some(last_id) => {
                match squares.squares.clone().into_iter().find(|s| s.id == *last_id) {
                    Some(s) => {
                        let promotion_row = match self.current_player_number {
                            1 => 7,
                            2 => 0,
                            _ => return Err("invalid player number"),
                        };
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

    #[test]
    fn parsing() {
        let encoded = String::from("bbbbbbbbbbbb--------wwwwwwwwwwwwb");
        let result = parse(&encoded).unwrap();
        assert_eq!(result.current_player_number, 1);
        assert_eq!(result.squares.len(), 32);
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
    fn possible_moves() {
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
                    Some(s) => {
                        match s.piece {
                            Some(_) => assert!(false, "piece should not exist"), 
                            None => assert!(true),
                        }
                    },
                    None => assert!(false, "square not found"),
                }

                match n.squares.squares.clone().into_iter().find(|s| s.id == 13) {
                    Some(s) => {
                        match s.piece {
                            Some(_) => assert!(true), 
                            None => assert!(false, "piece should exists"),
                        }
                    },
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
                    Some(s) => {
                        match s.piece {
                            Some(p) => assert_eq!(true, p.king), 
                            None => assert!(false, "piece should exist"),
                        }
                    },
                    None => assert!(false, "square not found"),
                }
            },
            Err(e) => assert!(false, e),
        } 
    }
}
