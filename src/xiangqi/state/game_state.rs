use crate::xiangqi::state::point::valid;
use crate::xiangqi::state::piece_factory::parse as parse_piece;
use crate::xiangqi::state::square::destinations;
use crate::xiangqi::state::square::opposing_player;
use crate::xiangqi::state::square::PieceKind;
use crate::xiangqi::state::square::Square;
// use crate::xiangqi::state::point::diff;
// use crate::xiangqi::state::point::between;
// use crate::xiangqi::state::point::king_destination_points;
use crate::xiangqi::state::square_set::find_by_x_and_y_mut;
use crate::xiangqi::state::square_set::find_by_x_and_y;
use crate::xiangqi::state::square_set::find_king_point_for_player;
// use crate::xiangqi::state::square_set::threats_to_point;
use crate::xiangqi::state::square_set::any_threats_to_point;
// use crate::xiangqi::state::square_set::any_threats_to_point_through_pin;
// use crate::xiangqi::state::square_set::pinned_to_point;
use crate::xiangqi::state::mov::Move;

#[derive(Clone)]
pub struct GameState {
    pub current_player_number: i8,
    pub squares: Vec<Vec<Square>>
}

impl GameState {
    // pub fn winner(&mut self) -> Option<i8> {
    //     if self.in_checkmate(1) {
    //         Some(2)
    //     } else if self.in_checkmate(2) {
    //         Some(1)
    //     } else {
    //         None
    //     }
    // }

    // pub fn in_checkmate(&self, player_number: i8) -> bool {
    //     if let Some(ou_point) = find_king_point_for_player(&self.squares, player_number) {
    //         self.in_check(player_number, ou_point) && self.king_cannot_move(player_number, ou_point) && !self.threats_to_king_can_be_captured_or_blocked(player_number, ou_point)
    //     } else {
    //         false
    //     }
    // }

    pub fn in_check(&self, player_number: i8, king_point: (i8, i8)) -> bool {
        any_threats_to_point(&self.squares, king_point, player_number)
    }

    // pub fn king_cannot_move(&self, player_number: i8, king_point: (i8, i8)) -> bool {
    //     let mut can_move = false;

    //     for to in king_destination_points(king_point, player_number) {
    //         if let Some(to_square) = find_by_x_and_y(&self.squares, to) {
    //             // square is free or owned by other player
    //             if to_square.player_number != player_number {
    //                 can_move = !any_threats_to_point(&self.squares, to, player_number) && !any_threats_to_point_through_pin(&self.squares, to, player_number, king_point);
    //             }
    //         }
    //         if can_move {
    //             break;
    //         }
    //     }

    //     !can_move
    // }

    // pub fn threats_to_king_can_be_captured_or_blocked(&self, player_number: i8, king_point: (i8, i8)) -> bool {
    //     let threats_to_king = threats_to_point(&self.squares, king_point, player_number);

    //     match threats_to_king.len() {
    //         0 => {
    //             // return true since there are no threats
    //             true
    //         },
    //         1 => {
    //             // can threat be captured?
    //             let opposing_player_number = opposing_player(player_number);
    //             let pinned_to_king = pinned_to_point(&self.squares, king_point, player_number, self);
    //             let threat = threats_to_king[0];
    //             let threats_to_threats = threats_to_point(&self.squares, threat, opposing_player_number);
    //             // is there  a non pinned threat to the threatening piece?
    //             if !diff(&threats_to_threats, &pinned_to_king).is_empty() {
    //                 true
    //             } else {
    //                 let between_points = between(threat, king_point);
    //                 // any square between threat and ou can be blocked by move
    //                 between_points.iter().any(|b| {
    //                     let threats_to_between = threats_to_point(&self.squares, *b, opposing_player_number);
    //                     let has_threats = !diff(&threats_to_between, &pinned_to_king).is_empty();
    //                     has_threats
    //                 })
    //             }
    //         },
    //         _ => {
    //             // return false since any move will still leave at least 1 threat
    //             false
    //         }
    //     }
    // }

    pub fn possible_moves(&mut self) -> Vec<Move> {
        self.possible_moves_for_player(self.current_player_number)
    }

    pub fn possible_moves_for_player(&mut self, subject_player_number: i8) -> Vec<Move> {
        let mut moves = vec![];

        for (y, row) in self.squares.iter().enumerate() {
            for (x, from) in row.iter().enumerate() {
                if from.player_number == subject_player_number {
                    let from_point = (x as i8, y as i8);
                    for to_point in destinations(from.kind, from.player_number, from_point, &self, false) {

                        if let Some(to) = find_by_x_and_y(&self.squares, to_point) {
                            let capture_piece_kind = if to.player_number != 0 && subject_player_number != to.player_number {
                                Some(to.kind)
                            } else {
                                None
                            };

                            let mov = Move {
                                from: from_point,
                                to: to_point,
                                moving_piece_kind: from.kind,
                                capture_piece_kind,
                            };
                            moves.push(mov);
                        }
                    }
                }
            }
        }


        // keep moves that don't result in check for the current player.
        moves.retain(|m| {
            let perform_result = self.perform_move(&m);
            let in_check = match find_king_point_for_player(&self.squares, subject_player_number) {
                Some(king_point) => self.in_check(subject_player_number, king_point),
                None => false
            };
            let undo_result = self.undo_move(&m);
            perform_result.is_ok() && undo_result.is_ok() && !in_check
        });

        moves
    }

    pub fn perform_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        let piece_player_number: i8;
        let piece_kind: PieceKind;

        if let Some(s) = find_by_x_and_y_mut(&mut self.squares, mov.from) {
            if s.occupied() {
                piece_player_number = s.player_number;
                piece_kind = s.kind;
                s.player_number = 0;
                s.kind = PieceKind::Empty;
            } else {
                return Err("game_state::perform_move - No piece on from");
            }
        } else {
            return Err("game_state::perform_move - Invalid from square");
        }

        if let Some(s) = find_by_x_and_y_mut(&mut self.squares, mov.to) {
            s.player_number = piece_player_number;
            s.kind = piece_kind;
        } else {
            return Err("game_state::perform_move - Invalid to square");
        }

        self.current_player_number = opposing_player(self.current_player_number);

        Ok(())
    }

    pub fn undo_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        let moving_piece_player_number: i8;
        let moving_piece_kind: PieceKind;

        // get piece on to
        if let Some(s) = find_by_x_and_y_mut(&mut self.squares, mov.to) {
            moving_piece_kind = s.kind;
            moving_piece_player_number = s.player_number;
            s.kind = PieceKind::Empty;
            s.player_number = 0;
        } else {
            return Err("game_state::undo_move - Invalid to square")
        };

        // place piece onto from
        if let Some(s) = find_by_x_and_y_mut(&mut self.squares, mov.from) {
            s.kind = moving_piece_kind;
            s.player_number = moving_piece_player_number;
        } else {
            return Err("game_state::undo_move - Invalid from square")
        }

        // capture
        if let Some(capture_piece_kind) = mov.capture_piece_kind {
            if let Some(s) = find_by_x_and_y_mut(&mut self.squares, mov.to) {
                s.kind = capture_piece_kind;
                s.player_number = opposing_player(moving_piece_player_number);
            } else {
                return Err("game_state::undo_move - Invalid to square");
            }
        }

        self.current_player_number = opposing_player(self.current_player_number);

        Ok(())
    }
}

// state player hand move count
// K A C E R H P
// w Capital  b lower case
// castle -
// en passant -
// half move
// full move
// 4kaR2/4a4/3hR4/7H1/9/9/9/9/4Ap1r1/3AK3c w - - 0 1
pub fn parse(encoded: &String) -> Result<GameState, &'static str> {
    let mut read_board = true;
    let mut read_player_number = false;
    let mut read_move_count = false;
    let mut read_castle = false;
    let mut read_en_passant = false;
    let mut parse_error = false;

    let mut y: i8 = 0;
    let mut x: i8 = 0;

    let mut squares: Vec<Vec<Square>> = vec![
        vec![
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty }
        ],
        vec![
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty }
        ],
        vec![
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty }
        ],
        vec![
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty }
        ],
        vec![
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty }
        ],
        vec![
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty }
        ],
        vec![
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty }
        ],
        vec![
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty }
        ],
        vec![
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty }
        ],
        vec![
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty },
            Square { player_number: 0, kind: PieceKind::Empty }
        ]
    ];

    let mut current_player_number = 1;

    for c in encoded.chars() {
        match c {
            'p' | 'P' | 'c' | 'C' | 'r' | 'R' | 'h' | 'H' | 'e' | 'E' | 'a' | 'A' | 'k' | 'K' => {
                if read_board {
                    if let Ok(square) = parse_piece(c) {
                        if valid((x, y)) {
                            squares[y as usize][x as usize] = square;
                        } else {
                            parse_error = true;
                        }
                    } else {
                        parse_error = true;
                    }
                    x += 1;
                } else {
                    parse_error = true;
                }
            },
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if read_board {
                    if let Some(number_of_spaces) = c.to_digit(10) {
                        let mut empty_counter = 0;
                        while empty_counter < number_of_spaces {
                            let square = Square { player_number: 0, kind: PieceKind::Empty };
                            if valid((x, y)) {
                                squares[y as usize][x as usize] = square;
                            } else {
                                parse_error = true;
                            }
                            x += 1; // increment column
                            empty_counter += 1;
                        }
                    } else {
                        parse_error = true;
                    }
                } else if read_move_count {
                    // do nothing
                    ()
                } else {
                    parse_error = true;
                }
            },
            '/' => {
                if read_board {
                    y += 1; // new row
                    x = 0; // reset column
                }
            },
            ' ' => {
                if read_board {
                    // board reading finished
                    read_board = false;
                    read_player_number = true;
                } else if read_player_number {
                    // player reading finished
                    read_player_number = false;
                    read_en_passant = true;
                } else if read_en_passant {
                    read_en_passant = false;
                    read_castle = true;
                } else if read_castle {
                    read_castle = false;
                    read_move_count = true;
                } else if read_move_count {
                    // read_move_count = false;
                }
            },
            'w' => {
                if read_player_number {
                    current_player_number = 1;
                }
            },
            'b' => {
                if read_player_number {
                    current_player_number = 2;
                }
            },
            '-' => {
                if read_castle {
                    () // do nothing
                } else if read_en_passant {
                    () // do nothing
                } else {
                    parse_error = true;
                }
            },
            '0' => (), // ignore for now
            _ => parse_error = true
        }
    }

    if parse_error {
        Err("Error parsing state")
    } else {
        Ok(GameState {
            current_player_number,
            squares
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let encoded = String::from("4kaR2/4a4/3hR4/7H1/9/9/9/9/4Ap1r1/3AK3c w - - 0 1");
        let result = parse(&encoded).unwrap();
        assert_eq!(result.current_player_number, 1);

        assert_eq!(result.squares.len(), 10);
        assert_eq!(result.squares[0][6].kind, PieceKind::Chariot);
        assert_eq!(result.squares[0][6].player_number, 1);

        assert_eq!(result.squares[8][0].kind, PieceKind::Empty);
        assert_eq!(result.squares[8][0].player_number, 0);

        assert_eq!(result.squares[9][8].kind, PieceKind::Cannon);
        assert_eq!(result.squares[9][8].player_number, 2);
    }

    // #[test]
    // fn winner_test() {
    //     let encoded = String::from("1R1k1a3/2R1a4/9/9/9/9/9/9/9/4K4 w - - 0 1");
    //     let mut game_state = parse(&encoded).unwrap();
    //     let result = game_state.winner();
    //     assert_eq!(result, Some(1));
    // }

    // #[test]
    // fn in_checkmate_test() {
    //     let encoded = String::from("1R1k1a3/2R1a4/9/9/9/9/9/9/9/4K4 w - - 0 1");
    //     let game_state = parse(&encoded).unwrap();
    //     let result = game_state.in_checkmate(2);
    //     assert_eq!(result, true);
    // }

    // #[test]
    // fn in_checkmate_threat_can_be_captured_test() {
    //     let encoded = String::from("1R1k1a3/1rR1a4/9/9/9/9/9/9/9/4K4 b - - 0 1");
    //     let game_state = parse(&encoded).unwrap();
    //     let result = game_state.in_checkmate(2);
    //     assert_eq!(result, false);
    // }

    // #[test]
    // fn in_checkmate_threat_can_be_blocked_test() {
    //     let encoded = String::from("1R1k1a3/R3a4/2r6/9/9/9/9/9/9/4K4 b - - 0 1");
    //     let game_state = parse(&encoded).unwrap();
    //     let result = game_state.in_checkmate(2);
    //     assert_eq!(result, false);
    // }

    // #[test]
    // fn king_cannot_move_true_test() {
    //     let encoded = String::from("3k1a3/R3a4/4R4/9/9/9/9/9/9/4K4 b - - 0 1");
    //     let game_state = parse(&encoded).unwrap();
    //     let king_point = (3, 0);
    //     let result = game_state.king_cannot_move(2, king_point);
    //     assert_eq!(result, true);
    // }

    // #[test]
    // fn king_cannot_move_false_test() {
    //     let encoded = String::from("3k1a3/R3a4/9/9/9/9/9/9/9/4K4 b - - 0 1");
    //     let game_state = parse(&encoded).unwrap();
    //     let king_point = (3, 0);
    //     let result = game_state.king_cannot_move(1, king_point);
    //     assert_eq!(result, false);
    // }

    // #[test]
    // fn threats_to_king_can_be_captured_true_test() {
    //     let encoded = String::from("1R1k1a3/Rr2a4/9/9/9/9/9/9/9/4K4 b - - 0 1");
    //     let game_state = parse(&encoded).unwrap();
    //     let king_point = (3, 0);
    //     let result = game_state.threats_to_king_can_be_captured_or_blocked(1, kingu_point);
    //     assert_eq!(result, true);
    // }

    // #[test]
    // fn threats_to_king_can_be_captured_pinned_false_test() {
    //     // horse can block rook, but reveals check through other rook
    //     let encoded = String::from("1Rhk5/9/9/9/3R5/9/9/9/9/4K4 b - - 0 1");
    //     let game_state = parse(&encoded).unwrap();
    //     let king_point = (3, 0);
    //     let result = game_state.threats_to_king_can_be_captured_or_blocked(2, king_point);
    //     assert_eq!(result, false);
    // }

    // #[test]
    // fn threats_to_king_can_be_captured_false_test() {
    //     let encoded = String::from("1R1k1a3/R3a4/9/9/9/9/9/9/9/4K4 b - - 0 1");
    //     let game_state = parse(&encoded).unwrap();
    //     let king_point = (3, 0);
    //     let result = game_state.threats_to_king_can_be_captured_or_blocked(2, king_point);
    //     assert_eq!(result, false);
    // }

    // #[test]
    // fn threats_to_king_can_be_blocked_by_move_true_test() {
    //     let encoded = String::from("1R1k1a3/R1r1a4/9/9/9/9/9/9/9/4K4 b - - 0 1");
    //     let game_state = parse(&encoded).unwrap();
    //     let king_point = (3, 0);
    //     let result = game_state.threats_to_king_can_be_captured_or_blocked(2, king_point);
    //     assert_eq!(result, true);
    // }

    // #[test]
    // fn threats_to_king_can_be_blocked_by_move_pinned_test() {
    //     let encoded = String::from("1R1ak4/9/4R4/9/9/9/9/9/9/5K3 b - - 0 1");
    //     let game_state = parse(&encoded).unwrap();
    //     let king_point = (4, 0);
    //     let result = game_state.threats_to_king_can_be_captured_or_blocked(2, king_point);
    //     assert_eq!(result, false);
    // }

    // #[test]
    // fn threats_to_king_can_be_blocked_false_test() {
    //     let encoded = String::from("1R1k1a3/2R1a4/9/9/9/9/9/9/9/4K4 w - - 0 1");
    //     let game_state = parse(&encoded).unwrap();
    //     let king_point = (3, 0);
    //     let result = game_state.threats_to_king_can_be_captured_or_blocked(2, king_point);
    //     assert_eq!(result, false);
    // }

    #[test]
    fn possible_moves_test() {
        // P 1  * 5 =  5
        // C 12 * 2 = 24
        // R 2  * 2 =  4
        // H 2  * 2 =  4
        // E 2  * 2 =  4
        // A 1  * 2 =  2
        // K 1  * 1 =  1
        //            44
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves();

        assert_eq!(result.len(), 44);

        assert_eq!(result[0].from, (0, 6));
        assert_eq!(result[0].to, (0, 5));
        assert_eq!(result[0].moving_piece_kind, PieceKind::Soldier);
        assert_eq!(result[0].capture_piece_kind, None);

        assert_eq!(result[43].from, (8, 9));
        assert_eq!(result[43].to, (8, 7));
        assert_eq!(result[43].moving_piece_kind, PieceKind::Chariot);
        assert_eq!(result[43].capture_piece_kind, None);
    }

    #[test]
    fn possible_moves_for_player_test() {
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);

        assert_eq!(result.len(), 44);

        assert_eq!(result[0].from, (0, 6));
        assert_eq!(result[0].to, (0, 5));
        assert_eq!(result[0].moving_piece_kind, PieceKind::Soldier);
        assert_eq!(result[0].capture_piece_kind, None);

        assert_eq!(result[43].from, (8, 9));
        assert_eq!(result[43].to, (8, 7));
        assert_eq!(result[43].moving_piece_kind, PieceKind::Chariot);
        assert_eq!(result[43].capture_piece_kind, None);
    }

    #[test]
    fn possible_moves_for_player_checkmate_test() {
        let encoded = String::from("1R1k1a3/1R2a4/9/9/9/9/9/9/9/4K4 w - - 0 1");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(2);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn perform_move_test() {
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 6),
            to: (0, 5),
            moving_piece_kind: PieceKind::Soldier,
            capture_piece_kind: None,
        };
        let result = game_state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 2);
        let from = find_by_x_and_y(&game_state.squares, (0, 6)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);
        let to = find_by_x_and_y(&game_state.squares, (0, 5)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::Soldier);
    }

    #[test]
    fn perform_move_capture_test() {
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            from: (1, 7),
            to: (1, 0),
            moving_piece_kind: PieceKind::Cannon,
            capture_piece_kind: Some(PieceKind::Horse),
        };
        let result = game_state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 2);

        let from = find_by_x_and_y(&game_state.squares, (1, 7)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);

        let to = find_by_x_and_y(&game_state.squares, (1, 0)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::Cannon);
    }

    #[test]
    fn undo_move_test() {
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/P8/2P1P1P1P/1C5C1/9/RHEAKAEHR b - - 0 1");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 6),
            to: (0, 5),
            moving_piece_kind: PieceKind::Soldier,
            capture_piece_kind: None,
        };
        let result = game_state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 1);

        let from = find_by_x_and_y(&game_state.squares, (0, 6)).unwrap();
        assert_eq!(from.player_number, 1);
        assert_eq!(from.kind, PieceKind::Soldier);

        let to = find_by_x_and_y(&game_state.squares, (0, 5)).unwrap();
        assert_eq!(to.player_number, 0);
        assert_eq!(to.kind, PieceKind::Empty);
    }

    #[test]
    fn undo_move_capture_test() {
        let encoded = String::from("rCeakaehr/9/7c1/p1p1p1p1p/9/9/P1P1P1P1P/7C1/9/RHEAKAEHR b - - 0 1");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            from: (1, 7),
            to: (1, 0),
            moving_piece_kind: PieceKind::Cannon,
            capture_piece_kind: Some(PieceKind::Horse)
        };
        let result = game_state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 1);

        let from = find_by_x_and_y(&game_state.squares, (1, 7)).unwrap();
        assert_eq!(from.player_number, 1);
        assert_eq!(from.kind, PieceKind::Cannon);

        let to = find_by_x_and_y(&game_state.squares, (1, 0)).unwrap();
        assert_eq!(to.player_number, 2);
        assert_eq!(to.kind, PieceKind::Horse);
    }
}

