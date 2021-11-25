use regex::Regex;

pub struct Bar {
    pub pieces: Vec<i8>
}

impl Clone for Bar {
    fn clone(&self) -> Bar {
       Bar {
            pieces: self.pieces.clone()
       }
    }
}

impl Bar {
    pub fn pop_piece(&mut self, player_number: i8) -> Result<i8, &'static str> {
        let index = self.pieces.iter().position(|p| *p == player_number); 
        match index {
            Some(i) => Ok(self.pieces.remove(i)),
            None => Err("no piece to pop")
        }
    }

    pub fn push_piece(&mut self, piece: i8) -> Result<(), &'static str> {
        self.pieces.push(piece);
        Ok(())
    }
}

pub fn parse_bar(encoded: &str) -> Result<Bar, &'static str> {
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

            let bar = Bar { pieces };
            Ok(bar)
        },
        None => Err("invalid bar")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_bar_full_test() {
        let encoded = "23";
        let point = parse_bar(encoded).unwrap();
        assert_eq!(point.pieces.len(), 5);
        assert_eq!(point.pieces[0], 1);
        assert_eq!(point.pieces[1], 1);
        assert_eq!(point.pieces[2], 2);
        assert_eq!(point.pieces[3], 2);
        assert_eq!(point.pieces[4], 2);
    }

    #[test]
    fn parsing_bar_empty_test() {
        let encoded = "00";
        let point = parse_bar(encoded).unwrap();
        assert_eq!(point.pieces.len(), 0);
    }

    #[test]
    fn parsing_above_10_test() {
        let encoded = "b0";
        let point = parse_bar(encoded).unwrap();
        assert_eq!(point.pieces.len(), 11);
        assert_eq!(point.pieces[0], 1);
    }

    #[test]
    fn parsing_above_15_test() {
        let encoded = "g0";
        let point = parse_bar(encoded);

        match point {
            Ok(_) => assert!(false, "should not return point"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn pop_piece_valid_test() {
        let player_one_piece = 1;
        let player_two_piece = 2;
        let mut bar = Bar { pieces: vec![player_one_piece, player_two_piece] };  
        let result = bar.pop_piece(1);
        match result {
            Ok(p) => assert_eq!(1, p),
            Err(_) => assert!(false, "expected number")    
        }
    }

    #[test]
    fn pop_piece_invalid_test() {
        let player_two_piece = 2;
        let mut bar = Bar { pieces: vec![player_two_piece] };  
        let result = bar.pop_piece(1);
        match result {
            Ok(_) => assert!(false, "expected no number"),
            Err(_) => assert!(true)    
        }
    }

    #[test]
    fn push_piece_test() {
        let piece = 1;
        let mut bar = Bar { pieces: vec![] };    
        match bar.push_piece(piece) {
            Ok(_) => assert_eq!(1, bar.pieces.len()),
            Err(_) => assert!(false, "expected no error")
        }
    }
}
