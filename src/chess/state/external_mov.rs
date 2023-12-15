use std::convert::TryFrom;
use crate::chess::state::point::Point;
use crate::chess::state::piece::PieceKind;
use crate::chess::state::castle_move::CastleMove;
use crate::chess::state::castle_move::Side;

const X_FORMAT: [char; 8] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'
];

const Y_FORMAT: [char; 8] = [
    '8', '7', '6', '5', '4', '3', '2', '1'
];

pub struct ExternalMove {
    pub from: Point, 
    pub to: Point,
    pub moving_piece_kind: PieceKind,
    pub capture_piece_kind: Option<PieceKind>, // Undo -> place piece back
    pub promote_piece_kind: Option<PieceKind>, // Undo -> revert promotion
    pub en_passant_point: Option<Point>, // Undo - add capture piece back next to from 
    pub en_passant_target: Option<Point>, // Undo - set game state en_passant_target back 
    pub castle_move: Option<CastleMove>, // Undo - Move king and rook back to start.
    pub file_disambiguation: bool,
    pub rank_disambiguation: bool,
    pub in_check: bool,
    pub in_checkmate: bool
}

impl ExternalMove {
    // Reference: 
    //   No letter for Pawn e.g. c5
    //   x for capture e.g. Bxe5
    //   en passant e.g. exd6 e.p.
    //   disambiguation priority:
    //      file of departure e.g. Rdf8
    //      rank of departure e.g. R1a3
    //      file and rank of departure. e.g. Qh4e1
    //   promotion e.g. e8Q
    //   castle e.g 0-0 kingside, 0-0-0 queenside
    //   check e.g. + at end
    //   checkmate e.g. # at end
    pub fn format(&self) -> String {
        match self.castle_move {
            Some(cm) => {
                match cm.side {
                    Side::King => String::from("0-0"),
                    Side::Queen => String::from("0-0-0")
                }
            },
            None => {
                String::from(format!("{}{}{}{}{}{}{}", self.prefix(), self.piece_format(), self.from_format(), self.capture_format(), self.to_format(), self.en_passant_suffix(), self.check_and_mate_suffix()))
            }
        }
    }

    fn prefix(&self) -> String {
        match self.en_passant_point {
            Some(_) => {
                let x = usize::try_from(self.from.x).unwrap_or(0); 
                String::from(X_FORMAT[x])
            },
            None => String::from("")
        }
    }

    fn piece_format(&self) -> String {
        let piece_letter = match self.moving_piece_kind {
             PieceKind::King => "K",
             PieceKind::Queen => "Q",
             PieceKind::Rook => "R",
             PieceKind::Knight => "N",
             PieceKind::Bishop => "B",
             PieceKind::Pawn => "" 
        };

        String::from(format!("{}", piece_letter))
    }

    fn capture_format(&self) -> String {
        match self.capture_piece_kind {
            Some(_) => String::from("x"),
            None => String::from("")
        }
    }

    fn from_format(&self) -> String {
        if self.file_disambiguation && self.rank_disambiguation {
            let x = usize::try_from(self.from.x).unwrap_or(0); 
            let y = usize::try_from(self.from.y).unwrap_or(0); 
            String::from(format!("{}{}", X_FORMAT[x], Y_FORMAT[y]))
        } else if self.file_disambiguation {
            let x = usize::try_from(self.from.x).unwrap_or(0); 
            String::from(format!("{}", X_FORMAT[x]))
        } else if self.rank_disambiguation {
            let y = usize::try_from(self.from.y).unwrap_or(0); 
            String::from(format!("{}", Y_FORMAT[y]))
        } else {
            String::from("")
        }
    }

    fn to_format(&self) -> String {
        let x = usize::try_from(self.to.x).unwrap_or(0); 
        let y = usize::try_from(self.to.y).unwrap_or(0); 
        String::from(format!("{}{}", X_FORMAT[x], Y_FORMAT[y]))
    }

    fn en_passant_suffix(&self) -> String {
        match self.en_passant_point {
            Some(_) => String::from(" e.p."),
            None => String::from("")
        }
    }

    fn check_and_mate_suffix(&self) -> String {
        if self.in_check {
            String::from("+")
        } else if self.in_checkmate {
            String::from("#")
        } else {
            String::from("")
        }
    }
}

impl Clone for ExternalMove {
    fn clone(&self) -> ExternalMove {
        ExternalMove {
            from: self.from, 
            to: self.to,
            moving_piece_kind: self.moving_piece_kind,
            capture_piece_kind: self.capture_piece_kind, 
            promote_piece_kind: self.promote_piece_kind, 
            en_passant_point: self.en_passant_point,  
            en_passant_target: self.en_passant_target,  
            castle_move: self.castle_move,
            file_disambiguation: self.file_disambiguation,
            rank_disambiguation: self.rank_disambiguation,
            in_check: self.in_check,
            in_checkmate: self.in_checkmate 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_piece_test() {
        let from = Point { x: 3, y: 2 };
        let to = Point { x: 4, y: 3 };
        let moving_piece_kind = PieceKind::Bishop;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None,
            file_disambiguation: false,
            rank_disambiguation: false,
            in_check: false,
            in_checkmate: false
        };

        assert_eq!("Be5", mov.format());
    }

    #[test]
    fn format_pawn_test() {
        let from = Point { x: 4, y: 6 };
        let to = Point { x: 4, y: 4 };
        let moving_piece_kind = PieceKind::Pawn;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None,
            file_disambiguation: false,
            rank_disambiguation: false,
            in_check: false,
            in_checkmate: false
        };
        
        assert_eq!("e4", mov.format());
    }

    #[test]
    fn format_capture_test() {
        let from = Point { x: 3, y: 2 };
        let to = Point { x: 4, y: 3 };
        let moving_piece_kind = PieceKind::Bishop;
        let capture_piece_kind = Some(PieceKind::Pawn);
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None,
            file_disambiguation: false,
            rank_disambiguation: false,
            in_check: false,
            in_checkmate: false
        };
        
        assert_eq!("Bxe5", mov.format());
    }

    #[test]
    fn format_en_passant_test() {
        let from = Point { x: 4, y: 3 };
        let to = Point { x: 3, y: 2 };
        let moving_piece_kind = PieceKind::Pawn;
        let capture_piece_kind = Some(PieceKind::Pawn);
        let en_passant_point = Some(Point { x: 3, y: 3 });
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind,
            promote_piece_kind: None,
            en_passant_point,
            en_passant_target: Some(Point { x: 3, y: 2 }),
            castle_move: None,
            file_disambiguation: false,
            rank_disambiguation: false,
            in_check: false,
            in_checkmate: false
        };
        
        assert_eq!("exd6 e.p.", mov.format());
    }

    #[test]
    fn format_disambiguation_file_test() {
        let from = Point { x: 3, y: 0 };
        let to = Point { x: 5, y: 0 };
        let moving_piece_kind = PieceKind::Rook;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None,
            file_disambiguation: true,
            rank_disambiguation: false,
            in_check: false,
            in_checkmate: false
        };
        
        assert_eq!("Rdf8", mov.format());
    }

    #[test]
    fn format_disambiguation_rank_test() {
        let from = Point { x: 0, y: 7 };
        let to = Point { x: 0, y: 5 };
        let moving_piece_kind = PieceKind::Rook;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None,
            file_disambiguation: false,
            rank_disambiguation: true,
            in_check: false,
            in_checkmate: false
        };
        
        assert_eq!("R1a3", mov.format());
    }

    #[test]
    fn format_disambiguation_file_and_rank_test() {
        let from = Point { x: 7, y: 4 };
        let to = Point { x: 4, y: 7 };
        let moving_piece_kind = PieceKind::Queen;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None,
            file_disambiguation: true,
            rank_disambiguation: true,
            in_check: false,
            in_checkmate: false
        };
        
        assert_eq!("Qh4e1", mov.format());
    }

    #[test]
    fn format_kingside_castle_test() {
        let from = Point { x: 4, y: 3 };
        let to = Point { x: 3, y: 2 };
        let moving_piece_kind = PieceKind::King;
        let castle_move = Some(CastleMove { player_number: 1, side: Side::King }); 
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move,
            file_disambiguation: false,
            rank_disambiguation: false,
            in_check: false,
            in_checkmate: false
        };
        
        assert_eq!("0-0", mov.format());
    }

    #[test]
    fn format_queenside_castle_test() {
        let from = Point { x: 4, y: 3 };
        let to = Point { x: 3, y: 2 };
        let moving_piece_kind = PieceKind::King;
        let castle_move = Some(CastleMove { player_number: 1, side: Side::Queen }); 
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move,
            file_disambiguation: false,
            rank_disambiguation: false,
            in_check: false,
            in_checkmate: false
        };

        assert_eq!("0-0-0", mov.format());
    }

    #[test]
    fn format_check_test() {
        let from = Point { x: 4, y: 3 };
        let to = Point { x: 3, y: 2 };
        let moving_piece_kind = PieceKind::Rook;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None,
            file_disambiguation: false,
            rank_disambiguation: false,
            in_check: true,
            in_checkmate: false
        };

        assert_eq!("Rd6+", mov.format());
    }

    #[test]
    fn format_checkmate_test() {
        let from = Point { x: 4, y: 3 };
        let to = Point { x: 3, y: 2 };
        let moving_piece_kind = PieceKind::Rook;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote_piece_kind: None,
            en_passant_point: None,
            en_passant_target: None,
            castle_move: None,
            file_disambiguation: false,
            rank_disambiguation: false,
            in_check: false,
            in_checkmate: true 
        };

        assert_eq!("Rd6#", mov.format());
    }
} 

