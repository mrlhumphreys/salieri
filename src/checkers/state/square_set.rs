use std::fmt;

use crate::checkers::state::vector::Vector;
use crate::checkers::state::vector::Direction;
use crate::checkers::state::square::Square;
use crate::checkers::state::piece::Piece;
use crate::checkers::state::mov::Move;
use crate::checkers::state::square::parse as parse_square;

#[derive(PartialEq, Debug)]
pub struct SquareSet {
    pub squares: Vec<Square>,
}

impl Clone for SquareSet {
    fn clone(&self) -> SquareSet {
        SquareSet {
            squares: self.squares.clone(),
        }
    }
}

impl fmt::Display for SquareSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let characters = self.squares.iter().map(|s| {
            match s.piece {
                Some(p) => {
                   match p.player_number {
                        1 => "b",
                        2 => "w",
                        _ => "?"
                   }
                },
                None => "-"
            }
        }).collect::<String>();
        write!(f, "{}", characters)
    }
}

impl SquareSet {
    pub fn perform_move(&self, from: i8, to: i8) -> Result<SquareSet, &'static str> {
        let mut squares = self.squares.clone();
        let player_number: i8;
        let king: bool;
        // println!("Moving: {} {} {}", self, from, to);

        match squares.iter_mut().find(|s| s.id == from) {
            Some(s) => {
                match s.piece {
                    Some(p) => {
                        player_number = p.player_number;
                        king = p.king;
                    },
                    None => {
                        return Err("square_set::perform_move - No piece on from");
                    },
                }
                s.piece = None;
            },
            None => return Err("Invalid From Square"),
        }

        match squares.iter_mut().find(|s| s.id == to) {
            Some(s) => { 
                let piece = Piece { player_number, king };
                s.piece = Some(piece); 
            },
            None => return Err("Invalid To Square"),
        }

        let from_square = squares.iter().find(|s| s.id == from);
        let to_square = squares.iter().find(|s| s.id == to);

        match self.between(&from_square.unwrap(), &to_square.unwrap()).first() {
            Some(b) => {
                let new_between = squares.iter_mut().find(|s| s.id == b.id);
                match new_between {
                    Some(n) => n.piece = None,
                    None => (),
                }
            },
            None => (),
        }

        Ok(SquareSet { squares }) 
    }

    pub fn promote(&self, id: i8) -> Result<SquareSet, &'static str> {
        let mut squares = self.squares.clone();
        match squares.iter_mut().find(|s| s.id == id) {
            Some(s) => { 
                match s.promote() {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            },
            None => return Err("Invalid From Square"),
        }
        Ok(SquareSet { squares }) 
    }

    pub fn jumps_for_player(&self, player_number: i8, board: &SquareSet) -> Vec<Move> {
        let jump_froms: Vec<&Square> = self.squares.iter().filter(|s| {
            match &s.piece {
                Some(p) => s.occupied_by_player(player_number) && s.can_jump(p, &board),
                None => false,
            }
        }).collect();

        let mut list = Vec::new(); 
        
        for from in jump_froms {
            match from.piece {
                Some(p) => list.append(&mut from.jumps(&p, &board)), 
                None => (),
            }
        }

        list 
    }

    pub fn moves_for_player(&self, player_number: i8, board: &SquareSet) -> Vec<Move> {
        let move_froms: Vec<&Square> = self.squares.iter().filter(|s| {
            match &s.piece {
                Some(p) => s.occupied_by_player(player_number) && s.can_move(p, &board),
                None => false,
            }
        }).collect();

        let mut list = Vec::new();

        for from in move_froms {
            match from.piece {
                Some(p) => list.append(&mut from.moves(&p, &board)),
                None => (),
            }
        }

        list
    }

    pub fn first(&self) -> Option<&Square> {
       self.squares.first() 
    }

    pub fn find_by_x_and_y(&self, x: i8, y: i8) -> Option<&Square> {
        self.squares.iter().find(|s| { s.x == x && s.y == y }) 
    }

    pub fn between(&self, from: &Square, to: &Square) -> SquareSet {
        let vector = Vector { from: from.point(), to: to.point() };

        let squares = match vector.direction() {
          Direction::Other => {
              Vec::new()
          },
          _ => {
            let direction_unit = vector.direction_unit();
            let end = to.point();
            let mut counter = from.point() + direction_unit; 
            let mut acc = Vec::new(); 
            while counter != end {
                let square = self.find_by_x_and_y(counter.x, counter.y);
                match square {
                    Some(s) => acc.push(*s),
                    None => {}, 
                }
                counter = counter + direction_unit;
            }
            acc 
          },
        };

        SquareSet { squares }
    }
}

pub fn parse_square_set(encoded: &str) -> Result<SquareSet, &'static str> {
    let mut squares = Vec::new();

    // add index, pass vars to parse square
    for (i, c) in encoded.char_indices() {
        match parse_square(i, c) {
            Ok(s) => squares.push(s),
            Err(e) => return Err(e),
        }
    }

    let square_set = SquareSet { squares };
    Ok(square_set)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_square_set() {
        let encoded = "bbbbbbbbbbbb--------wwwwwwwwwwww";
        let square_set = parse_square_set(encoded).unwrap();
        let squares = square_set.squares;
        assert_eq!(squares.len(), 32);
        let square = &squares[0];
        match &square.piece {
            Some(piece) => assert_eq!(piece.player_number, 1),
            None => assert!(false, "Expected Piece"),
        }
    }

    #[test]
    fn fetching_first() {
        let first = Square { id: 1, x: 1, y: 1, piece: None };
        let second = Square { id: 2, x: 2, y: 2, piece: None };
        let squares = vec![first, second];
        let square_set = SquareSet { squares };        
        let result = square_set.first();
        match result {
            Some(square) => {
                assert_eq!(square.x, 1);
                assert_eq!(square.y, 1);
            },
            None => assert!(false, "Expected Square"),
        }
    }

    #[test]
    fn fetching_by_x_and_y() {
        let first = Square { id: 1, x: 1, y: 1, piece: None };
        let second = Square { id: 2, x: 2, y: 2, piece: None };
        let squares = vec![first, second];
        let square_set = SquareSet { squares };        
        let result = square_set.find_by_x_and_y(1, 1);
        match result {
            Some(square) => {
                assert_eq!(square.x, 1);
                assert_eq!(square.y, 1);
            },
            None => assert!(false, "Expected Square"),
        }
    }

    #[test]
    fn fetching_between_diagonal() {
        let from = Square { id: 1, x: 4, y: 4, piece: None };    
        let between = Square { id: 2, x: 5, y: 3, piece: None };    
        let to = Square { id: 3, x: 6, y: 2, piece: None };    
        let square_set = SquareSet { squares: vec![from, between, to] };
        let result = square_set.between(&from, &to);
        assert_eq!(result.squares.len(), 1);
        let square = &result.squares[0];
        assert_eq!(square.x, 5);
        assert_eq!(square.y, 3);
    }

    #[test]
    fn fetching_between_l_shape() {
        let from = Square { id: 1, x: 4, y: 4, piece: None };    
        let between = Square { id: 2, x: 5, y: 4, piece: None };    
        let to = Square { id: 3, x: 6, y: 3, piece: None };    
        let square_set = SquareSet { squares: vec![from, between, to] };
        let result = square_set.between(&from, &to);
        assert_eq!(result.squares.len(), 0);
    }

    #[test]
    fn perform_move_jump() {
        let jump_from = Square { id: 1, x: 4, y: 4, piece: Some(Piece { player_number: 1, king: false }) };
        let jump_over = Square { id: 2, x: 5, y: 5, piece: Some(Piece { player_number: 2, king: false }) };
        let jump_to = Square { id: 3, x: 6, y: 6, piece: None };
        let board = SquareSet { squares: vec![jump_from, jump_over, jump_to] };

        let new_board = match board.perform_move(jump_from.id, jump_to.id) {
            Ok(b) => b,
            Err(e) => return assert!(false, e), 
        };

        let mut iterator = new_board.squares.iter();
        let new_from = iterator.find(|s| s.id == 1);  

        match new_from {
            Some(square) => {
                match square.piece {
                    Some(_) => assert!(false, "expected no piece"),
                    None => assert!(true),
                }
            },
            None => assert!(false, "expected square"),
        }

        let new_over = iterator.find(|s| s.id == 2);

        match new_over {
            Some(square) => {
                match square.piece {
                    Some(_) => assert!(false, "expected no piece"),
                    None => assert!(true),
                }
            },
            None => assert!(false, "expected square"),
        }

        let new_to = iterator.find(|s| s.id == 3);

        match new_to {
            Some(square) => {
                match square.piece {
                    Some(p) => {
                        assert_eq!(p.player_number, 1);
                    },
                    None => assert!(false, "expected piece"),
                }
            },
            None => assert!(false, "expected square"),
        }
    }

    #[test]
    fn perform_move_move() {
        let from = Square { id: 1, x: 4, y: 4, piece: Some(Piece { player_number: 1, king: false }) };
        let to = Square { id: 2, x: 5, y: 5, piece: None };
        let board = SquareSet { squares: vec![from, to] };

        let new_board = match board.perform_move(from.id, to.id) {
            Ok(b) => b,
            Err(e) => return assert!(false, e), 
        };

        let mut iterator = new_board.squares.iter();
        let new_from = iterator.find(|s| s.id == 1);  

        match new_from {
            Some(square) => {
                match square.piece {
                    Some(_) => assert!(false, "expected no piece"),
                    None => assert!(true),
                }
            },
            None => assert!(false, "expected square"),
        }

        let new_to = iterator.find(|s| s.id == 2);

        match new_to {
            Some(square) => {
                match square.piece {
                    Some(p) => {
                        assert_eq!(p.player_number, 1);
                    },
                    None => assert!(false, "expected piece"),
                }
            },
            None => assert!(false, "expected square"),
        }
    }

    #[test]
    fn fetch_moves() {
        let from = Square { id: 1, x: 4, y: 4, piece: Some(Piece { player_number: 1, king: false })};
        let to = Square { id: 2, x: 5, y: 5, piece: None };
        let cant_to = Square { id: 3, x: 3, y: 5, piece: Some(Piece { player_number: 2, king: false })};
        let square_set = SquareSet { squares: vec![from] };
        let board = SquareSet { squares: vec![from, cant_to, to] };

        let result = square_set.moves_for_player(1, &board);

        assert_eq!(result.len(), 1);

        let mov = result.first();
        match mov {
            Some(m) => {
                assert_eq!(m.from, 1);
                assert_eq!(m.to, vec![2]);
            },
            None => assert!(false, "Expected Move"),
        }
    }

    #[test]
    fn fetch_jumps() {
        let from = Square { id: 1, x: 4, y: 4, piece: Some(Piece { player_number: 1, king: false })};
        let over = Square { id: 2, x: 5, y: 5, piece: Some(Piece { player_number: 2, king: false })};
        let to = Square { id: 3, x: 6, y: 6, piece: None };
        let cant_over = Square { id: 4, x: 3, y: 5, piece: Some(Piece { player_number: 2, king: false })};
        let cant_to = Square { id: 5, x: 2, y: 6, piece: Some(Piece { player_number: 1, king: false })};
        let square_set = SquareSet { squares: vec![from] };
        let board = SquareSet { squares: vec![from, over, to, cant_over, cant_to] };

        let result = square_set.jumps_for_player(1, &board);

        assert_eq!(result.len(), 1);

        let mov = result.first();
        match mov {
            Some(m) => {
                assert_eq!(m.from, 1);
                assert_eq!(m.to, vec![3]);
            },
            None => assert!(false, "Expected Move"),
        }
    }

    #[test]
    fn promote_piece() {
        let promoteable = Square { id: 1, x: 4, y: 4, piece: Some(Piece { player_number: 1, king: false })};
        let not_promoteable = Square { id: 2, x: 5, y: 5, piece: Some(Piece { player_number: 2, king: false })};
        let square_set = SquareSet { squares: vec![promoteable, not_promoteable] };

        match square_set.promote(1) {
            Ok(ss) => {
                match ss.squares.into_iter().find(|s| s.id == 1) {
                    Some(s) => {
                        match s.piece {
                            Some(p) => assert_eq!(true, p.king),
                            None => assert!(false, "expected piece"),
                        }
                    },
                    None => assert!(false, "expected square"),
                }
            },
            Err(e) => assert!(false, e),
        }
    }
}
