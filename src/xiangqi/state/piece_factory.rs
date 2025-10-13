use crate::xiangqi::state::square::Square;
use crate::xiangqi::state::square::PieceKind;

pub fn parse(encoded: char) -> Result<Square, &'static str> {
    match encoded {
        'p' => Ok(Square { kind: PieceKind::Soldier, player_number: 2 }),
        'P' => Ok(Square { kind: PieceKind::Soldier, player_number: 1 }),
        'r' => Ok(Square { kind: PieceKind::Chariot, player_number: 2 }),
        'R' => Ok(Square { kind: PieceKind::Chariot, player_number: 1 }),
        'h' => Ok(Square { kind: PieceKind::Horse, player_number: 2 }),
        'H' => Ok(Square { kind: PieceKind::Horse, player_number: 1 }),
        'e' => Ok(Square { kind: PieceKind::Elephant, player_number: 2 }),
        'E' => Ok(Square { kind: PieceKind::Elephant, player_number: 1 }),
        'a' => Ok(Square { kind: PieceKind::Advisor, player_number: 2 }),
        'A' => Ok(Square { kind: PieceKind::Advisor, player_number: 1 }),
        'k' => Ok(Square { kind: PieceKind::King, player_number: 2 }),
        'K' => Ok(Square { kind: PieceKind::King, player_number: 1 }),
        'c' => Ok(Square { kind: PieceKind::Cannon, player_number: 2 }),
        'C' => Ok(Square { kind: PieceKind::Cannon, player_number: 1 }),
        _ => Err("unknown piece")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_soldier_one_test() {
       let expected = Ok(Square { kind: PieceKind::Soldier, player_number: 1 });
       let result = parse('P');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_soldier_two_test() {
       let expected = Ok(Square { kind: PieceKind::Soldier, player_number: 2 });
       let result = parse('p');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_chariot_one_test() {
       let expected = Ok(Square { kind: PieceKind::Chariot, player_number: 1 });
       let result = parse('R');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_chariot_two_test() {
       let expected = Ok(Square { kind: PieceKind::Chariot, player_number: 2 });
       let result = parse('r');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_horse_one_test() {
       let expected = Ok(Square { kind: PieceKind::Horse, player_number: 1 });
       let result = parse('H');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_horse_two_test() {
       let expected = Ok(Square { kind: PieceKind::Horse, player_number: 2 });
       let result = parse('h');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_elephant_one_test() {
       let expected = Ok(Square { kind: PieceKind::Elephant, player_number: 1 });
       let result = parse('E');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_elephant_two_test() {
       let expected = Ok(Square { kind: PieceKind::Elephant, player_number: 2 });
       let result = parse('e');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_advisor_one_test() {
       let expected = Ok(Square { kind: PieceKind::Advisor, player_number: 1 });
       let result = parse('A');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_advisor_two_test() {
       let expected = Ok(Square { kind: PieceKind::Advisor, player_number: 2 });
       let result = parse('a');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_king_one_test() {
       let expected = Ok(Square { kind: PieceKind::King, player_number: 1 });
       let result = parse('K');
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_king_two_test() {
       let expected = Ok(Square { kind: PieceKind::King, player_number: 2 });
       let result = parse('k');
       assert_eq!(result, expected);
    }
}

