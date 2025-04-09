use crate::shogi::state::point::one_step_forward_destination_points;
use crate::shogi::state::point::forward_destination_points;
use crate::shogi::state::point::l_shape_forwards_destination_points;
use crate::shogi::state::point::gin_destination_points;
use crate::shogi::state::point::kin_destination_points;
use crate::shogi::state::point::diagonal_destination_points;
use crate::shogi::state::point::orthogonal_destination_points;
use crate::shogi::state::point::one_step_destination_points;
use crate::shogi::state::point::ryuuma_destination_points;
use crate::shogi::state::point::ryuuou_destination_points;
use crate::shogi::state::square_set::find_by_x_and_y;
use crate::shogi::state::square_set::between_unoccupied;
use crate::shogi::state::game_state::GameState;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceKind {
   Oushou,
   Gyokushou,
   Hisha,
   Ryuuou,
   Kakugyou,
   Ryuuma,
   Kinshou,
   Ginshou,
   Narigin,
   Keima,
   Narikei,
   Kyousha,
   Narikyou,
   Fuhyou,
   Tokin,
   Empty
}

pub const PROMOTABLE_PIECE_KINDS: [PieceKind; 6] = [
    PieceKind::Fuhyou,
    PieceKind::Kyousha,
    PieceKind::Keima,
    PieceKind::Ginshou,
    PieceKind::Hisha,
    PieceKind::Kakugyou
];

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
}

pub fn destinations(piece_kind: PieceKind, player_number: i8, point: (i8, i8), game_state: &GameState, ignore_blocks: bool) -> Vec<(i8, i8)> {
    let mut acc = vec![];
    match piece_kind {
        PieceKind::Empty => (),
        PieceKind::Fuhyou => {
            for to_point in one_step_forward_destination_points(point, player_number) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Kyousha => {
            for to_point in forward_destination_points(point, player_number) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number)  && (ignore_blocks || between_unoccupied(&game_state.squares, point, to_point)) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Keima => {
            for to_point in l_shape_forwards_destination_points(point, player_number) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Ginshou => {
            for to_point in gin_destination_points(point, player_number) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Kinshou | PieceKind::Tokin | PieceKind::Narikyou | PieceKind::Narikei | PieceKind::Narigin => {
            for to_point in kin_destination_points(point, player_number) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Kakugyou => {
            for to_point in diagonal_destination_points(point) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) && (ignore_blocks || between_unoccupied(&game_state.squares, point, to_point)) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Hisha => {
            for to_point in orthogonal_destination_points(point) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) && (ignore_blocks || between_unoccupied(&game_state.squares, point, to_point)) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Gyokushou | PieceKind::Oushou => {
            for to_point in one_step_destination_points(point) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Ryuuma => {
            for to_point in ryuuma_destination_points(point) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) && (ignore_blocks || between_unoccupied(&game_state.squares, point, to_point)) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Ryuuou => {
            for to_point in ryuuou_destination_points(point) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) && (ignore_blocks || between_unoccupied(&game_state.squares, point, to_point)) {
                        acc.push(to_point);
                    }
                }
            }
        }
    }
    acc
}

pub fn promotion_ranks(kind: PieceKind, player_number: i8) -> Vec<i8> {
    if PROMOTABLE_PIECE_KINDS.iter().any(|pk| *pk == kind) {
        if player_number == 1 {
            vec![0, 1, 2]
        } else {
            vec![6, 7, 8]
        }
    } else {
        vec![]
    }
}

pub fn compulsory_promotion_ranks(kind: PieceKind, player_number: i8) -> Vec<i8> {
    match kind {
        PieceKind::Fuhyou | PieceKind::Kyousha => {
            if player_number == 1 {
                vec![0]
            } else {
                vec![8]
            }
        },
        PieceKind::Keima => {
            if player_number == 1 {
                vec![0, 1]
            } else {
                vec![7, 8]
            }
        }
        _ => vec![]
    }
}

pub fn promotes_to(kind: PieceKind) -> Option<PieceKind> {
    match kind {
        PieceKind::Fuhyou => Some(PieceKind::Tokin),
        PieceKind::Kyousha => Some(PieceKind::Narikyou),
        PieceKind::Keima => Some(PieceKind::Narikei),
        PieceKind::Ginshou => Some(PieceKind::Narigin),
        PieceKind::Hisha => Some(PieceKind::Ryuuou),
        PieceKind::Kakugyou => Some(PieceKind::Ryuuma),
        _ => None
    }
}

pub fn demotes_to(kind: PieceKind) -> Option<PieceKind> {
    match kind {
        PieceKind::Tokin => Some(PieceKind::Fuhyou),
        PieceKind::Narikyou => Some(PieceKind::Kyousha),
        PieceKind::Narikei => Some(PieceKind::Keima),
        PieceKind::Narigin => Some(PieceKind::Ginshou),
        PieceKind::Ryuuou => Some(PieceKind::Hisha),
        PieceKind::Ryuuma => Some(PieceKind::Kakugyou),
        _ => None
    }
}

pub fn has_legal_moves_from_y(kind: PieceKind, player_number: i8, y: i8) -> bool {
    match kind {
        PieceKind::Fuhyou => {
            if player_number == 1 {
              y != 0
            } else {
              y != 8
            }
        },
        PieceKind::Kyousha  => {
            if player_number == 1 {
              y != 0
            } else {
              y != 8
            }
        },
        PieceKind::Keima => {
            if player_number == 1 {
              y != 0 && y != 1
            } else {
              y != 8 && y != 7
            }
        },
        _ => true
    }
}

pub fn ou_kind(player_number: i8) -> PieceKind {
    if player_number == 1 {
        PieceKind::Oushou
    } else {
        PieceKind::Gyokushou
    }
}

pub fn opposing_player(player_number: i8) -> i8 {
    if player_number == 1 {
        2
    } else {
        1
    }
}

pub fn ranging(kind: PieceKind) -> bool {
    vec![PieceKind::Hisha, PieceKind::Ryuuou, PieceKind::Kakugyou, PieceKind::Ryuuma, PieceKind::Kyousha].contains(&kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shogi::state::game_state::parse as parse_game_state;

    #[test]
    fn occupied_true_test() {
        let square = Square { player_number: 1, kind: PieceKind::Fuhyou };
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
        let square = Square { player_number: 1, kind: PieceKind::Fuhyou };
        let expected = false;
        let result = square.unoccupied();
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_one_test() {
        let square = Square { player_number: 1, kind: PieceKind::Fuhyou };
        let expected = false;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_two_test() {
        let square = Square { player_number: 2, kind: PieceKind::Fuhyou };
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
        let square = Square { player_number: 1, kind: PieceKind::Fuhyou };
        let expected = false;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_or_occupied_by_opponent_two_test() {
        let square = Square { player_number: 2, kind: PieceKind::Fuhyou };
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
    fn destinations_fuhyou_moves_test() {
        let kind = PieceKind::Fuhyou;
        let player_number = 1;
        let point = (4, 6);
        let encoded = String::from("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (4, 5)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_kyousha_moves_test() {
        let kind = PieceKind::Kyousha;
        let player_number = 1;
        let point = (0, 8);
        let encoded = String::from("lnsgkgsnl/1r5b1/9/9/9/p8/9/1B5R1/LNSGKGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (0, 7), (0, 6), (0, 5)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_kyousha_moves_ignore_blocks_test() {
        let kind = PieceKind::Kyousha;
        let player_number = 1;
        let point = (0, 8);
        let encoded = String::from("lnsgkgsnl/1r5b1/9/9/9/p8/9/1B5R1/LNSGKGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, true);
                let expected = vec![
                    (0, 7), (0, 6), (0, 5), (0, 4), (0, 3), (0, 2), (0, 1), (0, 0)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_keima_moves_test() {
        let kind = PieceKind::Keima;
        let player_number = 1;
        let point = (1, 8);
        let encoded = String::from("lnsgkgsnl/1r5b1/9/9/9/p8/9/1B5R1/LNSGKGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (0, 6), (2, 6)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_ginshou_moves_test() {
        let kind = PieceKind::Ginshou;
        let player_number = 1;
        let point = (2, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/2p6/2S6/L3KGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (1, 6), (3, 6), (3, 8), (1, 8), (2, 6)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_kinshou_moves_test() {
        let kind = PieceKind::Kinshou;
        let player_number = 1;
        let point = (3, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/3p5/3G5/8K b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (3, 6), (4, 7), (3, 8), (2, 7), (2, 6), (4, 6)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_kakugyou_moves_test() {
        let kind = PieceKind::Kakugyou;
        let player_number = 1;
        let point = (1, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/2p6/1B7/8K b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (0, 6), (2, 6), (2, 8), (0, 8)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_kakugyou_moves_ignore_blocks_test() {
        let kind = PieceKind::Kakugyou;
        let player_number = 1;
        let point = (1, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/2p6/1B7/8K b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, true);
                let expected = vec![
                    (0, 6), (2, 6), (3, 5), (4, 4), (5, 3), (6, 2), (7, 1), (8, 0), (2, 8), (0, 8)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_hisha_moves_test() {
        let kind = PieceKind::Hisha;
        let player_number = 1;
        let point = (7, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/7p1/7R1/8K b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (7, 6), (8, 7), (7, 8), (6, 7), (5, 7), (4, 7), (3, 7), (2, 7), (1, 7), (0, 7)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_hisha_moves_ignore_blocks_test() {
        let kind = PieceKind::Hisha;
        let player_number = 1;
        let point = (7, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/7p1/7R1/8K b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, true);
                let expected = vec![
                    (7, 6), (7, 5), (7, 4), (7, 3), (7, 2), (7, 1), (7, 0), (8, 7), (7, 8), (6, 7), (5, 7), (4, 7), (3, 7), (2, 7), (1, 7), (0, 7)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_gyokushou_moves_test() {
        let kind = PieceKind::Gyokushou;
        let player_number = 2;
        let point = (4, 1);
        let encoded = String::from("9/4k4/9/9/9/9/4p4/4K4/9 b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (3, 0), (4, 0), (5, 0), (5, 1), (5, 2), (4, 2), (3, 2), (3, 1)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_oushou_moves_test() {
        let kind = PieceKind::Oushou;
        let player_number = 1;
        let point = (4, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/4p4/4K4/9 b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (3, 6), (4, 6), (5, 6), (5, 7), (5, 8), (4, 8), (3, 8), (3, 7)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_tokin_moves_test() {
        let kind = PieceKind::Tokin;
        let player_number = 1;
        let point = (2, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/2p6/2+P6/L3KGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (2, 6), (3, 7), (2, 8), (1, 7), (1, 6), (3, 6)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_narikyou_moves_test() {
        let kind = PieceKind::Narikyou;
        let player_number = 1;
        let point = (2, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/2p6/2+L6/L3KGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (2, 6), (3, 7), (2, 8), (1, 7), (1, 6), (3, 6)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_narikei_moves_test() {
        let kind = PieceKind::Narikei;
        let player_number = 1;
        let point = (2, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/2p6/2+N6/L3KGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (2, 6), (3, 7), (2, 8), (1, 7), (1, 6), (3, 6)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_narigin_moves_test() {
        let kind = PieceKind::Narigin;
        let player_number = 1;
        let point = (2, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/2p6/2+S6/L3KGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (2, 6), (3, 7), (2, 8), (1, 7), (1, 6), (3, 6)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_ryuuou_moves_test() {
        let kind = PieceKind::Ryuuou;
        let player_number = 1;
        let point = (2, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/2p6/2+R6/L3KGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (2, 6), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7), (8, 7), (2, 8), (1, 7), (0, 7), (1, 6), (3, 6), (3, 8), (1, 8)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_ryuuou_moves_ignore_blocks_test() {
        let kind = PieceKind::Ryuuou;
        let player_number = 1;
        let point = (2, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/2p6/2+R6/L3KGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, true);
                let expected = vec![
                    (2, 6), (2, 5), (2, 4), (2, 3), (2, 2), (2, 1), (2, 0), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7), (8, 7), (2, 8), (1, 7), (0, 7), (1, 6), (3, 6), (3, 8), (1, 8)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_ryuuma_moves_test() {
        let kind = PieceKind::Ryuuma;
        let player_number = 1;
        let point = (2, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/1p7/2+B6/L3KGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, false);
                let expected = vec![
                    (1, 6), (3, 6), (4, 5), (5, 4), (6, 3), (7, 2), (8, 1), (3, 8), (1, 8), (2, 6), (3, 7), (2, 8), (1, 7)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn destinations_ryuuma_moves_ignore_blocks_test() {
        let kind = PieceKind::Ryuuma;
        let player_number = 1;
        let point = (2, 7);
        let encoded = String::from("lnsgkgsnl/9/9/9/9/9/2p6/2+B6/L3KGSNL b -");
        let result = parse_game_state(&encoded);

        match result {
            Ok(game_state) => {
                let result = destinations(kind, player_number, point, &game_state, true);
                let expected = vec![
                    (1, 6), (0, 5), (3, 6), (4, 5), (5, 4), (6, 3), (7, 2), (8, 1), (3, 8), (1, 8), (2, 6), (3, 7), (2, 8), (1, 7)
                ];
                assert_eq!(result, expected);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn promotion_ranks_promotable_one_test() {
        let kind = PieceKind::Fuhyou;
        let player_number = 1;
        let result = promotion_ranks(kind, player_number);
        let expected = vec![0, 1, 2];
        assert_eq!(result, expected);
    }

    #[test]
    fn promotion_ranks_promotable_two_test() {
        let kind = PieceKind::Fuhyou;
        let player_number = 2;
        let result = promotion_ranks(kind, player_number);
        let expected = vec![6, 7, 8];
        assert_eq!(result, expected);
    }

    #[test]
    fn promotion_ranks_unpromotable_test() {
        let kind = PieceKind::Tokin;
        let player_number = 1;
        let result = promotion_ranks(kind, player_number);
        let expected: Vec<i8> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn compulsory_promotion_fuhyou_test() {
        let kind = PieceKind::Fuhyou;
        let player_number = 1;
        let result = compulsory_promotion_ranks(kind, player_number);
        let expected: Vec<i8> = vec![0];
        assert_eq!(result, expected);
    }

    #[test]
    fn compulsory_promotion_kyousha_test() {
        let kind = PieceKind::Kyousha;
        let player_number = 1;
        let result = compulsory_promotion_ranks(kind, player_number);
        let expected: Vec<i8> = vec![0];
        assert_eq!(result, expected);
    }

    #[test]
    fn compulsory_promotion_keima_test() {
        let kind = PieceKind::Keima;
        let player_number = 1;
        let result = compulsory_promotion_ranks(kind, player_number);
        let expected: Vec<i8> = vec![0, 1];
        assert_eq!(result, expected);
    }

    #[test]
    fn compulsory_promotion_ginshou_test() {
        let kind = PieceKind::Ginshou;
        let player_number = 1;
        let result = compulsory_promotion_ranks(kind, player_number);
        let expected: Vec<i8> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn promotes_to_fuhyou_test() {
        let kind = PieceKind::Fuhyou;
        let result = promotes_to(kind);
        let expected = Some(PieceKind::Tokin);
        assert_eq!(result, expected);
    }

    #[test]
    fn promotes_to_kyousha_test() {
        let kind = PieceKind::Kyousha;
        let result = promotes_to(kind);
        let expected = Some(PieceKind::Narikyou);
        assert_eq!(result, expected);
    }

    #[test]
    fn promotes_to_keima_test() {
        let kind = PieceKind::Keima;
        let result = promotes_to(kind);
        let expected = Some(PieceKind::Narikei);
        assert_eq!(result, expected);
    }

    #[test]
    fn promotes_to_ginshou_test() {
        let kind = PieceKind::Ginshou;
        let result = promotes_to(kind);
        let expected = Some(PieceKind::Narigin);
        assert_eq!(result, expected);
    }

    #[test]
    fn promotes_to_hisha_test() {
        let kind = PieceKind::Hisha;
        let result = promotes_to(kind);
        let expected = Some(PieceKind::Ryuuou);
        assert_eq!(result, expected);
    }

    #[test]
    fn promotes_to_kakugyou_test() {
        let kind = PieceKind::Kakugyou;
        let result = promotes_to(kind);
        let expected = Some(PieceKind::Ryuuma);
        assert_eq!(result, expected);
    }

    #[test]
    fn promotes_to_kinshou_test() {
        let kind = PieceKind::Kinshou;
        let result = promotes_to(kind);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn demotes_to_tokin_test() {
        let kind = PieceKind::Tokin;
        let result = demotes_to(kind);
        let expected = Some(PieceKind::Fuhyou);
        assert_eq!(result, expected);
    }

    #[test]
    fn demotes_to_narikyou_test() {
        let kind = PieceKind::Narikyou;
        let result = demotes_to(kind);
        let expected = Some(PieceKind::Kyousha);
        assert_eq!(result, expected);
    }

    #[test]
    fn demotes_to_narikei_test() {
        let kind = PieceKind::Narikei;
        let result = demotes_to(kind);
        let expected = Some(PieceKind::Keima);
        assert_eq!(result, expected);
    }

    #[test]
    fn demotes_to_narigin_test() {
        let kind = PieceKind::Narigin;
        let result = demotes_to(kind);
        let expected = Some(PieceKind::Ginshou);
        assert_eq!(result, expected);
    }

    #[test]
    fn demotes_to_ryuuou_test() {
        let kind = PieceKind::Ryuuou;
        let result = demotes_to(kind);
        let expected = Some(PieceKind::Hisha);
        assert_eq!(result, expected);
    }

    #[test]
    fn demotes_to_ryuuma_test() {
        let kind = PieceKind::Ryuuma;
        let result = demotes_to(kind);
        let expected = Some(PieceKind::Kakugyou);
        assert_eq!(result, expected);
    }

    #[test]
    fn demotes_to_kinshou_test() {
        let kind = PieceKind::Kinshou;
        let result = demotes_to(kind);
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn has_legal_moves_from_y_fuhyou_player_one_true_test() {
        let kind = PieceKind::Fuhyou;
        let player_number = 1;
        let y = 1;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, true);
    }

    #[test]
    fn has_legal_moves_from_y_fuhyou_player_one_false_test() {
        let kind = PieceKind::Fuhyou;
        let player_number = 1;
        let y = 0;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, false);
    }

    #[test]
    fn has_legal_moves_from_y_fuhyou_player_two_true_test() {
        let kind = PieceKind::Fuhyou;
        let player_number = 2;
        let y = 7;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, true);
    }

    #[test]
    fn has_legal_moves_from_y_fuhyou_player_two_false_test() {
        let kind = PieceKind::Fuhyou;
        let player_number = 2;
        let y = 8;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, false);
    }

    #[test]
    fn has_legal_moves_from_y_keima_player_one_true_test() {
        let kind = PieceKind::Keima;
        let player_number = 1;
        let y = 2;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, true);
    }

    #[test]
    fn has_legal_moves_from_y_keima_player_one_false_test() {
        let kind = PieceKind::Keima;
        let player_number = 1;
        let y = 1;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, false);
    }

    #[test]
    fn has_legal_moves_from_y_keima_player_two_true_test() {
        let kind = PieceKind::Keima;
        let player_number = 2;
        let y = 6;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, true);
    }

    #[test]
    fn has_legal_moves_from_y_keima_player_two_false_test() {
        let kind = PieceKind::Keima;
        let player_number = 2;
        let y = 7;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, false);
    }

    #[test]
    fn has_legal_moves_from_y_kyousha_player_one_true_test() {
        let kind = PieceKind::Kyousha;
        let player_number = 1;
        let y = 1;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, true);
    }

    #[test]
    fn has_legal_moves_from_y_kyousha_player_one_false_test() {
        let kind = PieceKind::Kyousha;
        let player_number = 1;
        let y = 0;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, false);
    }

    #[test]
    fn has_legal_moves_from_y_kyousha_player_two_true_test() {
        let kind = PieceKind::Kyousha;
        let player_number = 2;
        let y = 7;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, true);
    }

    #[test]
    fn has_legal_moves_from_y_kyousha_player_two_false_test() {
        let kind = PieceKind::Kyousha;
        let player_number = 2;
        let y = 8;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, false);
    }

    #[test]
    fn has_legal_moves_from_y_default_test() {
        let kind = PieceKind::Ginshou;
        let player_number = 1;
        let y = 0;
        let result = has_legal_moves_from_y(kind, player_number, y);
        assert_eq!(result, true);
    }

    #[test]
    fn ou_kind_player_one_test() {
        let result = ou_kind(1);
        assert_eq!(result, PieceKind::Oushou);
    }

    #[test]
    fn ou_kind_player_two_test() {
        let result = ou_kind(2);
        assert_eq!(result, PieceKind::Gyokushou);
    }

    #[test]
    fn opposing_player_one_test() {
        let result = opposing_player(1);
        assert_eq!(result, 2);
    }

    #[test]
    fn opposing_player_two_test() {
        let result = opposing_player(2);
        assert_eq!(result, 1);
    }

    #[test]
    fn ranging_true_test() {
       let result = ranging(PieceKind::Hisha);
       assert_eq!(result, true);
    }

    #[test]
    fn ranging_false_test() {
       let result = ranging(PieceKind::Fuhyou);
       assert_eq!(result, false);
    }
}
