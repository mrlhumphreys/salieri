use std::cmp::Ordering;
use crate::chess::state::castle_move::Side;

pub const MIN_N: i8 = 0;
pub const MAX_N: i8 = 7;

pub const PLAYER_ONE_CASTLE_KING_SIDE: (i8, i8) = (6, 7);
pub const PLAYER_ONE_CASTLE_QUEEN_SIDE: (i8, i8) = (2, 7);
pub const PLAYER_TWO_CASTLE_KING_SIDE: (i8, i8) = (6, 0);
pub const PLAYER_TWO_CASTLE_QUEEN_SIDE: (i8, i8) =  (2, 0);

pub const PLAYER_ONE_KING_SIDE_ROOK: (i8, i8) = (7, 7);
pub const PLAYER_ONE_QUEEN_SIDE_ROOK: (i8, i8) = (0, 7);
pub const PLAYER_TWO_KING_SIDE_ROOK: (i8, i8) = (7, 0);
pub const PLAYER_TWO_QUEEN_SIDE_ROOK: (i8, i8) = (0, 0);

pub fn valid(point: (i8, i8)) -> bool {
    point.0 >= MIN_N && point.0 <= MAX_N && point.1 >= MIN_N && point.1 <= MAX_N
}

pub fn length(from: (i8, i8), to: (i8, i8)) -> i8 {
    let dx = (to.0 - from.0).abs();
    let dy = (to.1 - from.1).abs();
    if dx > dy {
        dx
    } else {
        dy
    }
}

pub fn direction_unit_n(from_n: i8, to_n: i8) -> i8 {
    let dn = to_n - from_n;
    if let Some(c) = dn.partial_cmp(&0) {
        match c {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        }
    } else {
        0
    }
}

pub fn side(from_x: i8, to_x: i8) -> Side {
    if to_x > from_x {
        Side::King
    } else {
        Side::Queen
    }
}

pub fn orthogonal_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = vec![];
    for y in MIN_N..=MAX_N {
       for x in MIN_N..=MAX_N {
           let to = (x, y);
           if orthogonal(from, to) {
                acc.push(to);
           }
       }
    }
    acc
}

pub fn l_shape_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = vec![];
    for y in MIN_N..=MAX_N {
       for x in MIN_N..=MAX_N {
            let to = (x, y);
            if l_shape(from, to) {
                acc.push(to);
            }
       }
    }
    acc
}

pub fn diagonal_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = vec![];
    for y in MIN_N..=MAX_N {
       for x in MIN_N..=MAX_N {
            let to = (x, y);
            if diagonal(from, to) {
                acc.push(to);
            }
       }
    }
    acc
}

pub fn orthogonal_or_diagonal_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = vec![];
    for y in MIN_N..=MAX_N {
       for x in MIN_N..=MAX_N {
            let to = (x, y);
            if orthogonal_or_diagonal(from, to) {
                acc.push(to);
            }
       }
    }
    acc
}

pub fn one_step_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = vec![];
    let min_x = from.0 - 1;
    let max_x = from.0 + 1;
    let min_y = from.1 - 1;
    let max_y = from.1 + 1;
    for y in MIN_N..=MAX_N {
       for x in MIN_N..=MAX_N {
            let to = (x, y);
            if to.0 >= min_x && to.0 <= max_x && to.1 >= min_y && to.1 <= max_y {
                acc.push(to);
            }
       }
    }
    acc
}

pub fn king_castle_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = vec![];
    if from.1 == 7 {
        acc.push(PLAYER_ONE_CASTLE_KING_SIDE);
        acc.push(PLAYER_ONE_CASTLE_QUEEN_SIDE);
    } else if from.1 == 0 {
        acc.push(PLAYER_TWO_CASTLE_KING_SIDE);
        acc.push(PLAYER_TWO_CASTLE_QUEEN_SIDE);
    }
    acc
}

pub fn pawn_destination_points(from: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    let mut acc = vec![];
    let move_x = from.0;
    let move_single_y = from.1 + forwards_direction(player_number);
    let move_double_y = from.1 + 2*forwards_direction(player_number);
    let r = range(from.1, player_number);
    if r == 2 {
        acc.push((move_x, move_double_y));
        acc.push((move_x, move_single_y));
    } else {
        acc.push((move_x, move_single_y));
    }
    acc
}

pub fn forward_diagonal_step_destination_points(from: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    let mut acc = vec![];
    let to_x_a = from.0 + 1;
    let to_x_b = from.0 - 1;
    let to_y = from.1 + forwards_direction(player_number);
    acc.push((to_x_a, to_y));
    acc.push((to_x_b, to_y));
    acc
}

pub fn orthogonal(from: (i8, i8), to: (i8, i8)) -> bool {
    (to.0 == from.0) ^ (to.1 == from.1)
}

pub fn l_shape(from: (i8, i8), to: (i8, i8)) -> bool {
    let abs_dx = (to.0 - from.0).abs();
    let abs_dy = (to.1 - from.1).abs();
    (abs_dx == 2 && abs_dy == 1) || (abs_dx == 1 && abs_dy == 2)
}


pub fn diagonal(from: (i8, i8), to: (i8, i8)) -> bool {
    let abs_dx = (to.0 - from.0).abs();
    abs_dx != 0 && abs_dx == (to.1 - from.1).abs()
}


pub fn orthogonal_or_diagonal(from: (i8, i8), to: (i8, i8)) -> bool {
    let abs_dx = (to.0 - from.0).abs();
    let abs_dy = (to.1 - from.1).abs();
    (abs_dx == 0 || abs_dy == 0) || (abs_dx != 0 && abs_dx == abs_dy)
}

pub fn forwards_direction(player_number: i8) -> i8 {
    match player_number {
        1 => -1,
        _ => 1,
    }
}

fn starting_rank(player_number: i8) -> i8 {
    match player_number {
        1 => 6,
        _ => 1
    }
}

fn range(y: i8, player_number: i8) -> i8 {
    if y == starting_rank(player_number) {
        2
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_true_test() {
        let point = (4, 4);
        let result = valid(point);
        assert_eq!(result, true);
    }

    #[test]
    fn valid_false_test() {
        let point = (4, 8);
        let result = valid(point);
        assert_eq!(result, false);
    }

    #[test]
    fn length_test() {
        let from = (1, 1);
        let to = (2, 4);
        let result = length(from, to);
        assert_eq!(result, 3);
    }

    #[test]
    fn direction_unit_y_test() {
        let from_y = 4;
        let to_y = 6;
        let result = direction_unit_n(from_y, to_y);
        assert_eq!(result, 1);
    }

    #[test]
    fn side_king_test() {
        let from_x = 5;
        let to_x = 7;
        let result = side(from_x, to_x);
        assert_eq!(result, Side::King);
    }

    #[test]
    fn side_queen_test() {
        let from_x = 5;
        let to_x = 3;
        let result = side(from_x, to_x);
        assert_eq!(result, Side::Queen);
    }

    #[test]
    fn orthogonal_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (4, 0), (4, 1), (4, 2), (4, 3), (0, 4), (1, 4), (2, 4), (3, 4), (5, 4), (6, 4), (7, 4), (4, 5), (4, 6), (4, 7)
        ];
        let result = orthogonal_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn l_shape_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (3, 2), (5, 2), (2, 3), (6, 3), (2, 5), (6, 5), (3, 6), (5, 6)
        ];
        let result = l_shape_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn diagonal_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (0, 0), (1, 1), (7, 1), (2, 2), (6, 2), (3, 3), (5, 3), (3, 5), (5, 5), (2, 6), (6, 6), (1, 7), (7, 7)
        ];
        let result = diagonal_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn orthogonal_or_diagonal_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (0, 0), (4, 0),
            (1, 1), (4, 1), (7, 1),
            (2, 2), (4, 2), (6, 2),
            (3, 3), (4, 3), (5, 3),
            (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (7, 4),
            (3, 5), (4, 5), (5, 5),
            (2, 6), (4, 6), (6, 6),
            (1, 7), (4, 7), (7, 7)
        ];
        let result = orthogonal_or_diagonal_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn one_step_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (3, 3), (4, 3), (5, 3), (3, 4), (4, 4), (5, 4), (3, 5), (4, 5), (5, 5)
        ];
        let result = one_step_destination_points(from);
        assert_eq!(result, expected);

    }

    #[test]
    fn king_castle_destination_points_row_zero_test() {
        let from = (4, 0);
        let expected = vec![
            (6, 0), (2, 0)
        ];
        let result = king_castle_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn king_castle_destination_points_row_middle_test() {
        let from = (4, 4);
        let expected = vec![ ];
        let result = king_castle_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn king_castle_destination_points_row_seven_test() {
        let from = (4, 7);
        let expected = vec![
            (6, 7), (2, 7)
        ];
        let result = king_castle_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn pawn_destination_points_starting_row_test() {
        let from = (0, 6);
        let player_number = 1;
        let expected = vec![
            (0, 4), (0, 5)
        ];
        let result = pawn_destination_points(from, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn pawn_destination_points_not_starting_row_test() {
        let from = (0, 5);
        let player_number = 1;
        let expected = vec![
            (0, 4)
        ];
        let result = pawn_destination_points(from, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn forward_diagonal_step_destination_points_test() {
        let from = (4, 4);
        let player_number = 1;
        let expected = vec![
            (5, 3), (3, 3)
        ];
        let result = forward_diagonal_step_destination_points(from, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn orthogonal_true_test() {
        let from = (4, 6);
        let to = (2, 6);
        let result = orthogonal(from, to);
        assert_eq!(result, true);
    }

    #[test]
    fn orthogonal_false_test() {
        let from = (4, 5);
        let to = (2, 6);
        let result = orthogonal(from, to);
        assert_eq!(result, false);
    }

    #[test]
    fn diagonal_true_test() {
        let from = (4, 4);
        let to = (2, 6);
        let result = diagonal(from, to);
        assert_eq!(result, true);
    }

    #[test]
    fn diagonal_false_test() {
        let from = (4, 6);
        let to = (2, 6);
        let result = diagonal(from, to);
        assert_eq!(result, false);
    }

    #[test]
    fn l_shape_true_test() {
        let from = (4, 4);
        let to = (5, 6);
        let result = l_shape(from, to);
        assert_eq!(result, true);
    }

    #[test]
    fn l_shape_false_test() {
        let from = (4, 4);
        let to = (6, 6);
        let result = l_shape(from, to);
        assert_eq!(result, false);
    }
}
