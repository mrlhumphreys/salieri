use crate::shogi::state::point::valid;
use crate::shogi::state::piece_factory::parse as parse_piece;
use crate::shogi::state::square::compulsory_promotion_ranks;
use crate::shogi::state::square::destinations;
use crate::shogi::state::square::Square;
use crate::shogi::state::square::PieceKind;
use crate::shogi::state::square_set::find_by_x_and_y_mut;
use crate::shogi::state::square_set::find_by_x_and_y;
use crate::shogi::state::mov::Move;

#[derive(Clone)]
pub struct GameState {
    pub current_player_number: i8,
    pub squares: Vec<Vec<Square>>,
    pub hands: Vec<Vec<PieceKind>>
}

impl GameState {
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

    pub fn in_check(&self, player_number: i8) -> bool {
        let other_player_number = match player_number {
            1 => 2,
            _ => 1
        };

        let mut check = false;

        let mut king_point: (i8, i8) = (0, 0);

        for (y, row) in self.squares.iter().enumerate() {
            for (x, s) in row.iter().enumerate() {
                if (s.kind == PieceKind::Oushou || s.kind == PieceKind::Gyokushou) && s.player_number == player_number {
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
                        check = square.player_number == other_player_number && destinations(square.kind, square.player_number, (x as i8, y as i8), self).iter().any(|s| *s == king_point );
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
                    for to_point in destinations(from.kind, from.player_number, from_point, &self) {

                        let mut capture_piece_kind: Option<PieceKind> = None;
                        if let Some(to) = find_by_x_and_y(&self.squares, to_point) {
                            if to.player_number != 0 && subject_player_number != to.player_number {
                               capture_piece_kind = Some(to.kind);
                            }

                            let promote = from.promotion_ranks().contains(&to_point.1);
                            let compulsory_promote = compulsory_promotion_ranks(from.kind, from.player_number).contains(&to_point.1);

                            if promote {
                                let mov = Move {
                                    from: Some(from_point),
                                    to: to_point,
                                    moving_piece_kind: from.kind,
                                    capture_piece_kind,
                                    promote
                                };
                                moves.push(mov);
                            }

                            if !compulsory_promote {
                                let mov = Move {
                                    from: Some(from_point),
                                    to: to_point,
                                    moving_piece_kind: from.kind,
                                    capture_piece_kind,
                                    promote: false 
                                };
                                moves.push(mov);
                            }
                        }
                    }
                }
            }
        }

        // drops
        // all piece in players hands
        // all unoccupied squares
        // exclude squares with compulsory promotion
        // exclude squares that put opponents king in check - 
        let hand = &self.hands[subject_player_number as usize];
        let opponent_king_kind = if subject_player_number == 1 {
            PieceKind::Gyokushou
        } else {
            PieceKind::Oushou
        };

        let mut opponent_king_point = (0, 0);

        for (y, row) in self.squares.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                if square.kind == opponent_king_kind {
                    opponent_king_point = (x as i8, y as i8);
                    break;
                }
            }
            if opponent_king_point != (0, 0) {
                break;
            }
        }

        for piece_kind in hand.iter() {
            for (y, row) in self.squares.iter().enumerate() {
                let compulsory_promote = compulsory_promotion_ranks(*piece_kind, subject_player_number).contains(&(y as i8));
                for (x, square) in row.iter().enumerate() {

                    let potential_destinations = destinations(*piece_kind, subject_player_number, (x as i8, y as i8), &self);

                    let puts_in_check = potential_destinations.contains(&opponent_king_point);
                    if square.unoccupied() && !compulsory_promote && !puts_in_check {
                        let mov = Move {
                            from: None,
                            to: (x as i8, y as i8),
                            moving_piece_kind: *piece_kind,
                            capture_piece_kind: None,
                            promote: false 
                        };
                        moves.push(mov);
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

        if let Some(from) = mov.from {
            if let Some(s) = find_by_x_and_y_mut(&mut self.squares, from) {
                if s.occupied() {
                    piece_player_number = s.player_number;
                    piece_kind = s.kind;
                    s.player_number = 0;
                    s.kind = PieceKind::Empty;
                } else {
                    return Err("game_state::perform_move - No piece on from");
                }
            } else {
                return Err("Invalid From Square");
            }
        } else {
            piece_player_number = self.current_player_number;
            piece_kind = mov.moving_piece_kind;

            // remove piece from hand
            let player_hand = &mut self.hands[self.current_player_number as usize];
            if let Some(idx) = player_hand.iter().position(|pk| *pk == piece_kind) {
                player_hand.remove(idx);
            } else {
                return Err("Invalid Drop");
            }
        }

        if let Some(s) = find_by_x_and_y_mut(&mut self.squares, mov.to) {
            // add piece to hand
            if s.kind != PieceKind::Empty {
                let hand = &mut self.hands[self.current_player_number as usize];        
                hand.push(s.kind);
            }
            s.player_number = piece_player_number;
            s.kind = piece_kind;
        } else {
            return Err("Invalid To Square");
        }

        if mov.promote {
           if let Some(s) = find_by_x_and_y_mut(&mut self.squares, mov.to) {
               if let Some(promote_piece_kind) = s.promotes_to() {
                   let promote_piece_player_number = self.current_player_number;
                   s.player_number = promote_piece_player_number;
                   s.kind = promote_piece_kind;
               } else {
                   return Err("Invalid Promote")
               }
           } else {
               return Err("Invalid To Square")
           }
        }

        match self.current_player_number {
            1 => self.current_player_number = 2,
            _ => self.current_player_number = 1
        }

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
            return Err("Invalid To Square")
        };

        // place piece onto from
        if let Some(from) = mov.from {
            if let Some(s) = find_by_x_and_y_mut(&mut self.squares, from) {
                s.kind = moving_piece_kind;
                s.player_number = moving_piece_player_number;
            } else {
                return Err("Invalid From Square")
            }
        } else {
            // undo drop
            let hand = &mut self.hands[moving_piece_player_number as usize];
            hand.push(moving_piece_kind);
        }

        let other_player_number = if moving_piece_player_number == 1 {
            2
        } else {
            1
        };

        // capture
        if let Some(pk) = mov.capture_piece_kind {
            let capture_piece_kind = pk;
            let capture_piece_player_number = other_player_number;

            if let Some(s) = find_by_x_and_y_mut(&mut self.squares, mov.to) {
                s.kind = capture_piece_kind;
                s.player_number = capture_piece_player_number;

                // remove piece from hand
                let hand = &mut self.hands[moving_piece_player_number as usize];
                if let Some(idx) = hand.iter().position(|pk| *pk == capture_piece_kind) {
                    hand.remove(idx);
                } else {
                    return Err("Invalid Drop");
                }
            } else {
                return Err("Invalid To Square");
            }
        }

        if mov.promote {
            if let Some(from) = mov.from {
                if let Some(s) = find_by_x_and_y_mut(&mut self.squares, from) {
                    if let Some(unpromote_piece_kind) = s.demotes_to() {
                        let unpromote_player_number = moving_piece_player_number;
                        s.kind = unpromote_piece_kind;
                        s.player_number = unpromote_player_number;
                    } else {
                        return Err("Invalid Demote");
                    }
                } else {
                    return Err("Invalid To Square");
                }
            } else {
                return Err("From should be present when promoting");
            }
        };

        self.current_player_number = if self.current_player_number == 1 {
            2
        } else {
            1
        };

        Ok(())
    }
}

// state player hand move count 
// lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb
pub fn parse(encoded: &String) -> Result<GameState, &'static str> {
    let mut read_board = true;
    let mut read_player_number = false;
    let mut read_hand = false;
    let mut read_move_count = false;
    let mut parse_error = false;

    let mut y: i8 = 0;
    let mut x: i8 = 0;
    let mut promoted_piece = false;

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
    ];

    let mut current_player_number = 1;
    let mut hands: Vec<Vec<PieceKind>> = vec![
        vec![],
        vec![],
        vec![]
    ];

    for c in encoded.chars() {
        match c {
            'p' | 'P' | 'l' | 'L' | 'n' | 'N' | 's' | 'S' | 'g' | 'G' | 'B' | 'r' | 'R' | 'k' | 'K' => {
                if read_board {
                    match parse_piece(c, promoted_piece) {
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
                    }
                    x += 1;
                    promoted_piece = false;
                } else if read_hand {
                    match parse_piece(c, false) {
                        Ok(p) => {
                           hands[p.player_number as usize].push(p.kind); 
                        },
                        Err(_) => {
                            parse_error = true;
                        }
                    }
                } else {
                    parse_error = true;
                }
            },
            '+' => {
                if read_board {
                    promoted_piece = true;
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
                    } else if read_move_count {
                        // do nothing
                        () 
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
                    read_hand = true;
                } else if read_hand {
                    read_hand = false;
                    read_move_count = true;
                } else if read_move_count {
                    read_move_count = false;
                }
            },
            'w' => {
                if read_player_number {
                    current_player_number = 2;
                }
            },
            'b' => {
                if read_board {
                    match parse_piece(c, promoted_piece) {
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
                    }
                    x += 1;
                    promoted_piece = false;
                } else if read_hand {
                    match parse_piece(c, false) {
                        Ok(p) => {
                           hands[p.player_number as usize].push(p.kind); 
                        },
                        Err(_) => {
                            parse_error = true;
                        }
                    }
                } else if read_player_number {
                    current_player_number = 1;
                }
            },
            '-' => {
                if read_hand {
                    () // do nothing 
                } else {
                    parse_error = true;
                }
            },
            '0' => {
                () // ignore for now
            }, 
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
            hands
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
        let result = parse(&encoded).unwrap();
        assert_eq!(result.current_player_number, 1);

        assert_eq!(result.squares.len(), 9);
        assert_eq!(result.squares[0][0].kind, PieceKind::Kyousha);
        assert_eq!(result.squares[0][0].player_number, 2);

        assert_eq!(result.squares[0][6].kind, PieceKind::Empty);
        assert_eq!(result.squares[0][6].player_number, 0);

        assert_eq!(result.squares[6][0].kind, PieceKind::Fuhyou);
        assert_eq!(result.squares[6][0].player_number, 1);
    }

    #[test]
    fn possible_moves_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves();

        assert_eq!(result.len(), 71);

        assert_eq!(result[0].from, Some((7, 4)));
        assert_eq!(result[0].to, (7, 3));
        assert_eq!(result[0].moving_piece_kind, PieceKind::Fuhyou);
        assert_eq!(result[0].capture_piece_kind, None);
        assert_eq!(result[0].promote, false);

        assert_eq!(result[70].from, None);
        assert_eq!(result[70].to, (3, 8));
        assert_eq!(result[70].moving_piece_kind, PieceKind::Kakugyou);
        assert_eq!(result[70].capture_piece_kind, None);
        assert_eq!(result[70].promote, false);
    }

    #[test]
    fn possible_moves_for_player_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);

        assert_eq!(result.len(), 71);

        assert_eq!(result[0].from, Some((7, 4)));
        assert_eq!(result[0].to, (7, 3));
        assert_eq!(result[0].moving_piece_kind, PieceKind::Fuhyou);
        assert_eq!(result[0].capture_piece_kind, None);
        assert_eq!(result[0].promote, false);

        assert_eq!(result[70].from, None);
        assert_eq!(result[70].to, (3, 8));
        assert_eq!(result[70].moving_piece_kind, PieceKind::Kakugyou);
        assert_eq!(result[70].capture_piece_kind, None);
        assert_eq!(result[70].promote, false);
    }

    #[test]
    fn possible_moves_for_player_drops_test() {
        let encoded = String::from("k8/9/9/9/9/9/9/9/8K b Pp");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);

        assert_eq!(result.len(), 73);
        
        assert_eq!(result[72].from, None);
        assert_eq!(result[72].to, (7, 8));
        assert_eq!(result[72].moving_piece_kind, PieceKind::Fuhyou);
        assert_eq!(result[72].capture_piece_kind, None);
        assert_eq!(result[72].promote, false);
    }

    #[test]
    fn possible_moves_for_player_compulsory_promotion_test() {
        let encoded = String::from("k8/9/9/9/9/9/9/9/8K b Pp");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);

        let mov = result.iter().find(|s| s.to == (1, 0));
        
        assert_eq!(mov.is_none(), true);
    }

    #[test]
    fn possible_moves_for_player_check_test() {
        let encoded = String::from("k8/9/9/9/9/9/9/9/8K b Pp");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);

        assert_eq!(result.len(), 73);

        let mov = result.iter().find(|s| s.to == (0, 1));
        
        assert_eq!(mov.is_none(), true);
    }

    #[test]
    fn perform_move_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            from: Some((3, 6)),
            to: (3, 5),
            moving_piece_kind: PieceKind::Fuhyou,
            capture_piece_kind: None,
            promote: false
        };
        let result = game_state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 2);
        let from = find_by_x_and_y(&game_state.squares, (3, 6)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);
        let to = find_by_x_and_y(&game_state.squares, (3, 5)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::Fuhyou);
    }

    #[test]
    fn perform_move_drop_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            from: None,
            to: (3, 5),
            moving_piece_kind: PieceKind::Kakugyou,
            capture_piece_kind: None,
            promote: false
        };
        let result = game_state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 2);

        assert_eq!(game_state.hands[1].len(), 0);
        assert_eq!(game_state.hands[2][0], PieceKind::Kakugyou);

        let to = find_by_x_and_y(&game_state.squares, (3, 5)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::Kakugyou);
    }

    #[test]
    fn perform_move_capture_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p5p1/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            from: Some((7, 4)),
            to: (7, 3),
            moving_piece_kind: PieceKind::Fuhyou,
            capture_piece_kind: Some(PieceKind::Fuhyou),
            promote: false
        };
        let result = game_state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 2);

        let from = find_by_x_and_y(&game_state.squares, (7, 4)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);

        let to = find_by_x_and_y(&game_state.squares, (7, 3)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::Fuhyou);

        let hand = &game_state.hands[1];
        assert_eq!(hand, &vec![PieceKind::Kakugyou, PieceKind::Fuhyou]);
    }

    #[test]
    fn perform_move_promote_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p5P1/9/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b BPb");
        let mut game_state = parse(&encoded).unwrap();

        let mov = Move {
            from: Some((7, 3)),
            to: (7, 2),
            moving_piece_kind: PieceKind::Fuhyou,
            capture_piece_kind: None,
            promote: true 
        };
        let result = game_state.perform_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 2);

        let from = find_by_x_and_y(&game_state.squares, (7, 3)).unwrap();
        assert_eq!(from.player_number, 0);
        assert_eq!(from.kind, PieceKind::Empty);
        
        let to = find_by_x_and_y(&game_state.squares, (7, 2)).unwrap();
        assert_eq!(to.player_number, 1);
        assert_eq!(to.kind, PieceKind::Tokin);
    }

    #[test]
    fn undo_move_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2PP5/PP2PPP1P/1SG4R1/LN2KGSNL w Bb");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            from: Some((3, 6)),
            to: (3, 5),
            moving_piece_kind: PieceKind::Fuhyou,
            capture_piece_kind: None,
            promote: false
        };
        let result = game_state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 1);
        let from = find_by_x_and_y(&game_state.squares, (3, 6)).unwrap();
        assert_eq!(from.player_number, 1);
        assert_eq!(from.kind, PieceKind::Fuhyou);
        let to = find_by_x_and_y(&game_state.squares, (3, 5)).unwrap();
        assert_eq!(to.player_number, 0);
        assert_eq!(to.kind, PieceKind::Empty);
    }

    #[test]
    fn undo_move_drop_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2PB5/PP1PPPP1P/1SG4R1/LN2KGSNL w b");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            from: None,
            to: (3, 5),
            moving_piece_kind: PieceKind::Kakugyou,
            capture_piece_kind: None,
            promote: false
        };
        let result = game_state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 1);

        assert_eq!(game_state.hands[1].len(), 1);
        assert_eq!(game_state.hands[1][0], PieceKind::Kakugyou);
        assert_eq!(game_state.hands[2][0], PieceKind::Kakugyou);

        let to = find_by_x_and_y(&game_state.squares, (3, 5)).unwrap();
        assert_eq!(to.player_number, 0);
        assert_eq!(to.kind, PieceKind::Empty);
    }

    #[test]
    fn undo_move_capture_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1pp/1p5P1/9/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL w BPb");
        let mut game_state = parse(&encoded).unwrap();
        let mov = Move {
            from: Some((7, 4)),
            to: (7, 3),
            moving_piece_kind: PieceKind::Fuhyou,
            capture_piece_kind: Some(PieceKind::Fuhyou),
            promote: false
        };
        let result = game_state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 1);

        let from = find_by_x_and_y(&game_state.squares, (7, 4)).unwrap();
        assert_eq!(from.player_number, 1);
        assert_eq!(from.kind, PieceKind::Fuhyou);

        let to = find_by_x_and_y(&game_state.squares, (7, 3)).unwrap();
        assert_eq!(to.player_number, 2);
        assert_eq!(to.kind, PieceKind::Fuhyou);

        let hand = &game_state.hands[1];
        assert_eq!(hand, &vec![PieceKind::Kakugyou]);
    }

    #[test]
    fn undo_move_promote_test() {
        let encoded = String::from("lnsgk2nl/1r4gs1/p1pppp1+Pp/1p52/9/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL w Bb");
        let mut game_state = parse(&encoded).unwrap();

        let mov = Move {
            from: Some((7, 3)),
            to: (7, 2),
            moving_piece_kind: PieceKind::Fuhyou,
            capture_piece_kind: None,
            promote: true 
        };
        let result = game_state.undo_move(&mov);

        assert_eq!(result, Ok(()));
        assert_eq!(game_state.current_player_number, 1);

        let from = find_by_x_and_y(&game_state.squares, (7, 3)).unwrap();
        assert_eq!(from.player_number, 1);
        assert_eq!(from.kind, PieceKind::Fuhyou);
        
        let to = find_by_x_and_y(&game_state.squares, (7, 2)).unwrap();
        assert_eq!(to.player_number, 0);
        assert_eq!(to.kind, PieceKind::Empty);
    }
}

