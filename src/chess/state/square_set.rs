use crate::chess::state::vector::Vector;
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

impl SquareSet {
    pub fn find_by_x_and_y(&self, x: i8, y: i8) -> Option<&Square> {
        self.squares.iter().find(|s| s.x == x && s.y == y) 
    }

    pub fn between_unoccupied(&self, from: &Square, to: &Square) -> bool {
        let vector = Vector { from: from.point(), to: to.point() };
        let mut result = true;

        if vector.length() > 1 && (vector.diagonal() || vector.orthogonal()) {
            let direction_unit = vector.direction_unit();
            let end = to.point();
            let mut counter = from.point() + direction_unit; 
            while counter != end {
                let square = self.find_by_x_and_y(counter.x, counter.y);
                match square {
                    Some(s) => {
                        if s.occupied() {
                            result = false;
                        }
                    },
                    None => (), 
                }
                counter = counter + direction_unit;
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
    fn find_by_x_and_y_test() {
        let square_a = Square { x: 1, y: 0, piece: None };
        let square_b = Square { x: 1, y: 1, piece: None };
        let squares = vec![square_a, square_b];
        let square_set = SquareSet { squares };   

        let result = square_set.find_by_x_and_y(1, 1);
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
        let destination = Square { x: 1, y: 3, piece: None };
        let beyond = Square { x: 1, y: 4, piece: None };
        let squares = vec![origin, between, destination, beyond];
        let square_set = SquareSet { squares };   

        let result = square_set.between_unoccupied(&origin, &destination);
        assert_eq!(result, false);
    }
}
