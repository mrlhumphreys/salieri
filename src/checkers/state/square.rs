use crate::checkers::state::point::Point;
use crate::checkers::state::square_set::between;
use crate::checkers::state::game_state::GameState;
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
        Point { x: self.x, y: self.y, }
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

    pub fn potential_jump_points(&self, player_number: i8, king: bool) -> Vec<Point> {
        if king {
            vec![
                Point { x: self.x - 2, y: self.y - 2 },
                Point { x: self.x - 2, y: self.y + 2 },
                Point { x: self.x + 2, y: self.y - 2 },
                Point { x: self.x + 2, y: self.y + 2 }
            ]
        } else {
            if player_number == 2 {
                vec![
                    Point { x: self.x - 2, y: self.y + 2 },
                    Point { x: self.x + 2, y: self.y + 2 }
                ]
            } else {
                vec![
                    Point { x: self.x - 2, y: self.y - 2 },
                    Point { x: self.x + 2, y: self.y - 2 }
                ]
            }
        }
    }

    pub fn potential_move_points(&self, player_number: i8, king: bool) -> Vec<Point> {
        if king {
            vec![
                Point { x: self.x - 1, y: self.y - 1 },
                Point { x: self.x - 1, y: self.y + 1 },
                Point { x: self.x + 1, y: self.y - 1 },
                Point { x: self.x + 1, y: self.y + 1 }
            ]
        } else {
            if player_number == 2 {
                vec![
                    Point { x: self.x - 1, y: self.y + 1 },
                    Point { x: self.x + 1, y: self.y + 1 }
                ]
            } else {
                vec![
                    Point { x: self.x - 1, y: self.y - 1 },
                    Point { x: self.x + 1, y: self.y - 1 }
                ]
            }
        }
    }

    pub fn can_jump(&self, player_number: i8, king: bool, game_state: &GameState) -> bool {
        let potential_destinations = self.potential_jump_points(player_number, king);

        game_state.squares.iter().any(|to| {
            potential_destinations.iter().any(|p| p.x == to.x && p.y == to.y) &&
                to.unoccupied() &&
                match between(&game_state.squares, self.point(), to.point()) {
                    Some(b) => b.occupied_by_opponent(player_number),
                    None => false,
                }
        })
    }

    pub fn can_move(&self, player_number: i8, king: bool, game_state: &GameState) -> bool {
        let potential_destinations = self.potential_move_points(player_number, king);
        game_state.squares.iter().any(|to| {
            potential_destinations.iter().any(|p| p.x == to.x && p.y == to.y) && to.unoccupied()
        })
    }

    pub fn jump_destinations<'a>(&self, player_number: i8, king: bool, game_state: &'a GameState) -> Vec<&'a Square> {
        let potential_destinations = self.potential_jump_points(player_number, king);
        game_state.squares.iter().filter(|to| {
            potential_destinations.iter().any(|p| p.x == to.x && p.y == to.y) &&
                to.unoccupied() &&
                match between(&game_state.squares, self.point(), to.point()) {
                    Some(b) => b.occupied_by_opponent(player_number),
                    None => false,
                }
        }).collect()
    }

    pub fn move_destinations<'a>(&self, player_number: i8, king: bool, game_state: &'a GameState) -> Vec<&'a Square> {
        let potential_destinations = self.potential_move_points(player_number, king);
        game_state.squares.iter().filter(|to| {
            potential_destinations.iter().any(|p| p.x == to.x && p.y == to.y) && to.unoccupied()
        }).collect()
    }

    pub fn jump_legs<'a>(&self, player_number: i8, king: bool, game_state: &GameState, mut accumulator: &'a mut Vec<Vec<i8>>, mut current_leg: &mut Vec<i8>) -> &'a mut Vec<Vec<i8>> {
        let destinations = self.jump_destinations(player_number, king, game_state);

        if !destinations.is_empty() {
            for destination in destinations.iter() {

                if current_leg.is_empty() {
                    current_leg.push(self.id);
                }

                current_leg.push(destination.id);

                let mut new_game_state = game_state.clone();
                match new_game_state.perform_move_leg(self.id, destination.id) {
                    Ok(_) => {
                        destination.jump_legs(player_number, king, &new_game_state, &mut accumulator, &mut current_leg);
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

    pub fn jumps(&self, player_number: i8, king: bool, game_state: &GameState) -> Vec<Move> {
        let mut accumulator = vec![];
        let mut current_leg = vec![];
        let all_legs = self.jump_legs(player_number, king, &game_state, &mut accumulator, &mut current_leg);
        all_legs.iter().map(|l| {
            let from_id = l[0];
            let to_ids = l[1..].to_vec();
            Move { kind: MoveKind::Jump, from: from_id, to: to_ids }
        }).collect()
    }

    pub fn moves(&self, player_number: i8, king: bool, game_state: &GameState) -> Vec<Move> {
        let destinations = self.move_destinations(player_number, king, &game_state);
        destinations.iter().map(|d| {
            Move { kind: MoveKind::Mov, from: self.id, to: vec![d.id] }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn potential_jump_points_king_test() {
        let player_number = 1;
        let king = true;
        let square = Square { id: 1, x: 4, y: 4, player_number, king };
        let expected = vec![
            Point { x: 2, y: 2 },
            Point { x: 2, y: 6 },
            Point { x: 6, y: 2 },
            Point { x: 6, y: 6 }
        ];
        let result = square.potential_jump_points(player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn potential_jump_points_player_one_test() {
        let player_number = 1;
        let king = false;
        let square = Square { id: 1, x: 4, y: 4, player_number, king };
        let expected = vec![
            Point { x: 2, y: 2 },
            Point { x: 6, y: 2 },
        ];
        let result = square.potential_jump_points(player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn potential_jump_points_player_two_test() {
        let player_number = 2;
        let king = false;
        let square = Square { id: 1, x: 4, y: 4, player_number, king };
        let expected = vec![
            Point { x: 2, y: 6 },
            Point { x: 6, y: 6 },
        ];
        let result = square.potential_jump_points(player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn potential_move_points_king_test() {
        let player_number = 1;
        let king = true;
        let square = Square { id: 1, x: 4, y: 4, player_number, king };
        let expected = vec![
            Point { x: 3, y: 3 },
            Point { x: 3, y: 5 },
            Point { x: 5, y: 3 },
            Point { x: 5, y: 5 }
        ];
        let result = square.potential_move_points(player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn potential_move_points_player_one_test() {
        let player_number = 1;
        let king = false;
        let square = Square { id: 1, x: 4, y: 4, player_number, king };
        let expected = vec![
            Point { x: 3, y: 3 },
            Point { x: 5, y: 3 }
        ];
        let result = square.potential_move_points(player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn potential_move_points_player_two_test() {
        let player_number = 2;
        let king = false;
        let square = Square { id: 1, x: 4, y: 4, player_number, king };
        let expected = vec![
            Point { x: 3, y: 5 },
            Point { x: 5, y: 5 }
        ];
        let result = square.potential_move_points(player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn pieces_can_jump() {
        let (player_number, king) = (2, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between_square = Square { id: 2, x: 3, y: 5, player_number: 1, king: false };
        let to = Square { id: 3, x: 2, y: 6, player_number: 0, king: false };
        let squares = vec![from, between_square, to];
        let game_state = GameState { current_player_number: 1, squares };

        let result = from.can_jump(player_number, king, &game_state);
        assert_eq!(result, true);

        let destinations = from.jump_destinations(player_number, king, &game_state);
        assert_eq!(destinations.len(), 1);

        let square = &destinations[0];
        assert_eq!(square.x, 2);
        assert_eq!(square.y, 6);
    }

    #[test]
    fn pieces_can_move() {
        let (player_number, king) = (2, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 2, king: false };
        let to = Square { id: 2, x: 5, y: 5, player_number: 0, king: false };
        let cant_to = Square { id: 4, x: 3, y: 5, player_number: 1, king: false };
        let squares = vec![from, to, cant_to];
        let game_state = GameState { current_player_number: 1, squares };

        let result = from.can_move(player_number, king, &game_state);
        assert_eq!(result, true);

        let destinations = from.move_destinations(player_number, king, &game_state);
        assert_eq!(destinations.len(), 1);

        let square = &destinations[0];
        assert_eq!(square.x, 5);
        assert_eq!(square.y, 5);
    }

    #[test]
    fn pieces_cannot_jump_over_friendly() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between_square = Square { id: 2, x: 3, y: 5, player_number: 1, king: false };
        let to = Square { id: 3, x: 2, y: 6, player_number: 0, king: false };
        let squares = vec![from, between_square, to];
        let game_state = GameState { current_player_number: 1, squares };

        let result = from.can_jump(player_number, king, &game_state);
        assert_eq!(result, false);

        let destinations = from.jump_destinations(player_number, king, &game_state);
        assert_eq!(destinations.len(), 0);
    }

    #[test]
    fn pieces_cannot_jump_over_empty() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between_square = Square { id: 2, x: 3, y: 5, player_number: 0, king: false };
        let to = Square { id: 3, x: 2, y: 6, player_number: 0, king: false };
        let squares = vec![from, between_square, to];
        let game_state = GameState { current_player_number: 1, squares };

        let result = from.can_jump(player_number, king, &game_state);
        assert_eq!(result, false);

        let destinations = from.jump_destinations(player_number, king, &game_state);
        assert_eq!(destinations.len(), 0);
    }

    #[test]
    fn pieces_cannot_jump_backwards() {
        let (player_number, king) = (1, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 0, king: false };
        let between_square = Square { id: 2, x: 3, y: 3, player_number: 1, king: false };
        let to = Square { id: 3, x: 2, y: 2, player_number: 0, king: false };
        let squares = vec![from, between_square, to];
        let game_state = GameState { current_player_number: 1, squares };

        let result = from.can_jump(player_number, king, &game_state);
        assert_eq!(result, false);

        let destinations = from.jump_destinations(player_number, king, &game_state);
        assert_eq!(destinations.len(), 0);
    }

    #[test]
    fn fetch_jump_legs() {
        let (player_number, king) = (2, false);
        let from = Square { id: 1, x: 3, y: 3, player_number: 2, king: false };
        let over_a = Square { id: 2, x: 4, y: 4, player_number: 1, king: false };
        let to_a = Square { id: 3, x: 5, y: 5, player_number: 0, king: false };
        let over_aa = Square { id: 4, x: 6, y: 6, player_number: 1, king: false };
        let to_aa = Square { id: 5, x: 7, y: 7, player_number: 0, king: false };

        let over_b = Square { id: 6, x: 2, y: 4, player_number: 1, king: false };
        let to_b = Square { id: 7, x: 1, y: 5, player_number: 0, king: false };
        let squares = vec![from, over_a, to_a, over_aa, to_aa, over_b, to_b];
        let game_state = GameState { current_player_number: 1, squares };

        let mut accumulator = vec![];
        let mut current_leg = vec![];

        let result = from.jump_legs(player_number, king, &game_state, &mut accumulator, &mut current_leg);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![1,3,5]);
        assert_eq!(result[1], vec![1,7]);
    }

    #[test]
    fn fetch_branching_jump_legs() {
        let (player_number, king) = (2, false);
        let from = Square { id: 1, x: 3, y: 3, player_number: 2, king: false };
        let over_a = Square { id: 2, x: 4, y: 4, player_number: 1, king: false };
        let to_a = Square { id: 3, x: 5, y: 5, player_number: 0, king: false };
        let over_aa = Square { id: 4, x: 6, y: 6, player_number: 1, king: false };
        let to_aa = Square { id: 5, x: 7, y: 7, player_number: 0, king: false };

        let over_b = Square { id: 6, x: 4, y: 6, player_number: 1, king: false };
        let to_b = Square { id: 7, x: 3, y: 7, player_number: 0, king: false };
        let squares = vec![from, over_a, to_a, over_aa, to_aa, over_b, to_b];
        let game_state = GameState { current_player_number: 1, squares };

        let mut accumulator = vec![];
        let mut current_leg = vec![];

        let result = from.jump_legs(player_number, king, &game_state, &mut accumulator, &mut current_leg);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![1,3,5]);
        assert_eq!(result[1], vec![1,3,7]);
    }

    #[test]
    fn fetch_jumps_test() {
        let (player_number, king) = (2, false);
        let from = Square { id: 1, x: 3, y: 3, player_number: 2, king: false };
        let over_a = Square { id: 2, x: 4, y: 4, player_number: 1, king: false };
        let to_a = Square { id: 3, x: 5, y: 5, player_number: 0, king: false };
        let over_aa = Square { id: 4, x: 6, y: 6, player_number: 1, king: false };
        let to_aa = Square { id: 5, x: 7, y: 7, player_number: 0, king: false };

        let over_b = Square { id: 6, x: 2, y: 4, player_number: 1, king: false };
        let to_b = Square { id: 7, x: 1, y: 5, player_number: 0, king: false };
        let squares = vec![from, over_a, to_a, over_aa, to_aa, over_b, to_b];
        let game_state = GameState { current_player_number: 1, squares };

        let result = from.jumps(player_number, king, &game_state);

        assert_eq!(result[0].from, 1);
        assert_eq!(result[0].to, vec![3, 5]);
        assert_eq!(result[1].from, 1);
        assert_eq!(result[1].to, vec![7]);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn fetch_branching_jumps_test() {
        let (player_number, king) = (2, false);
        let from = Square { id: 1, x: 3, y: 3, player_number: 2, king: false };
        let over_a = Square { id: 2, x: 4, y: 4, player_number: 1, king: false };
        let to_a = Square { id: 3, x: 5, y: 5, player_number: 0, king: false };
        let over_aa = Square { id: 4, x: 6, y: 6, player_number: 1, king: false };
        let to_aa = Square { id: 5, x: 7, y: 7, player_number: 0, king: false };

        let over_b = Square { id: 6, x: 4, y: 6, player_number: 1, king: false };
        let to_b = Square { id: 7, x: 3, y: 7, player_number: 0, king: false };
        let squares = vec![from, over_a, to_a, over_aa, to_aa, over_b, to_b];
        let game_state = GameState { current_player_number: 1, squares };
        let result = from.jumps(player_number, king, &game_state);

        assert_eq!(result[0].from, 1);
        assert_eq!(result[0].to, vec![3, 5]);

        assert_eq!(result[1].from, 1);
        assert_eq!(result[1].to, vec![3, 7]);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn fetch_moves() {
        let (player_number, king) = (2, false);
        let from = Square { id: 1, x: 4, y: 4, player_number: 2, king: false };
        let to = Square { id: 2, x: 5, y: 5, player_number: 0, king: false };
        let cant_to = Square { id: 4, x: 3, y: 5, player_number: 1, king: false };
        let squares = vec![from, to, cant_to];
        let game_state = GameState { current_player_number: 1, squares };

        let result = from.moves(player_number, king, &game_state);
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
