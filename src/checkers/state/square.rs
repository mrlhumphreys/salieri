use crate::checkers::state::point::potential_jump_points;
use crate::checkers::state::point::potential_move_points;
use crate::checkers::state::square_set::find_by_x_and_y;
use crate::checkers::state::square_set::between_point;
use crate::checkers::state::game_state::GameState;
use crate::checkers::state::mov::Move;
use crate::checkers::state::mov::MoveKind;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pub player_number: i8,
    pub king: bool
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

    pub fn can_jump(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &GameState) -> bool {
        let potential_destinations = potential_jump_points(point, player_number, king);

        potential_destinations.iter().any(|p| {
            if let Some(to) = find_by_x_and_y(&game_state.squares, p.0, p.1) {
                if let Some(point) = between_point(point, *p) {
                    let between_occupied_by_opponent = match find_by_x_and_y(&game_state.squares, point.0, point.1) {
                        Some(b) => b.occupied_by_opponent(player_number),
                        None => false
                    };
                    to.unoccupied() && between_occupied_by_opponent
                } else {
                    false
                }
            } else {
                false
            }
        })
    }

    pub fn can_move(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &GameState) -> bool {
        let potential_destinations = potential_move_points(point, player_number, king);
        potential_destinations.iter().any(|p| {
            match find_by_x_and_y(&game_state.squares, p.0, p.1) {
                Some(to) => to.unoccupied(),
                None => false
            }
        })
    }

    pub fn jump_destinations<'a>(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &'a GameState) -> Vec<(i8, i8)> {
        let mut destinations = vec![];
        let potential_destinations = potential_jump_points(point, player_number, king);

        potential_destinations.iter().for_each(|p| {
            if let Some(to) = find_by_x_and_y(&game_state.squares, p.0, p.1) {
                if to.unoccupied() {
                    if let Some(point) = between_point(point, *p) {
                        if let Some(b) = find_by_x_and_y(&game_state.squares, point.0, point.1) {
                            if b.occupied_by_opponent(player_number) {
                                destinations.push(*p);
                            }
                        }
                    }
                }
            }
        });

        destinations
    }

    pub fn move_destinations<'a>(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &'a GameState) -> Vec<(i8, i8)> {
        let mut destinations = vec![];
        let potential_destinations = potential_move_points(point, player_number, king);

        potential_destinations.iter().for_each(|p| {
            if let Some(to) = find_by_x_and_y(&game_state.squares, p.0, p.1) {
                if to.unoccupied() {
                    destinations.push(*p);
                }
            }
        });

        destinations
    }

    pub fn jump_legs<'a>(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &GameState, mut accumulator: &'a mut Vec<Vec<(i8, i8)>>, mut current_leg: &mut Vec<(i8, i8)>) -> &'a mut Vec<Vec<(i8, i8)>> {
        let destinations = self.jump_destinations(point, player_number, king, game_state);

        if !destinations.is_empty() {
            for destination in destinations.iter() {

                if current_leg.is_empty() {
                    current_leg.push(point);
                }

                current_leg.push(*destination);

                let mut new_game_state = game_state.clone();
                match new_game_state.perform_move_leg(point, *destination) {
                    Ok(_) => {
                        match find_by_x_and_y(&new_game_state.squares, destination.0, destination.1) {
                            Some(destination_square) => {
                                destination_square.jump_legs(*destination, player_number, king, &new_game_state, &mut accumulator, &mut current_leg);
                            },
                            None => ()
                        }
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

    pub fn jumps(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &GameState) -> Vec<Move> {
        let mut accumulator = vec![];
        let mut current_leg = vec![];
        let all_legs = self.jump_legs(point, player_number, king, &game_state, &mut accumulator, &mut current_leg);
        all_legs.iter().map(|l| {
            let from_id = l[0];
            let to_ids = l[1..].to_vec();
            Move { kind: MoveKind::Jump, from: from_id, to: to_ids }
        }).collect()
    }

    pub fn moves(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &GameState) -> Vec<Move> {
        let destinations = self.move_destinations(point, player_number, king, &game_state);
        destinations.iter().map(|d| {
            Move { kind: MoveKind::Mov, from: point, to: vec![*d] }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupied_by_player_own_player() {
        let square = Square { player_number: 1, king: false };
        assert_eq!(square.occupied_by_player(1), true);
        assert_eq!(square.occupied_by_opponent(1), false);
        assert_eq!(square.unoccupied(), false);
    }

    #[test]
    fn occupied_by_player_other_player() {
        let square = Square { player_number: 2, king: false };
        assert_eq!(square.occupied_by_player(1), false);
        assert_eq!(square.occupied_by_opponent(1), true);
        assert_eq!(square.unoccupied(), false);
    }

    #[test]
    fn occupied_by_player_unoccupied() {
        let square = Square { player_number: 0, king: false };
        assert_eq!(square.occupied_by_player(1), false);
        assert_eq!(square.occupied_by_opponent(1), false);
        assert_eq!(square.unoccupied(), true);
    }

    #[test]
    fn pieces_can_jump() {
        let (player_number, king) = (2, false);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (5, 4);
        let from_square = game_state.squares[4][5];
        let result = from_square.can_jump(point, player_number, king, &game_state);
        assert_eq!(result, true);
        let destinations = from_square.jump_destinations(point, player_number, king, &game_state);
        assert_eq!(destinations.len(), 1);
        let square = &destinations[0];
        assert_eq!(*square, (3, 6));
    }

    #[test]
    fn pieces_can_move() {
        let (player_number, king) = (2, false);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (5, 4);
        let from_square = game_state.squares[4][5];
        let result = from_square.can_move(point, player_number, king, &game_state);
        assert_eq!(result, true);
        let destinations = from_square.move_destinations(point, player_number, king, &game_state);
        assert_eq!(destinations.len(), 1);
        let square = &destinations[0];
        assert_eq!(*square, (6, 5));
    }

    #[test]
    fn pieces_cannot_jump_over_friendly() {
        let (player_number, king) = (1, false);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (5, 4);
        let from_square = game_state.squares[4][5];
        let result = from_square.can_jump(point, player_number, king, &game_state);
        assert_eq!(result, false);
        let destinations = from_square.jump_destinations(point, player_number, king, &game_state);
        assert_eq!(destinations.len(), 0);
    }

    #[test]
    fn pieces_cannot_jump_over_empty() {
        let (player_number, king) = (1, false);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (5, 4);
        let from_square = game_state.squares[4][5];
        let result = from_square.can_jump(point, player_number, king, &game_state);
        assert_eq!(result, false);
        let destinations = from_square.jump_destinations(point, player_number, king, &game_state);
        assert_eq!(destinations.len(), 0);
    }

    #[test]
    fn pieces_cannot_jump_backwards() {
        let (player_number, king) = (1, false);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (5, 4);
        let from_square = game_state.squares[4][5];
        let result = from_square.can_jump(point, player_number, king, &game_state);
        assert_eq!(result, false);
        let destinations = from_square.jump_destinations(point, player_number, king, &game_state);
        assert_eq!(destinations.len(), 0);
    }

    #[test]
    fn fetch_jump_legs() {
        let (player_number, king) = (2, false);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let mut accumulator = vec![];
        let mut current_leg = vec![];

        let point = (2, 3);
        let from_square = game_state.squares[3][2];
        let result = from_square.jump_legs(point, player_number, king, &game_state, &mut accumulator, &mut current_leg);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![(2, 3), (0, 5)]);
        assert_eq!(result[1], vec![(2, 3), (4, 5), (6, 7)]);
    }

    #[test]
    fn fetch_branching_jump_legs() {
        let (player_number, king) = (2, false);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];

        let game_state = GameState { current_player_number: 2, squares };


        let mut accumulator = vec![];
        let mut current_leg = vec![];
        let point = (2, 3);
        let from_square = game_state.squares[3][2];
        let result = from_square.jump_legs(point, player_number, king, &game_state, &mut accumulator, &mut current_leg);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![(2, 3), (4, 5), (2, 7)]);
        assert_eq!(result[1], vec![(2, 3), (4, 5), (6, 7)]);
    }

    #[test]
    fn fetch_jumps_test() {
        let (player_number, king) = (2, false);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (2, 3);
        let from_square = game_state.squares[3][2];
        let result = from_square.jumps(point, player_number, king, &game_state);
        assert_eq!(result[0].from, (2, 3));
        assert_eq!(result[0].to, vec![(0, 5)]);
        assert_eq!(result[1].from, (2, 3));
        assert_eq!(result[1].to, vec![(4, 5), (6, 7)]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn fetch_branching_jumps_test() {
        let (player_number, king) = (2, false);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 2, squares };

        let point = (2, 3);
        let from_square = game_state.squares[3][2];
        let result = from_square.jumps(point, player_number, king, &game_state);
        assert_eq!(result[0].from, (2, 3));
        assert_eq!(result[0].to, vec![(4, 5), (2, 7)]);
        assert_eq!(result[1].from, (2, 3));
        assert_eq!(result[1].to, vec![(4, 5), (6, 7)]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn fetch_moves() {
        let (player_number, king) = (2, false);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (5, 4);
        let from_square = game_state.squares[4][5];
        let result = from_square.moves(point, player_number, king, &game_state);
        assert_eq!(result[0].from, (5, 4));
        assert_eq!(result[0].to, vec![(6, 5)]);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn promote_piece() {
        let mut square = Square { player_number: 1, king: false };
        match square.promote() {
            Ok(_) => assert!(square.king),
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn demote_piece() {
        let mut square = Square { player_number: 1, king: true };
        match square.demote() {
            Ok(_) => assert_eq!(square.king, false),
            Err(e) => assert!(false, "{}", e),
        }
    }
}
