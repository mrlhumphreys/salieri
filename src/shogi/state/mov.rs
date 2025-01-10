use crate::shogi::state::square::PieceKind;

#[derive(Clone)]
pub struct Move {
    pub from: Option<(i8, i8)>, // when None it is a drop move
    pub to: (i8, i8),
    pub moving_piece_kind: PieceKind,
    pub capture_piece_kind: Option<PieceKind>, // Undo -> place piece back
    pub promote: bool
}
