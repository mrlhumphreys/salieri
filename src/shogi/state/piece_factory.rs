use crate::shogi::state::square::Square;
use crate::shogi::state::square::PieceKind;

pub fn parse(encoded: char, promoted_piece: bool) -> Result<Square, &'static str> {
    if promoted_piece {
        match encoded {
            'p' => Ok(Square { kind: PieceKind::Tokin, player_number: 2 }),
            'P' => Ok(Square { kind: PieceKind::Tokin, player_number: 1 }),
            'l' => Ok(Square { kind: PieceKind::Narikyou, player_number: 2 }),
            'L' => Ok(Square { kind: PieceKind::Narikyou, player_number: 1 }),
            'n' => Ok(Square { kind: PieceKind::Narikei, player_number: 2 }),
            'N' => Ok(Square { kind: PieceKind::Narikei, player_number: 1 }),
            's' => Ok(Square { kind: PieceKind::Narigin, player_number: 2 }),
            'S' => Ok(Square { kind: PieceKind::Narigin, player_number: 1 }),
            'g' => Err("unpromotable piece"),
            'G' => Err("unpromotable piece"),
            'r' => Ok(Square { kind: PieceKind::Ryuuou, player_number: 2 }),
            'R' => Ok(Square { kind: PieceKind::Ryuuou, player_number: 1 }),
            'b' => Ok(Square { kind: PieceKind::Ryuuma, player_number: 2 }),
            'B' => Ok(Square { kind: PieceKind::Ryuuma, player_number: 1 }),
            'k' => Err("unpromotable piece"),
            'K' => Err("unpromotable piece"),
            _ => Err("unknown piece")
        }
    } else {
        match encoded {
            'p' => Ok(Square { kind: PieceKind::Fuhyou, player_number: 2 }),
            'P' => Ok(Square { kind: PieceKind::Fuhyou, player_number: 1 }),
            'l' => Ok(Square { kind: PieceKind::Kyousha, player_number: 2 }),
            'L' => Ok(Square { kind: PieceKind::Kyousha, player_number: 1 }),
            'n' => Ok(Square { kind: PieceKind::Keima, player_number: 2 }),
            'N' => Ok(Square { kind: PieceKind::Keima, player_number: 1 }),
            's' => Ok(Square { kind: PieceKind::Ginshou, player_number: 2 }),
            'S' => Ok(Square { kind: PieceKind::Ginshou, player_number: 1 }),
            'g' => Ok(Square { kind: PieceKind::Kinshou, player_number: 2 }),
            'G' => Ok(Square { kind: PieceKind::Kinshou, player_number: 1 }),
            'r' => Ok(Square { kind: PieceKind::Hisha, player_number: 2 }),
            'R' => Ok(Square { kind: PieceKind::Hisha, player_number: 1 }),
            'b' => Ok(Square { kind: PieceKind::Kakugyou, player_number: 2 }),
            'B' => Ok(Square { kind: PieceKind::Kakugyou, player_number: 1 }),
            'k' => Ok(Square { kind: PieceKind::Gyokushou, player_number: 2 }),
            'K' => Ok(Square { kind: PieceKind::Oushou, player_number: 1 }),
            _ => Err("unknown piece")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fuhyou_one_test() {
       let expected = Ok(Square { kind: PieceKind::Fuhyou, player_number: 1 });
       let result = parse('P', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_fuhyou_two_test() {
       let expected = Ok(Square { kind: PieceKind::Fuhyou, player_number: 2 });
       let result = parse('p', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_tokin_one_test() {
       let expected = Ok(Square { kind: PieceKind::Tokin, player_number: 1 });
       let result = parse('P', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_tokin_two_test() {
       let expected = Ok(Square { kind: PieceKind::Tokin, player_number: 2 });
       let result = parse('p', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_kyousha_one_test() {
       let expected = Ok(Square { kind: PieceKind::Kyousha, player_number: 1 });
       let result = parse('L', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_kyousha_two_test() {
       let expected = Ok(Square { kind: PieceKind::Kyousha, player_number: 2 });
       let result = parse('l', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_narikyou_one_test() {
       let expected = Ok(Square { kind: PieceKind::Narikyou, player_number: 1 });
       let result = parse('L', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_narikyou_two_test() {
       let expected = Ok(Square { kind: PieceKind::Narikyou, player_number: 2 });
       let result = parse('l', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_keima_one_test() {
       let expected = Ok(Square { kind: PieceKind::Keima, player_number: 1 });
       let result = parse('N', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_keima_two_test() {
       let expected = Ok(Square { kind: PieceKind::Keima, player_number: 2 });
       let result = parse('n', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_narikei_one_test() {
       let expected = Ok(Square { kind: PieceKind::Narikei, player_number: 1 });
       let result = parse('N', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_narikei_two_test() {
       let expected = Ok(Square { kind: PieceKind::Narikei, player_number: 2 });
       let result = parse('n', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_ginshou_one_test() {
       let expected = Ok(Square { kind: PieceKind::Ginshou, player_number: 1 });
       let result = parse('S', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_ginshou_two_test() {
       let expected = Ok(Square { kind: PieceKind::Ginshou, player_number: 2 });
       let result = parse('s', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_narigin_one_test() {
       let expected = Ok(Square { kind: PieceKind::Narigin, player_number: 1 });
       let result = parse('S', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_narigin_two_test() {
       let expected = Ok(Square { kind: PieceKind::Narigin, player_number: 2 });
       let result = parse('s', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_kinshou_one_test() {
       let expected = Ok(Square { kind: PieceKind::Kinshou, player_number: 1 });
       let result = parse('G', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_kinshou_two_test() {
       let expected = Ok(Square { kind: PieceKind::Kinshou, player_number: 2 });
       let result = parse('g', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_kinshou_promoted_test() {
       let expected = Err("unpromotable piece");
       let result = parse('G', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_kakugyou_one_test() {
       let expected = Ok(Square { kind: PieceKind::Kakugyou, player_number: 1 });
       let result = parse('B', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_kakugyou_two_test() {
       let expected = Ok(Square { kind: PieceKind::Kakugyou, player_number: 2 });
       let result = parse('b', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_ryuuma_one_test() {
       let expected = Ok(Square { kind: PieceKind::Ryuuma, player_number: 1 });
       let result = parse('B', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_ryuuma_two_test() {
       let expected = Ok(Square { kind: PieceKind::Ryuuma, player_number: 2 });
       let result = parse('b', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_hisha_one_test() {
       let expected = Ok(Square { kind: PieceKind::Hisha, player_number: 1 });
       let result = parse('R', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_hisha_two_test() {
       let expected = Ok(Square { kind: PieceKind::Hisha, player_number: 2 });
       let result = parse('r', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_ryuuou_one_test() {
       let expected = Ok(Square { kind: PieceKind::Ryuuou, player_number: 1 });
       let result = parse('R', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_ryuuou_two_test() {
       let expected = Ok(Square { kind: PieceKind::Ryuuou, player_number: 2 });
       let result = parse('r', true);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_oushou_one_test() {
       let expected = Ok(Square { kind: PieceKind::Oushou, player_number: 1 });
       let result = parse('K', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_gyokushou_two_test() {
       let expected = Ok(Square { kind: PieceKind::Gyokushou, player_number: 2 });
       let result = parse('k', false);
       assert_eq!(result, expected);
    }

    #[test]
    fn parse_oushou_promoted_test() {
       let expected = Err("unpromotable piece");
       let result = parse('K', true);
       assert_eq!(result, expected);
    }
}

