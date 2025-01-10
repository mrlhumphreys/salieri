use std::convert::TryFrom;
use crate::chess::state::point::valid;
use crate::chess::state::point::direction_unit_n;
use crate::chess::state::point::length;
use crate::chess::state::point::side;
use crate::chess::state::mov::Move;
use crate::chess::state::piece_factory::parse as parse_piece;
use crate::chess::state::castle_move::parse as parse_castle_move;
use crate::chess::state::castle_move::CastleMove;
use crate::chess::state::castle_move::Side;
use crate::chess::state::square_set::find_by_x_and_y;
use crate::chess::state::square_set::find_by_x_and_y_mut;
use crate::chess::state::square::Square;
use crate::chess::state::square::PieceKind;

const PROMOTE_PIECE_KINDS: [PieceKind; 4] = [
  PieceKind::Queen, PieceKind::Bishop, PieceKind::Knight, PieceKind::Rook
];

pub struct GameState {
    pub current_player_number: i8,
    pub squares: Vec<Vec<Square>>,
    pub en_passant_target: Option<(i8, i8)>,
    pub castle_moves: Vec<CastleMove>
}

impl Clone for GameState {
    fn clone(&self) -> GameState {
        GameState {
            current_player_number: self.current_player_number,
            squares: self.squares.clone(),
            en_passant_target: self.en_passant_target,
            castle_moves: self.castle_moves.clone()
        }
    }
}

impl GameState {
    // pub fn game_over(&mut self) -> bool {
    //     (self.in_checkmate(1) || self.in_stalemate(1)) ||
    //         (self.in_checkmate(2) || self.in_stalemate(1))
    // }

    pub fn winner(&mut self) -> Option<i8> {
        if self.in_checkmate(1) {
            Some(2)
        } else if self.in_checkmate(2) {
            Some(1)
        } else {
            None
        }
    }

    pub fn in_checkmate(&mut self, player_number: i8) -> bool {
        self.in_check(player_number) && self.possible_moves_for_player(player_number).is_empty()
    }

    // pub fn in_stalemate(&mut self, player_number: i8) -> bool {
    //     let in_check = self.in_check(player_number);
    //     let no_moves = self.possible_moves_for_player(player_number).is_empty();
    //     !in_check && no_moves
    // }

    pub fn in_check(&self, player_number: i8) -> bool {
        let other_player_number = match player_number {
            1 => 2,
            _ => 1
        };

        let mut check = false;

        let mut king_point: (i8, i8) = (0, 0);

        for (y, row) in self.squares.iter().enumerate() {
            for (x, s) in row.iter().enumerate() {
                if s.kind == PieceKind::King && s.player_number == player_number {
                   king_point = (x as i8, y as i8);
                }
            }
        }

        for (y, row) in self.squares.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                if check {
                    break;
                } else {
                    if square.player_number != 0 {
                        // if any capture square match king square
                        check = square.player_number == other_player_number && square.capture_squares((x as i8, y as i8), self).iter().any(|s| *s == king_point );
                    }
                }
            }
        }

        check
    }

    pub fn possible_moves(&mut self) -> Vec<Move> {
        self.possible_moves_for_player(self.current_player_number)
    }

    pub fn possible_moves_for_player(&mut self, subject_player_number: i8) -> Vec<Move> {
        let mut moves = vec![];

        for (y, row) in self.squares.iter().enumerate() {
            for (x, from) in row.iter().enumerate() {
                if from.player_number == subject_player_number {
                    let from_point = (x as i8, y as i8);
                    for to_point in from.destinations(from_point, &self) {

                        let mut capture_piece_kind: Option<PieceKind> = None;
                        if let Some(to) = find_by_x_and_y(&self.squares, to_point) {
                            if to.player_number != 0 {
                               if subject_player_number != to.player_number {
                                   capture_piece_kind = Some(to.kind);
                               }
                            }

                            // move: en_passant_point
                            //     - indicates that the move is en_passant
                            //     - contains the landing square of the pawn that jumped previously
                            //     - this pawn can be captured by en passant
                            // state: en_passant_target
                            //     - indicates that the previous move was a pawn that jumped previously
                            //     - contains the jumped over square of the pawn that jumped previously.
                            let mut en_passant_point: Option<(i8, i8)> = None;
                            if let Some(target) =  self.en_passant_target {
                                if from.kind == PieceKind::Pawn && to_point == target &&
                                    (from_point.0 == target.0 + 1 || from_point.0 == target.0 - 1) &&
                                    from_point.1 + from.forwards_direction() == target.1 {

                                    let capture = (target.0, target.1 - from.forwards_direction());
                                    if let Some(capture_square) = find_by_x_and_y(&self.squares, capture) {
                                        if capture_square.occupied_by_opponent(from.player_number) {
                                            capture_piece_kind = Some(PieceKind::Pawn);
                                            en_passant_point = Some(capture);
                                        }
                                    }
                                }
                            }

                            let mut castle_move: Option<CastleMove> = None;
                            if from.kind == PieceKind::King {
                                // exclude castle move if in check
                                if !self.in_check(subject_player_number) {
                                    if from_point.1 == to_point.1 && length(from_point, to_point) == 2 {
                                        let s = side(from_point.0, to_point.0);
                                        let cm = CastleMove { player_number: subject_player_number, side: s };
                                        castle_move = Some(cm);
                                    }
                                }
                            }

                            let promote = from.kind == PieceKind::Pawn && to_point.1 == from.promotion_rank();

                            if promote {
                                for piece_kind in PROMOTE_PIECE_KINDS {
                                    let promote_piece_kind = Some(piece_kind);
                                    let mov = Move {
                                        from: from_point,
                                        to: to_point,
                                        moving_piece_kind: from.kind,
                                        capture_piece_kind,
                                        promote_piece_kind,
                                        en_passant_point,
                                        en_passant_target: self.en_passant_target,
                                        castle_move
                                    };
                                    moves.push(mov);
                                };
                            } else {
                                let promote_piece_kind = None;
                                let mov = Move {
                                    from: from_point,
                                    to: to_point,
                                    moving_piece_kind: from.kind,
                                    capture_piece_kind,
                                    promote_piece_kind,
                                    en_passant_point,
                                    en_passant_target: self.en_passant_target,
                                    castle_move
                                };
                                moves.push(mov);
                            }
                        }
                    }
                }
            }
        }

        moves.retain(|m| {
            let perform_result = self.perform_move(&m);
            let in_check = self.in_check(subject_player_number);
            let undo_result = self.undo_move(&m);
            perform_result.is_ok() && undo_result.is_ok() && !in_check
        });

        moves
    }

    pub fn perform_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        let piece_player_number: i8;
        let piece_kind: PieceKind;

        match find_by_x_and_y_mut(&mut self.squares, mov.from) {
            Some(s) => {
                if s.occupied() {
                    piece_player_number = s.player_number;
                    piece_kind = s.kind;
                } else {
                    return Err("game_state::perform_move - No piece on from");
                }
                s.player_number = 0;
                s.kind = PieceKind::Empty;
            },
            None => return Err("Invalid From Square")
        }

        match find_by_x_and_y_mut(&mut self.squares, mov.to) {
            Some(s) => {
                s.player_number = piece_player_number;
                s.kind = piece_kind;
            },
            None => return Err("Invalid To Square")
        }

        // maybe use en_passant_target
        if let Some(p) = mov.en_passant_point {
            match find_by_x_and_y_mut(&mut self.squares, p) {
                Some(s) => {
                    s.player_number = 0;
                    s.kind = PieceKind::Empty;
                },
                None => return Err("Invalid En Passant Square")
            }
        }

        if let Some(cm) = &mov.castle_move {
            let from = cm.from();
            let to = cm.to();
            let piece_player_number: i8;
            let piece_kind: PieceKind;

            match find_by_x_and_y_mut(&mut self.squares, from) {
               Some(s) => {
                    piece_player_number = s.player_number;
                    piece_kind = s.kind;
                    s.player_number = 0;
                    s.kind = PieceKind::Empty;
               },
               None => return Err("Invalid From Square")
            }

            match find_by_x_and_y_mut(&mut self.squares, to) {
                Some(s) => {
                    s.kind = piece_kind;
                    s.player_number = piece_player_number;
                },
                None => return Err("Invalid To Square")
            }
        }

        if let Some(pk) =  mov.promote_piece_kind {
           let promote_piece_kind = pk;
           let promote_piece_player_number = self.current_player_number;

           match find_by_x_and_y_mut(&mut self.squares, mov.to) {
               Some(s) => {
                   s.player_number = promote_piece_player_number;
                   s.kind = promote_piece_kind;
               },
               None => return Err("Invalid To Square")
           }
        }

        // set en passant target
        if mov.moving_piece_kind == PieceKind::Pawn && length(mov.from, mov.to) == 2 {
            let backwards = direction_unit_n(mov.from.1, mov.to.1)*-1;
            self.en_passant_target = Some((mov.to.0, mov.to.1 + backwards));
        } else {
            self.en_passant_target = None;
        }

        if mov.moving_piece_kind == PieceKind::Rook {
            match mov.from.0 {
                7 => {
                    if let Some(pos) = self.castle_moves.iter().position(|cm| cm.player_number == self.current_player_number && cm.side == Side::King) {
                        self.castle_moves.remove(pos);
                    }
                },
                0 => {
                    if let Some(pos) = self.castle_moves.iter().position(|cm| cm.player_number == self.current_player_number && cm.side == Side::Queen) {
                        self.castle_moves.remove(pos);
                    }
                },
                _ => ()
            }
        }

        if mov.moving_piece_kind == PieceKind::King {
            let player_number = self.current_player_number;
            self.castle_moves.retain(|cm| cm.player_number != player_number);
        }

        match self.current_player_number {
            1 => self.current_player_number = 2,
            _ => self.current_player_number = 1
        }

        Ok(())
    }

    pub fn undo_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        if let Some(cm) = &mov.castle_move {
            let from = cm.from();
            let to = cm.to();
            let piece_player_number: i8;
            let piece_kind: PieceKind;

            match find_by_x_and_y_mut(&mut self.squares, to) {
               Some(s) => {
                    piece_player_number = s.player_number;
                    piece_kind = s.kind;
                    s.kind = PieceKind::Empty;
                    s.player_number = 0;
               },
               None => return Err("Invalid From Square")
            }

            match find_by_x_and_y_mut(&mut self.squares, from) {
                Some(s) => {
                    s.player_number = piece_player_number;
                    s.kind = piece_kind;
                },
                None => return Err("Invalid To Square")
            }
        };

        let moving_piece_player_number: i8;
        let moving_piece_kind: PieceKind;

        // move
        match find_by_x_and_y_mut(&mut self.squares, mov.to) {
            Some(s) => {
                moving_piece_kind = s.kind;
                moving_piece_player_number = s.player_number;
                s.kind = PieceKind::Empty;
                s.player_number = 0;
            },
            None => return Err("Invalid To Square")
        };

        match find_by_x_and_y_mut(&mut self.squares, mov.from) {
            Some(s) => {
                s.kind = moving_piece_kind;
                s.player_number = moving_piece_player_number;
            },
            None => return Err("Invalid From Square")
        };

        let other_player_number = if moving_piece_player_number == 1 {
            2
        } else {
            1
        };

        // capture
        if let Some(pk) = mov.capture_piece_kind {
            // maybe use en passant target
            match mov.en_passant_point {
                Some(p) => {
                    // en passant
                    match find_by_x_and_y_mut(&mut self.squares, p) {
                        Some(s) => {
                            s.kind = PieceKind::Pawn;
                            s.player_number = other_player_number;
                        },
                        None => return Err("Invalid En Passant Square")
                    }
                },
                None => {
                    // regular capture
                    let capture_piece_kind = pk;
                    let capture_piece_player_number = other_player_number;

                    match find_by_x_and_y_mut(&mut self.squares, mov.to) {
                        Some(s) => {
                            s.kind = capture_piece_kind;
                            s.player_number = capture_piece_player_number;
                        },
                        None => return Err("Invalid To Square")
                    }
                }
            }
        }

        if mov.promote_piece_kind.is_some() {
            let unpromote_piece_kind = PieceKind::Pawn;
            let unpromote_player_number = moving_piece_player_number;

            match find_by_x_and_y_mut(&mut self.squares, mov.from) {
                Some(s) => {
                    s.kind = unpromote_piece_kind;
                    s.player_number = unpromote_player_number;
                },
                None => return Err("Invalid To Square")
            }
        };

        // castle moves
        if mov.moving_piece_kind == PieceKind::King {
            if let Some(from) = find_by_x_and_y(&self.squares, mov.from) {
                if from.player_number == 1 && mov.from == (4, 7) {
                     let castle_move_a = CastleMove { player_number: 1, side: Side::King };
                     let castle_move_b = CastleMove { player_number: 1, side: Side::Queen };
                     if !self.castle_moves.contains(&castle_move_a) {
                         self.castle_moves.push(castle_move_a);
                     }

                     if !self.castle_moves.contains(&castle_move_b) {
                         self.castle_moves.push(castle_move_b);
                     }
                } else if from.player_number == 2 && mov.from == (4, 0) {
                     let castle_move_a = CastleMove { player_number: 2, side: Side::King };
                     let castle_move_b = CastleMove { player_number: 2, side: Side::Queen };

                     if !self.castle_moves.contains(&castle_move_a) {
                         self.castle_moves.push(castle_move_a);
                     }

                     if !self.castle_moves.contains(&castle_move_b) {
                         self.castle_moves.push(castle_move_b);
                     }
                }
            }
        }

        if mov.moving_piece_kind == PieceKind::Rook {
            if let Some(from) = find_by_x_and_y(&self.squares, mov.from) {
                if from.player_number == 1 && mov.from == (0, 7) {
                     let castle_move = CastleMove { player_number: 1, side: Side::Queen };
                     if !self.castle_moves.contains(&castle_move) {
                         self.castle_moves.push(castle_move);
                     }
                } else if from.player_number == 1 && mov.from == (7, 7) {
                     let castle_move = CastleMove { player_number: 1, side: Side::King };
                     if !self.castle_moves.contains(&castle_move) {
                         self.castle_moves.push(castle_move);
                     }
                } else if from.player_number == 2 && mov.from == (0, 0) {
                     let castle_move = CastleMove { player_number: 2, side: Side::Queen };
                     if !self.castle_moves.contains(&castle_move) {
                         self.castle_moves.push(castle_move);
                     }
                } else if from.player_number == 2 && mov.from == (7, 0) {
                     let castle_move = CastleMove { player_number: 2, side: Side::King };
                     if !self.castle_moves.contains(&castle_move) {
                         self.castle_moves.push(castle_move);
                     }
                }
            }
        }

        // set en passant target
        self.en_passant_target = mov.en_passant_target;

        self.current_player_number = if self.current_player_number == 1 {
            2
        } else {
            1
        };

        Ok(())
    }
}

// state player castle en_passant
// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
// rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1
pub fn parse(encoded: &String) -> Result<GameState, &'static str> {
    let mut read_board = true;
    let mut read_player_number = false;
    let mut read_castle_moves = false;
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
            Square { player_number: 0, kind: PieceKind::Empty }
        ]
    ];
    let mut current_player_number = 1;
    let mut castle_moves = vec![];
    let mut en_passant_target = None;
    let mut en_passant_x: i8 = 0;

    for c in encoded.chars() {
        match c {
            'p' | 'P' | 'r' | 'R' | 'n' | 'N' | 'B' => {
                if read_board {
                    match parse_piece(c) {
                        Ok(square) => {
                            if valid((x, y)) {
                                squares[y as usize][x as usize] = square;
                            } else {
                                parse_error = true;
                            }
                        },
                        Err(_) => {
                            parse_error = true;
                        }
                    };
                    x += 1; // increment column
                }
            },
            'q' | 'Q' | 'k' | 'K' => {
                if read_board {
                    match parse_piece(c) {
                        Ok(square) => {
                            if valid((x, y)) {
                                squares[y as usize][x as usize] = square;
                            } else {
                                parse_error = true
                            }
                        },
                        Err(_) => {
                            parse_error = true;
                        }
                    };
                    x += 1; // increment column
                } else if read_castle_moves {
                    if let Some(cm) = parse_castle_move(c) {
                        castle_moves.push(cm);
                    };
                }
            }
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
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
                } else if read_en_passant {
                    if let Some(unwrapped) = c.to_digit(10) {
                        // row/y/ reverse
                        if let Ok(i8_num) = i8::try_from(unwrapped) {
                            let en_passant_y = 8 - i8_num;
                            en_passant_target = Some((en_passant_x, en_passant_y));
                        } else {
                            parse_error = true;
                        }
                    } else {
                        parse_error = true;
                    }
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
                    read_castle_moves = true;
                } else if read_castle_moves {
                    // castle moves reading finished
                    read_castle_moves = false;
                    read_en_passant = true;
                } else if read_en_passant {
                    read_en_passant = false;
                }
            },
            'w' => {
                if read_player_number {
                    current_player_number = 1;
                }
            },
            'b' => {
                if read_board {
                    match parse_piece(c) {
                        Ok(square) => {
                            if valid((x, y)) {
                                squares[y as usize][x as usize] = square;
                            } else {
                                parse_error = true;
                            }
                        },
                        Err(_) => {
                            parse_error = true;
                        }
                    };
                    x += 1; // increment column
                } else if read_player_number {
                    current_player_number = 2;
                } else if read_en_passant {
                    let integer = c as i8; // column/x
                    en_passant_x = integer - 97;
                }
            },
            'a' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => {
                if read_en_passant {
                    let integer = c as i8; // column/x
                    en_passant_x = integer - 97;
                }
            },
            '-' => {
                if read_castle_moves {
                    ()
                } else if read_en_passant {
                    en_passant_target = None;
                }
            }
            '0' | '9' => {
                () //ignore for now
            }
            _ => {
                parse_error = true;
            }
        }
    }

    if parse_error {
        Err("Error parsing state")
    } else {
        Ok(GameState {
            current_player_number,
            squares,
            castle_moves,
            en_passant_target
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::state::castle_move::Side;

    // #[test]
    // fn game_over_test() {
    //     let encoded = String::from("4k2R/7R/8/8/8/8/8/4K3 b - - 0 1");
    //     let mut state = parse(&encoded).unwrap();
    //     let result = state.game_over();

    //     assert_eq!(result, true);
    // }

    // #[test]
    // fn not_game_over_test() {
    //     let encoded = String::from("4k3/7R/8/8/8/8/8/4K3 b - - 0 1");
    //     let mut state = parse(&encoded).unwrap();
    //     let result = state.game_over();

    //     assert_eq!(result, false);
    // }

    #[test]
    fn winner_test() {
        let encoded = String::from("4k2R/7R/8/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.winner();

        assert_eq!(result, Some(1));
    }

    #[test]
    fn not_winner_test() {
        let encoded = String::from("4k3/7R/8/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.winner();

        assert_eq!(result, None);
    }

    #[test]
    fn in_checkmate_test() {
        let encoded = String::from("4k2R/7R/8/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.in_checkmate(2);

        assert_eq!(result, true);
    }

    #[test]
    fn not_in_checkmate_test() {
        let encoded = String::from("4k3/7R/8/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.in_checkmate(2);

        assert_eq!(result, false);
    }

    // #[test]
    // fn in_stalemate_test() {
    //     let encoded = String::from("k7/2Q5/8/8/8/8/8/4K3 b - - 0 1");
    //     let mut state = parse(&encoded).unwrap();
    //     let result = state.in_stalemate(2);

    //     assert_eq!(result, true);
    // }

    // #[test]
    // fn not_in_stalemate_test() {
    //     let encoded = String::from("k7/2R5/8/8/8/8/8/4K3 b - - 0 1");
    //     let mut state = parse(&encoded).unwrap();
    //     let result = state.in_stalemate(2);

    //     assert_eq!(result, false);
    // }

    #[test]
    fn in_check_test() {
        let encoded = String::from("4k3/8/8/8/8/8/8/R3K2r w Q - 0 1");
        let state = parse(&encoded).unwrap();
        let result = state.in_check(1);

        assert_eq!(result, true);
    }

    #[test]
    fn not_in_check_test() {
        let encoded = String::from("4k2r/8/8/8/8/8/8/R3K3 w Qq - 0 1");
        let state = parse(&encoded).unwrap();
        let result = state.in_check(1);

        assert_eq!(result, false);
    }

    #[test]
    fn possible_moves_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 20);
        assert_eq!(result[0].from, (0, 6));
        assert_eq!(result[0].to, (0, 4));
        assert_eq!(result[0].moving_piece_kind, PieceKind::Pawn);
        assert_eq!(result[0].capture_piece_kind, None);
        assert_eq!(result[0].promote_piece_kind, None);
        assert_eq!(result[0].en_passant_point, None);
        assert_eq!(result[0].en_passant_target, None);
        assert_eq!(result[0].castle_move, None);
    }

    #[test]
    fn possible_moves_two_space_pawn_invalid_test() {
        let encoded = String::from("4k3/8/8/2p5/P7/8/8/4K3 w - c2 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 6);
    }

    #[test]
    fn possible_moves_for_player_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves_for_player(1);

        assert_eq!(result.len(), 20);
        assert_eq!(result[0].from, (0, 6));
        assert_eq!(result[0].to, (0, 4));
        assert_eq!(result[0].moving_piece_kind, PieceKind::Pawn);
        assert_eq!(result[0].capture_piece_kind, None);
        assert_eq!(result[0].promote_piece_kind, None);
        assert_eq!(result[0].en_passant_point, None);
        assert_eq!(result[0].castle_move, None);
    }

    #[test]
    fn possible_moves_capture_test() {
        let encoded = String::from("4k3/8/8/8/8/1p6/P7/4K3 w - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 8);
        assert_eq!(result[2].from, (0, 6));
        assert_eq!(result[2].to, (1, 5));
        assert_eq!(result[2].moving_piece_kind, PieceKind::Pawn);
        assert_eq!(result[2].capture_piece_kind, Some(PieceKind::Pawn));
        assert_eq!(result[2].promote_piece_kind, None);
        assert_eq!(result[2].en_passant_point, None);
        assert_eq!(result[2].castle_move, None);
    }

    #[test]
    fn possible_moves_promote_test() {
        let encoded = String::from("4k3/P7/8/8/8/8/8/4K3 w - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 9);
        assert_eq!(result[0].from, (0, 1));
        assert_eq!(result[0].to, (0, 0));
        assert_eq!(result[0].moving_piece_kind, PieceKind::Pawn);
        assert_eq!(result[0].capture_piece_kind, None);
        assert_eq!(result[0].promote_piece_kind, Some(PieceKind::Queen));
        assert_eq!(result[0].en_passant_point, None);
        assert_eq!(result[0].castle_move, None);
    }

    #[test]
    fn possible_moves_en_passant_test() {
        let encoded = String::from("4k3/8/8/Pp6/8/8/8/4K3 w - b6 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 7);
        assert_eq!(result[1].from, (0, 3));
        assert_eq!(result[1].to, (1, 2));
        assert_eq!(result[1].moving_piece_kind, PieceKind::Pawn);
        assert_eq!(result[1].capture_piece_kind, Some(PieceKind::Pawn));
        assert_eq!(result[1].promote_piece_kind, None);
        assert_eq!(result[1].en_passant_point, Some((1, 3)));
        assert_eq!(result[1].en_passant_target, Some((1, 2)));
        assert_eq!(result[1].castle_move, None);
    }

    #[test]
    fn possible_moves_en_passant_two_pawns_in_same_row_test() {
        let encoded = String::from("4k3/8/8/Pp6/8/1p6/Pp6/4K3 w - b6 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 10);
        assert_eq!(result[4].from, (0, 6));
        assert_eq!(result[4].to, (1, 5));
        assert_eq!(result[4].moving_piece_kind, PieceKind::Pawn);
        assert_eq!(result[4].capture_piece_kind, Some(PieceKind::Pawn));
        assert_eq!(result[4].promote_piece_kind, None);
        assert_eq!(result[4].en_passant_point, None);
        assert_eq!(result[4].castle_move, None);
    }

    #[test]
    fn possible_moves_castle_move_test() {
        let encoded = String::from("4k3/8/8/8/8/8/8/4K2R w K - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 15);
        assert_eq!(result[5].from, (4, 7));
        assert_eq!(result[5].to, (6, 7));
        assert_eq!(result[5].moving_piece_kind, PieceKind::King);
        assert_eq!(result[5].capture_piece_kind, None);
        assert_eq!(result[5].promote_piece_kind, None);
        assert_eq!(result[5].en_passant_point, None);
        assert_eq!(result[5].castle_move, Some(CastleMove { player_number: 1, side: Side::King }));
    }

    #[test]
    fn possible_moves_castle_move_blocked_king_side_test() {
        let encoded = String::from("4k3/8/8/8/8/8/8/4KN1R w K - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 16);
    }

    #[test]
    fn possible_moves_castle_move_blocked_queen_side_test() {
        let encoded = String::from("4k3/8/8/8/8/8/8/RN2K3 w Q - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 15);
    }

    #[test]
    fn possible_moves_castle_move_in_check_test() {
        let encoded = String::from("4k3/8/8/8/8/8/8/r3KN1R w K - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn possible_moves_in_check_test() {
        let encoded = String::from("4k2R/8/8/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn perform_move_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 6),
            to: (0, 5),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 2);
        let from = find_by_x_and_y(&state.squares, (0, 6)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);
        let to = find_by_x_and_y(&state.squares, (0, 5)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::Pawn);
        assert_eq!(state.en_passant_target, None);
    }

    #[test]
    fn perform_move_king_test() {
        let encoded = String::from("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (4, 7),
            to: (4, 6),
            moving_piece_kind: PieceKind::King,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.perform_move(&mov);
        let expected = vec![
            CastleMove { player_number: 2, side: Side::King },
            CastleMove { player_number: 2, side: Side::Queen }
        ];
        assert_eq!(result, Ok(()));
        assert_eq!(state.castle_moves, expected);
    }

    #[test]
    fn perform_move_rook_test() {
        let encoded = String::from("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (7, 7),
            to: (7, 6),
            moving_piece_kind: PieceKind::Rook,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };
        let result = state.perform_move(&mov);
        let expected = vec![
            CastleMove { player_number: 1, side: Side::Queen },
            CastleMove { player_number: 2, side: Side::King },
            CastleMove { player_number: 2, side: Side::Queen }
        ];
        assert_eq!(result, Ok(()));
        assert_eq!(state.castle_moves, expected);
    }

    #[test]
    fn perform_move_two_space_pawn_test() {
        let encoded = String::from("4k3/8/8/8/8/8/P7/4K3 w - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 6),
            to: (0, 4),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 2);
        let from = find_by_x_and_y(&state.squares, (0, 6)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);
        let to = find_by_x_and_y(&state.squares, (0, 4)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::Pawn);
        assert_eq!(state.en_passant_target, Some((0, 5)));
    }

    #[test]
    fn perform_move_two_space_pawn_player_two_test() {
        let encoded = String::from("4k3/p7/8/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 1),
            to: (0, 3),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 1);
        let from = find_by_x_and_y(&state.squares, (0, 1)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);
        let to = find_by_x_and_y(&state.squares, (0, 3)).unwrap();
        assert_eq!(to.player_number, 2);
        assert_eq!(to.kind, PieceKind::Pawn);
        assert_eq!(state.en_passant_target, Some((0, 2)));
    }

    #[test]
    fn perform_move_capture_test() {
        let encoded = String::from("4k3/8/8/8/8/1p6/P7/4K3 w - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 6),
            to: (1, 5),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: Some(PieceKind::Pawn),
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 2);
        let from = find_by_x_and_y(&state.squares,(0, 6)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);
        let to = find_by_x_and_y(&state.squares, (1, 5)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::Pawn);
    }

    #[test]
    fn perform_move_promote_test() {
        let encoded = String::from("4k3/P7/8/8/8/8/8/4K3 w - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 1),
            to: (0, 0),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: Some(PieceKind::Queen),
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 2);
        let from = find_by_x_and_y(&state.squares, (0, 1)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);
        let to = find_by_x_and_y(&state.squares, (0, 0)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::Queen);
    }

    #[test]
    fn perform_move_en_passant_test() {
        let encoded = String::from("4k3/8/8/Pp6/8/8/8/4K3 w - b6 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 3),
            to: (1, 2),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: Some(PieceKind::Pawn),
            promote_piece_kind: None,
            en_passant_point: Some((1, 3)),
            en_passant_target: Some((1, 2)),
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 2);
        let from = find_by_x_and_y(&state.squares, (0, 3)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);
        let to = find_by_x_and_y(&state.squares, (1, 2)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::Pawn);
        let capture = find_by_x_and_y(&state.squares, (1, 3)).unwrap();
        assert_eq!(capture.player_number, 0);
        assert_eq!(capture.kind, PieceKind::Empty);
    }

    #[test]
    fn peform_move_pawn_moves_two_spaces_test() {
        let encoded = String::from("4k3/8/p7/8/8/8/P7/4K3 w - a6 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 6),
            to: (0, 4),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: Some((0, 2)),
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.en_passant_target, Some((0, 5)));
    }

    #[test]
    fn peform_move_pawn_moves_two_spaces_player_two_test() {
        let encoded = String::from("4k3/p7/8/8/8/8/P7/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 1),
            to: (0, 3),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.en_passant_target, Some((0, 2)));
    }

    #[test]
    fn perform_move_non_pawn_moves_test() {
        let encoded = String::from("4k3/8/p7/8/P7/8/8/4K2R w - a3 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (7, 7),
            to: (7, 6),
            moving_piece_kind: PieceKind::Rook,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: Some((0, 5)),
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.en_passant_target, None);
    }

    #[test]
    fn perform_move_castle_test() {
        let encoded = String::from("4k3/8/8/8/8/8/8/4K2R w K - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (4, 7),
            to: (6, 7),
            moving_piece_kind: PieceKind::King,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: Some(CastleMove { player_number: 1, side: Side::King })
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 2);
        let from = find_by_x_and_y(&state.squares, (4, 7)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);
        let to = find_by_x_and_y(&state.squares, (6, 7)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::King);

        let from_rook = find_by_x_and_y(&state.squares, (7, 7)).unwrap();
        assert_eq!(from_rook.player_number, 0);
        assert_eq!(from_rook.kind, PieceKind::Empty);

        let to_rook = find_by_x_and_y(&state.squares, (5, 7)).unwrap();
        assert_eq!(to_rook.player_number, 1);
        assert_eq!(to_rook.kind, PieceKind::Rook);

        assert_eq!(state.castle_moves, vec![]);
    }

    #[test]
    fn undo_move_test() {
        let encoded = String::from("4k3/8/8/8/8/P7/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 6),
            to: (0, 5),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 1);
        let from = find_by_x_and_y(&state.squares, (0, 6)).unwrap();
        assert_eq!(from.player_number, 1);
        assert_eq!(from.kind, PieceKind::Pawn);
        let to = find_by_x_and_y(&state.squares, (0, 5)).unwrap();
        assert_eq!(to.player_number, 0);
        assert_eq!(to.kind, PieceKind::Empty);
    }

    #[test]
    fn undo_move_king_test() {
        let encoded = String::from("rnbqkbnr/8/8/8/8/8/4K3/RNBQ1BNR w kq - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (4, 7),
            to: (4, 6),
            moving_piece_kind: PieceKind::King,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.undo_move(&mov);
        let expected = vec![
            CastleMove { player_number: 2, side: Side::King },
            CastleMove { player_number: 2, side: Side::Queen },
            CastleMove { player_number: 1, side: Side::King },
            CastleMove { player_number: 1, side: Side::Queen }
        ];
        assert_eq!(result, Ok(()));
        assert_eq!(state.castle_moves, expected);
    }

    #[test]
    fn undo_move_rook_test() {
        let encoded = String::from("rnbqkbnr/8/8/8/8/8/7R/RNBQKBN1 w Qkq - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (7, 7),
            to: (7, 6),
            moving_piece_kind: PieceKind::Rook,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };
        let result = state.undo_move(&mov);
        let expected = vec![
            CastleMove { player_number: 1, side: Side::Queen },
            CastleMove { player_number: 2, side: Side::King },
            CastleMove { player_number: 2, side: Side::Queen },
            CastleMove { player_number: 1, side: Side::King }
        ];
        assert_eq!(result, Ok(()));
        assert_eq!(state.castle_moves, expected);
    }

    #[test]
    fn perform_move_and_undo_move_test() {
        let encoded = String::from("r1bqkbnr/p1pppppp/n7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 1");
        let state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 3),
            to: (1, 2),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: Some(PieceKind::Pawn),
            promote_piece_kind: None,
            en_passant_point: Some((1, 3)),
            en_passant_target: Some((1, 2)),
            castle_move: None
        };

        let mut new_state = state.clone();

        let perform_result = new_state.perform_move(&mov);
        let undo_result = new_state.undo_move(&mov);

        assert_eq!(perform_result, Ok(()));
        assert_eq!(undo_result, Ok(()));
        assert_eq!(state.current_player_number, new_state.current_player_number);
        assert_eq!(state.squares, new_state.squares);
        assert_eq!(state.en_passant_target, new_state.en_passant_target);
    }

    #[test]
    fn perform_and_undo_multiple_test() {
        let encoded = String::from("rnbqkbnr/p1pppppp/8/1p6/P7/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 1");
        let state_a = parse(&encoded).unwrap();

        let mut state_b = state_a.clone();
        state_b.possible_moves().iter().for_each(|mov_b| {
            let perform_result_b = state_b.perform_move(&mov_b);

            let state_c = state_b.clone();
            state_b.possible_moves().iter().for_each(|mov_c| {
                let perform_result_c = state_b.perform_move(&mov_c);

                let state_d = state_b.clone();
                state_b.possible_moves().iter().for_each(|mov_d| {
                    let perform_result_d = state_b.perform_move(&mov_d);
                    let undo_result_d = state_b.undo_move(&mov_d);
                    assert_eq!(perform_result_d, Ok(()));
                    assert_eq!(undo_result_d, Ok(()));
                    assert_eq!(state_b.current_player_number, state_d.current_player_number);
                    assert_eq!(state_b.squares, state_d.squares);
                    assert_eq!(state_b.en_passant_target, state_d.en_passant_target);
                });
                let undo_result_c = state_b.undo_move(&mov_c);

                assert_eq!(perform_result_c, Ok(()));
                assert_eq!(undo_result_c, Ok(()));
                assert_eq!(state_b.current_player_number, state_c.current_player_number);
                assert_eq!(state_b.squares, state_c.squares);
                assert_eq!(state_b.en_passant_target, state_c.en_passant_target);
            });
            let undo_result_b = state_b.undo_move(&mov_b);

            assert_eq!(perform_result_b, Ok(()));
            assert_eq!(undo_result_b, Ok(()));
            assert_eq!(state_b.current_player_number, state_a.current_player_number);
            assert_eq!(state_b.squares, state_a.squares);
            assert_eq!(state_b.en_passant_target, state_a.en_passant_target);
        });
    }

    #[test]
    fn undo_move_two_player_one_test() {
        let encoded = String::from("4k3/p7/8/8/P7/8/8/4K3 b - a3 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 6),
            to: (0, 4),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 1);
        let from = find_by_x_and_y(&state.squares, (0, 6)).unwrap();
        assert_eq!(from.player_number, 1);
        assert_eq!(from.kind, PieceKind::Pawn);
        let to = find_by_x_and_y(&state.squares, (0, 4)).unwrap();
        assert_eq!(to.player_number, 0);
        assert_eq!(to.kind, PieceKind::Empty);
        assert_eq!(state.en_passant_target, None);
    }

    #[test]
    fn undo_move_two_player_two_test() {
        let encoded = String::from("4k3/8/8/p7/8/P7/8/4K3 w - a6 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 1),
            to: (0, 3),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 2);
        let from = find_by_x_and_y(&state.squares, (0, 1)).unwrap();
        assert_eq!(from.player_number, 2);
        assert_eq!(from.kind, PieceKind::Pawn);
        let to = find_by_x_and_y(&state.squares, (0, 2)).unwrap();
        assert_eq!(to.player_number, 0);
        assert_eq!(to.kind, PieceKind::Empty);
        assert_eq!(state.en_passant_target, None);
    }

    #[test]
    fn undo_move_capture_test() {
        let encoded = String::from("4k3/8/8/8/8/1P6/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 6),
            to: (1, 5),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: Some(PieceKind::Pawn),
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 1);
        let from = find_by_x_and_y(&state.squares, (0, 6)).unwrap();
        assert_eq!(from.player_number, 1);
        assert_eq!(from.kind, PieceKind::Pawn);
        let to = find_by_x_and_y(&state.squares, (1, 5)).unwrap();
        assert_eq!(to.player_number, 2);
        assert_eq!(to.kind, PieceKind::Pawn);
    }

    #[test]
    fn undo_move_promote_test() {
        let encoded = String::from("Q3k3/8/8/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 1),
            to: (0, 0),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: Some(PieceKind::Queen),
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 1);
        let from = find_by_x_and_y(&state.squares, (0, 1)).unwrap();
        assert_eq!(from.player_number, 1);
        assert_eq!(from.kind, PieceKind::Pawn);
        let to = find_by_x_and_y(&state.squares, (0, 0)).unwrap();
        assert_eq!(to.player_number, 0);
        assert_eq!(to.kind, PieceKind::Empty);
    }

    #[test]
    fn undo_move_en_passant_test() {
        let encoded = String::from("4k3/8/1P6/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (0, 3),
            to: (1, 2),
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: Some(PieceKind::Pawn),
            promote_piece_kind: None,
            en_passant_point: Some((1, 3)),
            en_passant_target: Some((1, 2)),
            castle_move: None
        };

        let result = state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 1);
        let from = find_by_x_and_y(&state.squares, (0, 3)).unwrap();
        assert_eq!(from.player_number, 1);
        assert_eq!(from.kind, PieceKind::Pawn);
        let to = find_by_x_and_y(&state.squares, (1, 2)).unwrap();
        assert_eq!(to.player_number, 0);
        assert_eq!(to.kind, PieceKind::Empty);
        let capture = find_by_x_and_y(&state.squares, (1, 3)).unwrap();
        assert_eq!(capture.player_number, 2);
        assert_eq!(capture.kind, PieceKind::Pawn);
        assert_eq!(state.en_passant_target, Some((1, 2)));
    }

    #[test]
    fn undo_move_castle_move_test() {
        let encoded = String::from("4k3/8/8/8/8/8/8/5RK1 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: (4, 7),
            to: (6, 7),
            moving_piece_kind: PieceKind::King,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: Some(CastleMove { player_number: 1, side: Side::King })
        };

        let result = state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 1);
        let from = find_by_x_and_y(&state.squares, (4, 7)).unwrap();
        assert_eq!(from.player_number, 1);
        assert_eq!(from.kind, PieceKind::King);
        let to = find_by_x_and_y(&state.squares, (6, 7)).unwrap();
        assert_eq!(to.player_number, 0);
        assert_eq!(to.kind, PieceKind::Empty);
        let from_rook = find_by_x_and_y(&state.squares, (7, 7)).unwrap();
        assert_eq!(from_rook.player_number, 1);
        assert_eq!(from_rook.kind, PieceKind::Rook);
        let to_rook = find_by_x_and_y(&state.squares, (5, 7)).unwrap();
        assert_eq!(to_rook.player_number, 0);
        assert_eq!(to_rook.kind, PieceKind::Empty);
    }

    #[test]
    fn parse_test() {
        // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse(&encoded).unwrap();
        assert_eq!(result.current_player_number, 1);

        assert_eq!(result.squares.len(), 8);
        assert_eq!(result.squares[0].len(), 8);
        assert_eq!(result.squares[0][0].kind, PieceKind::Rook);
        assert_eq!(result.squares[2][0].player_number, 0);
        assert_eq!(result.squares[2][0].kind, PieceKind::Empty);

        assert_eq!(result.castle_moves.len(), 4);
        assert_eq!(result.castle_moves[0].player_number, 1);
        assert_eq!(result.castle_moves[0].side, Side::King);

        assert_eq!(result.en_passant_target, None);
    }

    #[test]
    fn parse_invalid_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/4pP4/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse(&encoded);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn ninth_turn_test_test() {
        let encoded = String::from("rnb1kb1r/ppp2ppp/3q4/4p3/2PP4/PP6/4PnPP/RNB1KBR1 w KQkq - 0 9");
        let result = parse(&encoded).unwrap();
        assert_eq!(result.current_player_number, 1);

        assert_eq!(result.squares.len(), 8);
        assert_eq!(result.squares[0].len(), 8);
        assert_eq!(result.squares[0][0].kind, PieceKind::Rook);
        assert_eq!(result.squares[2][0].kind, PieceKind::Empty);
        assert_eq!(result.squares[2][0].player_number, 0);

        assert_eq!(result.castle_moves.len(), 4);
        assert_eq!(result.castle_moves[0].player_number, 1);
        assert_eq!(result.castle_moves[0].side, Side::King);

        assert_eq!(result.en_passant_target, None);
    }
}
