pub const ID_COORDINATE_MAP: [(i8, i8); 33] = [
    (8, 8),

    (6, 7),
    (4, 7),
    (2, 7),
    (0, 7),

    (7, 6),
    (5, 6),
    (3, 6),
    (1, 6),

    (6, 5),
    (4, 5),
    (2, 5),
    (0, 5),

    (7, 4),
    (5, 4),
    (3, 4),
    (1, 4),

    (6, 3),
    (4, 3),
    (2, 3),
    (0, 3),

    (7, 2),
    (5, 2),
    (3, 2),
    (1, 2),

    (6, 1),
    (4, 1),
    (2, 1),
    (0, 1),

    (7, 0),
    (5, 0),
    (3, 0),
    (1, 0)
];

pub fn point_to_id(point: (i8, i8)) -> i8 {
    match point {
        (6, 7) => 1,
        (4, 7) => 2,
        (2, 7) => 3,
        (0, 7) => 4,

        (7, 6) => 5,
        (5, 6) => 6,
        (3, 6) => 7,
        (1, 6) => 8,

        (6, 5) => 9,
        (4, 5) => 10,
        (2, 5) => 11,
        (0, 5) => 12,

        (7, 4) => 13,
        (5, 4) => 14,
        (3, 4) => 15,
        (1, 4) => 16,

        (6, 3) => 17,
        (4, 3) => 18,
        (2, 3) => 19,
        (0, 3) => 20,

        (7, 2) => 21,
        (5, 2) => 22,
        (3, 2) => 23,
        (1, 2) => 24,

        (6, 1) => 25,
        (4, 1) => 26,
        (2, 1) => 27,
        (0, 1) => 28,

        (7, 0) => 29,
        (5, 0) => 30,
        (3, 0) => 31,
        (1, 0) => 32,

        _ => 0
    }
}

pub fn potential_jump_points(point: (i8, i8), player_number: i8, king: bool) -> Vec<(i8, i8)> {
    if king {
        vec![
            (point.0 - 2, point.1 - 2),
            (point.0 - 2, point.1 + 2),
            (point.0 + 2, point.1 - 2),
            (point.0 + 2, point.1 + 2)
        ]
    } else {
        if player_number == 2 {
            vec![
                (point.0 - 2, point.1 + 2),
                (point.0 + 2, point.1 + 2)
            ]
        } else {
            vec![
                (point.0 - 2, point.1 - 2),
                (point.0 + 2, point.1 - 2)
            ]
        }
    }
}

pub fn potential_move_points(point: (i8, i8), player_number: i8, king: bool) -> Vec<(i8, i8)> {
    if king {
        vec![
            (point.0 - 1, point.1 - 1),
            (point.0 - 1, point.1 + 1),
            (point.0 + 1, point.1 - 1),
            (point.0 + 1, point.1 + 1)
        ]
    } else {
        if player_number == 2 {
            vec![
                (point.0 - 1, point.1 + 1),
                (point.0 + 1, point.1 + 1)
            ]
        } else {
            vec![
                (point.0 - 1, point.1 - 1),
                (point.0 + 1, point.1 - 1)
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potential_move_points_player_one_test() {
        let player_number = 1;
        let king = false;
        let point = (4, 4);
        let expected = vec![
            (3, 3),
            (5, 3)
        ];
        let result = potential_move_points(point, player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn potential_move_points_player_two_test() {
        let player_number = 2;
        let king = false;
        let point = (4, 4);
        let expected = vec![
            (3, 5),
            (5, 5)
        ];
        let result = potential_move_points(point, player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn potential_move_points_king_test() {
        let player_number = 1;
        let king = true;
        let point = (4, 4);
        let expected = vec![
            (3, 3),
            (3, 5),
            (5, 3),
            (5, 5)
        ];
        let result = potential_move_points(point, player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn potential_jump_points_player_one_test() {
        let player_number = 1;
        let king = false;
        let point = (4, 4);
        let expected = vec![
            (2, 2),
            (6, 2),
        ];
        let result = potential_jump_points(point, player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn potential_jump_points_king_test() {
        let player_number = 1;
        let king = true;
        let point = (4, 4);
        let expected = vec![
            (2, 2),
            (2, 6),
            (6, 2),
            (6, 6)
        ];
        let result = potential_jump_points(point, player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn potential_jump_points_player_two_test() {
        let player_number = 2;
        let king = false;
        let point = (4, 4);
        let expected = vec![
            (2, 6),
            (6, 6),
        ];
        let result = potential_jump_points(point, player_number, king);
        assert_eq!(result, expected);
    }
}
