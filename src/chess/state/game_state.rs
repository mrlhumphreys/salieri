use std::convert::TryFrom;
use crate::chess::state::vector::Vector;
use crate::chess::state::point::Point;
use crate::chess::state::mov::Move;
use crate::chess::state::piece_factory::parse as parse_piece;
use crate::chess::state::castle_move::parse as parse_castle_move;
use crate::chess::state::castle_move::CastleMove;
use crate::chess::state::square_set::SquareSet;
use crate::chess::state::square::Square;
use crate::chess::state::piece::PieceKind;
use crate::chess::state::piece::Piece;

const PROMOTE_PIECE_KINDS: [PieceKind; 4] = [
  PieceKind::Queen, PieceKind::Bishop, PieceKind::Knight, PieceKind::Rook
];

pub struct GameState {
    pub current_player_number: i8,
    pub squares: SquareSet,
    pub en_passant_target: Option<Point>,
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
    pub fn in_check(&self, player_number: i8) -> bool {
        let other_player_number = match player_number {
            1 => 2,
            _ => 1
        };

        let mut check = false;

        if let Some(king_square) = self.squares.squares.iter().find(|s| s.piece == Some(Piece {player_number: player_number, kind: PieceKind::King})) {
           for square in self.squares.squares.iter().filter(|s| s.occupied_by_player(other_player_number)) {
               if check {
                   break;
               } else {
                   if let Some(piece) = square.piece { 
                       // if any destinations match king square
                       if piece.destinations(&square, self).iter().any(|s| *s == king_square) {
                           check = true;
                       }
                   }
               }
           }
        } 

        check
    }

    pub fn in_checkmate(&mut self, player_number: i8) -> bool {
        let in_check = self.in_check(player_number);
        let no_moves = self.possible_moves_for_player(player_number).is_empty();
        in_check && no_moves
    }

    pub fn possible_moves(&mut self) -> Vec<Move> {
        self.possible_moves_for_player(self.current_player_number)
    }

    pub fn possible_moves_for_player(&mut self, current_player_number: i8) -> Vec<Move> {
        let mut moves = vec![];
        for from in self.squares.squares.iter() {
            if from.occupied_by_player(current_player_number) {
                let player_number = match &from.piece {
                    Some(p) => p.player_number,
                    None => 0
                };

                let moving_piece = &from.piece;

                for to in from.destinations(&self) {
                    let mut capture_piece_kind = match to.piece {
                        Some(p) => {
                            if player_number != p.player_number {
                                Some(p.kind)   
                            } else {
                                None
                            }
                        },
                        None => None 
                    };

                    // move: en_passant_point 
                    //     - indicates that the move is en_passant
                    //     - contains the landing square of the pawn that jumped previously
                    //     - this pawn can be captured by en passant
                    // state: en_passant_target 
                    //     - indicates that the previous move was a pawn that jumped previously
                    //     - contains the jumped over square of the pawn that jumped previously.
                    let en_passant_point = match self.en_passant_target {
                        Some(_target) => {
                            if let Some(p) = from.piece {
                                if p.kind == PieceKind::Pawn {
                                    // is from and to diagonal, forward and one square apart
                                    let vector = Vector { from: from.point(), to: to.point() };
                                    if  vector.diagonal() && vector.direction_unit().y == p.forwards_direction() {
                                        if let Some(eps) = self.squares.find_by_x_and_y(to.x, from.y) {
                                            if eps.occupied_by_opponent(player_number) { 
                                                capture_piece_kind = Some(PieceKind::Pawn);
                                                Some(Point { y: from.y, x: to.x })
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                   None
                                }
                            } else {
                                None
                            }
                        },
                        None => None
                    };

                    let castle_move = match &from.piece {
                        Some(p) => {
                            if p.kind == PieceKind::King {
                                 let vector = Vector { from: from.point(), to: to.point() };
                                 if vector.length() == 2 {
                                     let side = vector.side();
                                     let cm = CastleMove { player_number: current_player_number, side };
                                     Some(cm)
                                 } else {
                                     None
                                 }
                            } else {
                                None
                            }
                        },
                        None => None
                    };

                    let promote = match from.piece {
                        Some(p) => p.kind == PieceKind::Pawn && to.y == p.promotion_rank(),
                        None => false
                    };

                    match moving_piece {
                        Some(p) => {
                            if promote {
                                for piece_kind in PROMOTE_PIECE_KINDS {
                                    let promote_piece_kind = Some(piece_kind);
                                    let mov = Move { 
                                        from: from.point(), 
                                        to: to.point(), 
                                        moving_piece_kind: p.kind, 
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
                                    from: from.point(), 
                                    to: to.point(), 
                                    moving_piece_kind: p.kind, 
                                    capture_piece_kind, 
                                    promote_piece_kind, 
                                    en_passant_point, 
                                    en_passant_target: self.en_passant_target,
                                    castle_move
                                };
                                moves.push(mov);
                            }
                        },
                        None => ()
                    }
                }
            }
        }

        moves.retain(|m| {
            // calling perform move will change self.current_player_number
            // storing the current player number here for the in_check check
            let player_in_question = self.current_player_number;  
            let perform_result = self.perform_move(&m);
            let in_check = self.in_check(player_in_question);
            let undo_result = self.undo_move(&m); 
            perform_result.is_ok() && undo_result.is_ok() && !in_check 
        });

        moves
    }

    pub fn perform_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        let piece: Option<Piece>;

        match self.squares.squares.iter_mut().find(|s| s.x == mov.from.x && s.y == mov.from.y) {
            Some(s) => {
                if s.occupied() {
                    piece = s.piece.clone();
                } else {
                    return Err("game_state::perform_move - No piece on from");
                }
                s.piece = None
            },
            None => return Err("Invalid From Square")
        }

        match self.squares.squares.iter_mut().find(|s| s.x == mov.to.x && s.y == mov.to.y) {
            Some(s) => s.piece = piece,
            None => return Err("Invalid To Square")
        }

        // maybe use en_passant_target
        match mov.en_passant_point {
            Some(p) => {
                match self.squares.squares.iter_mut().find(|s| s.x == p.x && s.y == p.y) {
                    Some(s) => s.piece = None,
                    None => return Err("Invalid En Passant Square")
                }
            },
            None => () 
        }

        match &mov.castle_move {
            Some(cm) => {
                let from = cm.from();
                let to = cm.to();
                let piece: Option<Piece>;

                match self.squares.squares.iter_mut().find(|s| s.x == from.x && s.y == from.y) {
                   Some(s) => {
                        piece = s.piece.clone(); 
                        s.piece = None;
                   },
                   None => return Err("Invalid From Square")
                }

                match self.squares.squares.iter_mut().find(|s| s.x == to.x && s.y == to.y) {
                    Some(s) => s.piece = piece,
                    None => return Err("Invalid To Square")
                }
                
                self.castle_moves.retain(|&x| x.player_number != cm.player_number)
            },
            None => ()
        }

        match mov.promote_piece_kind {
           Some(pk) => {
               let promote_piece = Piece { kind: pk, player_number: self.current_player_number }; 

               match self.squares.squares.iter_mut().find(|s| s.x == mov.to.x && s.y == mov.to.y) {
                   Some(s) => s.piece = Some(promote_piece),
                   None => return Err("Invalid To Square")
               }
           },
           None => ()
        }
        
        // set en passant target
        if mov.moving_piece_kind == PieceKind::Pawn {
            let vector = Vector { from: mov.from, to: mov.to };
            if vector.length() == 2 {
                let backwards = vector.direction_unit().y*-1;
                let x = mov.to.x;
                let y = mov.to.y + backwards; // 3: 4 -1 or 2 + 1
                self.en_passant_target = Some(Point { x, y });
            } else {
                self.en_passant_target = None;
            }
        } else {
            self.en_passant_target = None;
        }

        match self.current_player_number {
            1 => self.current_player_number = 2,
            _ => self.current_player_number = 1
        }

        Ok(())
    }

    pub fn undo_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        let other_player_number = match self.current_player_number {
            1 => 2,
            _ => 1
        };

        match &mov.promote_piece_kind {
            Some(_) => {
                let unpromote_piece = Piece { kind: PieceKind::Pawn,  player_number: other_player_number };

                match self.squares.squares.iter_mut().find(|s| s.x == mov.to.x && s.y == mov.to.y) {
                    Some(s) => s.piece = Some(unpromote_piece),
                    None => return Err("Invalid To Square")
                }
            },
            None => ()
        };

        match &mov.castle_move {
            Some(cm) => {
                let from = cm.from();
                let to = cm.to();
                let piece: Option<Piece>;

                match self.squares.squares.iter_mut().find(|s| s.x == to.x && s.y == to.y) {
                   Some(s) => {
                        piece = s.piece.clone(); 
                        s.piece = None;
                   },
                   None => return Err("Invalid From Square")
                }

                match self.squares.squares.iter_mut().find(|s| s.x == from.x && s.y == from.y) {
                    Some(s) => s.piece = piece,
                    None => return Err("Invalid To Square")
                }
            },
            None => ()
        };

        let moving_piece: Option<Piece>;

        // move
        match self.squares.squares.iter_mut().find(|s| s.x == mov.to.x && s.y == mov.to.y) {
            Some(s) => {
                moving_piece = s.piece.clone();
                s.piece = None
            },
            None => return Err("Invalid To Square")
        };

        match self.squares.squares.iter_mut().find(|s| s.x == mov.from.x && s.y == mov.from.y) {
            Some(s) => s.piece = moving_piece,
            None => return Err("Invalid From Square")
        };

        // capture
        match mov.capture_piece_kind {
            Some(pk) => {
                // maybe use en passant target
                match mov.en_passant_point {
                    Some(p) => {
                        // en passant
                        match self.squares.squares.iter_mut().find(|s| s.x == p.x && s.y == p.y) {
                            Some(s) => {
                                s.piece = Some(Piece { kind: PieceKind::Pawn, player_number: self.current_player_number });
                            },
                            None => return Err("Invalid En Passant Square")
                        }
                    },
                    None => {
                        // regular capture
                        let capture_piece = Piece { kind: pk, player_number: self.current_player_number };

                        match self.squares.squares.iter_mut().find(|s| s.x == mov.to.x && s.y == mov.to.y) {
                            Some(s) => s.piece = Some(capture_piece),
                            None => return Err("Invalid To Square")
                        };

                    }
                };
            },
            None => ()
        }

        // set en passant target 
        self.en_passant_target = mov.en_passant_target;

        self.current_player_number = other_player_number;

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

    let mut squares = vec![];
    let mut current_player_number = 1;
    let mut castle_moves = vec![];
    let mut en_passant_target = None;
    let mut en_passant_x: i8 = 0;

    for c in encoded.chars() {
        match c {
            'p' | 'P' | 'r' | 'R' | 'n' | 'N' | 'B' => {
                if read_board {
                    match parse_piece(c) {
                        Ok(piece) => {
                            let square = Square { x, y, piece: Some(piece) };
                            squares.push(square);
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
                        Ok(piece) => {
                            let square = Square { x, y, piece: Some(piece) };
                            squares.push(square);
                        },
                        Err(_) => {
                            parse_error = true;
                        }
                    };
                    x += 1; // increment column
                } else if read_castle_moves {
                    match parse_castle_move(c) {
                        Some(cm) => castle_moves.push(cm),
                        None => () 
                    };
                }
            }
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                if read_board {
                    match c.to_digit(10) {
                        Some(number_of_spaces) => {
                            let mut empty_counter = 0;
                            while empty_counter < number_of_spaces {
                                let square = Square { x: x, y: y, piece: None }; 
                                squares.push(square);
                                x += 1; // increment column
                                empty_counter += 1;
                            };
                        },
                        None => {
                            parse_error = true;
                        }
                    }
                } else if read_en_passant {
                    match c.to_digit(10) {
                        Some(unwrapped) => {
                            // row/y/ reverse
                            match i8::try_from(unwrapped) {
                                Ok(i8_num) => {
                                    let en_passant_y = 8 - i8_num;
                                    en_passant_target = Some(Point { x: en_passant_x, y: en_passant_y });                            
                                },
                                Err(_) => ()
                            }; 
                        },
                        None => ()
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
                        Ok(piece) => {
                            let square = Square { x, y, piece: Some(piece) };
                            squares.push(square);
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
            '0' => {
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
            squares: SquareSet { squares }, 
            castle_moves, 
            en_passant_target 
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    use crate::chess::state::castle_move::Side;

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
    fn in_checkmate_test() {
        let encoded = String::from("4k2R/7R/8/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.in_checkmate(2);

        assert_eq!(result, true);
    }

    #[test]
    fn not_in_chemkate_test() {
        let encoded = String::from("4k3/7R/8/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.in_checkmate(2);

        assert_eq!(result, false);
    }

    #[test]
    fn possible_moves_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();

        assert_eq!(result.len(), 20);
        assert_eq!(result[0].from, Point { x: 0 , y: 6 }); 
        assert_eq!(result[0].to, Point { x: 0 , y: 4 }); 
        assert_eq!(result[0].moving_piece_kind, PieceKind::Pawn); 
        assert_eq!(result[0].capture_piece_kind, None); 
        assert_eq!(result[0].promote_piece_kind, None); 
        assert_eq!(result[0].en_passant_point, None); 
        assert_eq!(result[0].castle_move, None); 
    }

    #[test]
    fn possible_moves_two_space_pawn_invalid_test() {
        // en passant move allows extra move?
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
        assert_eq!(result[0].from, Point { x: 0 , y: 6 }); 
        assert_eq!(result[0].to, Point { x: 0 , y: 4 }); 
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
        assert_eq!(result[2].from, Point { x: 0, y: 6 });
        assert_eq!(result[2].to, Point { x: 1, y: 5 });
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
        assert_eq!(result[0].from, Point { x: 0, y: 1 });
        assert_eq!(result[0].to, Point { x: 0, y: 0 });
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
        assert_eq!(result[1].from, Point { x: 0, y: 3 });
        assert_eq!(result[1].to, Point { x: 1, y: 2 });
        assert_eq!(result[1].moving_piece_kind, PieceKind::Pawn);
        assert_eq!(result[1].capture_piece_kind, Some(PieceKind::Pawn));
        assert_eq!(result[1].promote_piece_kind, None); 
        assert_eq!(result[1].en_passant_point, Some(Point { x: 1, y: 3})); 
        assert_eq!(result[1].castle_move, None); 
    }

    #[test]
    fn possible_moves_castle_move_test() {
        let encoded = String::from("4k3/8/8/8/8/8/8/4K2R w K - 0 1");
        let mut state = parse(&encoded).unwrap();
        let result = state.possible_moves();
        
        assert_eq!(result.len(), 15);
        assert_eq!(result[5].from, Point { x: 4, y: 7 });
        assert_eq!(result[5].to, Point { x: 6, y: 7 });
        assert_eq!(result[5].moving_piece_kind, PieceKind::King);
        assert_eq!(result[5].capture_piece_kind, None);
        assert_eq!(result[5].promote_piece_kind, None); 
        assert_eq!(result[5].en_passant_point, None); 
        assert_eq!(result[5].castle_move, Some(CastleMove { player_number: 1, side: Side::King })); 
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
            from: Point { x: 0, y: 6 },
            to: Point { x: 0, y: 5 },
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
        let from = state.squares.find_by_x_and_y(0, 6).unwrap();
        assert_eq!(from.piece, None);
        let to = state.squares.find_by_x_and_y(0, 5).unwrap();
        assert_eq!(to.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Pawn });
        assert_eq!(state.en_passant_target, None);
    }

    #[test]
    fn perform_move_two_space_pawn_test() {
        let encoded = String::from("4k3/8/8/8/8/8/P7/4K3 w - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 6 },
            to: Point { x: 0, y: 4 },
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
        let from = state.squares.find_by_x_and_y(0, 6).unwrap();
        assert_eq!(from.piece, None);
        let to = state.squares.find_by_x_and_y(0, 4).unwrap();
        assert_eq!(to.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Pawn });
        assert_eq!(state.en_passant_target, Some(Point { x: 0, y: 5 }));
    }

    #[test]
    fn perform_move_two_space_pawn_player_two_test() {
        let encoded = String::from("4k3/p7/8/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 1 },
            to: Point { x: 0, y: 3 },
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
        let from = state.squares.find_by_x_and_y(0, 1).unwrap();
        assert_eq!(from.piece, None);
        let to = state.squares.find_by_x_and_y(0, 3).unwrap();
        assert_eq!(to.piece.unwrap(), Piece { player_number: 2, kind: PieceKind::Pawn });
        assert_eq!(state.en_passant_target, Some(Point { x: 0, y: 2 }));
    }

    #[test]
    fn perform_move_capture_test() {
        let encoded = String::from("4k3/8/8/8/8/1p6/P7/4K3 w - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 6 },
            to: Point { x: 1, y: 5 },
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
        let from = state.squares.find_by_x_and_y(0, 6).unwrap();
        assert_eq!(from.piece, None);
        let to = state.squares.find_by_x_and_y(1, 5).unwrap();
        assert_eq!(to.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Pawn });
    }

    #[test]
    fn perform_move_promote_test() {
        let encoded = String::from("4k3/P7/8/8/8/8/8/4K3 w - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 1 },
            to: Point { x: 0, y: 0 },
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
        let from = state.squares.find_by_x_and_y(0, 1).unwrap();
        assert_eq!(from.piece, None);
        let to = state.squares.find_by_x_and_y(0, 0).unwrap();
        assert_eq!(to.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Queen });
    }

    #[test]
    fn perform_move_en_passant_test() {
        let encoded = String::from("4k3/8/8/Pp6/8/8/8/4K3 w - b6 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 3 },
            to: Point { x: 1, y: 2 },
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: Some(PieceKind::Pawn),
            promote_piece_kind: None,
            en_passant_point: Some(Point { x: 1, y: 3 }),
            en_passant_target: Some(Point { x: 1, y: 2 }),
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 2);
        let from = state.squares.find_by_x_and_y(0, 3).unwrap();
        assert_eq!(from.piece, None);
        let to = state.squares.find_by_x_and_y(1, 2).unwrap();
        assert_eq!(to.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Pawn });
        let capture = state.squares.find_by_x_and_y(1, 3).unwrap();
        assert_eq!(capture.piece, None);
    }

    #[test]
    fn peform_move_pawn_moves_two_spaces_test() {
        let encoded = String::from("4k3/8/p7/8/8/8/P7/4K3 w - a6 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 6 },
            to: Point { x: 0, y: 4 },
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: Some(Point { x: 0, y: 2 }),
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.en_passant_target, Some(Point { x: 0, y: 5 }));
    }

    #[test]
    fn peform_move_pawn_moves_two_spaces_player_two_test() {
        let encoded = String::from("4k3/p7/8/8/8/8/P7/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 1 },
            to: Point { x: 0, y: 3 },
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None
        };

        let result = state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.en_passant_target, Some(Point { x: 0, y: 2 }));
    }

    #[test]
    fn perform_move_non_pawn_moves_test() {
        let encoded = String::from("4k3/8/p7/8/P7/8/8/4K2R w - a3 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 7, y: 7 },
            to: Point { x: 7, y: 6 },
            moving_piece_kind: PieceKind::Rook,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: Some(Point { x: 0, y: 5 }),
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
            from: Point { x: 4, y: 7 },
            to: Point { x: 6, y: 7 },
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
        let from = state.squares.find_by_x_and_y(4, 7).unwrap();
        assert_eq!(from.piece, None);
        let to = state.squares.find_by_x_and_y(6, 7).unwrap();
        assert_eq!(to.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::King });

        let from_rook = state.squares.find_by_x_and_y(7, 7).unwrap();
        assert_eq!(from_rook.piece, None);

        let to_rook = state.squares.find_by_x_and_y(5, 7).unwrap();
        assert_eq!(to_rook.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Rook });

        assert_eq!(state.castle_moves, vec![]);
    }

    #[test]
    fn undo_move_test() {
        let encoded = String::from("4k3/8/8/8/8/P7/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 6 },
            to: Point { x: 0, y: 5 },
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
        let from = state.squares.find_by_x_and_y(0, 6).unwrap();
        assert_eq!(from.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Pawn });
        let to = state.squares.find_by_x_and_y(0, 5).unwrap();
        assert_eq!(to.piece, None);
    }

    #[test]
    fn perform_move_and_undo_move_test() {
        let encoded = String::from("r1bqkbnr/p1pppppp/n7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 1");
        let state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 3 },
            to: Point { x: 1, y: 2 },
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: Some(PieceKind::Pawn),
            promote_piece_kind: None,
            en_passant_point: Some(Point { x: 1, y: 3}),
            en_passant_target: Some(Point { x: 1, y: 2}),
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
    fn undo_move_two_player_one_test() {
        let encoded = String::from("4k3/p7/8/8/P7/8/8/4K3 b - a3 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 6 },
            to: Point { x: 0, y: 4 },
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
        let from = state.squares.find_by_x_and_y(0, 6).unwrap();
        assert_eq!(from.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Pawn });
        let to = state.squares.find_by_x_and_y(0, 4).unwrap();
        assert_eq!(to.piece, None);
        assert_eq!(state.en_passant_target, None);
    }

    #[test]
    fn undo_move_two_player_two_test() {
        let encoded = String::from("4k3/8/8/p7/8/P7/8/4K3 w - a6 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 1 },
            to: Point { x: 0, y: 3 },
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
        let from = state.squares.find_by_x_and_y(0, 1).unwrap();
        assert_eq!(from.piece.unwrap(), Piece { player_number: 2, kind: PieceKind::Pawn });
        let to = state.squares.find_by_x_and_y(0, 2).unwrap();
        assert_eq!(to.piece, None);
        assert_eq!(state.en_passant_target, None);
    }

    #[test]
    fn undo_move_capture_test() {
        let encoded = String::from("4k3/8/8/8/8/1P6/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 6 },
            to: Point { x: 1, y: 5 },
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
        let from = state.squares.find_by_x_and_y(0, 6).unwrap();
        assert_eq!(from.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Pawn });
        let to = state.squares.find_by_x_and_y(1, 5).unwrap();
        assert_eq!(to.piece.unwrap(), Piece { player_number: 2, kind: PieceKind::Pawn });
    }

    #[test]
    fn undo_move_promote_test() {
        let encoded = String::from("Q3k3/8/8/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 1 },
            to: Point { x: 0, y: 0 },
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
        let from = state.squares.find_by_x_and_y(0, 1).unwrap();
        assert_eq!(from.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Pawn });
        let to = state.squares.find_by_x_and_y(0, 0).unwrap();
        assert_eq!(to.piece, None);
    }

    #[test]
    fn undo_move_en_passant_test() {
        let encoded = String::from("4k3/8/1P6/8/8/8/8/4K3 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 0, y: 3 },
            to: Point { x: 1, y: 2 },
            moving_piece_kind: PieceKind::Pawn,
            capture_piece_kind: Some(PieceKind::Pawn),
            promote_piece_kind: None,
            en_passant_point: Some(Point { x: 1, y: 3 }),
            en_passant_target: Some(Point { x: 1, y: 2 }),
            castle_move: None
        };

        let result = state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(state.current_player_number, 1);
        let from = state.squares.find_by_x_and_y(0, 3).unwrap();
        assert_eq!(from.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Pawn });
        let to = state.squares.find_by_x_and_y(1, 2).unwrap();
        assert_eq!(to.piece, None);
        let capture = state.squares.find_by_x_and_y(1, 3).unwrap();
        assert_eq!(capture.piece.unwrap(), Piece { player_number: 2, kind: PieceKind::Pawn });
        assert_eq!(state.en_passant_target, Some(Point { x: 1, y: 2 }));
    }

    #[test]
    fn undo_move_castle_move_test() {
        let encoded = String::from("4k3/8/8/8/8/8/8/5RK1 b - - 0 1");
        let mut state = parse(&encoded).unwrap();
        let mov = Move {
            from: Point { x: 4, y: 7 },
            to: Point { x: 6, y: 7 },
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
        let from = state.squares.find_by_x_and_y(4, 7).unwrap();
        assert_eq!(from.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::King });
        let to = state.squares.find_by_x_and_y(6, 7).unwrap();
        assert_eq!(to.piece, None);
        let from_rook = state.squares.find_by_x_and_y(7, 7).unwrap();
        assert_eq!(from_rook.piece.unwrap(), Piece { player_number: 1, kind: PieceKind::Rook });
        let to_rook = state.squares.find_by_x_and_y(5, 7).unwrap();
        assert_eq!(to_rook.piece, None);
    }

    #[test]
    fn parse_test() {
        // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse(&encoded).unwrap();
        assert_eq!(result.current_player_number, 1);

        assert_eq!(result.squares.squares.len(), 64);
        assert_eq!(result.squares.squares[0].piece.unwrap().kind, PieceKind::Rook);
        assert_eq!(result.squares.squares[16].piece, None);

        assert_eq!(result.castle_moves.len(), 4);
        assert_eq!(result.castle_moves[0].player_number, 1);
        assert_eq!(result.castle_moves[0].side, Side::King);

        assert_eq!(result.en_passant_target, None);
    }
} 
