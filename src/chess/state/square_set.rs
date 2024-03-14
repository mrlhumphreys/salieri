use crate::chess::state::vector::direction_unit_n;
use crate::chess::state::vector::length;
use crate::chess::state::vector::orthogonal_or_diagonal;
use crate::chess::state::square::Square;

#[derive(PartialEq, Debug)]
pub struct SquareSet {
    pub squares: Vec<Square>
}

impl Clone for SquareSet {
    fn clone(&self) -> SquareSet {
        SquareSet {
            squares: self.squares.clone(),
        }
    }
}

pub fn find_by_x_and_y(squares: &Vec<Square>, x: i8, y: i8) -> Option<&Square> {
    squares.iter().find(|s| s.x == x && s.y == y)
}

impl SquareSet {
    pub fn between_unoccupied(&self, from: &Square, to: &Square) -> bool {
        let mut result = true;

        if length(from.x, from.y, to.x, to.y) > 1 && orthogonal_or_diagonal(from.x, from.y, to.x, to.y) {
            let direction_unit_x = direction_unit_n(from.x, to.x);
            let direction_unit_y = direction_unit_n(from.y, to.y);
            let end_x = to.x;
            let end_y = to.y;
            let mut counter_x = from.x + direction_unit_x;
            let mut counter_y = from.y + direction_unit_y;
            while counter_x != end_x || counter_y != end_y {
                let square = find_by_x_and_y(&self.squares, counter_x, counter_y);
                match square {
                    Some(s) => {
                        if s.occupied() {
                            result = false;
                        }
                    },
                    None => (),
                }
                counter_x = counter_x + direction_unit_x;
                counter_y = counter_y + direction_unit_y;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::state::piece::Piece;
    use crate::chess::state::piece::PieceKind;

    #[test]
    fn squares_set_find_by_x_and_y_test() {
        let square_a = Square { x: 1, y: 0, piece: None };
        let square_b = Square { x: 1, y: 1, piece: None };
        let squares = vec![square_a, square_b];

        let result = find_by_x_and_y(&squares, 1, 1);
        match result {
            Some(s) => {
                assert_eq!(s.x, 1);
                assert_eq!(s.y, 1);
            },
            None => assert!(false, "expected square")
        }
    }

    #[test]
    fn between_unoccupied_true_test() {
        let origin = Square { x: 1, y: 1, piece: None };
        let between = Square { x: 1, y: 2, piece: None };
        let destination = Square { x: 1, y: 3, piece: None };
        let beyond = Square { x: 1, y: 4, piece: None };
        let squares = vec![origin, between, destination, beyond];
        let square_set = SquareSet { squares };

        let result = square_set.between_unoccupied(&origin, &destination);
        assert_eq!(result, true);
    }

    #[test]
    fn between_unoccupied_false_test() {
        let origin = Square { x: 1, y: 1, piece: None };
        let between = Square { x: 1, y: 2, piece: Some(Piece { player_number: 1, kind: PieceKind::Pawn })};
        let gap = Square { x: 1, y: 3, piece: None };
        let destination = Square { x: 1, y: 4, piece: None };
        let beyond = Square { x: 1, y: 5, piece: None };
        let squares = vec![origin, between, gap, destination, beyond];
        let square_set = SquareSet { squares };

        let result = square_set.between_unoccupied(&origin, &destination);
        assert_eq!(result, false);
    }
}
