use crate::checkers::state::point::Point;
use crate::checkers::state::vector::Vector;
use crate::checkers::state::vector::Direction;
use crate::checkers::state::square::Square;

pub fn find_by_x_and_y(squares: &Vec<Vec<Square>>, x: usize, y: usize) -> Square {
    squares[y][x]
}

pub fn between<'a>(squares: &'a Vec<Vec<Square>>, from: Point, to: Point) -> Option<Square> {
    let vector = Vector { from, to };

    if vector.direction() == Direction::Other {
       return None;
    } else {
        let direction_unit = vector.direction_unit();
        let between_x = from.x as i8 + direction_unit.x;
        let between_y = from.y as i8 + direction_unit.y;
        if between_x != to.x && between_y != to.y {
            let square = find_by_x_and_y(squares, between_x as usize, between_y as usize);
            return Some(square);
        } else {
            return None;
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetching_by_x_and_y() {
        let squares = vec![
            vec![
                Square { id: 1, x: 0, y: 0, player_number: 0, king: false },
                Square { id: 2, x: 1, y: 0, player_number: 0, king: false }
            ],
            vec![
                Square { id: 3, x: 0, y: 1, player_number: 0, king: false },
                Square { id: 4, x: 1, y: 1, player_number: 0, king: false }
            ]
        ];
        let result = find_by_x_and_y(&squares, 1, 1);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn fetching_between_diagonal() {
        let from_point = Point { x: 0, y: 0 };
        let to_point = Point { x: 2, y: 2 };
        let squares = vec![
            vec![
                Square { id: 1, x: 0, y: 0, player_number: 0, king: false },
                Square { id: 2, x: 1, y: 0, player_number: 0, king: false },
                Square { id: 3, x: 2, y: 0, player_number: 0, king: false }
            ],
            vec![
                Square { id: 4, x: 0, y: 1, player_number: 0, king: false },
                Square { id: 5, x: 1, y: 1, player_number: 1, king: false },
                Square { id: 6, x: 2, y: 1, player_number: 1, king: false }
            ],
            vec![
                Square { id: 7, x: 0, y: 2, player_number: 0, king: false },
                Square { id: 8, x: 1, y: 2, player_number: 0, king: false },
                Square { id: 9, x: 2, y: 2, player_number: 0, king: false }
            ]
        ];
        let result = between(&squares, from_point, to_point).unwrap();
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }

    #[test]
    fn fetching_between_l_shape() {
        let from_point = Point { x: 0, y: 0 };
        let to_point = Point { x: 2, y: 1 };
        let squares = vec![
            vec![
                Square { id: 1, x: 0, y: 0, player_number: 0, king: false },
                Square { id: 2, x: 1, y: 0, player_number: 0, king: false },
                Square { id: 3, x: 2, y: 0, player_number: 0, king: false }
            ],
            vec![
                Square { id: 4, x: 0, y: 1, player_number: 0, king: false },
                Square { id: 5, x: 1, y: 1, player_number: 1, king: false },
                Square { id: 6, x: 2, y: 1, player_number: 1, king: false }
            ],
            vec![
                Square { id: 7, x: 0, y: 2, player_number: 0, king: false },
                Square { id: 8, x: 1, y: 2, player_number: 0, king: false },
                Square { id: 9, x: 2, y: 2, player_number: 0, king: false }
            ]
        ];
        let result = between(&squares, from_point, to_point);
        assert_eq!(result, None);
    }
}
