use std::cmp::Ordering;

pub struct Vector {
    pub from: (i8, i8),
    pub to: (i8, i8)
}

impl Vector {
    pub fn direction_unit(&self) -> (i8, i8) {
        let dx = self.to.0 - self.from.0;
        let dy = self.to.1 - self.from.1;
        let ux = if let Some(c) = dx.partial_cmp(&0) {
            match c {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0
            }
        } else {
            0
        };
        let uy = if let Some(c) = dy.partial_cmp(&0) {
            match c {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0
            }
        } else {
            0
        };

        (ux, uy)
    }

    pub fn diagonal(&self) -> bool {
        let abs_dx = (self.to.0 as i8 - self.from.0 as i8).abs();
        abs_dx != 0 && abs_dx == (self.to.1 as i8 - self.from.1 as i8).abs()
    }

    pub fn orthogonal(&self) -> bool {
        (self.to.0 == self.from.0) ^ (self.to.1 == self.from.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_unit_test() {
        let from = (5, 4);
        let to = (4, 6);
        let vector = Vector { from, to };
        let result = vector.direction_unit();
        assert_eq!(result.0, -1);
        assert_eq!(result.1, 1);
    }

    #[test]
    fn same_dx_and_dy() {
        let from = (4, 4);
        let to = (2, 6);
        let vector = Vector { from, to };
        assert_eq!(vector.diagonal(), true);
        assert_eq!(vector.orthogonal(), false);
    }

    #[test]
    fn same_y_or_x() {
        let from = (4, 4);
        let to = (4, 6);
        let vector = Vector { from, to };
        assert_eq!(vector.diagonal(), false);
        assert_eq!(vector.orthogonal(), true);
    }

    #[test]
    fn different_dx_and_dy_and_x_and_y() {
        let from = (5, 4);
        let to = (4, 6);
        let vector = Vector { from, to };
        assert_eq!(vector.diagonal(), false);
        assert_eq!(vector.orthogonal(), false);
    }

    #[test]
    fn same_points() {
        let from = (5, 4);
        let to = (5, 4);
        let vector = Vector { from, to };
        assert_eq!(vector.diagonal(), false);
        assert_eq!(vector.orthogonal(), false);
    }
}
