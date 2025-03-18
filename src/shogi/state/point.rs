use std::cmp::Ordering;

pub const MIN_N: i8 = 0;
pub const MAX_N: i8 = 8;

pub fn diff<'a>(a: &'a Vec<(i8, i8)>, b: &'a Vec<(i8, i8)>) -> Vec<&'a (i8, i8)> {
    return a.iter().filter(|ae| {
        return !b.iter().any(|be| { return be == *ae; });
    }).collect();
}

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

pub fn ryuuou_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = orthogonal_destination_points(from);
    acc.push(add(from, (-1, -1)));
    acc.push(add(from, (1, -1)));
    acc.push(add(from, (1, 1)));
    acc.push(add(from, (-1, 1)));
    acc
}

pub fn l_shape_forwards_destination_points(from: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    let one_step_left = from.0 - 1;
    let one_step_right = from.0 + 1;
    let two_steps_forwards = from.1 + 2*forwards_direction(player_number);

    vec![
        (one_step_left, two_steps_forwards),
        (one_step_right, two_steps_forwards)
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

pub fn ryuuma_destination_points(from: (i8, i8)) -> Vec<(i8, i8)> {
    let mut acc = diagonal_destination_points(from);
    acc.push(add(from, (0, -1)));
    acc.push(add(from, (1, 0)));
    acc.push(add(from, (0, 1)));
    acc.push(add(from, (-1, 0)));
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

pub fn one_step_forward_destination_points(from: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    vec![ (from.0, from.1 + forwards_direction(player_number)) ]
}

pub fn forward_destination_points(from: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    let mut acc = vec![];

    let direction_unit = (0, forwards_direction(player_number));
    let mut counter = add(from, direction_unit);
    while (direction_unit.1 == 1 && counter.1 <= MAX_N) || (direction_unit.1 == -1 && counter.1 >= MIN_N) {
        acc.push(counter);
        counter = add(counter, direction_unit);
    }

    acc
}

pub fn gin_destination_points(from: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    vec![
        add(from, (-1, -1)),
        add(from, (1, -1)),
        add(from, (1, 1)),
        add(from, (-1, 1)),
        add(from, (0, forwards_direction(player_number)))
    ]
}

pub fn kin_destination_points(from: (i8, i8), player_number: i8) -> Vec<(i8, i8)> {
    vec![
        add(from, (0, -1)),
        add(from, (1, 0)),
        add(from, (0, 1)),
        add(from, (-1, 0)),
        add(from, (-1, forwards_direction(player_number))),
        add(from, (1, forwards_direction(player_number)))
    ]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_test_test() {
       let a = vec![(0, 0), (1, 1), (2, 2), (3, 3)];
       let b = vec![(2, 2), (3, 3)];
       let expected: Vec<&(i8, i8)> = vec![&(0, 0), &(1, 1)];
       let result = diff(&a, &b);
       assert_eq!(result, expected);
    }

    #[test]
    fn valid_true_test() {
        let point = (4, 4);
        let result = valid(point);
        assert_eq!(result, true);
    }

    #[test]
    fn valid_false_test() {
        let point = (4, 9);
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

    #[test]
    fn orthogonal_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (4, 3), (4, 2), (4, 1), (4, 0), (5, 4), (6, 4), (7, 4), (8, 4), (4, 5), (4, 6), (4, 7), (4, 8), (3, 4), (2, 4), (1, 4), (0, 4)
        ];
        let result = orthogonal_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn ryuuou_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (4, 3), (4, 2), (4, 1), (4, 0), (5, 4), (6, 4), (7, 4), (8, 4), (4, 5), (4, 6), (4, 7), (4, 8), (3, 4), (2, 4), (1, 4), (0, 4), (3, 3), (5, 3), (5, 5), (3, 5)
        ];
        let result = ryuuou_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn l_shape_forwards_destination_points_test() {
        let from = (4, 4);
        let player_number = 1;
        let expected = vec![
            (3, 2), (5, 2)
        ];
        let result = l_shape_forwards_destination_points(from, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn diagonal_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (3, 3), (2, 2), (1, 1), (0, 0), (5, 3), (6, 2), (7, 1), (8, 0), (5, 5), (6, 6), (7, 7), (8, 8), (3, 5), (2, 6), (1, 7), (0, 8)
        ];
        let result = diagonal_destination_points(from);
        assert_eq!(result, expected);
    }

    #[test]
    fn ryuuma_destination_points_test() {
        let from = (4, 4);
        let expected = vec![
            (3, 3), (2, 2), (1, 1), (0, 0), (5, 3), (6, 2), (7, 1), (8, 0), (5, 5), (6, 6), (7, 7), (8, 8), (3, 5), (2, 6), (1, 7), (0, 8), (4, 3), (5, 4), (4, 5), (3, 4)
        ];
        let result = ryuuma_destination_points(from);
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
    fn one_step_forward_destination_points_test() {
        let from = (4, 4);
        let player_number = 1;
        let expected = vec![
            (4, 3)
        ];
        let result = one_step_forward_destination_points(from, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn forward_destination_points_test() {
        let from = (4, 4);
        let player_number = 1;
        let expected = vec![
            (4, 3), (4, 2), (4, 1), (4, 0)
        ];
        let result = forward_destination_points(from, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn gin_destination_points_test() {
        let from = (4, 4);
        let player_number = 1;
        let expected = vec![
            (3, 3), (5, 3), (5, 5), (3, 5), (4, 3)
        ];
        let result = gin_destination_points(from, player_number);
        assert_eq!(result, expected);
    }

    #[test]
    fn kin_destination_points_test() {
        let from = (4, 4);
        let player_number = 1;
        let expected = vec![
            (4, 3), (5, 4), (4, 5), (3, 4), (3, 3), (5, 3)
        ];
        let result = kin_destination_points(from, player_number);
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
