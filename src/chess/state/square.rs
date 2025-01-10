use crate::chess::state::point::orthogonal_destination_points;
use crate::chess::state::point::l_shape_destination_points;
use crate::chess::state::point::diagonal_destination_points;
use crate::chess::state::point::orthogonal_or_diagonal_destination_points;
use crate::chess::state::point::one_step_destination_points;
use crate::chess::state::point::king_castle_destination_points;
use crate::chess::state::point::forward_diagonal_step_destination_points;
use crate::chess::state::point::pawn_destination_points;
use crate::chess::state::square_set::find_by_x_and_y;
use crate::chess::state::square_set::between_unoccupied;
use crate::chess::state::game_state::GameState;
use crate::chess::state::castle_move::Side;

use crate::chess::state::point::PLAYER_ONE_CASTLE_KING_SIDE;
use crate::chess::state::point::PLAYER_ONE_CASTLE_QUEEN_SIDE;
use crate::chess::state::point::PLAYER_TWO_CASTLE_KING_SIDE;
use crate::chess::state::point::PLAYER_TWO_CASTLE_QUEEN_SIDE;

use crate::chess::state::point::PLAYER_ONE_KING_SIDE_ROOK;
use crate::chess::state::point::PLAYER_ONE_QUEEN_SIDE_ROOK;
use crate::chess::state::point::PLAYER_TWO_KING_SIDE_ROOK;
use crate::chess::state::point::PLAYER_TWO_QUEEN_SIDE_ROOK;

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Square {
    pub player_number: i8,
    pub kind: PieceKind
}

impl Square {
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

    pub fn destinations<'a>(&'a self, point: (i8, i8), game_state: &'a GameState) -> Vec<(i8, i8)> {
        let mut acc = vec![];
        match self.kind {
            PieceKind::Empty => (),
            PieceKind::Pawn => {
                for to_point in pawn_destination_points(point, self.player_number) {
                    if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                        if to.unoccupied() && between_unoccupied(&game_state.squares, point, to_point) {
                            acc.push(to_point);
                        }
                    }
                }

                for to_point in forward_diagonal_step_destination_points(point, self.player_number) {
                    if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                        if to.occupied_by_opponent(self.player_number) || self.en_passant_condition(point, to_point, game_state) {
                            acc.push(to_point);
                        }
                    }
                }
            },
            PieceKind::Rook => {
                for to_point in orthogonal_destination_points(point) {
                    if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                        if to.unoccupied_or_occupied_by_opponent(self.player_number) &&
                            between_unoccupied(&game_state.squares, point, to_point) {
                            acc.push(to_point);
                        }
                    }
                }
            },
            PieceKind::Knight => {
                for to_point in l_shape_destination_points(point) {
                    if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                        if to.unoccupied_or_occupied_by_opponent(self.player_number) {
                            acc.push(to_point);
                        }
                    }
                }
            },
            PieceKind::Bishop => {
                for to_point in diagonal_destination_points(point) {
                    if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                        if to.unoccupied_or_occupied_by_opponent(self.player_number) &&
                            between_unoccupied(&game_state.squares, point, to_point) {
                            acc.push(to_point);
                        }
                    }
                }
            },
            PieceKind::Queen => {
                for to_point in orthogonal_or_diagonal_destination_points(point) {
                    if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                        if to.unoccupied_or_occupied_by_opponent(self.player_number) &&
                            between_unoccupied(&game_state.squares, point, to_point) {
                            acc.push(to_point);
                        }
                    }
                }
            },
            PieceKind::King => {
                for to_point in one_step_destination_points(point) {
                    if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                        if to.unoccupied_or_occupied_by_opponent(self.player_number) {
                            acc.push(to_point);
                        }
                    }
                }

                for to_point in king_castle_destination_points(point) {
                    if find_by_x_and_y(&game_state.squares, to_point).is_some() {
                        if self.castle_conditions(point, to_point, game_state) {
                            acc.push(to_point);
                        }
                    }
                }
            }
        }
        acc
    }

    pub fn capture_squares<'a>(&'a self, point: (i8, i8), game_state: &'a GameState) -> Vec<(i8, i8)> {
        let mut acc = vec![];
        match self.kind {
            PieceKind::Pawn => {
                for to_point in forward_diagonal_step_destination_points(point, self.player_number) {
                    if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                        if to.occupied_by_opponent(self.player_number) || self.en_passant_condition(point, to_point, game_state) {
                            acc.push(to_point);
                        }
                    }
                }
            },
            PieceKind::King => {
                for to_point in one_step_destination_points(point) {
                    if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                        if to.unoccupied_or_occupied_by_opponent(self.player_number) {
                            acc.push(to_point);
                        }
                    }
                }
            },
            _ => {
                acc = self.destinations(point, game_state);
            }
        }
        acc
    }

    fn castle_conditions(&self, point: (i8, i8), to_point: (i8, i8), game_state: &GameState) -> bool {
        match self.player_number {
            1 => {
                match to_point {
                    PLAYER_ONE_CASTLE_KING_SIDE => {
                        game_state.castle_moves.iter().any(|cm| cm.player_number == 1 && cm.side == Side::King) &&
                            between_unoccupied(&game_state.squares, point, PLAYER_ONE_KING_SIDE_ROOK)
                    },
                    PLAYER_ONE_CASTLE_QUEEN_SIDE => {
                        game_state.castle_moves.iter().any(|cm| cm.player_number == 1 && cm.side == Side::Queen) &&
                            between_unoccupied(&game_state.squares, point, PLAYER_ONE_QUEEN_SIDE_ROOK)
                    },
                    _ => false
                }
            },
            2 => {
                match to_point {
                    PLAYER_TWO_CASTLE_KING_SIDE => {
                        game_state.castle_moves.iter().any(|cm| cm.player_number == 2 && cm.side == Side::King) &&
                            between_unoccupied(&game_state.squares, point, PLAYER_TWO_KING_SIDE_ROOK)
                    },
                    PLAYER_TWO_CASTLE_QUEEN_SIDE => {
                        game_state.castle_moves.iter().any(|cm| cm.player_number == 2 && cm.side == Side::Queen) &&
                            between_unoccupied(&game_state.squares, point, PLAYER_TWO_QUEEN_SIDE_ROOK)
                    },
                    _ => false
                }
            },
            _ => false
        }
    }

    fn en_passant_condition(&self, point: (i8, i8), to_point: (i8, i8), game_state: &GameState) -> bool {
        let mut result = false;
        if let Some(target) = game_state.en_passant_target {
            if to_point == target {
                if let Some(opposing_to) = find_by_x_and_y(&game_state.squares, (target.0, point.1)) {
                    result = opposing_to.occupied_by_opponent(self.player_number);
                }
            }
        }
        result
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::state::game_state::parse as parse_game_state;

    #[test]
    fn occupied_true_test() {
        let square = Square { player_number: 1, kind: PieceKind::Pawn };
        let expected = true;
        let result = square.occupied();
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_false_test() {
        let square = Square { player_number: 0, kind: PieceKind::Empty };
        let expected = false;
        let result = square.occupied();
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_true_test() {
        let square = Square { player_number: 0, kind: PieceKind::Empty };
        let expected = true;
        let result = square.unoccupied();
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_false_test() {
        let square = Square { player_number: 1, kind: PieceKind::Pawn };
        let expected = false;
        let result = square.unoccupied();
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_one_test() {
        let square = Square { player_number: 1, kind: PieceKind::Pawn };
        let expected = false;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_two_test() {
        let square = Square { player_number: 2, kind: PieceKind::Pawn };
        let expected = true;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_zero_test() {
        let square = Square { player_number: 0, kind: PieceKind::Empty };
        let expected = false;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_or_occupied_by_opponent_one_test() {
        let square = Square { player_number: 1, kind: PieceKind::Pawn };
        let expected = false;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_or_occupied_by_opponent_two_test() {
        let square = Square { player_number: 2, kind: PieceKind::Pawn };
        let expected = true;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_or_occupied_by_opponent_zero_test() {
        let square = Square { player_number: 0, kind: PieceKind::Empty };
        let expected = true;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_some_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let game_state = parse_game_state(&encoded).unwrap();
        let square = Square { player_number: 1, kind: PieceKind::Pawn };
        let point = (4, 6);
        let expected = 2;
        let result = square.destinations(point, &game_state).len();
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_none_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let game_state = parse_game_state(&encoded).unwrap();
        let square = Square { player_number: 0, kind: PieceKind::Empty };
        let point = (4, 5);
        let expected = 0;
        let result = square.destinations(point, &game_state).len();
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_pawn_moves_test() {
        let from = Square { kind: PieceKind::Pawn, player_number: 1 };
        let point = (4, 6);
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected = vec![
                    (4, 4),
                    (4, 5)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_moves_from_non_starting_row_test() {
        let from = Square { kind: PieceKind::Pawn, player_number: 1 };
        let point = (0, 4);
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/P/8/1PPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected = vec![
                    (0, 3)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_captures_test() {
        let from = Square { kind: PieceKind::Pawn, player_number: 1 };
        let point = (4, 6);
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/8/3p4/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected = vec![
                    (4, 4),
                    (4, 5),
                    (3, 5)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_captures_blocked_test() {
        let from = Square { kind: PieceKind::Pawn, player_number: 2 };
        let point = (0, 1);
        let encoded = String::from("rnbqkbnr/pppppppp/P7/8/8/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected: Vec<(i8, i8)> = vec![];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_en_passant_test() {
        let from = Square { kind: PieceKind::Pawn, player_number: 1 };
        let point = (4, 3);
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPPPPPP/RNBQKBNR w KQkq d6 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected = vec![
                    (4, 2),
                    (3, 2)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_pawn_no_en_passant_test() {
        let from = Square {kind: PieceKind::Pawn, player_number: 1 };
        let point = (4, 3);
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = from.destinations(point, &game_state);
        let expected = vec![
            (4, 2)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_pawn_no_en_passant_same_row_test() {
        let from = Square {kind: PieceKind::Pawn, player_number: 1 };
        let point = (0, 4);
        let encoded = String::from("4k3/8/8/2p4/P7/8/8/4K3 w - c2 0 1");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = from.destinations(point, &game_state);
        let expected = vec![
            (0, 3)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_pawn_en_passant_same_column_test() {
        let from = Square { kind: PieceKind::Pawn, player_number: 1 };
        let point = (1, 6);
        let encoded = String::from("4k3/8/8/1Pp4/8/8/1Pp4/4K3 w - c2 0 1");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = from.destinations(point, &game_state);
        let expected = vec![
            (1, 4),
            (1, 5)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_rook_test() {
        let from = Square { kind: PieceKind::Rook, player_number: 1 };
        let point = (7, 7);
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected = vec![
                    (7, 6), (7, 5), (7, 4), (7, 3), (7, 2), (7, 1)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_knight_test() {
        let from = Square { kind: PieceKind::Knight, player_number: 1 };
        let point = (6, 7);
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected = vec![
                    (5, 5),
                    (7, 5)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_bishop_test() {
        let from = Square { kind: PieceKind::Bishop, player_number: 1 };
        let point = (5, 7);
        let encoded = String::from("rnbqkbnr/p1pppppp/8/1p6/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected = vec![
                    (4, 6), (3, 5), (2, 4), (1, 3)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_queen_test() {
        let from = Square { kind: PieceKind::Queen, player_number: 1 };
        let point = (3, 7);
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/3p4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected = vec![
                    (2, 6), (1, 5), (0, 4), (3, 6), (3, 5), (3, 4)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_king_normal_test() {
        let from = Square { kind: PieceKind::King, player_number: 1 };
        let point = (4, 7);
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/8/8/PPPp1PPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected = vec![
                    (3, 6),
                    (4, 6)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_king_castle_test() {
        let from = Square { kind: PieceKind::King, player_number: 1 };
        let point = (4, 7);
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/5BN1/PPPPPPPP/RNBQK2R w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected = vec![
                    (5, 7),
                    (6, 7)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_king_queen_side_castle_blocked_test() {
        let from = Square { kind: PieceKind::King, player_number: 1 };
        let point = (4, 7);
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/5BN1/PPPPPPPP/RN2KBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.destinations(point, &game_state);
                let expected = vec![
                    (3, 7)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn pawn_capture_squares_test() {
        let from = Square { kind: PieceKind::Pawn, player_number: 1 };
        let point = (4, 6);
        let encoded = String::from("rnbqkbnr/ppp1pppp/8/8/8/3p4/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.capture_squares(point, &game_state);
                let expected = vec![
                    (3, 5)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn king_capture_squares_test() {
        let from = Square { kind: PieceKind::King, player_number: 1 };
        let point = (4, 7);
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/5BN1/PPPPPPPP/RNBQK2R w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.capture_squares(point, &game_state);
                let expected = vec![
                    (5, 7)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn other_capture_squares_test() {
        let from = Square { kind: PieceKind::Rook, player_number: 1 };
        let point = (7, 7);
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = from.capture_squares(point, &game_state);
                let expected = vec![
                    (7, 6), (7, 5), (7, 4), (7, 3), (7, 2), (7, 1)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn forwards_direction_one_test() {
        let square = Square { kind: PieceKind::Pawn, player_number: 1 };
        assert_eq!(-1, square.forwards_direction());
    }

    #[test]
    fn forwards_direction_two_test() {
        let square = Square { kind: PieceKind::Pawn, player_number: 2 };
        assert_eq!(1, square.forwards_direction());
    }

    #[test]
    fn promotion_rank_one_test() {
        let square = Square { kind: PieceKind::Pawn, player_number: 1 };
        assert_eq!(0, square.promotion_rank());
    }

    #[test]
    fn promotion_rank_two_test() {
        let square = Square { kind: PieceKind::Pawn, player_number: 2 };
        assert_eq!(7, square.promotion_rank());
    }
}
