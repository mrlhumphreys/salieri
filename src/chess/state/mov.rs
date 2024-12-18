use crate::chess::state::square::PieceKind;
use crate::chess::state::castle_move::CastleMove;

pub struct Move {
    pub from: (i8, i8),
    pub to: (i8, i8),
    pub moving_piece_kind: PieceKind,
    pub capture_piece_kind: Option<PieceKind>, // Undo -> place piece back
    pub promote_piece_kind: Option<PieceKind>, // Undo -> revert promotion
    pub en_passant_point: Option<(i8, i8)>, // Undo - add capture piece back next to from
    pub en_passant_target: Option<(i8, i8)>, // Undo - set game state en_passant_target back
    pub castle_move: Option<CastleMove> // Undo - Move king and rook back to start.
}

impl Clone for Move {
    fn clone(&self) -> Move {
        Move {
            from: self.from,
            to: self.to,
            moving_piece_kind: self.moving_piece_kind,
            capture_piece_kind: self.capture_piece_kind,
            promote_piece_kind: self.promote_piece_kind,
            en_passant_point: self.en_passant_point,
            en_passant_target: self.en_passant_target,
            castle_move: self.castle_move
        }
    }
}
