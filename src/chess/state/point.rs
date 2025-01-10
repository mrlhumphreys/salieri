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

pub fn add(a: (i8, i8), b: (i8, i8)) -> (i8, i8) {
    (a.0 + b.0, a.1 + b.1)
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

pub fn direction_unit(from: (i8, i8), to: (i8, i8)) -> (i8, i8) {
    (direction_unit_n(from.0, to.0), direction_unit_n(from.1, to.1))
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

    let direction_unit = (0, -1);
    let mut counter = add(from, direction_unit);
    while counter.1 >= MIN_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (1, 0);
    let mut counter = add(from, direction_unit);
    while counter.0 <= MAX_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (0, 1);
    let mut counter = add(from, direction_unit);
    while counter.1 <= MAX_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (-1, 0);
    let mut counter = add(from, direction_unit);
    while counter.0 >= MIN_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    acc
}

pub fn l_shape_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    vec![
        add(from, (-1, -2)),
        add(from, ( 1, -2)),
        add(from, ( 2, -1)),
        add(from, ( 2,  1)),
        add(from, ( 1,  2)),
        add(from, (-1,  2)),
        add(from, (-2,  1)),
        add(from, (-2, -1))
    ]
}

pub fn diagonal_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = vec![];

    let direction_unit = (-1, -1);
    let mut counter = add(from, direction_unit);
    while counter.0 >= MIN_N && counter.1 >= MIN_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (1, -1);
    let mut counter = add(from, direction_unit);
    while counter.0 <= MAX_N && counter.1 >= MIN_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (1, 1);
    let mut counter = add(from, direction_unit);
    while counter.0 <= MAX_N && counter.1 <= MAX_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (-1, 1);
    let mut counter = add(from, direction_unit);
    while counter.0 >= MIN_N && counter.1 <= MAX_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    acc
}

pub fn orthogonal_or_diagonal_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = vec![];

    let direction_unit = (-1, -1);
    let mut counter = add(from, direction_unit);
    while counter.0 >= MIN_N && counter.1 >= MIN_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (0, -1);
    let mut counter = add(from, direction_unit);
    while counter.1 >= MIN_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (1, -1);
    let mut counter = add(from, direction_unit);
    while counter.0 <= MAX_N && counter.1 >= MIN_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (1, 0);
    let mut counter = add(from, direction_unit);
    while counter.0 <= MAX_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (1, 1);
    let mut counter = add(from, direction_unit);
    while counter.0 <= MAX_N && counter.1 <= MAX_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (0, 1);
    let mut counter = add(from, direction_unit);
    while counter.1 <= MAX_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (-1, 1);
    let mut counter = add(from, direction_unit);
    while counter.0 >= MIN_N && counter.1 <= MAX_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (-1, 0);
    let mut counter = add(from, direction_unit);
    while counter.0 >= MIN_N {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    acc
}

pub fn one_step_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    vec![
        add(from, (-1, -1)),
        add(from, ( 0, -1)),
        add(from, ( 1, -1)),
        add(from, ( 1,  0)),
        add(from, ( 1,  1)),
        add(from, ( 0,  1)),
        add(from, (-1,  1)),
        add(from, (-1,  0))
    ]
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
    }
    acc.push((move_x, move_single_y));
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

fn range(y: i8, player_number: i8) -> i8 {
    if (player_number == 1 && y == 6) || (player_number == 2 && y == 1) {
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
    fn add_test() {
        let a = (1, 1);
        let b = (-1, 0);
        let result = add(a, b);
        assert_eq!(result, (0, 1));
    }

    #[test]
    fn direction_unit_n_test() {
        let from_y = 4;
        let to_y = 6;
        let result = direction_unit_n(from_y, to_y);
        assert_eq!(result, 1);
    }

    #[test]
    fn direction_unit_test() {
        let from = (4, 4);
        let to = (6, 2);
        let result = direction_unit(from, to);
        assert_eq!(result, (1, -1));
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
            (4, 3), (4, 2), (4, 1), (4, 0), (5, 4), (6, 4), (7, 4), (4, 5), (4, 6), (4, 7), (3, 4), (2, 4), (1, 4), (0, 4)
        ];
        let result = orthogonal_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn l_shape_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (3, 2), (5, 2), (6, 3), (6, 5), (5, 6), (3, 6), (2, 5), (2, 3)
        ];
        let result = l_shape_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn diagonal_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (3, 3), (2, 2), (1, 1), (0, 0), (5, 3), (6, 2), (7, 1), (5, 5), (6, 6), (7, 7), (3, 5), (2, 6), (1, 7)
        ];
        let result = diagonal_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn orthogonal_or_diagonal_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (3, 3), (2, 2), (1, 1), (0, 0),
            (4, 3), (4, 2), (4, 1), (4, 0),
            (5, 3), (6, 2), (7, 1),
            (5, 4), (6, 4), (7, 4),
            (5, 5), (6, 6), (7, 7),
            (4, 5), (4, 6), (4, 7),
            (3, 5), (2, 6), (1, 7),
            (3, 4), (2, 4), (1, 4), (0, 4)
        ];
        let result = orthogonal_or_diagonal_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn one_step_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (3, 3), (4, 3), (5, 3), (5, 4), (5, 5), (4, 5), (3, 5), (3, 4)
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
    fn orthogonal_or_diagonal_true_test() {
       let from = (0, 1);
       let to = (0, 4);
       let result = orthogonal_or_diagonal(from, to);
       assert_eq!(result, true);
    }

    #[test]
    fn orthogonal_or_diagonal_false_test() {
       let from = (0, 1);
       let to = (1, 3);
       let result = orthogonal_or_diagonal(from, to);
       assert_eq!(result, false);
    }

    #[test]
    fn forwards_direction_one_test() {
        let player_number = 1;
        let result = forwards_direction(player_number);
        assert_eq!(result, -1);
    }

    #[test]
    fn forwards_direction_two_test() {
        let player_number = 2;
        let result = forwards_direction(player_number);
        assert_eq!(result, 1);
    }
}
