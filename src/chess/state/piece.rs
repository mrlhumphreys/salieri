use crate::chess::state::square::Square;
use crate::chess::state::game_state::GameState;
use crate::chess::state::castle_move::Side;
use crate::chess::state::vector::Vector;
use crate::chess::state::point::Point;

const PLAYER_ONE_CASTLE_KING_SIDE_POINT: Point = Point { x: 6, y: 7 };
const PLAYER_ONE_CASTLE_QUEEN_SIDE_POINT: Point = Point { x: 2, y: 7 };
const PLAYER_TWO_CASTLE_KING_SIDE_POINT: Point = Point { x: 6, y: 0 };
const PLAYER_TWO_CASTLE_QUEEN_SIDE_POINT: Point = Point { x: 2, y: 0 };

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
                game_state.squares.squares.iter().filter(|to| {
                    let vector = Vector { from: from.point(), to: to.point() };
                    vector.orthogonal() && 
                        to.unoccupied_or_occupied_by_opponent(self.player_number) && 
                        game_state.squares.between_unoccupied(from, &to)  
                }).collect()
            },
            PieceKind::Knight => {
                game_state.squares.squares.iter().filter(|to| {
                    let vector = Vector { from: from.point(), to: to.point() };
                    vector.not_orthogonal_or_diagonal() && 
                        vector.length() == 2 && 
                        to.unoccupied_or_occupied_by_opponent(self.player_number) 
                }).collect()
            },
            PieceKind::Bishop => {
                game_state.squares.squares.iter().filter(|to| {
                    let vector = Vector { from: from.point(), to: to.point() };
                    vector.diagonal() && 
                        to.unoccupied_or_occupied_by_opponent(self.player_number) && 
                        game_state.squares.between_unoccupied(from, &to)
                }).collect()
            },
            PieceKind::Queen => {
                game_state.squares.squares.iter().filter(|to| {
                    let vector = Vector { from: from.point(), to: to.point() };
                    vector.orthogonal_or_diagonal() && 
                        to.unoccupied_or_occupied_by_opponent(self.player_number) && 
                        game_state.squares.between_unoccupied(from, &to)
                }).collect()
            },
            PieceKind::King => {
                game_state.squares.squares.iter().filter(|to| {
                    let vector = Vector { from: from.point(), to: to.point() };
                    (vector.length() == 1 && 
                        to.unoccupied_or_occupied_by_opponent(self.player_number)
                    ) ||
                    (vector.length() == 2 && self.castle_conditions(from, to, game_state))
                }).collect()
            }
        }
    }

    fn castle_conditions(&self, from: &Square, to: &Square, game_state: &GameState) -> bool {
        match self.player_number {
            1 => {
                (to.point() == PLAYER_ONE_CASTLE_KING_SIDE_POINT && 
                 game_state.castle_moves.iter().any(|cm| cm.player_number == 1 && cm.side == Side::King ) && 
                 to.unoccupied() && 
                 game_state.squares.between_unoccupied(from, to)
                 ) || 
                 (to.point() == PLAYER_ONE_CASTLE_QUEEN_SIDE_POINT &&
                 game_state.castle_moves.iter().any(|cm| cm.player_number == 1 && cm.side == Side::Queen ) && 
                 to.unoccupied() && 
                 game_state.squares.between_unoccupied(from, to)
                 )
            },
            2 => {
                (to.point() == PLAYER_TWO_CASTLE_KING_SIDE_POINT && 
                 game_state.castle_moves.iter().any(|cm| cm.player_number == 2 && cm.side == Side::King ) && 
                 to.unoccupied() && 
                 game_state.squares.between_unoccupied(from, to)
                 ) || 
                 (to.point() == PLAYER_TWO_CASTLE_QUEEN_SIDE_POINT &&
                 game_state.castle_moves.iter().any(|cm| cm.player_number == 2 && cm.side == Side::Queen ) && 
                 to.unoccupied() && 
                 game_state.squares.between_unoccupied(from, to)
                 )
            },
            _ => false
        }
    }

    fn pawn_destinations<'a>(&'a self, from: &'a Square, game_state: &'a GameState) -> Vec<&Square> {
        match self.kind {
            PieceKind::Pawn => {
                game_state.squares.squares.iter().filter(|s| {
                    let vector = Vector { from: from.point(), to: s.point() };
                    
                    // Move
                    (vector.length() <= self.range(from) && 
                     vector.direction_unit().y == self.forwards_direction() && 
                     vector.orthogonal() && s.unoccupied() && 
                     game_state.squares.between_unoccupied(from, &s)
                     ) ||
                    (vector.length() == 1 && 
                     vector.direction_unit().y == self.forwards_direction() && 
                     vector.diagonal() && 
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
                    let capture_point = Point { x: target.x, y: from.y };
                    if let Some(capture_square) = game_state.squares.squares.iter().find(|s| s.point() == capture_point) {
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
