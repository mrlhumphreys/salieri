use std::fmt;

use crate::checkers::state::vector::Vector;
use crate::checkers::state::vector::Direction;
use crate::checkers::state::square::Square;
use crate::checkers::state::mov::Move;

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
            match s.player_number {
                1 => "b",
                2 => "w",
                _ => "-"
            }
        }).collect::<String>();
        write!(f, "{}", characters)
    }
}

impl SquareSet {
    pub fn perform_move(&mut self, from: i8, to: i8) -> Result<(), &'static str> {
        let player_number: i8;
        let king: bool;

        if let Some(s) = self.squares.iter_mut().find(|s| s.id == from) {
            if s.occupied() {
                player_number = s.player_number;
                king = s.king;
            } else {
                return Err("square_set::perform_move - No piece on from");
            }
            s.player_number = 0;
            s.king = false;
        } else {
            return Err("Invalid From Square")
        }

        if let Some(s) = self.squares.iter_mut().find(|s| s.id == to) {
            s.player_number = player_number;
            s.king = king;
        } else {
            return Err("Invalid To Square")
        }

        let from_square = self.squares.iter().find(|s| s.id == from);
        let to_square = self.squares.iter().find(|s| s.id == to);

        if let Some(b) = self.between(&from_square.unwrap(), &to_square.unwrap()).first() {
            if let Some(n) = self.squares.iter_mut().find(|s| s.id == b.id) {
                n.player_number = 0;
                n.king = false;
            }
        }

        Ok(())
    }

    pub fn undo_move(&mut self, from: i8, to: i8) -> Result<(), &'static str> {
        let player_number: i8;
        let king: bool;

        if let Some(s) = self.squares.iter_mut().find(|s| s.id == to) {
            if s.occupied() {
                player_number = s.player_number;
                king = s.king;
            } else {
                return Err("square_set::undo_move - No piece on from");
            }
            s.player_number = 0;
            s.king = false;
        } else {
            return Err("Invalid To Square")
        }

        if let Some(s) = self.squares.iter_mut().find(|s| s.id == from) {
            s.player_number = player_number;
            s.king = king;
        } else {
            return Err("Invalid From Square")
        }

        let from_square = self.squares.iter().find(|s| s.id == from);
        let to_square = self.squares.iter().find(|s| s.id == to);

        if let Some(b) = self.between(&from_square.unwrap(), &to_square.unwrap()).first() {
            if let Some(n) = self.squares.iter_mut().find(|s| s.id == b.id) {
                n.player_number = match player_number {
                    2 => 1,
                    1 => 2,
                    _ => 0
                };
                n.king = false;
            }
        }

        Ok(())
    }

    pub fn promote(&mut self, id: i8) -> Result<(), &'static str> {
        if let Some(s) = self.squares.iter_mut().find(|s| s.id == id) {
            s.promote()?;
        } else {
            return Err("Invalid From Square")
        }

        Ok(())
    }

    pub fn demote(&mut self, id: i8) -> Result<(), &'static str> {
        if let Some(s) = self.squares.iter_mut().find(|s| s.id == id) {
            s.demote()?;
        } else {
            return Err("Invalid From Square")
        }

        Ok(())
    }

    pub fn jumps_for_player(&self, player_number: i8, board: &SquareSet) -> Vec<Move> {
        let jump_froms: Vec<&Square> = self.squares.iter().filter(|s| {
            s.occupied_by_player(player_number) && s.can_jump(s.player_number, s.king, &board)
        }).collect();

        let mut list = Vec::new();

        for from in jump_froms {
            if from.occupied() {
               list.append(&mut from.jumps(from.player_number, from.king, &board));
            }
        }

        list
    }

    pub fn moves_for_player(&self, player_number: i8, board: &SquareSet) -> Vec<Move> {
        let move_froms: Vec<&Square> = self.squares.iter().filter(|s| {
            s.occupied_by_player(player_number) && s.can_move(s.player_number, s.king, &board)
        }).collect();

        let mut list = Vec::new();

        for from in move_froms {
            if from.occupied() {
                list.append(&mut from.moves(from.player_number, from.king, &board))
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

        let squares = if vector.direction() == Direction::Other {
            Vec::new()
        } else {
            let direction_unit = vector.direction_unit();
            let end = to.point();
            let mut counter = from.point() + direction_unit;
            let mut acc = Vec::new();
            while counter != end {
                let square = self.find_by_x_and_y(counter.x, counter.y);
                if let Some(s) = square {
                    acc.push(*s)
                }
                counter = counter + direction_unit;
            }
            acc
        };

        SquareSet { squares }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetching_first() {
        let first = Square { id: 1, x: 1, y: 1, player_number: 0, king: false };
        let second = Square { id: 2, x: 2, y: 2, player_number: 0, king: false };
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
        let first = Square { id: 1, x: 1, y: 1, player_number: 0, king: false };
        let second = Square { id: 2, x: 2, y: 2, player_number: 0, king: false };
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
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between = Square { id: 2, x: 5, y: 3, player_number: 1, king: false };
        let to = Square { id: 3, x: 6, y: 2, player_number: 0, king: false };
        let square_set = SquareSet { squares: vec![from, between, to] };
        let result = square_set.between(&from, &to);
        assert_eq!(result.squares.len(), 1);
        let square = &result.squares[0];
        assert_eq!(square.x, 5);
        assert_eq!(square.y, 3);
    }

    #[test]
    fn fetching_between_l_shape() {
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between = Square { id: 2, x: 5, y: 4, player_number: 0, king: false };
        let to = Square { id: 3, x: 6, y: 3, player_number: 0, king: false };
        let square_set = SquareSet { squares: vec![from, between, to] };
        let result = square_set.between(&from, &to);
        assert_eq!(result.squares.len(), 0);
    }

    #[test]
    fn perform_move_jump() {
        let jump_from = Square { id: 1, x: 4, y: 4, player_number: 1, king: false };
        let jump_over = Square { id: 2, x: 5, y: 5, player_number: 2, king: false };
        let jump_to = Square { id: 3, x: 6, y: 6, player_number: 0, king: false };
        let mut board = SquareSet { squares: vec![jump_from, jump_over, jump_to] };

        match board.perform_move(jump_from.id, jump_to.id) {
            Ok(_) => (),
            Err(e) => return assert!(false, "{}", e),
        };

        let mut iterator = board.squares.iter();
        let new_from = iterator.find(|s| s.id == 1);

        match new_from {
            Some(s) => assert_eq!(s.occupied(), false),
            None => assert!(false, "expected square"),
        }

        let new_over = iterator.find(|s| s.id == 2);

        match new_over {
            Some(s) => assert_eq!(s.occupied(), false),
            None => assert!(false, "expected square"),
        }

        let new_to = iterator.find(|s| s.id == 3);

        match new_to {
            Some(s) => assert_eq!(s.player_number, 1),
            None => assert!(false, "expected square"),
        }
    }

    #[test]
    fn perform_move_move() {
        let from = Square { id: 1, x: 4, y: 4, player_number: 1, king: false };
        let to = Square { id: 2, x: 5, y: 5, player_number: 0, king: false };
        let mut board = SquareSet { squares: vec![from, to] };

        match board.perform_move(from.id, to.id) {
            Ok(_) => (),
            Err(e) => return assert!(false, "{}", e),
        };

        let mut iterator = board.squares.iter();
        let new_from = iterator.find(|s| s.id == 1);

        match new_from {
            Some(s) => assert_eq!(s.occupied(), false),
            None => assert!(false, "expected square"),
        }

        let new_to = iterator.find(|s| s.id == 2);

        match new_to {
            Some(s) => assert_eq!(s.player_number, 1),
            None => assert!(false, "expected square"),
        }
    }

    #[test]
    fn undo_move_jump() {
        let jump_from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let jump_over = Square { id: 2, x: 5, y: 5, player_number: 0, king: false };
        let jump_to = Square { id: 3, x: 6, y: 6, player_number: 1, king: false };
        let mut board = SquareSet { squares: vec![jump_from, jump_over, jump_to] };

        match board.undo_move(jump_from.id, jump_to.id) {
            Ok(_) => (),
            Err(e) => return assert!(false, "{}", e)
        };

        let mut iterator = board.squares.iter();
        let new_from = iterator.find(|s| s.id == 1);

        match new_from {
            Some(s) => assert_eq!(s.player_number, 1),
            None => assert!(false, "expected square"),
        }

        let new_over = iterator.find(|s| s.id == 2);

        match new_over {
            Some(s) => {
                assert_eq!(s.occupied(), true);
                assert_eq!(s.player_number, 2);
            },
            None => assert!(false, "expected square"),
        }

        let new_to = iterator.find(|s| s.id == 3);

        match new_to {
            Some(s) => assert_eq!(s.occupied(), false),
            None => assert!(false, "expected square"),
        }
    }

    #[test]
    fn undo_move_move() {
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let to = Square { id: 2, x: 5, y: 5, player_number: 1, king: false };
        let mut board = SquareSet { squares: vec![from, to] };

        match board.undo_move(from.id, to.id) {
            Ok(_) => (),
            Err(e) => return assert!(false, "{}", e)
        };

        let mut iterator = board.squares.iter();
        let new_from = iterator.find(|s| s.id == 1);

        match new_from {
            Some(s) => assert_eq!(s.player_number, 1),
            None => assert!(false, "expected square"),
        }

        let new_to = iterator.find(|s| s.id == 2);

        match new_to {
            Some(s) => assert_eq!(s.occupied(), false),
            None => assert!(false, "expected square"),
        }
    }

    #[test]
    fn fetch_moves() {
        let from = Square { id: 1, x: 4, y: 4, player_number: 2, king: false };
        let to = Square { id: 2, x: 5, y: 5, player_number: 0, king: false };
        let cant_to = Square { id: 3, x: 3, y: 5, player_number: 1, king: false };
        let square_set = SquareSet { squares: vec![from] };
        let board = SquareSet { squares: vec![from, cant_to, to] };

        let result = square_set.moves_for_player(2, &board);

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
        let from = Square { id: 1, x: 4, y: 4, player_number: 2, king: false };
        let over = Square { id: 2, x: 5, y: 5, player_number: 1, king: false };
        let to = Square { id: 3, x: 6, y: 6, player_number: 0, king: false };
        let cant_over = Square { id: 4, x: 3, y: 5, player_number: 1, king: false };
        let cant_to = Square { id: 5, x: 2, y: 6, player_number: 2, king: false };
        let square_set = SquareSet { squares: vec![from] };
        let board = SquareSet { squares: vec![from, over, to, cant_over, cant_to] };

        let result = square_set.jumps_for_player(2, &board);

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
        let promoteable = Square { id: 1, x: 4, y: 4, player_number: 1, king: false };
        let not_promoteable = Square { id: 2, x: 5, y: 5, player_number: 2, king: false };
        let mut square_set = SquareSet { squares: vec![promoteable, not_promoteable] };

        match square_set.promote(1) {
            Ok(_) => {
                match square_set.squares.into_iter().find(|s| s.id == 1) {
                    Some(s) => assert_eq!(true, s.king),
                    None => assert!(false, "expected square"),
                }
            },
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn demote_piece() {
        let promoted = Square { id: 1, x: 4, y: 4, player_number: 1, king: true };
        let not_promoted = Square { id: 2, x: 5, y: 5, player_number: 2, king: false };
        let mut square_set = SquareSet { squares: vec![promoted, not_promoted] };

        match square_set.demote(1) {
            Ok(_) => {
                match square_set.squares.into_iter().find(|s| s.id == 1) {
                    Some(s) => assert_eq!(false, s.king),
                    None => assert!(false, "expected square"),
                }
            },
            Err(e) => assert!(false, "{}", e),
        }
    }
}
