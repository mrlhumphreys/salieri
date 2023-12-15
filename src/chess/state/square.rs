use crate::chess::state::point::Point;
use crate::chess::state::piece::Piece;
use crate::chess::state::game_state::GameState;

#[derive(Copy, Debug, PartialEq)]
pub struct Square {
    pub x: i8,
    pub y: i8,
    pub piece: Option<Piece>,
}

impl Clone for Square {
    fn clone(&self) -> Square {
        Square {
            x: self.x,
            y: self.y,
            piece: self.piece.clone()
        }
    }
}

impl Square {
    pub fn point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn occupied(&self) -> bool {
       self.piece.is_some() 
    }

    pub fn unoccupied(&self) -> bool {
       self.piece.is_none() 
    }
    
    pub fn occupied_by_player(&self, player_number: i8) -> bool {
        match &self.piece {
            Some(p) => p.player_number == player_number,
            None => false 
        }
    }

    pub fn occupied_by_opponent(&self, player_number: i8) -> bool {
        match &self.piece {
            Some(p) => p.player_number != player_number,
            None => false 
        }
    }

    pub fn unoccupied_or_occupied_by_opponent(&self, player_number: i8) -> bool {
        match &self.piece {
            Some(p) => p.player_number != player_number,
            None => true
        }
    }

    pub fn destinations<'a>(&'a self, game_state: &'a GameState) -> Vec<&Square> {
        match &self.piece {
            Some(p) => p.destinations(self, game_state),
            None => vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::state::piece::PieceKind;
    use crate::chess::state::game_state::parse as parse_game_state;

    #[test]
    fn point_test() {
        let square = Square { x: 1, y: 2, piece: None };        
        let expected = Point { x: 1, y: 2 };
        let result = square.point();
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_some_test() {
        let square = Square { x: 1, y: 2, piece: Some(Piece { player_number: 1, kind: PieceKind::Pawn }) };        
        let expected = true;
        let result = square.occupied();
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_none_test() {
        let square = Square { x: 1, y: 2, piece: None };        
        let expected = false;
        let result = square.occupied();
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_player_one_test() {
        let square = Square { x: 1, y: 2, piece: Some(Piece { player_number: 1, kind: PieceKind::Pawn }) };        
        let expected = true;
        let result = square.occupied_by_player(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_player_two_test() {
        let square = Square { x: 1, y: 2, piece: Some(Piece { player_number: 2, kind: PieceKind::Pawn }) };        
        let expected = false;
        let result = square.occupied_by_player(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_player_none_test() {
        let square = Square { x: 1, y: 2, piece: None };        
        let expected = false;
        let result = square.occupied_by_player(1);
        assert_eq!(result, expected);
    }
    
    #[test]
    fn occupied_by_opponent_one_test() {
        let square = Square { x: 1, y: 2, piece: Some(Piece { player_number: 1, kind: PieceKind::Pawn }) };        
        let expected = false;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_two_test() {
        let square = Square { x: 1, y: 2, piece: Some(Piece { player_number: 2, kind: PieceKind::Pawn }) };        
        let expected = true;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_by_opponent_none_test() {
        let square = Square { x: 1, y: 2, piece: None };        
        let expected = false;
        let result = square.occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_or_occupied_by_opponent_one_test() {
        let square = Square { x: 1, y: 2, piece: Some(Piece { player_number: 1, kind: PieceKind::Pawn }) };        
        let expected = false;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_or_occupied_by_opponent_two_test() {
        let square = Square { x: 1, y: 2, piece: Some(Piece { player_number: 2, kind: PieceKind::Pawn }) };        
        let expected = true;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn unoccupied_or_occupied_by_opponent_none_test() {
        let square = Square { x: 1, y: 2, piece: None };        
        let expected = true;
        let result = square.unoccupied_or_occupied_by_opponent(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_some_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let game_state = parse_game_state(&encoded).unwrap();
        let square = Square { x: 4, y: 6, piece: Some(Piece { player_number: 1, kind: PieceKind::Pawn }) };        
        let expected = 2;
        let result = square.destinations(&game_state).len();
        assert_eq!(result, expected);
    }

    #[test]
    fn destinations_none_test() {
        let encoded = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let game_state = parse_game_state(&encoded).unwrap();
        let square = Square { x: 4, y: 5, piece: None };        
        let expected = 0;
        let result = square.destinations(&game_state).len();
        assert_eq!(result, expected);
    }
} 
