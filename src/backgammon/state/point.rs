use std::convert::TryFrom;
use regex::Regex;
use crate::backgammon::state::piece::Piece;

pub struct Point {
    pub number: i8,
    pub pieces: Vec<Piece>
}

impl Point {
    pub fn home(&self, player_number: i8) -> bool {
        match player_number {
            1 => self.number > 18,
            2 => self.number <= 6,
            _ => false 
        }
    }

    pub fn prime(&self) -> bool {
        self.pieces.len() > 1
    }

    pub fn occupied_by_player(&self, player_number: i8) -> bool {
        self.pieces.iter().any(|p| p.player_number == player_number) 
    }

    pub fn occupied_by_opponent(&self, player_number: i8) -> bool {
        self.pieces.iter().any(|p| p.player_number != player_number) 
    }

    pub fn pop_piece(&mut self) -> Result<Piece, &'static str> {
        match self.pieces.pop() {
            Some(p) => Ok(p),
            None => Err("no piece to pop")
        }
    }

    pub fn push_piece(&mut self, piece: Piece) -> Result<Option<Piece>, &'static str> {
        let opponent_piece_count = self.pieces.iter().filter(|p| p.player_number != piece.player_number).count();

        match opponent_piece_count {
            0 => {
                self.pieces.push(piece);
                Ok(None)
            },
            1 => { 
                match self.pieces.pop() {
                    Some(popped) => {
                        self.pieces.push(piece);
                        Ok(Some(popped))
                    },
                    None => Err("no piece to pop")
                }
            },
            _ => Err("point occupied by opponent")
        }
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point {
            number: self.number,
            pieces: self.pieces.clone()
        }
    }
}

pub fn parse_point(index: usize, encoded: &str) -> Result<Point, &'static str> {
    let number = match i8::try_from(index) {
        Ok(n) => n,
        Err(_) => return Err("invalid point number"),
    };

    let re = Regex::new(r"^([0-9a-f])([0-9a-f])$").unwrap(); 
    let caps = re.captures(encoded);

    let mut pieces = Vec::new();

    match caps {
        Some(c) => {
            let number_of_player_one_pieces = c.get(1).unwrap().as_str().chars().nth(0).unwrap().to_digit(16).unwrap();
            let number_of_player_two_pieces = c.get(2).unwrap().as_str().chars().nth(0).unwrap().to_digit(16).unwrap();

            if number_of_player_one_pieces > 0 && number_of_player_two_pieces > 0 {
                Err("point must contain pieces from only one player")
            } else { 
                if number_of_player_one_pieces > 0 {
                    for _ in 0..number_of_player_one_pieces {
                        let piece = Piece { player_number: 1 };
                        pieces.push(piece);
                    }
                }

                if number_of_player_two_pieces > 0 {
                    for _ in 0..number_of_player_two_pieces {
                        let piece = Piece { player_number: 2 };
                        pieces.push(piece);
                    }
                }
                
                let point = Point { number, pieces };
                Ok(point)
            }
        },
        None => Err("invalid point")
    }
}

#[cfg(test)]
mod tests {   
    use super::*;

    #[test]
    fn home_player_1_test() {
        let player_number = 1;
        let point = Point { number: 19, pieces: vec![] };
        let result = point.home(player_number);
        assert!(result)
    }

    #[test]
    fn not_home_player_1_test() {
        let player_number = 1;
        let point = Point { number: 18, pieces: vec![] };
        let result = point.home(player_number);
        assert!(!result)
    }

    #[test]
    fn home_player_2_test() {
        let player_number = 2;
        let point = Point { number: 6, pieces: vec![] };
        let result = point.home(player_number);
        assert!(result)
    }

    #[test]
    fn not_home_player_2_test() {
        let player_number = 2;
        let point = Point { number: 7, pieces: vec![] };
        let result = point.home(player_number);
        assert!(!result)
    }

    #[test]
    fn home_player_x_test() {
        let player_number = 3;
        let point = Point { number: 7, pieces: vec![] };
        let result = point.home(player_number);
        assert!(!result)
    }

    #[test]
    fn prime_test() {
        let piece_a = Piece { player_number: 1 };
        let piece_b = Piece { player_number: 1 };
        let point = Point { number: 1, pieces: vec![piece_a, piece_b] };    
        let result = point.prime();
        assert!(result)
    }

    #[test]
    fn not_prime_test() {
        let piece_a = Piece { player_number: 1 };
        let point = Point { number: 1, pieces: vec![piece_a] };
        let result = point.prime();
        assert!(!result)
    }

    #[test]
    fn occupied_by_player_with_player_test() {
        let piece = Piece { player_number: 1 };
        let point = Point { number: 7, pieces: vec![piece] };
        let result = point.occupied_by_player(1);
        assert!(result)
    }

    #[test]
    fn occupied_by_player_with_opponent_test() {
        let piece = Piece { player_number: 1 };
        let point = Point { number: 7, pieces: vec![piece] };
        let result = point.occupied_by_player(2);
        assert!(!result)
    }

    #[test]
    fn occupied_by_player_empty_test() {
        let point = Point { number: 7, pieces: vec![] };
        let result = point.occupied_by_player(1);
        assert!(!result)
    }

    #[test]
    fn occupied_by_opponent_with_player_test() {
        let piece = Piece { player_number: 1 };
        let point = Point { number: 7, pieces: vec![piece] };
        let result = point.occupied_by_opponent(1);
        assert!(!result)
    }

    #[test]
    fn occupied_by_opponent_with_opponent_test() {
        let piece = Piece { player_number: 1 };
        let point = Point { number: 7, pieces: vec![piece] };
        let result = point.occupied_by_opponent(2);
        assert!(result)
    }

    #[test]
    fn occupied_by_opponent_empty_test() {
        let point = Point { number: 7, pieces: vec![] };
        let result = point.occupied_by_opponent(2);
        assert!(!result)
    }

    #[test]
    fn parsing_point_player_one_test() {
        let encoded = "50";
        let point = parse_point(1, encoded).unwrap();
        assert_eq!(point.number, 1);
        assert_eq!(point.pieces.len(), 5);
        assert_eq!(point.pieces[0].player_number, 1);
    }

    #[test]
    fn parsing_point_player_two_test() {
        let encoded = "03";
        let point = parse_point(2, encoded).unwrap();
        assert_eq!(point.number, 2);
        assert_eq!(point.pieces.len(), 3);
        assert_eq!(point.pieces[0].player_number, 2);
    }

    #[test]
    fn parsing_point_empty_test() {
        let encoded = "00";
        let point = parse_point(3, encoded).unwrap();
        assert_eq!(point.number, 3);
        assert_eq!(point.pieces.len(), 0);
    }

    #[test]
    fn parsing_above_10_test() {
        let encoded = "b0";
        let point = parse_point(4, encoded).unwrap();
        assert_eq!(point.number, 4);
        assert_eq!(point.pieces.len(), 11);
        assert_eq!(point.pieces[0].player_number, 1);
    }

    #[test]
    fn parsing_above_15_test() {
        let encoded = "g0";
        let point = parse_point(6, encoded);

        match point {
            Ok(_) => assert!(false, "should not return point"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn parsing_with_both_players_on_point_test() {
        let encoded = "12";
        let point = parse_point(7, encoded);

        match point {
            Ok(_) => assert!(false, "should not return point"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn pop_piece_valid_test() {
        let piece = Piece { player_number: 1 };
        let mut point = Point { number: 1, pieces: vec![piece] };  
        let result = point.pop_piece();
        match result {
            Ok(p) => assert_eq!(1, p.player_number),
            Err(_) => assert!(false, "expected piece")    
        }
    }

    #[test]
    fn pop_piece_invalid_test() {
        let mut point = Point { number: 1, pieces: vec![] };  
        let result = point.pop_piece();
        match result {
            Ok(_) => assert!(false, "expected no piece"),
            Err(_) => assert!(true)    
        }
    }

    #[test]
    fn push_empty_test() {
        let piece = Piece { player_number: 1 };
        let mut point = Point { number: 1, pieces: vec![] }; 
        let result = point.push_piece(piece);
        match result {
            Ok(piece) => {
                match piece {
                    Some(_) => assert!(false, "expected no piece"),
                    None => assert!(true)
                }
            },
            Err(_) => assert!(false, "expected no error")
        }
    }

    #[test]
    fn push_blot_test() {
       let piece = Piece { player_number: 1 }; 
       let opposing_piece = Piece { player_number: 2 };
       let mut point = Point { number: 1, pieces: vec![opposing_piece] };
       let result = point.push_piece(piece);
       match result {
            Ok(piece) => {
                match piece {
                    Some(p) => assert_eq!(2, p.player_number),
                    None => assert!(false, "expected piece")
                }
            },
            Err(_) => assert!(false, "expected no error")
       }
    }

    #[test]
    fn push_prime_test() {
        let piece = Piece { player_number: 1 };
        let opposing_piece_a = Piece { player_number: 2 };
        let opposing_piece_b = Piece { player_number: 2 };
        let mut point = Point { number: 1, pieces: vec![opposing_piece_a, opposing_piece_b] };
        let result = point.push_piece(piece);
        match result {
            Ok(_) => assert!(false, "expected error"), 
            Err(_) => assert!(true) 
        }
    }
} 
