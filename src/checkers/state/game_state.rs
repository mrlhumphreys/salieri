use crate::checkers::state::point::ID_COORDINATE_MAP;
use crate::checkers::state::square::Square;
use crate::checkers::state::square_set::find_by_x_and_y;
use crate::checkers::state::square_set::find_by_x_and_y_mut;
use crate::checkers::state::square_set::between_point;
use crate::checkers::state::mov::Move;

#[derive(PartialEq, Debug)]
pub struct GameState {
    pub current_player_number: i8,
    pub squares: Vec<Vec<Square>>,
}

impl Clone for GameState {
    fn clone(&self) -> GameState {
        GameState {
            current_player_number: self.current_player_number,
            squares: self.squares.clone(),
        }
    }
}

impl GameState {
    pub fn winner(&self) -> Option<i8> {
        if self.possible_moves_for_player(1).is_empty() {
            Some(2)
        } else if self.possible_moves_for_player(2).is_empty() {
            Some(1)
        } else {
            None
        }
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        self.possible_moves_for_player(self.current_player_number)
    }

    pub fn possible_moves_for_player(&self, player_number: i8) -> Vec<Move> {
        let jumps = self.jumps_for_player(player_number);
        if jumps.is_empty() {
            self.moves_for_player(player_number)
        } else {
            jumps
        }
    }

    pub fn jumps_for_player(&self, player_number: i8) -> Vec<Move> {
        let mut list = Vec::new();

        for (y, row) in self.squares.iter().enumerate() {
            for (x, from) in row.iter().enumerate() {
                let point = (x as i8, y as i8);
                if from.occupied_by_player(player_number) && from.can_jump(point, from.player_number, from.king, &self) {
                    list.append(&mut from.jumps(point, from.player_number, from.king, &self));
                }
            }
        }

        list
    }

    pub fn moves_for_player(&self, player_number: i8) -> Vec<Move> {
        let mut list = Vec::new();

        for (y, row) in self.squares.iter().enumerate() {
            for (x, from) in row.iter().enumerate() {
                let point = (x as i8, y as i8);
                if from.occupied_by_player(player_number) && from.can_move(point, from.player_number, from.king, &self) {
                    list.append(&mut from.moves(point, from.player_number, from.king, &self))
                }
            }
        }

        list
    }

    pub fn perform_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        let legs = mov.legs();

        for (origin, destination) in legs {
           self.perform_move_leg(origin, destination)?;
        }

        let (next_player_number, promotion_row) = match self.current_player_number {
            1 => (2, 0),
            2 => (1, 7),
            _ => return Err("invalid player number"),
        };

        if let Some(last_point) = mov.to.last() {
            let last_square = find_by_x_and_y(&self.squares, last_point.0, last_point.1);

            if last_square.is_some() {
                if promotion_row == last_point.1 {
                    self.promote(*last_point)?;
                }
            } else {
                return Err("invalid square id");
            }
        } else {
            return Err("no square id");
        };

        self.current_player_number = next_player_number;

        Ok(())
    }

    pub fn undo_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        let (previous_player_number, promotion_row) = match self.current_player_number {
            1 => (2, 0),
            2 => (1, 7),
            _ => return Err("invalid player number"),
        };

        if let Some(last_point) = mov.to.last() {
            let last_square = find_by_x_and_y(&self.squares, last_point.0, last_point.1);

            if last_square.is_some() {
                if promotion_row == last_point.1 {
                    self.demote(*last_point)?;
                }
            } else {
                return Err("invalid square id");
            }
        } else {
            return Err("no square id");
        };

        for (origin, destination) in mov.legs().into_iter().rev() {
           self.undo_move_leg(origin, destination)?;
        }

        self.current_player_number = previous_player_number;

        Ok(())
    }

    pub fn perform_move_leg(&mut self, from: (i8, i8), to: (i8, i8)) -> Result<(), &'static str> {
        let mut player_number: i8 = 0;
        let mut king: bool = false;

        if let Some(square) = find_by_x_and_y_mut(&mut self.squares, from.0, from.1) {
            if square.occupied() {
                player_number = square.player_number;
                king = square.king;
                square.player_number = 0;
                square.king = false;
            }
        }

        if let Some(square) = find_by_x_and_y_mut(&mut self.squares, to.0, to.1) {
            square.player_number = player_number;
            square.king = king;
        }

        if let Some(point) = between_point(from, to) {
            if let Some(square) = find_by_x_and_y_mut(&mut self.squares, point.0, point.1) {
                square.player_number = 0;
                square.king = false;
            }
        }

        Ok(())
    }

    pub fn undo_move_leg(&mut self, from: (i8, i8), to: (i8, i8)) -> Result<(), &'static str> {
        let mut player_number: i8 = 0;
        let mut king: bool = false;

        if let Some(square) = find_by_x_and_y_mut(&mut self.squares, to.0, to.1) {
            if square.occupied() {
                player_number = square.player_number;
                king = square.king;
                square.player_number = 0;
                square.king = false;
            }
        }

        if let Some(square) = find_by_x_and_y_mut(&mut self.squares, from.0, from.1) {
            square.player_number = player_number;
            square.king = king;
        }

        if let Some(point) = between_point(from, to) {
            if let Some(square) = find_by_x_and_y_mut(&mut self.squares, point.0, point.1) {
                square.player_number = match player_number {
                    2 => 1,
                    1 => 2,
                    _ => 0
                };
                square.king = false;
            }
        }

        Ok(())
    }

    pub fn promote(&mut self, point: (i8, i8)) -> Result<(), &'static str> {
        if let Some(s) = find_by_x_and_y_mut(&mut self.squares, point.0, point.1) {
            s.promote()?;
        }
        Ok(())
    }

    pub fn demote(&mut self, point: (i8, i8)) -> Result<(), &'static str> {
        if let Some(s) = find_by_x_and_y_mut(&mut self.squares, point.0, point.1) {
            s.demote()?;
        }
        Ok(())
    }
}

// B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12
pub fn parse(encoded: &String) -> Result<GameState, &'static str> {
    let mut read_player = true;
    let mut read_white_pieces = false;
    let mut read_black_pieces = false;
    let mut parse_error = false;
    let mut current_piece_king = false;
    let mut current_square_id = String::from("");
    let mut current_player_number = 1;
    let mut squares: Vec<Vec<Square>> = vec![
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

    for c in encoded.chars() {
        match c {
            'B' => {
                if read_player {
                   current_player_number = 1;
                } else {
                    read_white_pieces = false;
                    read_black_pieces = true;
                }
            },
            'W' => {
                if read_player {
                    current_player_number = 2;
                } else {
                    read_white_pieces = true;
                }
            },
            ':' => {
                if read_player {
                    read_player = false;
                } else if read_white_pieces || read_black_pieces {
                    if current_square_id != String::from("") {
                        let player_number = if read_white_pieces {
                            2
                        } else {
                            1
                        };
                        match parse_square(&current_square_id, current_piece_king, player_number) {
                            Ok(square) => {
                                match current_square_id.parse::<usize>() {
                                    Ok(parsed_id) => {
                                        let x = ID_COORDINATE_MAP[parsed_id].0;
                                        let y = ID_COORDINATE_MAP[parsed_id].1;
                                        squares[y as usize][x as usize] = square;
                                    },
                                    Err(_) => parse_error = true
                                }
                            },
                            Err(_) => parse_error = true
                        }
                    }
                    current_square_id = String::from("");
                    current_piece_king = false;
                    read_white_pieces = false;
                    read_black_pieces = false;
                }
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if read_white_pieces || read_black_pieces {
                   current_square_id.push(c);
                }
            },
            'K' => {
                if read_white_pieces || read_black_pieces {
                    current_piece_king = true;
                }
            },
            ',' => {
                if read_white_pieces {
                    match parse_square(&current_square_id, current_piece_king, 2) {
                        Ok(square) => {
                            match current_square_id.parse::<usize>() {
                                Ok(parsed_id) => {
                                    let x = ID_COORDINATE_MAP[parsed_id].0;
                                    let y = ID_COORDINATE_MAP[parsed_id].1;
                                    squares[y as usize][x as usize] = square;
                                },
                                Err(_) => parse_error = true
                            }
                        },
                        Err(_) => parse_error = true
                    }

                    current_square_id = String::from("");
                    current_piece_king = false;
                } else if read_black_pieces {
                    match parse_square(&current_square_id, current_piece_king, 1) {
                        Ok(square) => {
                            match current_square_id.parse::<usize>() {
                                Ok(parsed_id) => {
                                    let x = ID_COORDINATE_MAP[parsed_id].0;
                                    let y = ID_COORDINATE_MAP[parsed_id].1;
                                    squares[y as usize][x as usize] = square;
                                },
                                Err(_) => parse_error = true
                            }
                        },
                        Err(_) => parse_error = true
                    }

                    current_square_id = String::from("");
                    current_piece_king = false;
                }
            }
            _ => {
                parse_error = true;
            }
        }
    }

    // end of game state string, make a piece for the last one
    if current_square_id != String::from("") {
        if read_white_pieces {
            match parse_square(&current_square_id, current_piece_king, 2) {
                Ok(square) => {
                    match current_square_id.parse::<usize>() {
                        Ok(parsed_id) => {
                            let x = ID_COORDINATE_MAP[parsed_id].0;
                            let y = ID_COORDINATE_MAP[parsed_id].1;
                            squares[y as usize][x as usize] = square;
                        },
                        Err(_) => parse_error = true
                    }
                },
                Err(_) => parse_error = true
            }
        } else if read_black_pieces {
            match parse_square(&current_square_id, current_piece_king, 1) {
                Ok(square) => {
                    match current_square_id.parse::<usize>() {
                        Ok(parsed_id) => {
                            let x = ID_COORDINATE_MAP[parsed_id].0;
                            let y = ID_COORDINATE_MAP[parsed_id].1;
                            squares[y as usize][x as usize] = square;
                        },
                        Err(_) => parse_error = true
                    }
                },
                Err(_) => parse_error = true
            }
        }
    }

    if parse_error {
        Err("Error parsing state")
    } else {
        Ok(GameState { current_player_number, squares })
    }
}

fn parse_square(current_square_id: &String, current_piece_king: bool, player_number: i8) -> Result<Square, &'static str> {
    match current_square_id.parse::<usize>() {
        Ok(_) => {
            let player_number = player_number;
            let king = current_piece_king;
            let square = Square { player_number, king };
            Ok(square)
        },
        Err(_) => {
            Err("Parse Square Error")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::state::mov::MoveKind;

    #[test]
    fn parse_test() {
        let encoded = String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        let result = parse(&encoded).unwrap();
        assert_eq!(result.current_player_number, 1);
        let expected = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false }
            ],
            vec![
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false }
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
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false }
            ],
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false }
            ],
            vec![
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        assert_eq!(result.squares, expected);
    }

    #[test]
    fn parsing_example_b() {
        let encoded = String::from("W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,15");
        let result = parse(&encoded);
        match result {
            Err(e) => assert!(false, "{}", e),
            Ok(_) => assert!(true, "success"),
        }
    }

    #[test]
    fn parsing_invalid() {
        let encoded = String::from("X:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        let result = parse(&encoded);
        match result {
            Ok(_) => assert!(false, "Expected Error"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn winner_none_test() {
        let encoded = String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        let game_state = parse(&encoded).unwrap();
        assert_eq!(None, game_state.winner());
    }

    #[test]
    fn winner_some_test() {
        let encoded = String::from("W:W21,22,23,24,25,26,27,28,29,30,31,32:B");
        let game_state = parse(&encoded).unwrap();
        assert_eq!(Some(2), game_state.winner());
    }

    #[test]
    fn possible_moves_test() {
        let encoded = String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves();
        assert_eq!(result.len(), 7);

        assert_eq!((&result[0]).from, (0, 5));
        assert_eq!((&result[0]).to, vec![(1, 4)]);

        assert_eq!((&result[1]).from, (2, 5));
        assert_eq!((&result[1]).to, vec![(1, 4)]);

        assert_eq!((&result[2]).from, (2, 5));
        assert_eq!((&result[2]).to, vec![(3, 4)]);

        assert_eq!((&result[3]).from, (4, 5));
        assert_eq!((&result[3]).to, vec![(3, 4)]);

        assert_eq!((&result[4]).from, (4, 5));
        assert_eq!((&result[4]).to, vec![(5, 4)]);

        assert_eq!((&result[5]).from, (6, 5));
        assert_eq!((&result[5]).to, vec![(5, 4)]);

        assert_eq!((&result[6]).from, (6, 5));
        assert_eq!((&result[6]).to, vec![(7, 4)]);
    }

    #[test]
    fn possible_moves_for_player_test() {
        let encoded = String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(2);
        assert_eq!(result.len(), 7);

        assert_eq!((&result[0]).from, (1, 2));
        assert_eq!((&result[0]).to, vec![(0, 3)]);

        assert_eq!((&result[1]).from, (1, 2));
        assert_eq!((&result[1]).to, vec![(2, 3)]);

        assert_eq!((&result[2]).from, (3, 2));
        assert_eq!((&result[2]).to, vec![(2, 3)]);

        assert_eq!((&result[3]).from, (3, 2));
        assert_eq!((&result[3]).to, vec![(4, 3)]);

        assert_eq!((&result[4]).from, (5, 2));
        assert_eq!((&result[4]).to, vec![(4, 3)]);

        assert_eq!((&result[5]).from, (5, 2));
        assert_eq!((&result[5]).to, vec![(6, 3)]);

        assert_eq!((&result[6]).from, (7, 2));
        assert_eq!((&result[6]).to, vec![(6, 3)]);
    }

    #[test]
    fn possible_moves_b_test() {
        let encoded = String::from("W:W16,17,20,21,22,24,25,26,27,29,32:B1,2,3,4,5,7,10,11,13,19");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(2);
        assert_eq!(result.len(), 2);

        assert_eq!((&result[0]).from, (1, 2));
        assert_eq!((&result[0]).to, vec![(3, 4), (1, 6)]);

        assert_eq!((&result[1]).from, (1, 2));
        assert_eq!((&result[1]).to, vec![(3, 4), (5, 6)]);
    }

    #[test]
    fn jumps_for_player_test() {
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
                Square { player_number: 1, king: false },
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

        let result = game_state.jumps_for_player(2);

        assert_eq!(result.len(), 1);

        let mov = result.first();
        match mov {
            Some(m) => {
                assert_eq!(m.from, (5, 4));
                assert_eq!(m.to, vec![(7, 6)]);
            },
            None => assert!(false, "Expected Move"),
        }
    }

    #[test]
    fn moves_for_player_test() {
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

        let result = game_state.moves_for_player(2);

        assert_eq!(result.len(), 1);

        let mov = result.first();
        match mov {
            Some(m) => {
                assert_eq!(m.from, (5, 4));
                assert_eq!(m.to, vec![(6, 5)]);
            },
            None => assert!(false, "Expected Move"),
        }
    }

    #[test]
    fn perform_move_test() {
        let encoded = String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: (6, 5),
            to: vec![(7, 4)],
        };

        match game_state.perform_move(&mov) {
            Ok(_) => {
                let from_square = game_state.squares[5][6];
                assert_eq!(from_square.player_number, 0);

                let to_square = game_state.squares[4][7];
                assert_eq!(to_square.player_number, 1);

                assert_eq!(game_state.current_player_number, 2);
            },
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn perform_move_with_promote() {
        let encoded = String::from("B:W27:B1,2,3,4,5,6,7,8,9,10,11,28");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: (0, 1),
            to: vec![(1, 0)],
        };
        match game_state.perform_move(&mov) {
            Ok(_) => {
                let to_square = game_state.squares[0][1];
                assert_eq!(to_square.king, true);
                assert_eq!(game_state.current_player_number, 2);
            },
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn perform_undo() {
        let encoded = String::from("W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,11,12,13");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: (6, 5),
            to: vec![(7, 4)],
        };

        match game_state.undo_move(&mov) {
            Ok(_) => {
                let from_square = game_state.squares[5][6];
                assert_eq!(from_square.player_number != 0, true);

                let to_square = game_state.squares[4][7];
                assert_eq!(to_square.player_number, 0);
            },
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn perform_undo_with_demote() {
        let encoded = String::from("B:WK32:B1,2,3,4,5,6,7,8,9,10,11,28");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: (0, 1),
            to: vec![(1, 0)],
        };
        match game_state.undo_move(&mov) {
            Ok(_) => {
                let from_square = game_state.squares[1][0];
                assert_eq!(from_square.player_number == 0, false);
                assert_eq!(from_square.king, false);

                let to_square = game_state.squares[0][1];
                assert_eq!(to_square.player_number, 0);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn perform_move_jump_test() {
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
                Square { player_number: 2, king: false },
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
        let mut game_state = GameState { current_player_number: 1, squares };

        match game_state.perform_move_leg((5, 4), (7, 6)) {
            Ok(_) => (),
            Err(e) => return assert!(false, "{}", e),
        };

        let from_square = game_state.squares[4][5];
        assert_eq!(from_square.player_number, 0);

        let between_square = game_state.squares[5][6];
        assert_eq!(between_square.player_number, 0);

        let to_square = game_state.squares[6][7];
        assert_eq!(to_square.player_number, 1);
    }

    #[test]
    fn perform_move_move_test() {
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

        let mut game_state = GameState { current_player_number: 1, squares };


        match game_state.perform_move_leg((5, 4), (6, 5)) {
            Ok(_) => {
                let from_square = game_state.squares[4][5];
                assert_eq!(from_square.player_number, 0);

                let to_square = game_state.squares[5][6];
                assert_eq!(to_square.player_number, 1);
            },
            Err(e) => return assert!(false, "{}", e),
        };
    }

    #[test]
    fn undo_move_leg_jump_test() {
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
                Square { player_number: 1, king: false }
            ],
            vec![
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let mut game_state = GameState { current_player_number: 1, squares };

        match game_state.undo_move_leg((5, 4), (7, 6)) {
            Ok(_) => (),
            Err(e) => return assert!(false, "{}", e)
        };

        let from_square = game_state.squares[4][5];
        assert_eq!(from_square.player_number, 1);

        let between_square = game_state.squares[5][6];
        assert_eq!(between_square.player_number, 2);

        let to_square = game_state.squares[6][7];
        assert_eq!(to_square.player_number, 0);
    }

    #[test]
    fn undo_move_move_test() {
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
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        let mut game_state = GameState { current_player_number: 1, squares };

        match game_state.undo_move_leg((5, 4), (6, 5)) {
            Ok(_) => (),
            Err(e) => return assert!(false, "{}", e)
        };

        let from_square = game_state.squares[4][5];
        assert_eq!(from_square.player_number, 1);

        let to_square = game_state.squares[5][6];
        assert_eq!(to_square.player_number, 0);
    }

    #[test]
    fn promote_piece_test() {
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
                Square { player_number: 2, king: false },
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
        let mut game_state = GameState { current_player_number: 1, squares };

        match game_state.promote((5, 4)) {
            Ok(_) => (),
            Err(e) => assert!(false, "{}", e)
        }

        let square = game_state.squares[4][5];
        assert_eq!(square.king, true);
    }

    #[test]
    fn demote_piece() {
        let squares = vec![
            vec![
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false },
                Square { player_number: 0, king: false },
                Square { player_number: 2, king: false }
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
                Square { player_number: 1, king: true },
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
                Square { player_number: 2, king: false },
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
        let mut game_state = GameState { current_player_number: 1, squares };

        match game_state.demote((5, 4)) {
            Ok(_) => (),
            Err(e) => assert!(false, "{}", e),
        }

        for (y, row) in game_state.squares.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                if x == 5 && y == 4 {
                    assert_eq!(false, square.king);
                }
            }
        }
    }
}
