use crate::chess::state::point::Point;
use crate::chess::state::vector::orthogonal;
use crate::chess::state::vector::diagonal;
use crate::chess::state::vector::orthogonal_or_diagonal;
use crate::chess::state::vector::knight_jump;
use crate::chess::state::square_set::between_unoccupied;
use crate::chess::state::game_state::GameState;
use crate::chess::state::castle_move::Side;

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
    Pawn,
    Empty
}

#[derive(Copy, Debug, PartialEq)]
pub struct Square {
    pub x: i8,
    pub y: i8,
    pub player_number: i8,
    pub kind: PieceKind
}

impl Clone for Square {
    fn clone(&self) -> Square {
        Square {
            x: self.x,
            y: self.y,
            player_number: self.player_number,
            kind: self.kind
        }
    }
}

impl Square {
    pub fn point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn occupied(&self) -> bool {
        self.player_number != 0
    }

    pub fn unoccupied(&self) -> bool {
        self.player_number == 0
    }

    pub fn occupied_by_opponent(&self, player_number: i8) -> bool {
        self.player_number != 0 && self.player_number != player_number
    }

    pub fn unoccupied_or_occupied_by_opponent(&self, player_number: i8) -> bool {
        self.player_number == 0 || self.player_number != player_number
    }

    pub fn destinations<'a>(&'a self, game_state: &'a GameState) -> Vec<&Square> {
        match self.kind {
            PieceKind::Empty => {
                vec![]
            }
            PieceKind::Pawn => {
                self.pawn_destinations(game_state)
            },
            PieceKind::Rook => {
                let mut acc = vec![];
                for row in game_state.squares.iter() {
                    for to in row.iter() {
                        if orthogonal(self.x, self.y, to.x, to.y) &&
                            to.unoccupied_or_occupied_by_opponent(self.player_number) &&
                            between_unoccupied(&game_state.squares, (self.x, self.y), (to.x, to.y)) {
                            acc.push(to);
                        }
                    }
                }
                acc
            },
            PieceKind::Knight => {
                let mut acc = vec![];
                for row in game_state.squares.iter() {
                    for to in row.iter() {
                        if knight_jump(self.x, self.y, to.x, to.y) &&
                            to.unoccupied_or_occupied_by_opponent(self.player_number) {
                            acc.push(to);
                        }
                    }
                }
                acc
            },
            PieceKind::Bishop => {
                let mut acc = vec![];
                for row in game_state.squares.iter() {
                    for to in row.iter() {
                        if diagonal(self.x, self.y, to.x, to.y) &&
                            to.unoccupied_or_occupied_by_opponent(self.player_number) &&
                            between_unoccupied(&game_state.squares, (self.x, self.y), (to.x, to.y)) {
                            acc.push(to);
                        }
                    }
                }
                acc
            },
            PieceKind::Queen => {
                let mut acc = vec![];
                for row in game_state.squares.iter() {
                    for to in row.iter() {
                        if orthogonal_or_diagonal(self.x, self.y, to.x, to.y) &&
                            to.unoccupied_or_occupied_by_opponent(self.player_number) &&
                            between_unoccupied(&game_state.squares, (self.x, self.y), (to.x, to.y)) {
                            acc.push(to);
                        }
                    }
                }
                acc
            },
            PieceKind::King => {
                let min_x = self.x - 1;
                let max_x = self.x + 1;
                let min_y = self.y - 1;
                let max_y = self.y + 1;
                let mut acc = vec![];
                for row in game_state.squares.iter() {
                    for to in row.iter() {
                        if ((to.x >= min_x && to.x <= max_x && to.y >= min_y && to.y <= max_y) &&
                            to.unoccupied_or_occupied_by_opponent(self.player_number)
                        ) ||
                        (self.castle_conditions(to, game_state)) {
                            acc.push(to);
                        }
                    }
                }
                acc
            }
        }
    }

    pub fn capture_squares<'a>(&'a self, game_state: &'a GameState) -> Vec<&Square> {
        match self.kind {
            PieceKind::Pawn => {
                let to_x_a = self.x + 1;
                let to_x_b = self.x - 1;
                let to_y = self.y + self.forwards_direction();
                let mut acc = vec![];
                for row in game_state.squares.iter() {
                    for to in row.iter() {
                        if (to.x == to_x_a || to.x == to_x_b) && to.y == to_y &&
                        (to.occupied_by_opponent(self.player_number) || self.en_passant_condition(to, game_state)) {
                            acc.push(to);
                        }
                    }
                }
                acc
            },
            PieceKind::King => {
                let min_x = self.x - 1;
                let max_x = self.x + 1;
                let min_y = self.y - 1;
                let max_y = self.y + 1;
                let mut acc = vec![];
                for row in game_state.squares.iter() {
                    for to in row.iter() {
                        if (to.x >= min_x && to.x <= max_x && to.y >= min_y && to.y <= max_y) &&
                        to.unoccupied_or_occupied_by_opponent(self.player_number) {
                            acc.push(to);
                        }
                    }
                }
                acc
            },
            _ => {
                self.destinations(game_state)
            }
        }
    }

    fn castle_conditions(&self, to: &Square, game_state: &GameState) -> bool {
        match self.player_number {
            1 => {
                match (to.x, to.y) {
                    PLAYER_ONE_CASTLE_KING_SIDE => {
                        game_state.castle_moves.iter().any(|cm| cm.player_number == 1 && cm.side == Side::King) &&
                            between_unoccupied(&game_state.squares, (self.x, self.y), PLAYER_ONE_KING_SIDE_ROOK)
                    },
                    PLAYER_ONE_CASTLE_QUEEN_SIDE => {
                        game_state.castle_moves.iter().any(|cm| cm.player_number == 1 && cm.side == Side::Queen) &&
                            between_unoccupied(&game_state.squares, (self.x, self.y), PLAYER_ONE_QUEEN_SIDE_ROOK)
                    },
                    _ => false
                }
            },
            2 => {
                match (to.x, to.y) {
                    PLAYER_TWO_CASTLE_KING_SIDE => {
                        game_state.castle_moves.iter().any(|cm| cm.player_number == 2 && cm.side == Side::King) &&
                            between_unoccupied(&game_state.squares, (self.x, self.y), PLAYER_TWO_KING_SIDE_ROOK)
                    },
                    PLAYER_TWO_CASTLE_QUEEN_SIDE => {
                        game_state.castle_moves.iter().any(|cm| cm.player_number == 2 && cm.side == Side::Queen) &&
                            between_unoccupied(&game_state.squares, (self.x, self.y), PLAYER_TWO_QUEEN_SIDE_ROOK)
                    },
                    _ => false
                }
            },
            _ => false
        }
    }

    fn pawn_destinations<'a>(&'a self, game_state: &'a GameState) -> Vec<&Square> {
        match self.kind {
            PieceKind::Pawn => {
                let capture_x_a = self.x + 1;
                let capture_x_b = self.x - 1;
                let move_x = self.x;
                let move_capture_y = self.y + self.forwards_direction();
                let move_double_y = self.y + 2*self.forwards_direction();
                let r = self.range();
                let mut acc = vec![];
                for row in game_state.squares.iter() {
                    for to in row.iter() {
                        // Move
                        if (to.x == move_x && (r == 2 && to.y == move_double_y || to.y == move_capture_y) &&
                            to.unoccupied() &&
                            between_unoccupied(&game_state.squares, (self.x, self.y), (to.x, to.y))
                        ) ||
                        ((to.x == capture_x_a || to.x == capture_x_b) && to.y == move_capture_y &&
                            (to.occupied_by_opponent(self.player_number) || self.en_passant_condition(to, game_state))
                        ) {
                            acc.push(to);
                        }
                    }
                }
                acc
            },
            _ => self.destinations(game_state)
        }
    }

    fn en_passant_condition(&self, to: &Square, game_state: &GameState) -> bool {
        if let Some(target) = game_state.en_passant_target {
            if to.x == target.x && to.y == target.y {
                let mut result = false;
                for row in game_state.squares.iter() {
                    for to in row.iter() {
                         if to.x == target.x && to.y == self.y && to.occupied_by_opponent(self.player_number) {
                            result = true;
                         }
                    }
                }
                result
            } else {
                false
            }
        } else {
            false
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

    fn range(&self) -> i8 {
        if self.y == self.starting_rank() {
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
    fn point_test() {
        let square = Square { x: 1, y: 2, player_number: 0, kind: PieceKind::Empty };
        let expected = Point { x: 1, y: 2 };
        let result = square.point();
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_some_test() {
        let square = Square { x: 1, y: 2,  player_number: 1, kind: PieceKind::Pawn };
        let expected = true;
        let result = square.occupied();
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_none_test() {
        let square = Square { x: 1, y: 2, player_number: 0, kind: PieceKind::Empty };
        let expected = false;
        let result = square.occupied();
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_one_test() {
        let square = Square { x: 1, y: 2, player_number: 1, kind: PieceKind::Pawn };
        let expected = false;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_two_test() {
        let square = Square { x: 1, y: 2, player_number: 2, kind: PieceKind::Pawn };
        let expected = true;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_none_test() {
        let square = Square { x: 1, y: 2, player_number: 0, kind: PieceKind::Empty };
        let expected = false;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_or_occupied_by_opponent_one_test() {
        let square = Square { x: 1, y: 2, player_number: 1, kind: PieceKind::Pawn };
        let expected = false;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_or_occupied_by_opponent_two_test() {
        let square = Square { x: 1, y: 2, player_number: 2, kind: PieceKind::Pawn };
        let expected = true;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_or_occupied_by_opponent_none_test() {
        let square = Square { x: 1, y: 2, player_number: 0, kind: PieceKind::Empty };
        let expected = true;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_some_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let game_state = parse_game_state(&encoded).unwrap();
        let square = Square { x: 4, y: 6, player_number: 1, kind: PieceKind::Pawn };
        let expected = 2;
        let result = square.destinations(&game_state).len();
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_none_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let game_state = parse_game_state(&encoded).unwrap();
        let square = Square { x: 4, y: 5, player_number: 0, kind: PieceKind::Empty };
        let expected = 0;
        let result = square.destinations(&game_state).len();
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_pawn_moves_test() {
        let from = Square { x: 4 , y: 6, kind: PieceKind::Pawn, player_number: 1 };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected = vec![
                    &Square { x: 4, y: 4, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 4, y: 5, player_number: 0, kind: PieceKind::Empty },
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_moves_from_non_starting_row_test() {
        let from = Square { x: 0, y: 4, kind: PieceKind::Pawn, player_number: 1 };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/P/8/1PPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected = vec![
                    &Square { x: 0, y: 3, player_number: 0, kind: PieceKind::Empty }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_captures_test() {
        let from = Square { x: 4 , y: 6, kind: PieceKind::Pawn, player_number: 1 };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/8/3p4/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected = vec![
                    &Square { x: 4, y: 4, kind: PieceKind::Empty, player_number: 0 },
                    &Square { x: 3, y: 5, kind: PieceKind::Pawn, player_number: 2 },
                    &Square { x: 4, y: 5, kind: PieceKind::Empty, player_number: 0 }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_captures_blocked_test() {
        let from = Square { x: 0, y: 1, kind: PieceKind::Pawn, player_number: 2 };
        let encoded = String::from("rnbqkbnr/pppppppp/P7/8/8/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected: Vec<&Square> = vec![];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_en_passant_test() {
        let from = Square { x: 4, y: 3, kind: PieceKind::Pawn, player_number: 1 };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPPPPPP/RNBQKBNR w KQkq d6 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected = vec![
                    &Square { x: 3, y: 2, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 4, y: 2, player_number: 0, kind: PieceKind::Empty }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_no_en_passant_test() {
        let from = Square { x: 4, y: 3, kind: PieceKind::Pawn, player_number: 1 };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = from.destinations(&game_state);
        let expected = vec![
            &Square { x: 4, y: 2, player_number: 0, kind: PieceKind::Empty }
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_pawn_no_en_passant_same_row_test() {
        let from = Square { x: 0, y: 4, kind: PieceKind::Pawn, player_number: 1 };
        let encoded = String::from("4k3/8/8/2p4/P7/8/8/4K3 w - c2 0 1");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = from.destinations(&game_state);
        let expected = vec![
            &Square { x: 0, y: 3, player_number: 0, kind: PieceKind::Empty }
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_pawn_en_passant_same_column_test() {
        let from = Square { x: 1, y: 6, kind: PieceKind::Pawn, player_number: 1 };
        let encoded = String::from("4k3/8/8/1Pp4/8/8/1Pp4/4K3 w - c2 0 1");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = from.destinations(&game_state);
        let expected = vec![
            &Square { x: 1, y: 4, player_number: 0, kind: PieceKind::Empty },
            &Square { x: 1, y: 5, player_number: 0, kind: PieceKind::Empty }
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_rook_test() {
        let from = Square { x: 7, y: 7, kind: PieceKind::Rook, player_number: 1 };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected = vec![
                    &Square { x: 7, y: 1, kind: PieceKind::Pawn, player_number: 2 },
                    &Square { x: 7, y: 2, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 7, y: 3, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 7, y: 4, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 7, y: 5, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 7, y: 6, player_number: 0, kind: PieceKind::Empty }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_knight_test() {
        let from = Square { x: 6, y: 7, kind: PieceKind::Knight, player_number: 1 };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected = vec![
                    &Square { x: 5, y: 5, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 7, y: 5, player_number: 0, kind: PieceKind::Empty }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_bishop_test() {
        let from = Square { x: 5, y: 7, kind: PieceKind::Bishop, player_number: 1 };
        let encoded = String::from("rnbqkbnr/p1pppppp/8/1p6/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected = vec![
                    &Square { x: 1, y: 3, player_number: 2, kind: PieceKind::Pawn },
                    &Square { x: 2, y: 4, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 3, y: 5, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 4, y: 6, player_number: 0, kind: PieceKind::Empty }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_queen_test() {
        let from = Square { x: 3, y: 7, kind: PieceKind::Queen, player_number: 1 };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/3p4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected = vec![
                    &Square { x: 0, y: 4, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 3, y: 4, player_number: 2, kind: PieceKind::Pawn },
                    &Square { x: 1, y: 5, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 3, y: 5, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 2, y: 6, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 3, y: 6, player_number: 0, kind: PieceKind::Empty }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_king_normal_test() {
        let from = Square { x: 4, y: 7, kind: PieceKind::King, player_number: 1 };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/8/8/PPPp1PPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected = vec![
                    &Square { x: 3, y: 6, player_number: 2, kind: PieceKind::Pawn },
                    &Square { x: 4, y: 6, player_number: 0, kind: PieceKind::Empty }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_king_castle_test() {
        let from = Square { x: 4, y: 7, kind: PieceKind::King, player_number: 1 };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/5BN1/PPPPPPPP/RNBQK2R w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected = vec![
                    &Square { x: 5, y: 7, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 6, y: 7, player_number: 0, kind: PieceKind::Empty }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_king_queen_side_castle_blocked_test() {
        let from = Square { x: 4, y: 7, kind: PieceKind::King, player_number: 1 };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/5BN1/PPPPPPPP/RN2KBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(&game_state);
                let expected = vec![
                    &Square { x: 3, y: 7, player_number: 0, kind: PieceKind::Empty }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn pawn_capture_squares_test() {
        let from = Square { x: 4 , y: 6, kind: PieceKind::Pawn, player_number: 1 };
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/8/3p4/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.capture_squares(&game_state);
                let expected = vec![
                    &Square { x: 3, y: 5, kind: PieceKind::Pawn, player_number: 2 }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn king_capture_squares_test() {
        let from = Square { x: 4, y: 7, kind: PieceKind::King, player_number: 1 };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/5BN1/PPPPPPPP/RNBQK2R w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.capture_squares(&game_state);
                let expected = vec![
                    &Square { x: 5, y: 7, player_number: 0, kind: PieceKind::Empty }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn other_capture_squares_test() {
        let from = Square { x: 7, y: 7, kind: PieceKind::Rook, player_number: 1 };
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.capture_squares(&game_state);
                let expected = vec![
                    &Square { x: 7, y: 1, kind: PieceKind::Pawn, player_number: 2 },
                    &Square { x: 7, y: 2, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 7, y: 3, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 7, y: 4, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 7, y: 5, player_number: 0, kind: PieceKind::Empty },
                    &Square { x: 7, y: 6, player_number: 0, kind: PieceKind::Empty }
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn forwards_direction_one_test() {
        let square = Square { x: 0, y: 0, kind: PieceKind::Pawn, player_number: 1 };
        assert_eq!(-1, square.forwards_direction());
    }

    #[test]
    fn forwards_direction_two_test() {
        let square = Square { x: 0, y: 0,  kind: PieceKind::Pawn, player_number: 2 };
        assert_eq!(1, square.forwards_direction());
    }

    #[test]
    fn promotion_rank_one_test() {
        let square = Square { x: 0, y: 0,  kind: PieceKind::Pawn, player_number: 1 };
        assert_eq!(0, square.promotion_rank());
    }

    #[test]
    fn promotion_rank_two_test() {
        let square = Square { x: 0, y: 0, kind: PieceKind::Pawn, player_number: 2 };
        assert_eq!(7, square.promotion_rank());
    }
}
