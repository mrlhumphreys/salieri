use std::cmp::Ordering;

pub const MIN_X: i8 = 0;
pub const MAX_X: i8 = 8;
pub const MIN_Y: i8 = 0;
pub const MAX_Y: i8 = 9;

// pub fn diff<'a>(a: &'a Vec<(i8, i8)>, b: &'a Vec<(i8, i8)>) -> Vec<&'a (i8, i8)> {
//     return a.iter().filter(|ae| {
//         return !b.iter().any(|be| { return be == *ae; });
//     }).collect();
// }

pub fn valid(point: (i8, i8)) -> bool {
    point.0 >= MIN_X && point.0 <= MAX_X && point.1 >= MIN_Y && point.1 <= MAX_Y
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

// pub fn between(from: (i8, i8), to: (i8, i8)) -> Vec<(i8, i8)> {
//     let mut acc = vec![];
//
//     if orthogonal_or_diagonal(from, to) && length(from, to) > 1 {
//         let direction_unit = direction_unit(from, to);
//         let end = to;
//         let mut counter = add(from, direction_unit);
//         while counter != end {
//             acc.push(counter);
//             counter = add(counter, direction_unit);
//         }
//     }
//     acc
// }

// the next points in a line following a, b
// pub fn points_in_line(a: (i8, i8), b: (i8, i8)) -> Vec<(i8, i8)> {
//     let du = direction_unit(a, b);
//     let mut counter = add(b, du);
//     let mut acc = vec![];
//     while counter.0 <= MAX_X && counter.0 >= MIN_X && counter.1 <= MAX_Y && counter.1 >= MIN_Y {
//         acc.push(counter);
//         counter = add(counter, du);
//     }
//     acc
// }

pub fn soldier_destination_points(from: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    let direction = forwards_direction(player_number);

    if player_number == 1 {
        // 0, 1, 2, 3, 4
        if from.1 <= 4 {
            vec![
                (from.0, from.1 + direction),
                (from.0 - 1, from.1),
                (from.0 + 1, from.1)
            ]
        } else {
            vec![
                (from.0, from.1 + direction)
            ]
        }
    } else {
        // 9, 8, 7, 6, 5
        if from.1 >= 5 {
            vec![
                (from.0, from.1 + direction),
                (from.0 - 1, from.1),
                (from.0 + 1, from.1)
            ]
        } else {
            vec![
                (from.0, from.1 + direction)
            ]
        }
    }
}

pub fn orthogonal_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = vec![];

    let direction_unit = (0, -1);
    let mut counter = add(from, direction_unit);
    while counter.1 >= MIN_Y {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (1, 0);
    let mut counter = add(from, direction_unit);
    while counter.0 <= MAX_X {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (0, 1);
    let mut counter = add(from, direction_unit);
    while counter.1 <= MAX_Y {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    let direction_unit = (-1, 0);
    let mut counter = add(from, direction_unit);
    while counter.0 >= MIN_X {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    acc
}

pub fn horse_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let one_step_left = from.0 - 1;
    let one_step_right = from.0 + 1;
    let two_step_left = from.0 - 2;
    let two_step_right = from.0 + 2;

    let one_step_up = from.1 - 1;
    let one_step_down = from.1 + 1;
    let two_step_up = from.1 - 2;
    let two_step_down = from.1 + 2;

    vec![
        (one_step_left, two_step_up),
        (one_step_right, two_step_up),
        (two_step_left, one_step_up),
        (two_step_right, one_step_up),
        (two_step_left, one_step_down),
        (two_step_right, one_step_down),
        (one_step_left, two_step_down),
        (one_step_right, two_step_down)
    ]
}

pub fn elephant_destination_points(from: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    let two_step_left = from.0 - 2;
    let two_step_right = from.0 + 2;
    let two_step_up = from.1 - 2;
    let two_step_down = from.1 + 2;

    if player_number == 1 {
        // 9, 8, 7, 6, 5 : y >= 5
        if two_step_up >= 5 {
            vec![
                (two_step_left, two_step_up),
                (two_step_right, two_step_up),
                (two_step_left, two_step_down),
                (two_step_right, two_step_down)
            ]
        } else {
            vec![
                (two_step_left, two_step_down),
                (two_step_right, two_step_down)
            ]
        }
    } else {
        // 0, 1, 2, 3, 4 : y <= 4
        if two_step_up <= 4 {
            vec![
                (two_step_left, two_step_up),
                (two_step_right, two_step_up),
                (two_step_left, two_step_down),
                (two_step_right, two_step_down)
            ]
        } else {
            vec![
                (two_step_left, two_step_up),
                (two_step_right, two_step_up)
            ]
        }
    }
}

pub fn advisor_destination_points(from: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    // X: 3, 4, 5
    // Y1: 9, 8, 7
    // Y2: 0, 1, 2
    if player_number == 1 {
        match from {
            (3, 7) => vec![(4, 8)],
            (5, 7) => vec![(4, 8)],
            (4, 8) => vec![(3, 7), (5, 7), (3, 9), (5, 9)],
            (3, 9) => vec![(4, 8)],
            (5, 9) => vec![(4, 8)],
            _ => vec![]
        }
    } else {
        match from {
            (3, 0) => vec![(4, 1)],
            (5, 0) => vec![(4, 1)],
            (4, 1) => vec![(3, 0), (5, 0), (3, 2), (5, 2)],
            (3, 2) => vec![(4, 1)],
            (5, 2) => vec![(4, 1)],
            _ => vec![]
        }
    }
}

pub fn king_destination_points(from: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    if player_number == 1 {
        match from {
            (3, 7) => vec![(4, 7), (3, 8)],
            (4, 7) => vec![(3, 7), (5, 7), (4, 8)],
            (5, 7) => vec![(4, 7), (5, 8)],

            (3, 8) => vec![(3, 7), (4, 8), (3, 9)],
            (4, 8) => vec![(4, 7), (3, 8), (5, 8), (4, 9)],
            (5, 8) => vec![(5, 7), (4, 8), (5, 9)],

            (3, 9) => vec![(3, 8), (4, 9)],
            (4, 9) => vec![(4, 8), (3, 9), (5, 9)],
            (5, 9) => vec![(5, 8), (4, 8)],
            _ => vec![]
        }
    } else {
        match from {
            (3, 0) => vec![(4, 0), (3, 1)],
            (4, 0) => vec![(3, 0), (5, 0), (4, 1)],
            (5, 0) => vec![(4, 0), (5, 1)],

            (3, 1) => vec![(3, 0), (4, 1), (3, 2)],
            (4, 1) => vec![(4, 0), (3, 1), (5, 1), (4, 2)],
            (5, 1) => vec![(5, 0), (4, 1), (5, 2)],

            (3, 2) => vec![(3, 1), (4, 2)],
            (4, 2) => vec![(4, 1), (3, 2), (5, 2)],
            (5, 2) => vec![(5, 1), (4, 2)],
            _ => vec![]
        }
    }
}

pub fn orthogonal_or_diagonal(from: (i8, i8), to: (i8, i8)) -> bool {
    let abs_dx = (to.0 - from.0).abs();
    let abs_dy = (to.1 - from.1).abs();
    (abs_dx == 0 || abs_dy == 0) || (abs_dx != 0 && abs_dx == abs_dy)
}

// pub fn orthogonal(from: (i8, i8), to: (i8, i8)) -> bool {
//     (from.0 == to.0) || (from.1 == to.1)
// }

// pub fn diagonal(from: (i8, i8), to: (i8, i8)) -> bool {
//     let abs_dx = (to.0 - from.0).abs();
//     let abs_dy = (to.1 - from.1).abs();
//     abs_dx == abs_dy
// }

// pub fn forwards_for_player(from: (i8, i8), to: (i8, i8), player_number: i8) -> bool {
//     let dy = to.1 - from.1;
//     let direction = forwards_direction(player_number);
//     (dy > 0 && direction == 1) || (dy < 0 && direction == -1)
// }

pub fn forwards_direction(player_number: i8) -> i8 {
    match player_number {
        1 => -1,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn diff_test_test() {
    //    let a = vec![(0, 0), (1, 1), (2, 2), (3, 3)];
    //    let b = vec![(2, 2), (3, 3)];
    //    let expected: Vec<&(i8, i8)> = vec![&(0, 0), &(1, 1)];
    //    let result = diff(&a, &b);
    //    assert_eq!(result, expected);
    // }

    #[test]
    fn valid_true_test() {
        let point = (4, 4);
        let result = valid(point);
        assert_eq!(result, true);
    }

    #[test]
    fn valid_false_test() {
        let point = (4, 10);
        let result = valid(point);
        assert_eq!(result, false);
    }

    #[test]
    fn add_test() {
        let a = (1, 1);
        let b = (-1, 0);
        let result = add(a, b);
        assert_eq!(result, (0, 1));
    }

    #[test]
    fn length_test() {
        let from = (1, 1);
        let to = (2, 4);
        let result = length(from, to);
        assert_eq!(result, 3);
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

    // #[test]
    // fn between_orthogonal_test() {
    //     let from = (4, 4);
    //     let to = (8, 4);
    //     let expected = vec![
    //         (5, 4), (6, 4), (7, 4)
    //     ];
    //     let result = between(from, to);
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn between_diagonal_test() {
    //     let from = (4, 4);
    //     let to = (8, 8);
    //     let expected = vec![
    //         (5, 5), (6, 6), (7, 7)
    //     ];
    //     let result = between(from, to);
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn points_in_line_orthogonal_test() {
    //     let from = (4, 4);
    //     let to = (4, 3);
    //     let expected = vec![
    //         (4, 2), (4, 1), (4, 0)
    //     ];
    //     let result = points_in_line(from, to);
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn points_in_line_diagonal_test() {
    //     let from = (4, 4);
    //     let to = (5, 3);
    //     let expected = vec![
    //         (6, 2), (7, 1), (8, 0)
    //     ];
    //     let result = points_in_line(from, to);
    //     assert_eq!(result, expected);
    // }

    #[test]
    fn soldier_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (4, 3), (3, 4), (5, 4)
        ];
        let result = soldier_destination_points(from, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn orthogonal_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (4, 3), (4, 2), (4, 1), (4, 0), (5, 4), (6, 4), (7, 4), (8, 4), (4, 5), (4, 6), (4, 7), (4, 8), (4, 9), (3, 4), (2, 4), (1, 4), (0, 4)
        ];
        let result = orthogonal_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn horse_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (3, 2), (5, 2), (2, 3), (6, 3), (2, 5), (6, 5), (3, 6), (5, 6)
        ];
        let result = horse_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn elephant_destination_points_test() {
        let from = (2, 7);
        let player_number = 1;
        let expected = vec![
            (0, 5), (4, 5), (0, 9), (4, 9)
        ];
        let result = elephant_destination_points(from, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn advisor_destination_points_test() {
        let from = (3, 9);
        let player_number = 1;
        let expected = vec![
            (4, 8)
        ];
        let result = advisor_destination_points(from, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn king_destination_points_test() {
        let from = (4, 9);
        let player_number = 1;
        let expected = vec![
            (4, 8), (3, 9), (5, 9)
        ];
        let result = king_destination_points(from, player_number);
        assert_eq!(result, expected);
    }
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

    // #[test]
    // fn orthogonal_true_test() {
    //     let from = (4, 4);
    //     let to = (4, 3);
    //     let result = orthogonal(from, to);
    //     assert_eq!(result, true);
    // }

    // #[test]
    // fn orthogonal_false_test() {
    //     let from = (4, 4);
    //     let to = (5, 3);
    //     let result = orthogonal(from, to);
    //     assert_eq!(result, false);
    // }

    // #[test]
    // fn diagonal_true_test() {
    //     let from = (4, 4);
    //     let to = (5, 3);
    //     let result = diagonal(from, to);
    //     assert_eq!(result, true);
    // }

    // #[test]
    // fn diagonal_false_test() {
    //     let from = (4, 4);
    //     let to = (4, 3);
    //     let result = diagonal(from, to);
    //     assert_eq!(result, false);
    // }

    // #[test]
    // fn forwards_for_player_true_test() {
    //     let from = (4, 4);
    //     let to = (4, 3);
    //     let player_number = 1;
    //     let result = forwards_for_player(from, to, player_number);
    //     assert_eq!(result, true);
    // }

    // #[test]
    // fn forwards_for_player_false_test() {
    //     let from = (4, 4);
    //     let to = (4, 5);
    //     let player_number = 1;
    //     let result = forwards_for_player(from, to, player_number);
    //     assert_eq!(result, false);
    // }

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
