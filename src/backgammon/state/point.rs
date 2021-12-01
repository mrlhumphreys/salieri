use std::convert::TryFrom;
use regex::Regex;

pub struct Point {
    pub number: i8,
    pub player_one_piece_count: i8,
    pub player_two_piece_count: i8
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
        self.player_one_piece_count > 1 || self.player_two_piece_count > 1
    }

    pub fn blot(&self) -> bool {
        self.player_one_piece_count == 1 || self.player_two_piece_count == 1
    }

    pub fn occupied_by_player(&self, player_number: i8) -> bool {
        match player_number {
            1 => self.player_one_piece_count > 0,
            2 => self.player_two_piece_count > 0,
            _ => false
        }
    }

    pub fn occupied_by_opponent(&self, player_number: i8) -> bool {
        match player_number {
            1 => self.player_two_piece_count > 0,
            2 => self.player_one_piece_count > 0,
            _ => false
        }
    }

    pub fn pop_piece(&mut self) -> Result<i8, &'static str> {
        if self.player_one_piece_count > 0 {
            self.player_one_piece_count -= 1;
            Ok(1)
        } else if self.player_two_piece_count > 0 {
            self.player_two_piece_count -= 1;
            Ok(2)
        } else {
            Err("no piece to pop")
        }
    }

    pub fn push_piece(&mut self, piece: i8) -> Result<Option<i8>, &'static str> {
        match piece {
            1 => {
                match self.player_two_piece_count {
                    0 => {
                        // unoccupied
                        self.player_one_piece_count += 1;
                        Ok(None)
                    },
                    1 => {
                        // blot
                        self.player_two_piece_count -= 1;
                        self.player_one_piece_count += 1;
                        Ok(Some(2))
                    },
                    _ => {
                        // prime
                        Err("point occupied by opponent")
                    }
                }
            },
            2 => {
                match self.player_one_piece_count {
                    0 => {
                        // unoccupied
                        self.player_two_piece_count += 1;
                        Ok(None)
                    },
                    1 => {
                        // blot
                        self.player_one_piece_count -= 1;
                        self.player_two_piece_count += 1;
                        Ok(Some(1))
                    },
                    _ => {
                        // prime
                        Err("point occupied by opponent")
                    }
                }
            },
            _ => Err("invalid piece") 
        }
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point {
            number: self.number,
            player_one_piece_count: self.player_one_piece_count.clone(),
            player_two_piece_count: self.player_two_piece_count.clone()
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

    match caps {
        Some(c) => {
            let first_value = c.get(1).unwrap().as_str().chars().nth(0).unwrap().to_digit(16).unwrap();
            let second_value = c.get(2).unwrap().as_str().chars().nth(0).unwrap().to_digit(16).unwrap();

            if first_value > 0 && second_value > 0 {
                Err("point must contain pieces from only one player")
            } else { 
                match i8::try_from(first_value) {
                    Ok(player_one_piece_count) => {
                        match i8::try_from(second_value) {
                            Ok(player_two_piece_count) => {
                                let point = Point { 
                                    number, 
                                    player_one_piece_count, 
                                    player_two_piece_count 
                                };
                                Ok(point)
                            },
                            Err(_) => Err("invalid second value")
                        }
                    },
                    Err(_) => Err("invalid first value")
                }
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
        let point = Point { 
            number: 19, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let result = point.home(player_number);
        assert!(result)
    }

    #[test]
    fn not_home_player_1_test() {
        let player_number = 1;
        let point = Point { 
            number: 18, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let result = point.home(player_number);
        assert!(!result)
    }

    #[test]
    fn home_player_2_test() {
        let player_number = 2;
        let point = Point { 
            number: 6, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let result = point.home(player_number);
        assert!(result)
    }

    #[test]
    fn not_home_player_2_test() {
        let player_number = 2;
        let point = Point { 
            number: 7, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let result = point.home(player_number);
        assert!(!result)
    }

    #[test]
    fn home_player_x_test() {
        let player_number = 3;
        let point = Point { 
            number: 7, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let result = point.home(player_number);
        assert!(!result)
    }

    #[test]
    fn prime_test() {
        let point = Point { 
            number: 1, 
            player_one_piece_count: 2,
            player_two_piece_count: 0
        };
        let result = point.prime();
        assert!(result)
    }

    #[test]
    fn not_prime_test() {
        let point = Point { 
            number: 1, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let result = point.prime();
        assert!(!result)
    }

    #[test]
    fn blot_test() {
        let point = Point { 
            number: 1, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let result = point.blot();
        assert!(result)
    }

    #[test]
    fn not_blot_test() {
        let point = Point { 
            number: 1, 
            player_one_piece_count: 2,
            player_two_piece_count: 0
        };
        let result = point.blot();
        assert!(!result)
    }

    #[test]
    fn occupied_by_player_with_player_test() {
        let point = Point { 
            number: 7, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let result = point.occupied_by_player(1);
        assert!(result)
    }

    #[test]
    fn occupied_by_player_with_opponent_test() {
        let point = Point { 
            number: 7, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let result = point.occupied_by_player(2);
        assert!(!result)
    }

    #[test]
    fn occupied_by_player_empty_test() {
        let point = Point { 
            number: 7, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let result = point.occupied_by_player(1);
        assert!(!result)
    }

    #[test]
    fn occupied_by_opponent_with_player_test() {
        let point = Point { 
            number: 7, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let result = point.occupied_by_opponent(1);
        assert!(!result)
    }

    #[test]
    fn occupied_by_opponent_with_opponent_test() {
        let point = Point { 
            number: 7, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let result = point.occupied_by_opponent(2);
        assert!(result)
    }

    #[test]
    fn occupied_by_opponent_empty_test() {
        let point = Point { 
            number: 7, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let result = point.occupied_by_opponent(2);
        assert!(!result)
    }

    #[test]
    fn parsing_point_player_one_test() {
        let encoded = "50";
        let point = parse_point(1, encoded).unwrap();
        assert_eq!(point.number, 1);
        assert_eq!(point.player_one_piece_count, 5);
    }

    #[test]
    fn parsing_point_player_two_test() {
        let encoded = "03";
        let point = parse_point(2, encoded).unwrap();
        assert_eq!(point.number, 2);
        assert_eq!(point.player_two_piece_count, 3);
    }

    #[test]
    fn parsing_point_empty_test() {
        let encoded = "00";
        let point = parse_point(3, encoded).unwrap();
        assert_eq!(point.number, 3);
        assert_eq!(point.player_one_piece_count, 0);
        assert_eq!(point.player_two_piece_count, 0);
    }

    #[test]
    fn parsing_above_10_test() {
        let encoded = "b0";
        let point = parse_point(4, encoded).unwrap();
        assert_eq!(point.number, 4);
        assert_eq!(point.player_one_piece_count, 11);
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
        let mut point = Point { 
            number: 1, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };  
        let result = point.pop_piece();
        match result {
            Ok(p) => assert_eq!(1, p),
            Err(_) => assert!(false, "expected number")    
        }
    }

    #[test]
    fn pop_piece_invalid_test() {
        let mut point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let result = point.pop_piece();
        match result {
            Ok(_) => assert!(false, "expected no number"),
            Err(_) => assert!(true)    
        }
    }

    #[test]
    fn push_empty_test() {
        let piece = 1;
        let mut point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        }; 
        let result = point.push_piece(piece);
        match result {
            Ok(piece) => {
                match piece {
                    Some(_) => assert!(false, "expected no number"),
                    None => assert!(true)
                }
            },
            Err(_) => assert!(false, "expected no error")
        }
    }

    #[test]
    fn push_blot_test() {
       let piece = 1; 
       let mut point = Point { 
           number: 1, 
           player_one_piece_count: 0,
           player_two_piece_count: 1 
       };
       let result = point.push_piece(piece);
       match result {
            Ok(piece) => {
                match piece {
                    Some(p) => assert_eq!(2, p),
                    None => assert!(false, "expected number")
                }
            },
            Err(_) => assert!(false, "expected no error")
       }
    }

    #[test]
    fn push_prime_test() {
        let piece = 1;
        let mut point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 2 
        };
        let result = point.push_piece(piece);
        match result {
            Ok(_) => assert!(false, "expected error"), 
            Err(_) => assert!(true) 
        }
    }
} 
