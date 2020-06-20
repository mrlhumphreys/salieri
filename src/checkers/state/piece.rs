#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub player_number: i8,
    pub king: bool,
}

impl Piece {
    pub fn owned_by_player(&self, player_number: i8) -> bool {
        self.player_number == player_number
    }

    pub fn owned_by_opponent(&self, player_number: i8) -> bool {
        self.player_number != player_number
    }

    pub fn direction(&self) -> i8 {
        match self.king {
            true => 0,
            false => match self.player_number {
                1 => 1,
                2 => -1,
                _ => 0
            }
        }
    }
}

pub fn parse(encoded: char) -> Result<Piece, &'static str> {
    let (player_number, king) = match encoded {
        'w' => (2, false),
        'W' => (2, true),
        'b' => (1, false),
        'B' => (1, true),
        _ => return Err("Invalid Piece"),
    };
    let piece = Piece { player_number, king };
    Ok(piece)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_piece_white() {
        let encoded = 'w';
        let result = parse(encoded).unwrap();
        assert_eq!(result.player_number, 2);
        assert_eq!(result.king, false);
    }

    #[test]
    fn parsing_piece_white_king() {
        let encoded = 'W';
        let result = parse(encoded).unwrap();
        assert_eq!(result.player_number, 2);
        assert_eq!(result.king, true);
    }

    #[test]
    fn parsing_piece_black() {
        let encoded = 'b';
        let result = parse(encoded).unwrap();
        assert_eq!(result.player_number, 1);
        assert_eq!(result.king, false);
    }

    #[test]
    fn parsing_piece_black_king() {
        let encoded = 'B';
        let result = parse(encoded).unwrap();
        assert_eq!(result.player_number, 1);
        assert_eq!(result.king, true);
    }

    #[test]
    fn parsing_piece_invalid() {
        let encoded = 'a';
        let result = parse(encoded);
        match result {
            Ok(_) => assert!(false, "Expected Error"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn piece_occupied_by_player() {
        let piece = Piece { player_number: 1, king: false };    
        assert_eq!(piece.owned_by_player(1), true);
        assert_eq!(piece.owned_by_opponent(1), false);
    }

    #[test]
    fn piece_not_occupied_by_player() {
        let piece = Piece { player_number: 1, king: false };    
        assert_eq!(piece.owned_by_player(2), false);
        assert_eq!(piece.owned_by_opponent(2), true);
    }
}
