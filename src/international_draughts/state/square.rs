use crate::international_draughts::state::point::potential_jump_points;
use crate::international_draughts::state::point::potential_move_points;
use crate::international_draughts::state::square_set::find_by_x_and_y;
use crate::international_draughts::state::square_set::between_points;
use crate::international_draughts::state::game_state::GameState;
use crate::international_draughts::state::mov::Move;
use crate::international_draughts::state::mov::MoveKind;

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
        let potential_destinations = potential_jump_points(point, king);

        potential_destinations.iter().any(|p| {
            if let Some(to) = find_by_x_and_y(&game_state.squares, p.0, p.1) {
                let occupied_by_opponent_count = between_points(point, *p).iter().fold(0, |acc, between| {
                    if let Some(b) = find_by_x_and_y(&game_state.squares, between.0, between.1) {
                        if b.occupied_by_opponent(player_number) {
                            acc + 1
                        } else {
                            acc
                        }
                    } else {
                        acc
                    }
                });

                let occupied_count = between_points(point, *p).iter().fold(0, |acc, between| {
                    if let Some(b) = find_by_x_and_y(&game_state.squares, between.0, between.1) {
                        if b.occupied() {
                            acc + 1
                        } else {
                            acc
                        }
                    } else {
                        acc
                    }
                });

                to.unoccupied() && occupied_count == 1 && occupied_by_opponent_count == 1
            } else {
                false
            }
        })
    }

    pub fn can_move(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &GameState) -> bool {
        let potential_destinations = potential_move_points(point, player_number, king);
        potential_destinations.iter().any(|p| {
            match find_by_x_and_y(&game_state.squares, p.0, p.1) {
                Some(to) => {
                    let occupied_count = between_points(point, *p).iter().fold(0, |acc, between| {
                        if let Some(b) = find_by_x_and_y(&game_state.squares, between.0, between.1) {
                            if b.occupied() {
                                acc + 1
                            } else {
                                acc
                            }
                        } else {
                            acc
                        }
                    });
                    to.unoccupied() && occupied_count == 0
                },
                None => false
            }
        })
    }

    pub fn move_destinations<'a>(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &'a GameState) -> Vec<(i8, i8)> {
        let mut destinations = vec![];
        let potential_destinations = potential_move_points(point, player_number, king);

        potential_destinations.iter().for_each(|p| {
            if let Some(to) = find_by_x_and_y(&game_state.squares, p.0, p.1) {
                let occupied_count = between_points(point, *p).iter().fold(0, |acc, between| {
                    if let Some(b) = find_by_x_and_y(&game_state.squares, between.0, between.1) {
                        if b.occupied() {
                            acc + 1
                        } else {
                            acc
                        }
                    } else {
                        acc
                    }
                });
                if to.unoccupied() && occupied_count == 0 {
                    destinations.push(*p);
                }
            }
        });

        destinations
    }

    // legs can be:
    // a ab b
    // a ac c ce e
    // from_id capture_id_1 to_id_1 capture_id_2 to_id_2
    pub fn jump_legs<'a>(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &GameState, mut accumulator: &'a mut Vec<Vec<(i8, i8)>>, mut current_leg: &mut Vec<(i8, i8)>) -> &'a mut Vec<Vec<(i8, i8)>> {
        let potential_destinations = potential_jump_points(point, king);

        let mut destinations: Vec<Vec<(i8, i8)>> = vec![];
        potential_destinations.iter().for_each(|p| {
            if let Some(to) = find_by_x_and_y(&game_state.squares, p.0, p.1) {
                if to.unoccupied() {
                    let occupied_by_opponent: Vec<(i8, i8)> = between_points(point, *p).iter().fold(vec![], |mut acc, between| {
                        if let Some(found_square) = find_by_x_and_y(&game_state.squares, between.0, between.1) {

                            if found_square.occupied_by_opponent(player_number) {
                                acc.push(*between);
                                acc
                            } else {
                                acc
                            }
                        } else {
                            acc
                        }
                    });
                    if occupied_by_opponent.len() == 1 {
                        destinations.push(vec![occupied_by_opponent[0], *p]);
                    }
                }
            }
        });

        if !destinations.is_empty() {
            for destination in destinations.iter() {

                if current_leg.is_empty() {
                    current_leg.push(point); // from
                }

                current_leg.push(destination[0]); // capture
                current_leg.push(destination[1]); // to

                let mut new_game_state = game_state.clone();
                match new_game_state.perform_move_leg(point, destination[1], Some(destination[0])) {
                    Ok(_) => {
                        match find_by_x_and_y(&new_game_state.squares, destination[1].0, destination[1].1) {
                            Some(destination_square) => {
                                destination_square.jump_legs(destination[1], player_number, king, &new_game_state, &mut accumulator, &mut current_leg);
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

        // pop the last 2 element of the leg before we return and go back up the tree
        current_leg.pop(); // to
        current_leg.pop(); // destination
        accumulator
    }

    pub fn jumps(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &GameState) -> Vec<Move> {
        let mut accumulator = vec![];
        let mut current_leg = vec![];
        let all_legs = self.jump_legs(point, player_number, king, &game_state, &mut accumulator, &mut current_leg);
        all_legs.iter().map(|l| {
            // a ab b
            // a ac c cd d
            // from_id capture_id_1 to_id_1 capture_id_2 to_id_2
            let from_id = l[0];
            let capture_ids: Vec<(i8, i8)> = l.iter().skip(1).step_by(2).fold(vec![], |mut acc, e| {
                acc.push(*e);
                acc
            });
            let to_ids: Vec<(i8, i8)> = l.iter().skip(2).step_by(2).fold(vec![], |mut acc, e| {
                acc.push(*e);
                acc
            });
            Move {
                kind: MoveKind::Jump,
                from: from_id,
                captures: capture_ids,
                to: to_ids
            }
        }).collect()
    }

    pub fn moves(&self, point: (i8, i8), player_number: i8, king: bool, game_state: &GameState) -> Vec<Move> {
        let destinations = self.move_destinations(point, player_number, king, &game_state);
        destinations.iter().map(|d| {
            Move {
                kind: MoveKind::Mov,
                from: point,
                captures: vec![],
                to: vec![*d]
            }
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
    }

    #[test]
    fn can_jump_flying_king_test() {
        let (player_number, king) = (1, true);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
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
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
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
                Square { player_number: 1, king: true },
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
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (6, 7);
        let from_square = game_state.squares[7][6];
        let result = from_square.can_jump(point, player_number, king, &game_state);
        assert_eq!(result, true);
    }

    #[test]
    fn can_jump_flying_king_over_multiple_own_test() {
        let (player_number, king) = (1, true);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
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
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
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
                Square { player_number: 1, king: true },
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
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (6, 7);
        let from_square = game_state.squares[7][6];
        let result = from_square.can_jump(point, player_number, king, &game_state);
        assert_eq!(result, false);
    }

    #[test]
    fn can_jump_flying_king_over_multiple_opponent_test() {
        let (player_number, king) = (1, true);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
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
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
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
                Square { player_number: 1, king: true },
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
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (6, 7);
        let from_square = game_state.squares[7][6];
        let result = from_square.can_jump(point, player_number, king, &game_state);
        assert_eq!(result, false);
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
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: true },
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
    fn move_destinations_flying_king_test() {
        let (player_number, king) = (1, true);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
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
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
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
                Square { player_number: 1, king: true },
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
                Square { player_number: 2, king: false },
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
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (4, 5);
        let from_square = game_state.squares[5][4];
        let destinations = from_square.move_destinations(point, player_number, king, &game_state);
        let expected = vec![
            (5, 4),
            (5, 6),
            (6, 3),
            (6, 7),
            (7, 2),
            (7, 8),
            (8, 1),
            (8, 9),
            (9, 0)
        ];
        assert_eq!(destinations, expected);
    }

    #[test]
    fn can_move_flying_king_over_own_pieces_test() {
        let (player_number, king) = (1, true);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
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
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
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
                Square { player_number: 1, king: true },
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
                Square { player_number: 1, king: false },
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
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (6, 7);
        let from_square = game_state.squares[7][6];
        let result = from_square.can_move(point, player_number, king, &game_state);
        assert_eq!(result, false);
    }

    #[test]
    fn can_move_flying_king_over_opponent_pieces_test() {
        let (player_number, king) = (1, true);
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
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
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
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
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
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
                Square { player_number: 2, king: false },
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
                Square { player_number: 1, king: true },
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
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let game_state = GameState { current_player_number: 1, squares };

        let point = (6, 7);
        let from_square = game_state.squares[7][6];
        let result = from_square.can_move(point, player_number, king, &game_state);
        assert_eq!(result, false);
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
        assert_eq!(result[0], vec![(2, 3), (1, 4), (0, 5)]);
        assert_eq!(result[1], vec![(2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 6), (8, 5)]);
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
        assert_eq!(result[0], vec![(2, 3), (3, 4), (4, 5), (3, 6), (2, 7)]);
        assert_eq!(result[1], vec![(2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 6), (8, 5)]);
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
        assert_eq!(result[1].to, vec![(4, 5), (6, 7), (8, 5)]);
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
        assert_eq!(result[1].to, vec![(4, 5), (6, 7), (8, 5)]);
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
