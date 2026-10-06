pub const ID_COORDINATE_MAP: [(i8, i8); 51] = [
    (10, 10),

    (8, 9),
    (6, 9),
    (4, 9),
    (2, 9),
    (0, 9),

    (9, 8),
    (7, 8),
    (5, 8),
    (3, 8),
    (1, 8),

    (8, 7),
    (6, 7),
    (4, 7),
    (2, 7),
    (0, 7),

    (9, 6),
    (7, 6),
    (5, 6),
    (3, 6),
    (1, 6),

    (8, 5),
    (6, 5),
    (4, 5),
    (2, 5),
    (0, 5),

    (9, 4),
    (7, 4),
    (5, 4),
    (3, 4),
    (1, 4),

    (8, 3),
    (6, 3),
    (4, 3),
    (2, 3),
    (0, 3),

    (9, 2),
    (7, 2),
    (5, 2),
    (3, 2),
    (1, 2),

    (8, 1),
    (6, 1),
    (4, 1),
    (2, 1),
    (0, 1),

    (9, 0),
    (7, 0),
    (5, 0),
    (3, 0),
    (1, 0)
];

//   50  49  48  47  46
// 45  44  43  42  41
//   40  39  38  37  36
// 35  34  33  32  31
//   30  29  28  27  26
// 25  24  23  22  21
//   20  19  18  17  16
// 15  14  13  12  11
//   10  09  08  07  06
// 05  04  03  02  01
pub fn point_to_id(point: (i8, i8)) -> i8 {
    match point {
        (8, 9) => 1,
        (6, 9) => 2,
        (4, 9) => 3,
        (2, 9) => 4,
        (0, 9) => 5,

        (9, 8) => 6,
        (7, 8) => 7,
        (5, 8) => 8,
        (3, 8) => 9,
        (1, 8) => 10,

        (8, 7) => 11,
        (6, 7) => 12,
        (4, 7) => 13,
        (2, 7) => 14,
        (0, 7) => 15,

        (9, 6) => 16,
        (7, 6) => 17,
        (5, 6) => 18,
        (3, 6) => 19,
        (1, 6) => 20,

        (8, 5) => 21,
        (6, 5) => 22,
        (4, 5) => 23,
        (2, 5) => 24,
        (0, 5) => 25,

        (9, 4) => 26,
        (7, 4) => 27,
        (5, 4) => 28,
        (3, 4) => 29,
        (1, 4) => 30,

        (8, 3) => 31,
        (6, 3) => 32,
        (4, 3) => 33,
        (2, 3) => 34,
        (0, 3) => 35,

        (9, 2) => 36,
        (7, 2) => 37,
        (5, 2) => 38,
        (3, 2) => 39,
        (1, 2) => 40,

        (8, 1) => 41,
        (6, 1) => 42,
        (4, 1) => 43,
        (2, 1) => 44,
        (0, 1) => 45,

        (9, 0) => 46,
        (7, 0) => 47,
        (5, 0) => 48,
        (3, 0) => 49,
        (1, 0) => 50,

        _ => 0
    }
}

pub fn add(a: (i8, i8), b: (i8, i8)) -> (i8, i8) {
    (a.0 + b.0, a.1 + b.1)
}

pub fn potential_jump_points(point: (i8, i8), king: bool) -> Vec<(i8, i8)> {
    if king {
        vec![
            (point.0 - 2, point.1 - 2),
            (point.0 - 2, point.1 + 2),
            (point.0 + 2, point.1 - 2),
            (point.0 + 2, point.1 + 2),

            (point.0 - 3, point.1 - 3),
            (point.0 - 3, point.1 + 3),
            (point.0 + 3, point.1 - 3),
            (point.0 + 3, point.1 + 3),

            (point.0 - 4, point.1 - 4),
            (point.0 - 4, point.1 + 4),
            (point.0 + 4, point.1 - 4),
            (point.0 + 4, point.1 + 4),

            (point.0 - 5, point.1 - 5),
            (point.0 - 5, point.1 + 5),
            (point.0 + 5, point.1 - 5),
            (point.0 + 5, point.1 + 5),

            (point.0 - 6, point.1 - 6),
            (point.0 - 6, point.1 + 6),
            (point.0 + 6, point.1 - 6),
            (point.0 + 6, point.1 + 6),

            (point.0 - 7, point.1 - 7),
            (point.0 - 7, point.1 + 7),
            (point.0 + 7, point.1 - 7),
            (point.0 + 7, point.1 + 7),

            (point.0 - 8, point.1 - 8),
            (point.0 - 8, point.1 + 8),
            (point.0 + 8, point.1 - 8),
            (point.0 + 8, point.1 + 8),

            (point.0 - 9, point.1 - 9),
            (point.0 - 9, point.1 + 9),
            (point.0 + 9, point.1 - 9),
            (point.0 + 9, point.1 + 9)
        ]
    } else {
        vec![
            (point.0 - 2, point.1 - 2),
            (point.0 - 2, point.1 + 2),
            (point.0 + 2, point.1 - 2),
            (point.0 + 2, point.1 + 2)
        ]
    }
}

pub fn potential_move_points(point: (i8, i8), player_number: i8, king: bool) -> Vec<(i8, i8)> {
    if king {
        vec![
            (point.0 - 1, point.1 - 1),
            (point.0 - 1, point.1 + 1),
            (point.0 + 1, point.1 - 1),
            (point.0 + 1, point.1 + 1),

            (point.0 - 2, point.1 - 2),
            (point.0 - 2, point.1 + 2),
            (point.0 + 2, point.1 - 2),
            (point.0 + 2, point.1 + 2),

            (point.0 - 3, point.1 - 3),
            (point.0 - 3, point.1 + 3),
            (point.0 + 3, point.1 - 3),
            (point.0 + 3, point.1 + 3),

            (point.0 - 4, point.1 - 4),
            (point.0 - 4, point.1 + 4),
            (point.0 + 4, point.1 - 4),
            (point.0 + 4, point.1 + 4),

            (point.0 - 5, point.1 - 5),
            (point.0 - 5, point.1 + 5),
            (point.0 + 5, point.1 - 5),
            (point.0 + 5, point.1 + 5),

            (point.0 - 6, point.1 - 6),
            (point.0 - 6, point.1 + 6),
            (point.0 + 6, point.1 - 6),
            (point.0 + 6, point.1 + 6),

            (point.0 - 7, point.1 - 7),
            (point.0 - 7, point.1 + 7),
            (point.0 + 7, point.1 - 7),
            (point.0 + 7, point.1 + 7),

            (point.0 - 8, point.1 - 8),
            (point.0 - 8, point.1 + 8),
            (point.0 + 8, point.1 - 8),
            (point.0 + 8, point.1 + 8),

            (point.0 - 9, point.1 - 9),
            (point.0 - 9, point.1 + 9),
            (point.0 + 9, point.1 - 9),
            (point.0 + 9, point.1 + 9),
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
    fn add_test() {
        let a = (1, 1);
        let b = (-1, 0);
        let result = add(a, b);
        assert_eq!(result, (0, 1));
    }

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
            (5, 5),

            (2, 2),
            (2, 6),
            (6, 2),
            (6, 6),

            (1, 1),
            (1, 7),
            (7, 1),
            (7, 7),

            (0, 0),
            (0, 8),
            (8, 0),
            (8, 8),

            (-1, -1),
            (-1, 9),
            (9, -1),
            (9, 9),

            (-2, -2),
            (-2, 10),
            (10, -2),
            (10, 10),

            (-3, -3),
            (-3, 11),
            (11, -3),
            (11, 11),

            (-4, -4),
            (-4, 12),
            (12, -4),
            (12, 12),

            (-5, -5),
            (-5, 13),
            (13, -5),
            (13, 13)
        ];
        let result = potential_move_points(point, player_number, king);
        assert_eq!(result, expected);
    }

    #[test]
    fn potential_jump_points_king_test() {
        let king = true;
        let point = (4, 4);
        let expected = vec![
            (2, 2),
            (2, 6),
            (6, 2),
            (6, 6),

            (1, 1),
            (1, 7),
            (7, 1),
            (7, 7),

            (0, 0),
            (0, 8),
            (8, 0),
            (8, 8),

            (-1, -1),
            (-1, 9),
            (9, -1),
            (9, 9),

            (-2, -2),
            (-2, 10),
            (10, -2),
            (10, 10),

            (-3, -3),
            (-3, 11),
            (11, -3),
            (11, 11),

            (-4, -4),
            (-4, 12),
            (12, -4),
            (12, 12),

            (-5, -5),
            (-5, 13),
            (13, -5),
            (13, 13)
        ];
        let result = potential_jump_points(point, king);
        assert_eq!(result, expected);
    }
}
