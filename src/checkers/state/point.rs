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
