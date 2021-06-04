pub struct Piece {
    pub player_number: i8
}

impl Clone for Piece {
    fn clone(&self) -> Piece {
        Piece {
            player_number: self.player_number
        }
    }
}
