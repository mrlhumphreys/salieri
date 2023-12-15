use crate::chess::state::piece::Piece;
use crate::chess::state::piece::PieceKind;

pub fn parse(encoded: char) -> Result<Piece, &'static str> {
    match encoded {
        'p' => Ok(Piece { kind: PieceKind::Pawn, player_number: 2 }),
        'P' => Ok(Piece { kind: PieceKind::Pawn, player_number: 1 }),
        'r' => Ok(Piece { kind: PieceKind::Rook, player_number: 2 }),
        'R' => Ok(Piece { kind: PieceKind::Rook, player_number: 1 }),
        'n' => Ok(Piece { kind: PieceKind::Knight, player_number: 2 }),
        'N' => Ok(Piece { kind: PieceKind::Knight, player_number: 1 }),
        'b' => Ok(Piece { kind: PieceKind::Bishop, player_number: 2 }),
        'B' => Ok(Piece { kind: PieceKind::Bishop, player_number: 1 }),
        'q' => Ok(Piece { kind: PieceKind::Queen, player_number: 2 }),
        'Q' => Ok(Piece { kind: PieceKind::Queen, player_number: 1 }),
        'k' => Ok(Piece { kind: PieceKind::King, player_number: 2 }),
        'K' => Ok(Piece { kind: PieceKind::King, player_number: 1 }),
        _ => Err("unknown piece")
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pawn_two_test() {
       let expected = Ok(Piece { kind: PieceKind::Pawn, player_number: 2 });
       let result = parse('p');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_pawn_one_test() {
       let expected = Ok(Piece { kind: PieceKind::Pawn, player_number: 1 });
       let result = parse('P');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_rook_two_test() {
       let expected = Ok(Piece { kind: PieceKind::Rook, player_number: 2 });
       let result = parse('r');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_rook_one_test() {
       let expected = Ok(Piece { kind: PieceKind::Rook, player_number: 1 });
       let result = parse('R');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_knight_two_test() {
       let expected = Ok(Piece { kind: PieceKind::Knight, player_number: 2 });
       let result = parse('n');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_knight_one_test() {
       let expected = Ok(Piece { kind: PieceKind::Knight, player_number: 1 });
       let result = parse('N');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_bishop_two_test() {
       let expected = Ok(Piece { kind: PieceKind::Bishop, player_number: 2 });
       let result = parse('b');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_bishop_one_test() {
       let expected = Ok(Piece { kind: PieceKind::Bishop, player_number: 1 });
       let result = parse('B');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_queen_two_test() {
       let expected = Ok(Piece { kind: PieceKind::Queen, player_number: 2 });
       let result = parse('q');
       assert_eq!(result, expected);
    }
    
    #[test]
    fn parse_queen_one_test() {
       let expected = Ok(Piece { kind: PieceKind::Queen, player_number: 1 });
       let result = parse('Q');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_king_two_test() {
       let expected = Ok(Piece { kind: PieceKind::King, player_number: 2 });
       let result = parse('k');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_king_one_test() {
       let expected = Ok(Piece { kind: PieceKind::King, player_number: 1 });
       let result = parse('K');
       assert_eq!(result, expected);
    }
}
