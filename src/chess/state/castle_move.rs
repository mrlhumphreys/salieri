use crate::chess::state::point::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    Queen,
    King
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CastleMove {
    pub player_number: i8,
    pub side: Side
}

impl CastleMove {
    pub fn from(&self) -> Point {
        let x = match self.side {
            Side::King => 7,
            Side::Queen => 0
        };
        let y = match self.player_number {
            1 => 7,
            _ => 0
        }; 
        Point { x, y }
    }

    pub fn to(&self) -> Point {
        let x = match self.side {
            Side::King => 5,
            Side::Queen => 3 
        };
        let y = match self.player_number {
            1 => 7,
            _ => 0
        }; 
        Point { x, y }
    }
}

pub fn parse(encoded: char) -> Option<CastleMove> {
    match encoded {
        'K' => Some(CastleMove { player_number: 1, side: Side::King }),
        'Q' => Some(CastleMove { player_number: 1, side: Side::Queen }),
        'k' => Some(CastleMove { player_number: 2, side: Side::King }),
        'q' => Some(CastleMove { player_number: 2, side: Side::Queen }),
        _ => None 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_one_king_test() {
       let castle_move = CastleMove { player_number: 1, side: Side::King };
       let expected = Point { x: 7, y: 7 };
       let result = castle_move.from();
       assert_eq!(result, expected);
    }

    #[test]
    fn from_one_queen_test() {
       let castle_move = CastleMove { player_number: 1, side: Side::Queen };
       let expected = Point { x: 0, y: 7 };
       let result = castle_move.from();
       assert_eq!(result, expected);
    }

    #[test]
    fn from_two_king_test() {
       let castle_move = CastleMove { player_number: 2, side: Side::King };
       let expected = Point { x: 7, y: 0 };
       let result = castle_move.from();
       assert_eq!(result, expected);
    }

    #[test]
    fn from_two_queen_test() {
       let castle_move = CastleMove { player_number: 2, side: Side::Queen };
       let expected = Point { x: 0, y: 0 };
       let result = castle_move.from();
       assert_eq!(result, expected);
    }

    #[test]
    fn to_one_king_test() {
       let castle_move = CastleMove { player_number: 1, side: Side::King };
       let expected = Point { x: 5, y: 7 };
       let result = castle_move.to();
       assert_eq!(result, expected);
    }
    
    #[test]
    fn to_one_queen_test() {
       let castle_move = CastleMove { player_number: 1, side: Side::Queen };
       let expected = Point { x: 3, y: 7 };
       let result = castle_move.to();
       assert_eq!(result, expected);
    }

    #[test]
    fn to_two_king_test() {
       let castle_move = CastleMove { player_number: 2, side: Side::King };
       let expected = Point { x: 5, y: 0 };
       let result = castle_move.to();
       assert_eq!(result, expected);
    }

    #[test]
    fn to_two_queen_test() {
       let castle_move = CastleMove { player_number: 2, side: Side::Queen };
       let expected = Point { x: 3, y: 0 };
       let result = castle_move.to();
       assert_eq!(result, expected);
    }

   #[test]
   fn parse_king_one_test() {
       let expected = Some(CastleMove { player_number: 1, side: Side::King });
       let result = parse('K');
       assert_eq!(result, expected);
   } 

   #[test]
   fn parse_queen_one_test() {
       let expected = Some(CastleMove { player_number: 1, side: Side::Queen });
       let result = parse('Q');
       assert_eq!(result, expected);
   } 

   #[test]
   fn parse_king_two_test() {
       let expected = Some(CastleMove { player_number: 2, side: Side::King });
       let result = parse('k');
       assert_eq!(result, expected);
   }

   #[test]
   fn parse_queen_two_test() {
       let expected = Some(CastleMove { player_number: 2, side: Side::Queen });
       let result = parse('q');
       assert_eq!(result, expected);
   }

   #[test]
   fn parse_blank_test() {
       let expected = None;
       let result = parse(' ');
       assert_eq!(result, expected);
   }
}
