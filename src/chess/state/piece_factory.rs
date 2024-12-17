use crate::chess::state::square::Square;
use crate::chess::state::square::PieceKind;

pub fn parse(encoded: char) -> Result<Square, &'static str> {
    match encoded {
        'p' => Ok(Square { kind: PieceKind::Pawn, player_number: 2 }),
        'P' => Ok(Square { kind: PieceKind::Pawn, player_number: 1 }),
        'r' => Ok(Square { kind: PieceKind::Rook, player_number: 2 }),
        'R' => Ok(Square { kind: PieceKind::Rook, player_number: 1 }),
        'n' => Ok(Square { kind: PieceKind::Knight, player_number: 2 }),
        'N' => Ok(Square { kind: PieceKind::Knight, player_number: 1 }),
        'b' => Ok(Square { kind: PieceKind::Bishop, player_number: 2 }),
        'B' => Ok(Square { kind: PieceKind::Bishop, player_number: 1 }),
        'q' => Ok(Square { kind: PieceKind::Queen, player_number: 2 }),
        'Q' => Ok(Square { kind: PieceKind::Queen, player_number: 1 }),
        'k' => Ok(Square { kind: PieceKind::King, player_number: 2 }),
        'K' => Ok(Square { kind: PieceKind::King, player_number: 1 }),
        _ => Err("unknown piece")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pawn_two_test() {
       let expected = Ok(Square { kind: PieceKind::Pawn, player_number: 2 });
       let result = parse('p');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_pawn_one_test() {
       let expected = Ok(Square { kind: PieceKind::Pawn, player_number: 1 });
       let result = parse('P');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_rook_two_test() {
       let expected = Ok(Square { kind: PieceKind::Rook, player_number: 2 });
       let result = parse('r');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_rook_one_test() {
       let expected = Ok(Square { kind: PieceKind::Rook, player_number: 1 });
       let result = parse('R');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_knight_two_test() {
       let expected = Ok(Square { kind: PieceKind::Knight, player_number: 2 });
       let result = parse('n');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_knight_one_test() {
       let expected = Ok(Square { kind: PieceKind::Knight, player_number: 1 });
       let result = parse('N');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_bishop_two_test() {
       let expected = Ok(Square { kind: PieceKind::Bishop, player_number: 2 });
       let result = parse('b');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_bishop_one_test() {
       let expected = Ok(Square { kind: PieceKind::Bishop, player_number: 1 });
       let result = parse('B');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_queen_two_test() {
       let expected = Ok(Square { kind: PieceKind::Queen, player_number: 2 });
       let result = parse('q');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_queen_one_test() {
       let expected = Ok(Square { kind: PieceKind::Queen, player_number: 1 });
       let result = parse('Q');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_king_two_test() {
       let expected = Ok(Square { kind: PieceKind::King, player_number: 2 });
       let result = parse('k');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_king_one_test() {
       let expected = Ok(Square { kind: PieceKind::King, player_number: 1 });
       let result = parse('K');
       assert_eq!(result, expected);
    }
}
