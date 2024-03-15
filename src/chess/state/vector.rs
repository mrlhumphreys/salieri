use std::cmp::Ordering;
use crate::chess::state::castle_move::Side;

pub fn length(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> i8 {
    let dx = (to_x - from_x).abs();
    let dy = (to_y - from_y).abs();
    if dx > dy {
        dx
    } else {
        dy
    }
}

pub fn direction_unit_n(from_n: i8, to_n: i8) -> i8 {
    let dn = to_n - from_n;
    return match dn.partial_cmp(&0) {
        Some(c) => {
            match c {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0,
            }
        },
        None => 0,
    };
}

pub fn orthogonal(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> bool {
    (to_x == from_x) ^ (to_y == from_y)
}

pub fn diagonal(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> bool {
    let abs_dx = (to_x - from_x).abs();
    abs_dx != 0 && abs_dx == (to_y - from_y).abs()
}

pub fn orthogonal_or_diagonal(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> bool {
    let abs_dx = (to_x - from_x).abs();
    let abs_dy = (to_y - from_y).abs();
    (abs_dx == 0 || abs_dy == 0) || (abs_dx != 0 && abs_dx == abs_dy)
}

pub fn knight_jump(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> bool {
    let abs_dx = (to_x - from_x).abs();
    let abs_dy = (to_y - from_y).abs();
    (abs_dx == 2 && abs_dy == 1) || (abs_dx == 1 && abs_dy == 2)
}

pub fn side(from_x: i8, to_x: i8) -> Side {
    if to_x > from_x {
        Side::King
    } else {
        Side::Queen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_length_test() {
        let from_x = 1;
        let from_y = 1;
        let to_x = 2;
        let to_y = 4;
        let result = length(from_x, from_y, to_x, to_y);
        assert_eq!(result, 3);
    }

    #[test]
    fn vector_direction_unit_y_test() {
        let from_y = 4;
        let to_y = 6;
        let result = direction_unit_n(from_y, to_y);
        assert_eq!(result, 1);
    }

    #[test]
    fn vector_orthogonal_true_test() {
        let from_x = 4;
        let from_y = 6;
        let to_x = 2;
        let to_y = 6;
        let result = orthogonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, true);
    }

    #[test]
    fn vector_orthogonal_false_test() {
        let from_x = 4;
        let from_y = 5;
        let to_x = 2;
        let to_y = 6;
        let result = orthogonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, false);
    }

    #[test]
    fn vector_diagonal_true_test() {
        let from_x = 4;
        let from_y = 4;
        let to_x = 2;
        let to_y = 6;
        let result = diagonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, true);
    }

    #[test]
    fn vector_diagonal_false_test() {
        let from_x = 4;
        let from_y = 6;
        let to_x = 2;
        let to_y = 6;
        let result = diagonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, false);
    }

    #[test]
    fn vector_orthogonal_or_diagonal_diagonal_true_test() {
        let from_x = 4;
        let from_y = 4;
        let to_x = 2;
        let to_y = 6;
        let result = orthogonal_or_diagonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, true);
    }

    #[test]
    fn vector_orthogonal_or_diagonal_orthogonal_true_test() {
        let from_x = 4;
        let from_y = 6;
        let to_x = 2;
        let to_y = 6;
        let result = orthogonal_or_diagonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, true);
    }

    #[test]
    fn vector_orthogonal_or_diagonal_false_test() {
        let from_x = 4;
        let from_y = 4;
        let to_x = 5;
        let to_y = 6;
        let result = orthogonal_or_diagonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, false);
    }

    #[test]
    fn vector_knight_jump_true_test() {
        let from_x = 4;
        let from_y = 4;
        let to_x = 5;
        let to_y = 6;
        let result = knight_jump(from_x, from_y, to_x, to_y);
        assert_eq!(result, true);
    }

    #[test]
    fn vector_knight_jump_false_test() {
        let from_x = 4;
        let from_y = 4;
        let to_x = 6;
        let to_y = 6;
        let result = knight_jump(from_x, from_y, to_x, to_y);
        assert_eq!(result, false);
    }

    #[test]
    fn vector_side_king_test() {
        let from_x = 5;
        let to_x = 7;
        let result = side(from_x, to_x);
        assert_eq!(result, Side::King);
    }

    #[test]
    fn vector_side_false_test() {
        let from_x = 5;
        let to_x = 3;
        let result = side(from_x, to_x);
        assert_eq!(result, Side::Queen);
    }
}
