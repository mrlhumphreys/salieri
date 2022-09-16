use std::cmp;
use std::convert::TryFrom;
use crate::checkers::state::point::Point;
use crate::checkers::state::square_set::SquareSet;
use crate::checkers::state::mov::Move;
use crate::checkers::state::mov::MoveKind;

#[derive(Clone, Copy, Debug)]
pub struct Square {
    pub id: i8,
    pub x: i8,
    pub y: i8,
    pub player_number: i8,
    pub king: bool
}

impl PartialEq for Square {
    fn eq(&self, other: &Square) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Square {
    pub fn promote(&mut self) -> Result<bool, &'static str> {
        if self.occupied() {
            self.king = true;
            Ok(true)
        } else {
            Err("No Piece")
        }
    }

    pub fn demote(&mut self) -> Result<bool, &'static str> {
       if self.occupied() {
            self.king = false;
            Ok(true)
       } else {
            Err("No Piece")
       }
    }

    pub fn point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn occupied(&self) -> bool {
        self.player_number != 0
    }

    pub fn unoccupied(&self) -> bool {
        self.player_number == 0
    }

    pub fn occupied_by_player(&self, player_number: i8) -> bool {
        self.player_number == player_number
    }

    pub fn occupied_by_opponent(&self, player_number: i8) -> bool {
        self.occupied() && self.player_number != player_number
    }

    pub fn in_direction(&self, from: &Square, player_number: i8, king: bool) -> bool {
        let direction = match king {
            true => 0,
            false => match player_number {
                1 => 1,
                2 => -1,
                _ => 0
            }
        };

        match direction {
            -1 => self.y < from.y,
            0 => self.y != from.y,
            1 => self.y > from.y,
            _ => false,
        }
    }

    pub fn diagonal(&self, to: &Square) -> bool {
        let abs_dx = (to.x - self.x).abs();
        let abs_dy = (to.y - self.y).abs();
        abs_dx == abs_dy && abs_dx != 0 && abs_dy != 0
    }

    pub fn magnitude(&self, to: &Square) -> i8 {
        let abs_dx = (to.x - self.x).abs();
        let abs_dy = (to.y - self.y).abs();
        cmp::max(abs_dx, abs_dy) 
    }

    pub fn can_jump(&self, player_number: i8, king: bool, board: &SquareSet) -> bool {
        board.squares.iter().any(|s| {
            self.magnitude(&s) == 2 && 
                self.diagonal(&s) && 
                s.in_direction(&self, player_number, king) &&
                s.unoccupied() && 
                match board.between(&self, &s).first() {
                    Some(b) => b.occupied_by_opponent(player_number),
                    None => false,
                }
        })
    }

    pub fn can_move(&self, player_number: i8, king: bool, board: &SquareSet) -> bool {
        board.squares.iter().any(|s| {
            self.magnitude(&s) == 1 && 
                self.diagonal(&s) && 
                s.in_direction(&self, player_number, king) &&
                s.unoccupied()   
        })
    }

    pub fn jump_destinations<'a>(&self, player_number: i8, king: bool, board: &'a SquareSet) -> Vec<&'a Square> {
        board.squares.iter().filter(|s| {
            self.magnitude(&s) == 2 && 
                self.diagonal(&s) && 
                s.in_direction(&self, player_number, king) &&
                s.unoccupied() && 
                match board.between(&self, &s).first() {
                    Some(b) => b.occupied_by_opponent(player_number),
                    None => false,
                }
        }).collect()
    }

    pub fn move_destinations<'a>(&self, player_number: i8, king: bool, board: &'a SquareSet) -> Vec<&'a Square> {
        board.squares.iter().filter(|s| {
            self.magnitude(&s) == 1 && 
                self.diagonal(&s) && 
                s.in_direction(&self, player_number, king) &&
                s.unoccupied()   
        }).collect()
    }

    pub fn jump_legs<'a>(&self, player_number: i8, king: bool, board: &SquareSet, mut accumulator: &'a mut Vec<Vec<i8>>, mut current_leg: &mut Vec<i8>) -> &'a mut Vec<Vec<i8>> {
        let destinations = self.jump_destinations(player_number, king, board);

        if destinations.len() > 0 {
            for destination in destinations.iter() {
                
                if current_leg.len() == 0 {
                    current_leg.push(self.id);
                }
                
                current_leg.push(destination.id);

                let mut new_board = board.clone();
                match new_board.perform_move(self.id, destination.id) {
                    Ok(_) => {
                        destination.jump_legs(player_number, king, &new_board, &mut accumulator, &mut current_leg);
                    },
                    Err(_) => (),
                }
            }
        } else {
            accumulator.push(current_leg.clone());
        }

        // pop the last element of the leg before we return and go back up the tree
        current_leg.pop();
        accumulator
    }

    pub fn jumps(&self, player_number: i8, king: bool, board: &SquareSet) -> Vec<Move> {
        let mut accumulator = vec![];
        let mut current_leg = vec![];
        let all_legs = self.jump_legs(player_number, king, &board, &mut accumulator, &mut current_leg);
        all_legs.iter().map(|l| {
            let from_id = l[0];
            let to_ids = l[1..].to_vec();
            Move { kind: MoveKind::Jump, from: from_id, to: to_ids } 
        }).collect()
    }

    pub fn moves(&self, player_number: i8, king: bool, board: &SquareSet) -> Vec<Move> {
        let destinations = self.move_destinations(player_number, king, &board);
        destinations.iter().map(|d| {
            Move { kind: MoveKind::Mov, from: self.id, to: vec![d.id] }
        }).collect()
    }
}

pub fn parse(index: usize, encoded: char) -> Result<Square, &'static str> {
    let i = match i8::try_from(index) {
        Ok(num) => num,
        Err(_) => return Err("Invalid Board Length"),
    };

    let id = i + 1;

    let y = i / 4;

    let offset = if y % 2 == 0 {
       1
    } else {
       0 
    };

    let x = ((i % 4) * 2 ) + offset;

    let (player_number, king) = match encoded {
        'w' => (2, false),
        'W' => (2, true),
        'b' => (1, false),
        'B' => (1, true),
        '-' => (0, false),
        _ => return Err("Invalid Piece")
    };
    let square = Square { id, x, y, player_number, king };
    Ok(square)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_square_with_index() {
        let index = 11;
        let encoded = 'w';
        let square = parse(index, encoded).unwrap();
        assert_eq!(square.id, 12);
        assert_eq!(square.x, 7);
        assert_eq!(square.y, 2);
        assert_eq!(square.occupied(), true);
        assert_eq!(square.player_number, 2);
        assert_eq!(square.king, false);
    }

    #[test]
    fn parsing_square_occupied() {
        let index = 1;
        let encoded = 'w';
        let square = parse(index, encoded).unwrap();
        assert_eq!(square.occupied(), true);
    }

    #[test]
    fn parsing_square_unoccupied() {
        let index = 1;
        let encoded = '-';
        let square = parse(index, encoded).unwrap();
        assert_eq!(square.occupied(), false);
    }

    #[test]
    fn parsing_square_invalid() {
        let index = 1;
        let encoded = 'a';
        let result = parse(index, encoded);
        match result {
            Ok(_) => assert!(false, "Expected Error"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn occupied_by_player_own_player() {
        let square = Square { id: 1, x: 1, y: 1, player_number: 1, king: false };
        assert_eq!(square.occupied_by_player(1), true);
        assert_eq!(square.occupied_by_opponent(1), false);
        assert_eq!(square.unoccupied(), false);
    }

    #[test]
    fn occupied_by_player_other_player() {
        let square = Square { id: 1, x: 1, y: 1, player_number: 2, king: false };
        assert_eq!(square.occupied_by_player(1), false);
        assert_eq!(square.occupied_by_opponent(1), true);
        assert_eq!(square.unoccupied(), false);
    }

    #[test]
    fn occupied_by_player_unoccupied() {
        let square = Square { id: 1, x: 1, y: 1, player_number: 0, king: false };
        assert_eq!(square.occupied_by_player(1), false);
        assert_eq!(square.occupied_by_opponent(1), false);
        assert_eq!(square.unoccupied(), true);
    }

    #[test]
    fn moving_up_and_square_is_up_from_origin() {
        let square = Square { id: 1, x: 4, y: 3, player_number: 0, king: false };
        let from = Square { id: 2, x: 4, y: 4, player_number: 0, king: false };
        let (player_number, king) = (2, false);
        let result = square.in_direction(&from, player_number, king);
        assert_eq!(result, true);
    }

    #[test]
    fn moving_up_and_square_is_down_from_origin() {
        let square = Square { id: 1, x: 4, y: 5, player_number: 0, king: false };
        let from = Square { id: 2, x: 4, y: 4, player_number: 0, king: false };
        let (player_number, king) = (2, false);
        let result = square.in_direction(&from, player_number, king);
        assert_eq!(result, false);
    }

    #[test]
    fn moving_up_and_square_is_on_same_row_as_origin() {
        let square = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let from = Square { id: 2, x: 4, y: 4, player_number: 0, king: false };
        let (player_number, king) = (2, false);
        let result = square.in_direction(&from, player_number, king);
        assert_eq!(result, false);
    }

    #[test]
    fn moving_down_and_square_is_up_from_origin() {
        let square = Square { id: 1, x: 4, y: 3, player_number: 0, king: false };
        let from = Square { id: 2, x: 4, y: 4, player_number: 0, king: false };
        let (player_number, king) = (1, false);
        let result = square.in_direction(&from, player_number, king);
        assert_eq!(result, false);
    }

    #[test]
    fn moving_down_and_square_is_down_from_origin() {
        let square = Square { id: 1, x: 4, y: 5, player_number: 0, king: false };
        let from = Square { id: 2, x: 4, y: 4, player_number: 0, king: false };
        let (player_number, king) = (1, false);
        let result = square.in_direction(&from, player_number, king);
        assert_eq!(result, true);
    }

    #[test]
    fn moving_down_and_square_is_in_same_row_as_origin() {
        let square = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let (player_number, king) = (1, false);
        let result = square.in_direction(&from, player_number, king);
        assert_eq!(result, false);
    }

    #[test]
    fn moving_any_and_square_is_up_from_origin() {
        let square = Square { id: 1, x: 4, y: 3, player_number: 0, king: false };
        let from = Square { id: 2, x: 4, y: 4, player_number: 0, king: false };
        let (player_number, king) = (1, true);
        let result = square.in_direction(&from, player_number, king);
        assert_eq!(result, true);
    }

    #[test]
    fn moving_any_and_square_is_down_from_origin() {
        let square = Square { id: 1, x: 4, y: 5, player_number: 0, king: false };
        let from = Square { id: 2, x: 4, y: 4, player_number: 0, king: false };
        let (player_number, king) = (1, true);
        let result = square.in_direction(&from, player_number, king);
        assert_eq!(result, true);
    }

    #[test]
    fn moving_any_and_square_is_in_same_row_as_origin() {
        let square = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let from = Square { id: 2, x: 4, y: 4, player_number: 0, king: false };
        let (player_number, king) = (1, true);
        let result = square.in_direction(&from, player_number, king);
        assert_eq!(result, false);
    }

    #[test]
    fn pieces_can_jump() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between = Square { id: 2, x: 3, y: 5, player_number: 2, king: false };
        let to = Square { id: 3, x: 2, y: 6, player_number: 0, king: false };
        let board = SquareSet { squares: vec![from, between, to] };

        let result = from.can_jump(player_number, king, &board);
        assert_eq!(result, true);

        let destinations = from.jump_destinations(player_number, king, &board);
        assert_eq!(destinations.len(), 1);

        let square = &destinations[0];
        assert_eq!(square.x, 2);
        assert_eq!(square.y, 6);
    }

    #[test]
    fn pieces_can_move() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 1, king: false };
        let to = Square { id: 2, x: 5, y: 5, player_number: 0, king: false };
        let cant_to = Square { id: 4, x: 3, y: 5, player_number: 2, king: false };
        let board = SquareSet { squares: vec![from, to, cant_to] };

        let result = from.can_move(player_number, king, &board);
        assert_eq!(result, true);

        let destinations = from.move_destinations(player_number, king, &board);
        assert_eq!(destinations.len(), 1);

        let square = &destinations[0];
        assert_eq!(square.x, 5);
        assert_eq!(square.y, 5);
    }

    #[test]
    fn pieces_cannot_jump_over_friendly() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between = Square { id: 2, x: 3, y: 5, player_number: 1, king: false };
        let to = Square { id: 3, x: 2, y: 6, player_number: 0, king: false };
        let board = SquareSet { squares: vec![from, between, to] };

        let result = from.can_jump(player_number, king, &board);
        assert_eq!(result, false);

        let destinations = from.jump_destinations(player_number, king, &board);
        assert_eq!(destinations.len(), 0);
    }

    #[test]
    fn pieces_cannot_jump_over_empty() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between = Square { id: 2, x: 3, y: 5, player_number: 0, king: false };
        let to = Square { id: 3, x: 2, y: 6, player_number: 0, king: false };
        let board = SquareSet { squares: vec![from, between, to] };

        let result = from.can_jump(player_number, king, &board);
        assert_eq!(result, false);

        let destinations = from.jump_destinations(player_number, king, &board);
        assert_eq!(destinations.len(), 0);
    }

    #[test]
    fn pieces_cannot_jump_backwards() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between = Square { id: 2, x: 3, y: 3, player_number: 2, king: false };
        let to = Square { id: 3, x: 2, y: 2, player_number: 0, king: false };
        let board = SquareSet { squares: vec![from, between, to] };

        let result = from.can_jump(player_number, king, &board);
        assert_eq!(result, false);

        let destinations = from.jump_destinations(player_number, king, &board);
        assert_eq!(destinations.len(), 0);
    }

    #[test]
    fn fetch_jump_legs() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 3, y: 3, player_number: 1, king: false };
        let over_a = Square { id: 2, x: 4, y: 4, player_number: 2, king: false };
        let to_a = Square { id: 3, x: 5, y: 5, player_number: 0, king: false };
        let over_aa = Square { id: 4, x: 6, y: 6, player_number: 2, king: false };
        let to_aa = Square { id: 5, x: 7, y: 7, player_number: 0, king: false };

        let over_b = Square { id: 6, x: 2, y: 4, player_number: 2, king: false };
        let to_b = Square { id: 7, x: 1, y: 5, player_number: 0, king: false };
        let square_set = SquareSet { squares: vec![from, over_a, to_a, over_aa, to_aa, over_b, to_b] };
        let mut accumulator = vec![];
        let mut current_leg = vec![];
        let result = from.jump_legs(player_number, king, &square_set, &mut accumulator, &mut current_leg);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![1,3,5]);
        assert_eq!(result[1], vec![1,7]);
    }

    #[test]
    fn fetch_branching_jump_legs() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 3, y: 3, player_number: 1, king: false };
        let over_a = Square { id: 2, x: 4, y: 4, player_number: 2, king: false };
        let to_a = Square { id: 3, x: 5, y: 5, player_number: 0, king: false };
        let over_aa = Square { id: 4, x: 6, y: 6, player_number: 2, king: false };
        let to_aa = Square { id: 5, x: 7, y: 7, player_number: 0, king: false };

        let over_b = Square { id: 6, x: 4, y: 6, player_number: 2, king: false };
        let to_b = Square { id: 7, x: 3, y: 7, player_number: 0, king: false };
        let square_set = SquareSet { squares: vec![from, over_a, to_a, over_aa, to_aa, over_b, to_b] };
        let mut accumulator = vec![];
        let mut current_leg = vec![];
        let result = from.jump_legs(player_number, king, &square_set, &mut accumulator, &mut current_leg);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![1,3,5]);
        assert_eq!(result[1], vec![1,3,7]);
    }

    #[test]
    fn fetch_jumps_test() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 3, y: 3, player_number: 1, king: false };
        let over_a = Square { id: 2, x: 4, y: 4, player_number: 2, king: false };
        let to_a = Square { id: 3, x: 5, y: 5, player_number: 0, king: false };
        let over_aa = Square { id: 4, x: 6, y: 6, player_number: 2, king: false };
        let to_aa = Square { id: 5, x: 7, y: 7, player_number: 0, king: false };

        let over_b = Square { id: 6, x: 2, y: 4, player_number: 2, king: false };
        let to_b = Square { id: 7, x: 1, y: 5, player_number: 0, king: false };
        let square_set = SquareSet { squares: vec![from, over_a, to_a, over_aa, to_aa, over_b, to_b] };
        let result = from.jumps(player_number, king, &square_set);
        assert_eq!(result[0].from, 1);
        assert_eq!(result[0].to, vec![3, 5]);
        assert_eq!(result[1].from, 1);
        assert_eq!(result[1].to, vec![7]);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn fetch_branching_jumps_test() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 3, y: 3, player_number: 1, king: false };
        let over_a = Square { id: 2, x: 4, y: 4, player_number: 2, king: false };
        let to_a = Square { id: 3, x: 5, y: 5, player_number: 0, king: false };
        let over_aa = Square { id: 4, x: 6, y: 6, player_number: 2, king: false };
        let to_aa = Square { id: 5, x: 7, y: 7, player_number: 0, king: false };

        let over_b = Square { id: 6, x: 4, y: 6, player_number: 2, king: false };
        let to_b = Square { id: 7, x: 3, y: 7, player_number: 0, king: false };
        let square_set = SquareSet { squares: vec![from, over_a, to_a, over_aa, to_aa, over_b, to_b] };
        let result = from.jumps(player_number, king, &square_set);

        assert_eq!(result[0].from, 1);
        assert_eq!(result[0].to, vec![3, 5]);

        assert_eq!(result[1].from, 1);
        assert_eq!(result[1].to, vec![3, 7]);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn fetch_moves() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 1, king: false };
        let to = Square { id: 2, x: 5, y: 5, player_number: 0, king: false };
        let cant_to = Square { id: 4, x: 3, y: 5, player_number: 2, king: false };
        let board = SquareSet { squares: vec![from, to, cant_to] };

        let result = from.moves(player_number, king, &board);
        assert_eq!(result[0].from, 1);
        assert_eq!(result[0].to, vec![2]);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn promote_piece() {
        let mut square = Square { id: 1, x: 4, y: 4, player_number: 1, king: false };
        match square.promote() {
            Ok(_) => assert!(square.king),
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn demote_piece() {
        let mut square = Square { id: 1, x: 4, y: 4, player_number: 1, king: true };
        match square.demote() {
            Ok(_) => assert_eq!(square.king, false),
            Err(e) => assert!(false, "{}", e),
        }
    }
}
