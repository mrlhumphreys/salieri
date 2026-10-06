use crate::international_draughts::state::point::ID_COORDINATE_MAP;
use crate::international_draughts::state::square::Square;
use crate::international_draughts::state::square_set::find_by_x_and_y;
use crate::international_draughts::state::square_set::find_by_x_and_y_mut;
use crate::international_draughts::state::mov::Move;

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
    // pub fn winner(&self) -> Option<i8> {
    //     if self.possible_moves_for_player(1).is_empty() {
    //         Some(2)
    //     } else if self.possible_moves_for_player(2).is_empty() {
    //         Some(1)
    //     } else {
    //         None
    //     }
    // }

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

        if list.len() < 2 {
            list
        } else {
            // filter for moves with the most captures
            list.sort_by(|a, b| a.captures.len().cmp(&b.captures.len())); // small to big
            match list.last() {
                Some(most_capture_move) => {
                    let max_captures = most_capture_move.captures.len();
                    list.into_iter().filter(|m| m.captures.len() == max_captures).collect()
                },
                None => list
            }
        }
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

        for (origin, capture, destination) in legs {
           self.perform_move_leg(origin, destination, capture)?;
        }

        let (next_player_number, promotion_row) = match self.current_player_number {
            1 => (2, 0),
            2 => (1, 9),
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
            2 => (1, 9),
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

        for (origin, capture, destination) in mov.legs().into_iter().rev() {
           self.undo_move_leg(origin, destination, capture)?;
        }

        self.current_player_number = previous_player_number;

        Ok(())
    }

    pub fn perform_move_leg(&mut self, from: (i8, i8), to: (i8, i8), capture: Option<(i8, i8)>) -> Result<(), &'static str> {
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

        if let Some(c) = capture {
            if let Some(square) = find_by_x_and_y_mut(&mut self.squares, c.0, c.1) {
                square.player_number = 0;
                square.king = false;
            }
        }

        Ok(())
    }

    pub fn undo_move_leg(&mut self, from: (i8, i8), to: (i8, i8), capture: Option<(i8, i8)>) -> Result<(), &'static str> {
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

        if let Some(c) = capture {
            if let Some(square) = find_by_x_and_y_mut(&mut self.squares, c.0, c.1) {
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

// W:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50
// 1,2,3,4,5 -> light pieces, light plays first
//     0 1 2 3 4 5 6 7 8 9
// 0:   50  49  48  47  46 :9
// 1: 45  44  43  42  41   :8
// 2:   40  39  38  37  36 :7
// 3: 35  34  33  32  31   :6
// 4:   30  29  28  27  26 :5
// 5: 25  24  23  22  21   :4
// 6:   20  19  18  17  16 :3
// 7: 15  14  13  12  11   :2
// 8:   10  09  08  07  06 :1
// 9: 05  04  03  02  01   :0
//     9 8 7 6 5 4 3 2 1 0
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
        ]
    ];

    for c in encoded.chars() {
        match c {
            'W' => {
                if read_player {
                   current_player_number = 1;
                } else {
                    read_white_pieces = true;
                }
            },
            'B' => {
                if read_player {
                    current_player_number = 2;
                } else {
                    read_white_pieces = false;
                    read_black_pieces = true;
                }
            },
            ':' => {
                if read_player {
                    read_player = false;
                } else if read_white_pieces || read_black_pieces {
                    if current_square_id != String::from("") {
                        let player_number = if read_white_pieces {
                            1
                        } else {
                            2
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
                } else if read_black_pieces {
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
        } else if read_black_pieces {
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
    use crate::international_draughts::state::mov::MoveKind;

    #[test]
    fn parse_test() {
        let encoded = String::from("W:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
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
                Square { player_number: 1, king: false },
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
                Square { player_number: 0, king: false },
                Square { player_number: 1, king: false },
                Square { player_number: 0, king: false }
            ]
        ];
        assert_eq!(result.squares, expected);
    }

    #[test]
    fn parsing_example_b() {
        let encoded = String::from("B:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,25:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
        let result = parse(&encoded);
        match result {
            Err(e) => assert!(false, "{}", e),
            Ok(_) => assert!(true, "success"),
        }
    }

    #[test]
    fn parsing_invalid() {
        let encoded = String::from("X:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
        let result = parse(&encoded);
        match result {
            Ok(_) => assert!(false, "Expected Error"),
            Err(_) => assert!(true),
        }
    }

    // #[test]
    // fn winner_none_test() {
    //     let encoded = String::from("W:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
    //     let game_state = parse(&encoded).unwrap();
    //     assert_eq!(None, game_state.winner());
    // }

    // #[test]
    // fn winner_some_test() {
    //     let encoded = String::from("W:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B");
    //     let game_state = parse(&encoded).unwrap();
    //     assert_eq!(Some(1), game_state.winner());
    // }

    #[test]
    fn possible_moves_test() {
        let encoded = String::from("W:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves();
        assert_eq!(result.len(), 9);

        assert_eq!((&result[0]).from, (1, 6));
        assert_eq!((&result[0]).captures, vec![]);
        assert_eq!((&result[0]).to, vec![(0, 5)]);

        assert_eq!((&result[1]).from, (1, 6));
        assert_eq!((&result[1]).captures, vec![]);
        assert_eq!((&result[1]).to, vec![(2, 5)]);

        assert_eq!((&result[2]).from, (3, 6));
        assert_eq!((&result[2]).captures, vec![]);
        assert_eq!((&result[2]).to, vec![(2, 5)]);

        assert_eq!((&result[3]).from, (3, 6));
        assert_eq!((&result[3]).captures, vec![]);
        assert_eq!((&result[3]).to, vec![(4, 5)]);

        assert_eq!((&result[4]).from, (5, 6));
        assert_eq!((&result[4]).captures, vec![]);
        assert_eq!((&result[4]).to, vec![(4, 5)]);

        assert_eq!((&result[5]).from, (5, 6));
        assert_eq!((&result[5]).captures, vec![]);
        assert_eq!((&result[5]).to, vec![(6, 5)]);

        assert_eq!((&result[6]).from, (7, 6));
        assert_eq!((&result[6]).captures, vec![]);
        assert_eq!((&result[6]).to, vec![(6, 5)]);

        assert_eq!((&result[7]).from, (7, 6));
        assert_eq!((&result[7]).captures, vec![]);
        assert_eq!((&result[7]).to, vec![(8, 5)]);

        assert_eq!((&result[8]).from, (9, 6));
        assert_eq!((&result[8]).captures, vec![]);
        assert_eq!((&result[8]).to, vec![(8, 5)]);
    }

    #[test]
    fn possible_moves_for_player_test() {
        let encoded = String::from("W:W1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20:B31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(2);
        assert_eq!(result.len(), 9);

        assert_eq!((&result[0]).from, (0, 3));
        assert_eq!((&result[0]).captures, vec![]);
        assert_eq!((&result[0]).to, vec![(1, 4)]);

        assert_eq!((&result[1]).from, (2, 3));
        assert_eq!((&result[1]).captures, vec![]);
        assert_eq!((&result[1]).to, vec![(1, 4)]);

        assert_eq!((&result[2]).from, (2, 3));
        assert_eq!((&result[2]).captures, vec![]);
        assert_eq!((&result[2]).to, vec![(3, 4)]);

        assert_eq!((&result[3]).from, (4, 3));
        assert_eq!((&result[3]).captures, vec![]);
        assert_eq!((&result[3]).to, vec![(3, 4)]);

        assert_eq!((&result[4]).from, (4, 3));
        assert_eq!((&result[4]).captures, vec![]);
        assert_eq!((&result[4]).to, vec![(5, 4)]);

        assert_eq!((&result[5]).from, (6, 3));
        assert_eq!((&result[5]).captures, vec![]);
        assert_eq!((&result[5]).to, vec![(5, 4)]);

        assert_eq!((&result[6]).from, (6, 3));
        assert_eq!((&result[6]).captures, vec![]);
        assert_eq!((&result[6]).to, vec![(7, 4)]);

        assert_eq!((&result[7]).from, (8, 3));
        assert_eq!((&result[7]).captures, vec![]);
        assert_eq!((&result[7]).to, vec![(7, 4)]);

        assert_eq!((&result[8]).from, (8, 3));
        assert_eq!((&result[8]).captures, vec![]);
        assert_eq!((&result[8]).to, vec![(9, 4)]);
    }

    #[test]
    fn possible_moves_compulsory_capture_test() {
        let encoded = String::from("W:W24:B29");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);
        assert_eq!(result.len(), 1);

        assert_eq!((&result[0]).from, (2, 5));
        assert_eq!((&result[0]).captures, vec![(3, 4)]);
        assert_eq!((&result[0]).to, vec![(4, 3)]);
    }

    #[test]
    fn possible_moves_flying_kings_test() {
        let encoded = String::from("W:WK23:B34,28,32,19");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);

        assert_eq!(result.len(), 5);

        assert_eq!((&result[0]).from, (4, 5));
        assert_eq!((&result[0]).captures, vec![(3, 6)]);
        assert_eq!((&result[0]).to, vec![(2, 7)]);

        assert_eq!((&result[1]).from, (4, 5));
        assert_eq!((&result[1]).captures, vec![(2, 3)]);
        assert_eq!((&result[1]).to, vec![(1, 2)]);

        assert_eq!((&result[2]).from, (4, 5));
        assert_eq!((&result[2]).captures, vec![(3, 6)]);
        assert_eq!((&result[2]).to, vec![(1, 8)]);

        assert_eq!((&result[3]).from, (4, 5));
        assert_eq!((&result[3]).captures, vec![(2, 3)]);
        assert_eq!((&result[3]).to, vec![(0, 1)]);

        assert_eq!((&result[4]).from, (4, 5));
        assert_eq!((&result[4]).captures, vec![(3, 6)]);
        assert_eq!((&result[4]).to, vec![(0, 9)]);
    }

    #[test]
    fn possible_moves_no_jumps_over_same_piece_twice_test() {
        let encoded = String::from("W:WK23:B28,31,21");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);

        assert_eq!(result.len(), 4);

        assert_eq!((&result[0]).from, (4, 5));
        assert_eq!((&result[0]).captures, vec![(5, 4), (8, 3), (8, 5)]);
        assert_eq!((&result[0]).to, vec![(7, 2), (9, 4), (7, 6)]);

        assert_eq!((&result[1]).from, (4, 5));
        assert_eq!((&result[1]).captures, vec![(5, 4), (8, 3), (8, 5)]);
        assert_eq!((&result[1]).to, vec![(7, 2), (9, 4), (6, 7)]);

        assert_eq!((&result[2]).from, (4, 5));
        assert_eq!((&result[2]).captures, vec![(5, 4), (8, 3), (8, 5)]);
        assert_eq!((&result[2]).to, vec![(7, 2), (9, 4), (5, 8)]);

        assert_eq!((&result[3]).from, (4, 5));
        assert_eq!((&result[3]).captures, vec![(5, 4), (8, 3), (8, 5)]);
        assert_eq!((&result[3]).to, vec![(7, 2), (9, 4), (4, 9)]);
    }

    #[test]
    fn possible_moves_maximum_captures_test() {
        let encoded = String::from("W:W23:B28,29,37");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);

        assert_eq!(result.len(), 1);

        assert_eq!((&result[0]).from, (4, 5));
        assert_eq!((&result[0]).captures, vec![(5, 4), (7, 2)]);
        assert_eq!((&result[0]).to, vec![(6, 3), (8, 1)]);
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
                Square { player_number: 1, king: false },
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

        let result = game_state.jumps_for_player(2);

        assert_eq!(result.len(), 1);

        let mov = result.first();
        match mov {
            Some(m) => {
                assert_eq!(m.from, (5, 4));
                assert_eq!(m.captures, vec![(6, 5)]);
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

        let result = game_state.moves_for_player(2);

        assert_eq!(result.len(), 1);

        let mov = result.first();
        match mov {
            Some(m) => {
                assert_eq!(m.from, (5, 4));
                assert_eq!(m.captures, vec![]);
                assert_eq!(m.to, vec![(6, 5)]);
            },
            None => assert!(false, "Expected Move"),
        }
    }

    #[test]
    fn perform_move_test() {
        let encoded = String::from("W:W17:B34");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: (7, 6),
            captures: vec![],
            to: vec![(6, 5)],
        };

        match game_state.perform_move(&mov) {
            Ok(_) => {
                let from_square = game_state.squares[6][7];
                assert_eq!(from_square.player_number, 0);

                let to_square = game_state.squares[5][6];
                assert_eq!(to_square.player_number, 1);

                assert_eq!(game_state.current_player_number, 2);
            },
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn perform_move_with_promote() {
        let encoded = String::from("B:W41,42,43,44,45:B6,7,8,9,10");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: (1, 8),
            captures: vec![],
            to: vec![(2, 9)],
        };
        match game_state.perform_move(&mov) {
            Ok(_) => {
                let to_square = game_state.squares[9][2];
                assert_eq!(to_square.king, true);
                assert_eq!(game_state.current_player_number, 1);
            },
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn perform_move_skip_promote_if_passing_through_promotion_row_test() {
        let encoded = String::from("B:W39:B43,42");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: (3, 2),
            captures: vec![],
            to: vec![(5, 0), (7, 2)],
        };
        match game_state.perform_move(&mov) {
            Ok(_) => {
                let to_square = game_state.squares[2][7];
                assert_eq!(to_square.king, false);
            },
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn perform_undo() {
        let encoded = String::from("B:W42,43,44,45,47:BK4,6,7,8,9");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: (1, 8),
            captures: vec![],
            to: vec![(2, 9)],
        };

        match game_state.undo_move(&mov) {
            Ok(_) => {
                let from_square = game_state.squares[8][1];
                assert_eq!(from_square.player_number != 0, true);

                let to_square = game_state.squares[9][2];
                assert_eq!(to_square.player_number, 0);
            },
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn perform_undo_with_demote() {
        let encoded = String::from("B:W42,43,44,45,47:BK4,6,7,8,9");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: (1, 8),
            captures: vec![],
            to: vec![(2, 9)],
        };
        match game_state.undo_move(&mov) {
            Ok(_) => {
                let from_square = game_state.squares[8][1];
                assert_eq!(from_square.player_number == 0, false);
                assert_eq!(from_square.king, false);

                let to_square = game_state.squares[9][2];
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
                Square { player_number: 2, king: false },
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
        let mut game_state = GameState { current_player_number: 1, squares };

        match game_state.perform_move_leg((5, 4), (7, 6), Some((6, 5))) {
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

        let mut game_state = GameState { current_player_number: 1, squares };

        match game_state.perform_move_leg((5, 4), (6, 5), None) {
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
                Square { player_number: 1, king: false },
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
        let mut game_state = GameState { current_player_number: 1, squares };

        match game_state.undo_move_leg((5, 4), (7, 6), Some((6, 5))) {
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
        let mut game_state = GameState { current_player_number: 1, squares };

        match game_state.undo_move_leg((5, 4), (6, 5), None) {
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
                Square { player_number: 2, king: false },
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
                Square { player_number: 1, king: true },
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
                Square { player_number: 2, king: false },
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
