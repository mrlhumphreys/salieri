use crate::chess::state::square::Square;
use crate::chess::state::square::PieceKind;

pub fn parse(encoded: char, x: i8, y: i8) -> Result<Square, &'static str> {
    match encoded {
        'p' => Ok(Square { x, y, kind: PieceKind::Pawn, player_number: 2 }),
        'P' => Ok(Square { x, y, kind: PieceKind::Pawn, player_number: 1 }),
        'r' => Ok(Square { x, y, kind: PieceKind::Rook, player_number: 2 }),
        'R' => Ok(Square { x, y, kind: PieceKind::Rook, player_number: 1 }),
        'n' => Ok(Square { x, y, kind: PieceKind::Knight, player_number: 2 }),
        'N' => Ok(Square { x, y, kind: PieceKind::Knight, player_number: 1 }),
        'b' => Ok(Square { x, y, kind: PieceKind::Bishop, player_number: 2 }),
        'B' => Ok(Square { x, y, kind: PieceKind::Bishop, player_number: 1 }),
        'q' => Ok(Square { x, y, kind: PieceKind::Queen, player_number: 2 }),
        'Q' => Ok(Square { x, y, kind: PieceKind::Queen, player_number: 1 }),
        'k' => Ok(Square { x, y, kind: PieceKind::King, player_number: 2 }),
        'K' => Ok(Square { x, y, kind: PieceKind::King, player_number: 1 }),
        _ => Err("unknown piece")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pawn_two_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::Pawn, player_number: 2 });
       let result = parse('p',  0, 0);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_pawn_one_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::Pawn, player_number: 1 });
       let result = parse('P', 0, 0);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_rook_two_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::Rook, player_number: 2 });
       let result = parse('r', 0, 0);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_rook_one_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::Rook, player_number: 1 });
       let result = parse('R', 0, 0);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_knight_two_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::Knight, player_number: 2 });
       let result = parse('n', 0, 0);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_knight_one_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::Knight, player_number: 1 });
       let result = parse('N', 0, 0);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_bishop_two_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::Bishop, player_number: 2 });
       let result = parse('b', 0, 0);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_bishop_one_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::Bishop, player_number: 1 });
       let result = parse('B', 0,  0);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_queen_two_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::Queen, player_number: 2 });
       let result = parse('q', 0, 0);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_queen_one_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::Queen, player_number: 1 });
       let result = parse('Q', 0, 0);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_king_two_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::King, player_number: 2 });
       let result = parse('k', 0, 0);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_king_one_test() {
       let expected = Ok(Square { x: 0, y: 0, kind: PieceKind::King, player_number: 1 });
       let result = parse('K', 0, 0);
       assert_eq!(result, expected);
    }
}
