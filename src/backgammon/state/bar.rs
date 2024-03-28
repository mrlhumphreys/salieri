use std::convert::TryFrom;
use regex::Regex;

pub struct Bar {
    pub player_one_piece_count: i8,
    pub player_two_piece_count: i8
}

impl Clone for Bar {
    fn clone(&self) -> Bar {
       Bar {
            player_one_piece_count: self.player_one_piece_count.clone(),
            player_two_piece_count: self.player_two_piece_count.clone()
       }
    }
}

impl Bar {
    pub fn pop_piece(&mut self, player_number: i8) -> Result<i8, &'static str> {
        match player_number {
            1 => {
                if self.player_one_piece_count > 0 {
                    self.player_one_piece_count -= 1;
                    Ok(1)
                } else {
                    Err("no piece to pop")
                }
            },
            2 => {
                if self.player_two_piece_count > 0 {
                    self.player_two_piece_count -= 1;
                    Ok(2)
                } else {
                    Err("no piece to pop")
                }
            },
            _ => {
                Err("invalid player number")
            }
        }
    }

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

pub fn parse_bar(encoded: &str) -> Result<Bar, &'static str> {
    let re = Regex::new(r"^([0-9a-f])([0-9a-f])$").unwrap();
    let caps = re.captures(encoded);

    if let Some(c) = caps {
        let first_value = c.get(1).unwrap().as_str().chars().nth(0).unwrap().to_digit(16).unwrap();
        let second_value = c.get(2).unwrap().as_str().chars().nth(0).unwrap().to_digit(16).unwrap();

        match i8::try_from(first_value) {
            Ok(player_one_piece_count) => {
                match i8::try_from(second_value) {
                    Ok(player_two_piece_count) => {
                        let bar = Bar { player_one_piece_count, player_two_piece_count };
                        Ok(bar)
                    },
                    Err(_) => Err("invalid second value")
                }
            },
            Err(_) => Err("invalid first value")
        }
    } else {
        Err("invalid bar")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_bar_full_test() {
        let encoded = "23";
        let point = parse_bar(encoded).unwrap();
        assert_eq!(point.player_one_piece_count, 2);
        assert_eq!(point.player_two_piece_count, 3);
    }

    #[test]
    fn parsing_bar_empty_test() {
        let encoded = "00";
        let point = parse_bar(encoded).unwrap();
        assert_eq!(point.player_one_piece_count, 0);
        assert_eq!(point.player_two_piece_count, 0);
    }

    #[test]
    fn parsing_above_10_test() {
        let encoded = "b0";
        let point = parse_bar(encoded).unwrap();
        assert_eq!(point.player_one_piece_count, 11);
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
        let mut bar = Bar {
            player_one_piece_count: 1,
            player_two_piece_count: 2
        };
        let result = bar.pop_piece(1);
        match result {
            Ok(p) => assert_eq!(1, p),
            Err(_) => assert!(false, "expected number")
        }
    }

    #[test]
    fn pop_piece_invalid_test() {
        let mut bar = Bar {
            player_one_piece_count: 0,
            player_two_piece_count: 2
        };
        let result = bar.pop_piece(1);
        match result {
            Ok(_) => assert!(false, "expected no number"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn push_piece_test() {
        let piece = 1;
        let mut bar = Bar {
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        match bar.push_piece(piece) {
            Ok(_) => assert_eq!(1, bar.player_one_piece_count),
            Err(_) => assert!(false, "expected no error")
        }
    }
}
