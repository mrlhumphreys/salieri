use crate::xiangqi::state::square::PieceKind;

#[derive(Clone)]
pub struct Move {
    pub from: (i8, i8),
    pub to: (i8, i8),
    pub moving_piece_kind: PieceKind,
    pub capture_piece_kind: Option<PieceKind> // Undo -> place piece back
}
