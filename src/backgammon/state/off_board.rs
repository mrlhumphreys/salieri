use regex::Regex;

pub struct OffBoard {
    pub pieces: Vec<i8>
}

impl Clone for OffBoard {
    fn clone(&self) -> OffBoard {
        OffBoard {
            pieces: self.pieces.clone()
        }
    }
}

impl OffBoard {
    pub fn push_piece(&mut self, piece: i8) -> Result<(), &'static str> {
        self.pieces.push(piece);
        Ok(())
    }
}

pub fn parse_off_board(encoded: &str) -> Result<OffBoard, &'static str> {
    let re = Regex::new(r"^([0-9a-f])([0-9a-f])$").unwrap(); 
    let caps = re.captures(encoded);

    let mut pieces = Vec::new();

    match caps {
        Some(c) => {
            let number_of_player_one_pieces = c.get(1).unwrap().as_str().chars().nth(0).unwrap().to_digit(16).unwrap();
            let number_of_player_two_pieces = c.get(2).unwrap().as_str().chars().nth(0).unwrap().to_digit(16).unwrap();

            for _ in 0..number_of_player_one_pieces {
                let piece = 1;
                pieces.push(piece);
            }

            for _ in 0..number_of_player_two_pieces {
                let piece = 2;
                pieces.push(piece);
            }

            let off_board = OffBoard { pieces };
            Ok(off_board)
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
        assert_eq!(point.pieces.len(), 5);
        assert_eq!(point.pieces[0], 1);
        assert_eq!(point.pieces[1], 1);
        assert_eq!(point.pieces[2], 2);
        assert_eq!(point.pieces[3], 2);
        assert_eq!(point.pieces[4], 2);
    }

    #[test]
    fn parsing_off_board_empty_test() {
        let encoded = "00";
        let point = parse_off_board(encoded).unwrap();
        assert_eq!(point.pieces.len(), 0);
    }

    #[test]
    fn parsing_above_10_test() {
        let encoded = "b0";
        let point = parse_off_board(encoded).unwrap();
        assert_eq!(point.pieces.len(), 11);
        assert_eq!(point.pieces[0], 1);
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
        let mut off_board = OffBoard { pieces: vec![] };
        match off_board.push_piece(piece) {
            Ok(_) => assert_eq!(1, off_board.pieces.len()),
            Err(_) => assert!(false, "expected no error")
        }
    }
}

