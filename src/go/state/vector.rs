pub fn orthogonal(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> bool {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    dx == 0 || dy == 0
}

pub fn magnitude(from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> i8 {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    if dx == 0 {
        dy.abs()
    } else if dy == 0 {
        dx.abs()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orthogonal_true_test() {
        let from_x = 0;
        let from_y = 0;
        let to_x = 0;
        let to_y = 1;
        let result = orthogonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, true);
    }

    #[test]
    fn orthogonal_false_test() {
        let from_x = 0;
        let from_y = 0;
        let to_x = 1;
        let to_y = 1;
        let result = orthogonal(from_x, from_y, to_x, to_y);
        assert_eq!(result, false);
    }

    #[test]
    fn magnitude_test() {
        let from_x = 0;
        let from_y = 0;
        let to_x = 0;
        let to_y = 2;
        let result = magnitude(from_x, from_y, to_x, to_y);
        assert_eq!(result, 2);
    }
}
