use crate::chess::state::square::Square;
use crate::chess::state::square_set::find_by_x_and_y;
use crate::chess::state::square_set::between_unoccupied;
use crate::chess::state::game_state::GameState;
use crate::chess::state::castle_move::Side;
use crate::chess::state::vector::length;
use crate::chess::state::vector::direction_unit_n;
use crate::chess::state::vector::orthogonal;
use crate::chess::state::vector::diagonal;
use crate::chess::state::vector::orthogonal_or_diagonal;
use crate::chess::state::vector::knight_jump;

const PLAYER_ONE_CASTLE_KING_SIDE: (i8, i8) = (6, 7);
const PLAYER_ONE_CASTLE_QUEEN_SIDE: (i8, i8) = (2, 7);
const PLAYER_TWO_CASTLE_KING_SIDE: (i8, i8) = (6, 0);
const PLAYER_TWO_CASTLE_QUEEN_SIDE: (i8, i8) =  (2, 0);

const PLAYER_ONE_KING_SIDE_ROOK: (i8, i8) = (7, 7);
const PLAYER_ONE_QUEEN_SIDE_ROOK: (i8, i8) = (0, 7);
const PLAYER_TWO_KING_SIDE_ROOK: (i8, i8) = (7, 0);
const PLAYER_TWO_QUEEN_SIDE_ROOK: (i8, i8) = (0, 0);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceKind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub player_number: i8,
    pub kind: PieceKind
}

impl Piece {
    pub fn destinations<'a>(&'a self, from: &'a Square, game_state: &'a GameState) -> Vec<&Square> {
        match self.kind {
            PieceKind::Pawn => {
                self.pawn_destinations(from, game_state)
            },
            PieceKind::Rook => {
                game_state.squares.iter().filter(|to| {
                    orthogonal(from.x, from.y, to.x, to.y) &&
                        to.unoccupied_or_occupied_by_opponent(self.player_number) &&
                        between_unoccupied(&game_state.squares, from, &to)
                }).collect()
            },
            PieceKind::Knight => {
                game_state.squares.iter().filter(|to| {
                    knight_jump(from.x, from.y, to.x, to.y) &&
                        to.unoccupied_or_occupied_by_opponent(self.player_number)
                }).collect()
            },
            PieceKind::Bishop => {
                game_state.squares.iter().filter(|to| {
                    diagonal(from.x, from.y, to.x, to.y) &&
                        to.unoccupied_or_occupied_by_opponent(self.player_number) &&
                        between_unoccupied(&game_state.squares, from, &to)
                }).collect()
            },
            PieceKind::Queen => {
                game_state.squares.iter().filter(|to| {
                    orthogonal_or_diagonal(from.x, from.y, to.x, to.y) &&
                        to.unoccupied_or_occupied_by_opponent(self.player_number) &&
                        between_unoccupied(&game_state.squares, from, &to)
                }).collect()
            },
            PieceKind::King => {
                game_state.squares.iter().filter(|to| {
                    (length(from.x, from.y, to.x, to.y) == 1 &&
                        to.unoccupied_or_occupied_by_opponent(self.player_number)
                    ) ||
                    (length(from.x, from.y, to.x, to.y) == 2 && self.castle_conditions(from, to, game_state))
                }).collect()
            }
        }
    }

    pub fn capture_squares<'a>(&'a self, from: &'a Square, game_state: &'a GameState) -> Vec<&Square> {
        match self.kind {
            PieceKind::Pawn => {
                game_state.squares.iter().filter(|s| {

                    // Move
                    length(from.x, from.y, s.x, s.y) == 1 &&
                        direction_unit_n(from.y, s.y) == self.forwards_direction() &&
                        diagonal(from.x, from.y, s.x, s.y) &&
                        (s.occupied_by_opponent(self.player_number) || self.en_passant_condition(from, s, game_state))
                }).collect()
            },
            PieceKind::King => {
                game_state.squares.iter().filter(|to| {
                    length(from.x, from.y, to.x, to.y) == 1 &&
                        to.unoccupied_or_occupied_by_opponent(self.player_number)
                }).collect()
            },
            _ => {
                self.destinations(from, game_state)
            }
        }
    }

    fn castle_conditions(&self, from: &Square, to: &Square, game_state: &GameState) -> bool {
        match self.player_number {
            1 => {
                match (to.x, to.y) {
                    PLAYER_ONE_CASTLE_KING_SIDE => {
                        let rook_square = find_by_x_and_y(&game_state.squares, PLAYER_ONE_KING_SIDE_ROOK.0, PLAYER_ONE_KING_SIDE_ROOK.1);
                        match rook_square {
                            Some(rs) => {
                                game_state.castle_moves.iter().any(|cm| cm.player_number == 1 && cm.side == Side::King ) &&
                                between_unoccupied(&game_state.squares, from, rs)
                            },
                            None => false
                        }
                    },
                    PLAYER_ONE_CASTLE_QUEEN_SIDE => {
                        let rook_square = find_by_x_and_y(&game_state.squares, PLAYER_ONE_QUEEN_SIDE_ROOK.0, PLAYER_ONE_QUEEN_SIDE_ROOK.1);
                        match rook_square {
                            Some(rs) => {
                                game_state.castle_moves.iter().any(|cm| cm.player_number == 1 && cm.side == Side::Queen ) &&
                                between_unoccupied(&game_state.squares, from, rs)
                            },
                            None => false
                        }
                    },
                    _ => false
                }
            },
            2 => {
                match (to.x, to.y) {
                    PLAYER_TWO_CASTLE_KING_SIDE => {
                        let rook_square = find_by_x_and_y(&game_state.squares, PLAYER_TWO_KING_SIDE_ROOK.0, PLAYER_TWO_KING_SIDE_ROOK.1);
                        match rook_square {
                            Some(rs) => {
                                game_state.castle_moves.iter().any(|cm| cm.player_number == 2 && cm.side == Side::King ) &&
                                between_unoccupied(&game_state.squares, from, rs)
                            },
                            None => false
                        }
                    },
                    PLAYER_TWO_CASTLE_QUEEN_SIDE => {
                        let rook_square = find_by_x_and_y(&game_state.squares, PLAYER_TWO_QUEEN_SIDE_ROOK.0, PLAYER_TWO_QUEEN_SIDE_ROOK.1);
                        match rook_square {
                            Some(rs) => {
                                game_state.castle_moves.iter().any(|cm| cm.player_number == 2 && cm.side == Side::Queen ) &&
                                between_unoccupied(&game_state.squares, from, rs)
                            },
                            None => false
                        }
                    },
                    _ => false
                }
            },
            _ => false
        }
    }

    fn pawn_destinations<'a>(&'a self, from: &'a Square, game_state: &'a GameState) -> Vec<&Square> {
        match self.kind {
            PieceKind::Pawn => {
                game_state.squares.iter().filter(|s| {
                    // Move
                    (length(from.x, from.y, s.x, s.y) <= self.range(from) &&
                     direction_unit_n(from.y, s.y) == self.forwards_direction() &&
                     orthogonal(from.x, from.y, s.x, s.y) &&
                     s.unoccupied() &&
                     between_unoccupied(&game_state.squares, from, &s)
                     ) ||
                    (length(from.x, from.y, s.x, s.y) == 1 &&
                     direction_unit_n(from.y, s.y) == self.forwards_direction() &&
                     diagonal(from.x, from.y, s.x, s.y) &&
                     (s.occupied_by_opponent(self.player_number) || self.en_passant_condition(from, s, game_state)  )
                     )
                }).collect()
            },
            _ => self.destinations(from, game_state)
        }
    }

    fn en_passant_condition(&self, from: &Square, to: &Square, game_state: &GameState) -> bool {
        match game_state.en_passant_target {
            Some(target) => {
                if to.x == target.x {
                    if let Some(capture_square) = game_state.squares.iter().find(|s| s.x == target.x && s.y == from.y) {
                        capture_square.occupied_by_opponent(self.player_number)
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            None => false
        }
    }

    pub fn forwards_direction(&self) -> i8 {
        match self.player_number {
            1 => -1,
            _ => 1,
        }
    }

    pub fn promotion_rank(&self) -> i8 {
        match self.player_number {
            1 => 0,
            _ => 7
        }
    }

    fn range(&self, from: &Square) -> i8 {
        if from.y == self.starting_rank() {
            2
        } else {
            1
        }
    }

    fn starting_rank(&self) -> i8 {
        match self.player_number {
            1 => 6,
            _ => 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::state::game_state::parse as parse_game_state;

    #[test]
    fn destinations_pawn_moves_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 1 };
        let from = Square { x: 4 , y: 6, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected = vec![
                    &Square { x: 4, y: 4, piece: None },
                    &Square { x: 4, y: 5, piece: None },
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_moves_from_non_starting_row_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 1 };
        let from = Square { x: 0, y: 4, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/P/8/1PPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected = vec![
                    &Square { x: 0, y: 3, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_captures_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 1 };
        let from = Square { x: 4 , y: 6, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/8/3p4/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected = vec![
                    &Square { x: 4, y: 4, piece: None },
                    &Square { x: 3, y: 5, piece: Some(Piece { kind: PieceKind::Pawn, player_number: 2}) },
                    &Square { x: 4, y: 5, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_captures_blocked_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 2 };
        let from = Square { x: 0, y: 1, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/pppppppp/P7/8/8/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected: Vec<&Square> = vec![];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_en_passant_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 1 };
        let from = Square { x: 4, y: 3, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/3pP4/8/8/PPPPPPPP/RNBQKBNR w KQkq d6 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected = vec![
                    &Square { x: 3, y: 2, piece: None },
                    &Square { x: 4, y: 2, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_no_en_passant_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 1 };
        let from = Square { x: 4, y: 3, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/3pP4/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = piece.destinations(&from, &game_state);
        let expected = vec![
            &Square { x: 4, y: 2, piece: None }
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_pawn_no_en_passant_same_row_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 1 };
        let from = Square { x: 0, y: 4, piece: Some(piece) };
        let encoded = String::from("4k3/8/8/2p4/P7/8/8/4K3 w - c2 0 1");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = piece.destinations(&from, &game_state);
        let expected = vec![
            &Square { x: 0, y: 3, piece: None }
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_rook_test() {
        let piece = Piece { kind: PieceKind::Rook, player_number: 1 };
        let from = Square { x: 7, y: 7, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected = vec![
                    &Square { x: 7, y: 1, piece: Some(Piece { kind: PieceKind::Pawn, player_number: 2 }) },
                    &Square { x: 7, y: 2, piece: None },
                    &Square { x: 7, y: 3, piece: None },
                    &Square { x: 7, y: 4, piece: None },
                    &Square { x: 7, y: 5, piece: None },
                    &Square { x: 7, y: 6, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_knight_test() {
        let piece = Piece { kind: PieceKind::Knight, player_number: 1 };
        let from = Square { x: 6, y: 7, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected = vec![
                    &Square { x: 5, y: 5, piece: None },
                    &Square { x: 7, y: 5, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_bishop_test() {
        let piece = Piece { kind: PieceKind::Bishop, player_number: 1 };
        let from = Square { x: 5, y: 7, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/p1pppppp/8/1p6/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected = vec![
                    &Square { x: 1, y: 3, piece: Some(Piece { player_number: 2, kind: PieceKind::Pawn }) },
                    &Square { x: 2, y: 4, piece: None },
                    &Square { x: 3, y: 5, piece: None },
                    &Square { x: 4, y: 6, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_queen_test() {
        let piece = Piece { kind: PieceKind::Queen, player_number: 1 };
        let from = Square { x: 3, y: 7, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/3p4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected = vec![
                    &Square { x: 0, y: 4, piece: None },
                    &Square { x: 3, y: 4, piece: Some(Piece { player_number: 2, kind: PieceKind::Pawn }) },
                    &Square { x: 1, y: 5, piece: None },
                    &Square { x: 3, y: 5, piece: None },
                    &Square { x: 2, y: 6, piece: None },
                    &Square { x: 3, y: 6, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_king_normal_test() {
        let piece = Piece { kind: PieceKind::King, player_number: 1 };
        let from = Square { x: 4, y: 7, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/8/8/PPPp1PPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected = vec![
                    &Square { x: 3, y: 6, piece: Some(Piece { player_number: 2, kind: PieceKind::Pawn }) },
                    &Square { x: 4, y: 6, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_king_castle_test() {
        let piece = Piece { kind: PieceKind::King, player_number: 1 };
        let from = Square { x: 4, y: 7, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/5BN1/PPPPPPPP/RNBQK2R w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected = vec![
                    &Square { x: 5, y: 7, piece: None },
                    &Square { x: 6, y: 7, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_king_queen_side_castle_blocked_test() {
        let piece = Piece { kind: PieceKind::King, player_number: 1 };
        let from = Square { x: 4, y: 7, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/5BN1/PPPPPPPP/RN2KBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.destinations(&from, &game_state);
                let expected = vec![
                    &Square { x: 3, y: 7, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn pawn_capture_squares_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 1 };
        let from = Square { x: 4 , y: 6, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/8/3p4/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.capture_squares(&from, &game_state);
                let expected = vec![
                    &Square { x: 3, y: 5, piece: Some(Piece { kind: PieceKind::Pawn, player_number: 2}) }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn king_capture_squares_test() {
        let piece = Piece { kind: PieceKind::King, player_number: 1 };
        let from = Square { x: 4, y: 7, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/5BN1/PPPPPPPP/RNBQK2R w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.capture_squares(&from, &game_state);
                let expected = vec![
                    &Square { x: 5, y: 7, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn other_capture_squares_test() {
        let piece = Piece { kind: PieceKind::Rook, player_number: 1 };
        let from = Square { x: 7, y: 7, piece: Some(piece) };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = piece.capture_squares(&from, &game_state);
                let expected = vec![
                    &Square { x: 7, y: 1, piece: Some(Piece { kind: PieceKind::Pawn, player_number: 2 }) },
                    &Square { x: 7, y: 2, piece: None },
                    &Square { x: 7, y: 3, piece: None },
                    &Square { x: 7, y: 4, piece: None },
                    &Square { x: 7, y: 5, piece: None },
                    &Square { x: 7, y: 6, piece: None }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn forwards_direction_one_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 1 };
        assert_eq!(-1, piece.forwards_direction());
    }

    #[test]
    fn forwards_direction_two_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 2 };
        assert_eq!(1, piece.forwards_direction());
    }

    #[test]
    fn promotion_rank_one_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 1 };
        assert_eq!(0, piece.promotion_rank());
    }

    #[test]
    fn promotion_rank_two_test() {
        let piece = Piece { kind: PieceKind::Pawn, player_number: 2 };
        assert_eq!(7, piece.promotion_rank());
    }
}
