use std::convert::TryFrom;
use regex::Regex;

pub struct OffBoard {
    pub player_one_piece_count: i8,
    pub player_two_piece_count: i8
}

impl Clone for OffBoard {
    fn clone(&self) -> OffBoard {
        OffBoard {
            player_one_piece_count: self.player_one_piece_count.clone(),
            player_two_piece_count: self.player_two_piece_count.clone()
        }
    }
}

impl OffBoard {
    pub fn push_piece(&mut self, piece: i8) -> Result<(), &'static str> {
        match piece {
            1 => {
                self.player_one_piece_count += 1;
                Ok(())
            },
            2 => {
                self.player_two_piece_count += 1;
                Ok(())
            },
            _ => {
                Err("invalid player number")
            }
        }
    }
}

pub fn parse_off_board(encoded: &str) -> Result<OffBoard, &'static str> {
    let re = Regex::new(r"^([0-9a-f])([0-9a-f])$").unwrap(); 
    let caps = re.captures(encoded);

    match caps {
        Some(c) => {
            let first_value = c.get(1).unwrap().as_str().chars().nth(0).unwrap().to_digit(16).unwrap();
            let second_value = c.get(2).unwrap().as_str().chars().nth(0).unwrap().to_digit(16).unwrap();

            match i8::try_from(first_value) {
                Ok(player_one_piece_count) => {
                    match i8::try_from(second_value) {
                        Ok(player_two_piece_count) => {
                            let off_board = OffBoard { player_one_piece_count, player_two_piece_count };
                            Ok(off_board)
                        },
                        Err(_) => Err("invalid second value")
                    }
                },
                Err(_) => Err("invalid first value")
            }
        },
        None => Err("invalid off board") 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_off_board_full_test() {
        let encoded = "23";
        let point = parse_off_board(encoded).unwrap();
        assert_eq!(point.player_one_piece_count, 2);
        assert_eq!(point.player_two_piece_count, 3);
    }

    #[test]
    fn parsing_off_board_empty_test() {
        let encoded = "00";
        let point = parse_off_board(encoded).unwrap();
        assert_eq!(point.player_one_piece_count, 0);
        assert_eq!(point.player_two_piece_count, 0);
    }

    #[test]
    fn parsing_above_10_test() {
        let encoded = "b0";
        let point = parse_off_board(encoded).unwrap();
        assert_eq!(point.player_one_piece_count, 11);
    }

    #[test]
    fn parsing_above_15_test() {
        let encoded = "g0";
        let point = parse_off_board(encoded);

        match point {
            Ok(_) => assert!(false, "should not return point"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn push_piece_test() {
        let piece = 1;
        let mut off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0,
        };
        match off_board.push_piece(piece) {
            Ok(_) => assert_eq!(off_board.player_one_piece_count, 1),
            Err(_) => assert!(false, "expected no error")
        }
    }
}
