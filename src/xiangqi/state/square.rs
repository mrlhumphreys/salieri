use crate::xiangqi::state::point::soldier_destination_points;
use crate::xiangqi::state::point::orthogonal_destination_points;
use crate::xiangqi::state::point::horse_destination_points;
use crate::xiangqi::state::point::elephant_destination_points;
use crate::xiangqi::state::point::advisor_destination_points;
use crate::xiangqi::state::point::king_destination_points;
use crate::xiangqi::state::point::flying_king_destination_points;
use crate::xiangqi::state::square_set::find_by_x_and_y;
use crate::xiangqi::state::square_set::between_unoccupied;
use crate::xiangqi::state::square_set::between_l_unoccupied;
use crate::xiangqi::state::square_set::between_occupied_by_one;
use crate::xiangqi::state::game_state::GameState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceKind {
    King,
    Chariot,
    Horse,
    Elephant,
    Advisor,
    Soldier,
    Cannon,
    Empty
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Square {
    pub player_number: i8,
    pub kind: PieceKind
}

impl Square {
    pub fn unoccupied(&self) -> bool {
        self.player_number == 0
    }

    pub fn occupied(&self) -> bool {
        self.player_number != 0
    }

    pub fn unoccupied_or_occupied_by_opponent(&self, player_number: i8) -> bool {
        self.player_number == 0 || self.player_number != player_number
    }

    pub fn occupied_by_opponent(&self, player_number: i8) -> bool {
        self.player_number != 0 && self.player_number != player_number
    }
}

pub fn destinations(piece_kind: PieceKind, player_number: i8, point: (i8, i8), game_state: &GameState, ignore_blocks: bool) -> Vec<(i8, i8)> {
    let mut acc = vec![];
    match piece_kind {
        PieceKind::Empty => (),
        PieceKind::Soldier => {
            for to_point in soldier_destination_points(point, player_number) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Chariot => {
            for to_point in orthogonal_destination_points(point) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) && (ignore_blocks || between_unoccupied(&game_state.squares, point, to_point)) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Horse => {
            for to_point in horse_destination_points(point) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) && (ignore_blocks || between_l_unoccupied(&game_state.squares, point, to_point)) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Elephant => {
            for to_point in elephant_destination_points(point, player_number) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) && (ignore_blocks || between_unoccupied(&game_state.squares, point, to_point)) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Advisor => {
            for to_point in advisor_destination_points(point, player_number) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::King => {
            for to_point in king_destination_points(point, player_number) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied_or_occupied_by_opponent(player_number) {
                        acc.push(to_point);
                    }
                }
            }

            // flying king rule
            for to_point in flying_king_destination_points(point, player_number) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.player_number == opposing_player(player_number) && to.kind == PieceKind::King && between_unoccupied(&game_state.squares, point, to_point) {
                        acc.push(to_point);
                    }
                }
            }
        },
        PieceKind::Cannon => {
            for to_point in orthogonal_destination_points(point) {
                if let Some(to) = find_by_x_and_y(&game_state.squares, to_point) {
                    if to.unoccupied() && (ignore_blocks || between_unoccupied(&game_state.squares, point, to_point)) {
                        acc.push(to_point);
                    }
                    if to.occupied_by_opponent(player_number) && (ignore_blocks || between_occupied_by_one(&game_state.squares, point, to_point)) {
                        acc.push(to_point);
                    }
                }
            }
        },
    }
    acc
}

pub fn threats_matches_point(piece_kind: PieceKind, player_number: i8, from: (i8, i8), squares: &Vec<Vec<Square>>, target_point: (i8, i8)) -> bool {
    let mut result = false;
    match piece_kind {
        PieceKind::Empty => (),
        PieceKind::Soldier => {
            let points = soldier_destination_points(from, player_number);
            result = points.contains(&target_point);
        },
        PieceKind::Chariot => {
            let points = orthogonal_destination_points(from);
            result = points.contains(&target_point) && between_unoccupied(squares, from, target_point);
        },
        PieceKind::Horse => {
            let points = horse_destination_points(from);
            result = points.contains(&target_point) && between_l_unoccupied(squares, from, target_point);
        },
        PieceKind::Elephant => {
            let points = elephant_destination_points(from, player_number);
            result = points.contains(&target_point) && between_unoccupied(squares, from, target_point);
        },
        PieceKind::Advisor => {
            let points = advisor_destination_points(from, player_number);
            result = points.contains(&target_point);
        },
        PieceKind::King => {
            let points = king_destination_points(from, player_number);
            let flying_points = flying_king_destination_points(from, player_number);
            result = points.contains(&target_point) || (flying_points.contains(&target_point) && between_unoccupied(squares, from, target_point));
        },
        PieceKind::Cannon => {
            let points = orthogonal_destination_points(from);
            // captures are threats. Cannon can only capture by jumping over a piece
            result = points.contains(&target_point) && between_occupied_by_one(squares, from, target_point);
        }
    }
    result
}

pub fn opposing_player(player_number: i8) -> i8 {
    if player_number == 1 {
        2
    } else {
        1
    }
}

// pub fn ranging(kind: PieceKind) -> bool {
//     vec![PieceKind::Chariot, PieceKind::Cannon, PieceKind::Elephant, PieceKind::Horse].contains(&kind)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xiangqi::state::game_state::parse as parse_game_state;

    #[test]
    fn unoccupied_true_test() {
        let square = Square { player_number: 0, kind: PieceKind::Empty };
        let expected = true;
        let result = square.unoccupied();
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_false_test() {
        let square = Square { player_number: 1, kind: PieceKind::Soldier };
        let expected = false;
        let result = square.unoccupied();
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_true_test() {
        let square = Square { player_number: 1, kind: PieceKind::Soldier };
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
    fn unoccupied_or_occupied_by_opponent_one_test() {
        let square = Square { player_number: 1, kind: PieceKind::Soldier };
        let expected = false;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_or_occupied_by_opponent_two_test() {
        let square = Square { player_number: 2, kind: PieceKind::Soldier };
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
    fn occupied_by_opponent_zero_test() {
        let square = Square { player_number: 0, kind: PieceKind::Empty };
        let expected = false;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_one_test() {
        let square = Square { player_number: 1, kind: PieceKind::Soldier };
        let expected = false;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_two_test() {
        let square = Square { player_number: 2, kind: PieceKind::Soldier };
        let expected = true;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_soldier_moves_test() {
        let kind = PieceKind::Soldier;
        let player_number = 1;
        let point = (0, 6);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = destinations(kind, player_number, point, &game_state, false);
        let expected = vec![
            (0, 5)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_chariot_moves_test() {
        let kind = PieceKind::Chariot;
        let player_number = 1;
        let point = (8, 9);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P2/1C5C1/9/RHEAK3R w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = destinations(kind, player_number, point, &game_state, false);
        let expected = vec![
            (8, 8), (8, 7), (8, 6), (8, 5), (8, 4), (8, 3), (7, 9), (6, 9), (5, 9)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_chariot_moves_ignore_blocks_test() {
        let kind = PieceKind::Chariot;
        let player_number = 1;
        let point = (8, 9);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P2/1C5C1/9/RHEAK3R w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = destinations(kind, player_number, point, &game_state, true);
        let expected = vec![
            (8, 8), (8, 7), (8, 6), (8, 5), (8, 4), (8, 3), (8, 2), (8, 1), (8, 0), (7, 9), (6, 9), (5, 9)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_horse_moves_test() {
        let kind = PieceKind::Horse;
        let player_number = 1;
        let point = (1, 9);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = destinations(kind, player_number, point, &game_state, false);
        let expected = vec![
            (0, 7), (2, 7)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_horse_moves_ignore_blocks_test() {
        let kind = PieceKind::Horse;
        let player_number = 1;
        let point = (1, 9);
        let encoded = String::from("rheakaehr/9/1c5c1/2p1p1p1p/9/9/P1P1P1P1P/1C5C1/1P/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = destinations(kind, player_number, point, &game_state, true);
        let expected = vec![
            (0, 7), (2, 7), (3, 8)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_elephant_moves_test() {
        let kind = PieceKind::Elephant;
        let player_number = 1;
        let point = (2, 9);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = destinations(kind, player_number, point, &game_state, false);
        let expected = vec![
            (0, 7), (4, 7)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_elephant_moves_ignore_blocks_test() {
        let kind = PieceKind::Elephant;
        let player_number = 1;
        let point = (2, 9);
        let encoded = String::from("rheakaehr/9/1c5c1/2p1p1p1p/9/9/P1P1P1P1P/1C5C1/1p7/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = destinations(kind, player_number, point, &game_state, true);
        let expected = vec![
            (0, 7), (4, 7)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_advisor_moves_test() {
        let kind = PieceKind::Advisor;
        let player_number = 1;
        let point = (3, 9);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = destinations(kind, player_number, point, &game_state, false);
        let expected = vec![
            (4, 8)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_king_moves_test() {
        let kind = PieceKind::King;
        let player_number = 1;
        let point = (4, 9);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = destinations(kind, player_number, point, &game_state, false);
        let expected = vec![
            (4, 8)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_cannon_moves_test() {
        let kind = PieceKind::Cannon;
        let player_number = 1;
        let point = (1, 7);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = destinations(kind, player_number, point, &game_state, false);
        let expected = vec![
            (1, 6), (1, 5), (1, 4), (1, 3), (1, 0), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (1, 8), (0, 7)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_cannon_moves_ignore_blocks_test() {
        let kind = PieceKind::Cannon;
        let player_number = 1;
        let point = (1, 7);
        let encoded = String::from("rheakaehr/9/1c5c1/2p1p1p1p/9/9/PpP1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = destinations(kind, player_number, point, &game_state, true);
        let expected = vec![
            (1, 6), (1, 5), (1, 4), (1, 3), (1, 2), (1, 1), (1, 0), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (8, 7), (1, 8), (0, 7)
        ];
        assert_eq!(result, expected);
    }

    // threats_matches_point

    #[test]
    fn threats_matches_point_soldier_moves_true_test() {
        let kind = PieceKind::Soldier;
        let player_number = 1;
        let from = (0, 6);
        let target_point = (0, 5);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, from, &game_state.squares, target_point);
        assert_eq!(result, true);
    }

    #[test]
    fn threats_matches_point_soldier_moves_false_test() {
        let kind = PieceKind::Soldier;
        let player_number = 1;
        let from = (0, 6);
        let target_point = (0, 4);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, from, &game_state.squares, target_point);
        assert_eq!(result, false);
    }

    #[test]
    fn threat_matches_point_chariot_moves_true_test() {
        let kind = PieceKind::Chariot;
        let player_number = 1;
        let point = (8, 9);
        let target_point = (8, 3);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P2/1C5C1/9/RHEAK3R w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, true);
    }

    #[test]
    fn threat_matches_point_chariot_moves_false_test() {
        let kind = PieceKind::Chariot;
        let player_number = 1;
        let point = (8, 9);
        let target_point = (8, 2);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P2/1C5C1/9/RHEAK3R w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, false);
    }

    #[test]
    fn threats_matches_point_horse_moves_true_test() {
        let kind = PieceKind::Horse;
        let player_number = 1;
        let point = (1, 9);
        let target_point = (0, 7);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, true);
    }

    #[test]
    fn threats_matches_point_horse_moves_false_test() {
        let kind = PieceKind::Horse;
        let player_number = 1;
        let point = (1, 9);
        let target_point = (0, 6);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, false);
    }

    #[test]
    fn threats_matches_point_elephant_moves_true_test() {
        let kind = PieceKind::Elephant;
        let player_number = 1;
        let point = (2, 9);
        let target_point = (0, 7);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, true);
    }

    #[test]
    fn threats_matches_point_elephant_moves_false_test() {
        let kind = PieceKind::Elephant;
        let player_number = 1;
        let point = (2, 9);
        let target_point = (1, 8);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, false);
    }

    #[test]
    fn threats_matches_point_advisor_moves_true_test() {
        let kind = PieceKind::Advisor;
        let player_number = 1;
        let point = (3, 9);
        let target_point = (4, 8);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, true);
    }

    #[test]
    fn threats_matches_point_advisor_moves_false_test() {
        let kind = PieceKind::Advisor;
        let player_number = 1;
        let point = (3, 9);
        let target_point = (4, 7);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, false);
    }

    #[test]
    fn threats_matches_point_king_moves_true_test() {
        let kind = PieceKind::King;
        let player_number = 1;
        let point = (4, 9);
        let target_point = (4, 8);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, true);
    }

    #[test]
    fn threats_matches_point_king_moves_false_test() {
        let kind = PieceKind::King;
        let player_number = 1;
        let point = (4, 9);
        let target_point = (4, 7);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, false);
    }

    #[test]
    fn threats_matches_point_cannon_moves_true_test() {
        let kind = PieceKind::Cannon;
        let player_number = 1;
        let point = (1, 7);
        let target_point = (1, 0);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, true);
    }

    #[test]
    fn threats_matches_point_cannon_moves_false_test() {
        let kind = PieceKind::Cannon;
        let player_number = 1;
        let point = (1, 7);
        let target_point = (0, 0);
        let encoded = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        let game_state = parse_game_state(&encoded).unwrap();

        let result = threats_matches_point(kind, player_number, point, &game_state.squares, target_point);
        assert_eq!(result, false);
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

    // #[test]
    // fn ranging_true_test() {
    //    let result = ranging(PieceKind::Chariot);
    //    assert_eq!(result, true);
    // }

    // #[test]
    // fn ranging_false_test() {
    //    let result = ranging(PieceKind::Soldier);
    //    assert_eq!(result, false);
    // }
}
