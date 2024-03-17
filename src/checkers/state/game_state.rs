use crate::checkers::state::square::Square;
use crate::checkers::state::square_set::SquareSet;
use crate::checkers::state::square_set::parse_square_set;
use crate::checkers::state::mov::Move;

const ID_COORDINATE_MAP: [(i8, i8); 33] = [
    (-1, -1),

    (6, 7),
    (4, 7),
    (2, 7),
    (0, 7),

    (7, 6),
    (5, 6),
    (3, 6),
    (1, 6),

    (6, 5),
    (4, 5),
    (2, 5),
    (0, 5),

    (7, 4),
    (5, 4),
    (3, 4),
    (1, 4),

    (6, 3),
    (4, 3),
    (2, 3),
    (0, 3),

    (7, 2),
    (5, 2),
    (3, 2),
    (1, 2),

    (6, 1),
    (4, 1),
    (2, 1),
    (0, 1),

    (7, 0),
    (5, 0),
    (3, 0),
    (1, 0)
];

#[derive(PartialEq, Debug)]
pub struct GameState {
    pub current_player_number: i8,
    pub squares: SquareSet,
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
        if self.possible_moves_for_player(1).len() == 0 {
            Some(2)
        } else if self.possible_moves_for_player(2).len() == 0 {
            Some(1)
        } else {
            None
        }
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        self.possible_moves_for_player(self.current_player_number)
    }

    pub fn possible_moves_for_player(&self, player_number: i8) -> Vec<Move> {
        let jumps = self.squares.jumps_for_player(player_number, &self.squares);
        if jumps.len() == 0 {
            self.squares.moves_for_player(player_number, &self.squares)
        } else {
            jumps
        }
    }

    pub fn perform_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        let legs = mov.legs();

        for (origin, destination) in legs {
           match self.squares.perform_move(origin, destination) {
                Ok(_) => (),
                Err(e) => {
                    return Err(e);
                },
           }
        }

        let (next_player_number, promotion_row) = match self.current_player_number {
            1 => (2, 7),
            2 => (1, 0),
            _ => return Err("invalid player number"),
        };

        match mov.to.last() {
            Some(last_id) => {
                match self.squares.squares.iter_mut().find(|s| s.id == *last_id) {
                    Some(s) => {
                        if promotion_row == s.y {
                            match self.squares.promote(*last_id) {
                                Ok(_) => (),
                                Err(e) => return Err(e),
                            }
                        }
                    },
                    None => return Err("invalid square id"),
                }
            },
            None => return Err("no square id"),
        };

        self.current_player_number = next_player_number;

        Ok(())
    }

    pub fn undo_move(&mut self, mov: &Move) -> Result<(), &'static str> {

        let (previous_player_number, promotion_row) = match self.current_player_number {
            1 => (2, 7),
            2 => (1, 0),
            _ => return Err("invalid player number"),
        };

        match mov.to.last() {
            Some(last_id) => {
                match self.squares.squares.iter_mut().find(|s| s.id == *last_id) {
                    Some(s) => {
                        if promotion_row == s.y {
                            match self.squares.demote(*last_id) {
                                Ok(_) => (),
                                Err(e) => return Err(e),
                            }
                        }
                    },
                    None => return Err("invalid square id"),
                }
            },
            None => return Err("no square id"),
        };

        let reverse_legs = mov.legs().into_iter().rev();

        for (origin, destination) in reverse_legs {
           match self.squares.undo_move(origin, destination) {
                Ok(_) => (),
                Err(e) => {
                    return Err(e);
                },
           }
        }

        self.current_player_number = previous_player_number;

        Ok(())
    }
}

pub fn parse(encoded: &String) -> Result<GameState, &'static str> {
    if encoded.len() != 33 {
        return Err("Invalid State");
    }

    let squares_component = &encoded[0..32];
    let player_component = &encoded[32..33];

    let current_player_number = match player_component {
      "b" => 1,
      "w" => 2,
      _ => return Err("Invalid State"),
    };

    let squares = match parse_square_set(squares_component) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let game_state = GameState { current_player_number, squares };

    Ok(game_state)
}

// B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12
pub fn parse_fen(encoded: &String) -> Result<GameState, &'static str> {
    let mut read_player = true;
    let mut read_white_pieces = false;
    let mut read_black_pieces = false;
    let mut parse_error = false;
    let mut current_piece_king = false;
    let mut current_square_id = String::from("");
    let mut current_player_number = 1;
    let mut squares: Vec<Square> = vec![];

    for c in encoded.chars() {
        match c {
            'B' => {
                if read_player {
                   current_player_number = 1;
                } else {
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
                    match current_square_id.parse::<usize>() {
                        Ok(parsed_id) => {
                            let id = parsed_id as i8;
                            let x = ID_COORDINATE_MAP[parsed_id].0;
                            let y = ID_COORDINATE_MAP[parsed_id].1;
                            let player_number = 2;
                            let king = current_piece_king;
                            let square = Square { id, x, y, player_number, king };
                            squares.push(square);
                            ()
                        },
                        Err(_) => {
                            parse_error = false;
                            ()
                        }
                    }

                    current_square_id = String::from("");
                    current_piece_king = false;
                } else if read_black_pieces {
                    match current_square_id.parse::<usize>() {
                        Ok(parsed_id) => {
                            let id = parsed_id as i8;
                            let x = ID_COORDINATE_MAP[parsed_id].0;
                            let y = ID_COORDINATE_MAP[parsed_id].1;
                            let player_number = 1;
                            let king = current_piece_king;
                            let square = Square { id, x, y, player_number, king };
                            squares.push(square);
                            ()
                        },
                        Err(_) => {
                            parse_error = false;
                            ()
                        }
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

    // fill in unoccupied squares

    let occupied_square_ids: Vec<i8> = squares.iter().map(|s| s.id).collect();

    let mut counter: i8 = 1;
    while counter <= 32 {
        if !occupied_square_ids.contains(&counter) {
            let id = counter;
            let index = counter as usize;
            let x = ID_COORDINATE_MAP[index].0;
            let y = ID_COORDINATE_MAP[index].1;
            let player_number = 0;
            let king = false;
            let square = Square { id, x, y, player_number, king };
            squares.push(square);
        };
        counter += 1;
    }

    squares.sort_by(|a, b| a.id.cmp(&b.id));

    if parse_error {
        Err("Error parsing state")
    } else {
        Ok(GameState {
            current_player_number,
            squares: SquareSet { squares }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::state::mov::MoveKind;

    #[test]
    fn parsing() {
        let encoded = String::from("bbbbbbbbbbbb--------wwwwwwwwwwwwb");
        let result = parse(&encoded).unwrap();
        assert_eq!(result.current_player_number, 1);
        assert_eq!(result.squares.squares.len(), 32);
    }

    #[test]
    fn parse_fen_test() {
        let encoded = String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        let result = parse_fen(&encoded).unwrap();
        assert_eq!(result.current_player_number, 1);
        let expected = vec![
            Square { id: 1, x: 6, y: 7, player_number: 1, king: false },
            Square { id: 2, x: 4, y: 7, player_number: 1, king: false },
            Square { id: 3, x: 2, y: 7, player_number: 1, king: false },
            Square { id: 4, x: 0, y: 7, player_number: 1, king: false },
            Square { id: 5, x: 7, y: 6, player_number: 1, king: false },
            Square { id: 6, x: 5, y: 6, player_number: 1, king: false },
            Square { id: 7, x: 3, y: 6, player_number: 1, king: false },
            Square { id: 8, x: 1, y: 6, player_number: 1, king: false },
            Square { id: 9, x: 6, y: 5, player_number: 1, king: false },
            Square { id: 10, x: 4, y: 5, player_number: 1, king: false },
            Square { id: 11, x: 2, y: 5, player_number: 1, king: false },
            Square { id: 12, x: 0, y: 5, player_number: 0, king: false },
            Square { id: 13, x: 7, y: 4, player_number: 0, king: false },
            Square { id: 14, x: 5, y: 4, player_number: 0, king: false },
            Square { id: 15, x: 3, y: 4, player_number: 0, king: false },
            Square { id: 16, x: 1, y: 4, player_number: 0, king: false },
            Square { id: 17, x: 6, y: 3, player_number: 0, king: false },
            Square { id: 18, x: 4, y: 3, player_number: 0, king: false },
            Square { id: 19, x: 2, y: 3, player_number: 0, king: false },
            Square { id: 20, x: 0, y: 3, player_number: 0, king: false },
            Square { id: 21, x: 7, y: 2, player_number: 2, king: false },
            Square { id: 22, x: 5, y: 2, player_number: 2, king: false },
            Square { id: 23, x: 3, y: 2, player_number: 2, king: false },
            Square { id: 24, x: 1, y: 2, player_number: 2, king: false },
            Square { id: 25, x: 6, y: 1, player_number: 2, king: false },
            Square { id: 26, x: 4, y: 1, player_number: 2, king: false },
            Square { id: 27, x: 2, y: 1, player_number: 2, king: false },
            Square { id: 28, x: 0, y: 1, player_number: 2, king: false },
            Square { id: 29, x: 7, y: 0, player_number: 2, king: false },
            Square { id: 30, x: 5, y: 0, player_number: 2, king: false },
            Square { id: 31, x: 3, y: 0, player_number: 2, king: false },
            Square { id: 32, x: 1, y: 0, player_number: 0, king: false }
        ];
        assert_eq!(result.squares.squares, expected);
    }

    #[test]
    fn parsing_example_b() {
        let encoded = String::from("bbbbbbbbb-bb--b-----wwwwwwwwwwwww");
        let result = parse(&encoded);
        match result {
            Err(e) => assert!(false, "{}", e),
            Ok(_) => assert!(true, "success"),
        }
    }

    #[test]
    fn parsing_invalid() {
        let encoded = String::from("bbbbbbbbbbbb--------wwwwwwwwwwww");
        let result = parse(&encoded);
        match result {
            Ok(_) => assert!(false, "Expected Error"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn winner_none_test() {
        let encoded = String::from("bbbbbbbbbbbb--------wwwwwwwwwwwwb");
        let game_state = parse(&encoded).unwrap();
        assert_eq!(None, game_state.winner());
    }

    #[test]
    fn winner_some_test() {
        let encoded = String::from("bbbbbbbbbbbb--------------------b");
        let game_state = parse(&encoded).unwrap();
        assert_eq!(Some(1), game_state.winner());
    }

    #[test]
    fn possible_moves_test() {
        let encoded = String::from("bbbbbbbbbbbb--------wwwwwwwwwwwwb");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves();
        assert_eq!(result.len(), 7);

        assert_eq!((&result[0]).from, 9);
        assert_eq!((&result[0]).to, vec![13]);

        assert_eq!((&result[1]).from, 9);
        assert_eq!((&result[1]).to, vec![14]);

        assert_eq!((&result[2]).from, 10);
        assert_eq!((&result[2]).to, vec![14]);

        assert_eq!((&result[3]).from, 10);
        assert_eq!((&result[3]).to, vec![15]);

        assert_eq!((&result[4]).from, 11);
        assert_eq!((&result[4]).to, vec![15]);

        assert_eq!((&result[5]).from, 11);
        assert_eq!((&result[5]).to, vec![16]);

        assert_eq!((&result[6]).from, 12);
        assert_eq!((&result[6]).to, vec![16]);
    }

    #[test]
    fn possible_moves_for_player_test() {
        let encoded = String::from("bbbbbbbbbbbb--------wwwwwwwwwwwwb");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(2);
        assert_eq!(result.len(), 7);

        assert_eq!((&result[0]).from, 21);
        assert_eq!((&result[0]).to, vec![17]);

        assert_eq!((&result[1]).from, 22);
        assert_eq!((&result[1]).to, vec![17]);

        assert_eq!((&result[2]).from, 22);
        assert_eq!((&result[2]).to, vec![18]);

        assert_eq!((&result[3]).from, 23);
        assert_eq!((&result[3]).to, vec![18]);

        assert_eq!((&result[4]).from, 23);
        assert_eq!((&result[4]).to, vec![19]);

        assert_eq!((&result[5]).from, 24);
        assert_eq!((&result[5]).to, vec![19]);

        assert_eq!((&result[6]).from, 24);
        assert_eq!((&result[6]).to, vec![20]);
    }

    #[test]
    fn possible_moves_b_test() {
        let encoded = String::from("bbbbb-b--bb-b--ww-bwww-wwww-w--ww");
        let game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(2);
        assert_eq!(result.len(), 2);

        assert_eq!((&result[0]).from, 24);
        assert_eq!((&result[0]).to, vec![15, 6]);

        assert_eq!((&result[1]).from, 24);
        assert_eq!((&result[1]).to, vec![15, 8]);
    }

    #[test]
    fn perform_move_test() {
        let encoded = String::from("wwwwwwwwwwww--------bbbbbbbbbbbbw");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: 9,
            to: vec![13],
        };

        match game_state.perform_move(&mov) {
            Ok(_) => {
                match game_state.squares.squares.clone().into_iter().find(|s| s.id == 9) {
                    Some(s) => assert_eq!(s.occupied(), false),
                    None => assert!(false, "square not found"),
                }

                match game_state.squares.squares.clone().into_iter().find(|s| s.id == 13) {
                    Some(s) => assert_eq!(s.occupied(), true),
                    None => assert!(false, "square not found"),
                }

                assert_eq!(game_state.current_player_number, 1);
            },
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn perform_move_with_promote() {
        let encoded = String::from("bbbbbbbbbbb---------------wb----b");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: 28,
            to: vec![32],
        };
        match game_state.perform_move(&mov) {
            Ok(_) => {
                match game_state.squares.squares.clone().into_iter().find(|s| s.id == 32) {
                    Some(s) => assert_eq!(s.king, true),
                    None => assert!(false, "square not found"),
                }
                assert_eq!(game_state.current_player_number, 2);
            },
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn perform_undo() {
        let encoded = String::from("wwwwwwww-wwww-------bbbbbbbbbbbbw");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: 9,
            to: vec![13],
        };

        match game_state.undo_move(&mov) {
            Ok(_) => {
                match game_state.squares.squares.clone().into_iter().find(|s| s.id == 9) {
                    Some(s) => assert_eq!(s.occupied(), true),
                    None => assert!(false, "square not found"),
                }

                match game_state.squares.squares.clone().into_iter().find(|s| s.id == 13) {
                    Some(s) => assert_eq!(s.occupied(), false),
                    None => assert!(false, "square not found"),
                }
            },
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn perform_undo_with_demote() {
        let encoded = String::from("bbbbbbbbbbb----------------b---Wb");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: 28,
            to: vec![32],
        };
        match game_state.undo_move(&mov) {
            Ok(_) => {
                match game_state.squares.squares.clone().into_iter().find(|s| s.id == 28) {
                    Some(s) => {
                        assert_eq!(s.occupied(), true);
                        assert_eq!(s.king, false);
                    }
                    None => assert!(false, "square not found"),
                }

                match game_state.squares.squares.clone().into_iter().find(|s| s.id == 32) {
                    Some(s) => assert_eq!(s.occupied(), false),
                    None => assert!(false, "square not found"),
                }
            },
            Err(e) => assert!(false, "{}", e)
        }
    }
}
