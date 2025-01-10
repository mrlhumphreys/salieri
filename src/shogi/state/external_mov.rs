use std::convert::TryFrom;
use crate::shogi::state::square::PieceKind;

const X_FORMAT: [char; 9] = [
    '9', '8', '7', '6', '5', '4', '3', '2', '1'
];

const Y_FORMAT: [char; 9] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

pub struct ExternalMove {
    pub from: Option<(i8, i8)>,
    pub to: (i8, i8),
    pub moving_piece_kind: PieceKind,
    pub capture_piece_kind: Option<PieceKind>, // Undo -> place piece back
    pub promote: bool, // Undo -> revert promotion
    pub promotion_rank: bool,
    pub disambiguation: bool
}

impl ExternalMove {
    // Reference:
    //   x for capture e.g. Bxe5
    //   disambiguation priority:
    //      file of departure e.g. Rdf8
    //      rank of departure e.g. R1a3
    //      file and rank of departure. e.g. Qh4e1
    //   drop *
    //   promotion declined =
    //   promotion accepted +
    pub fn format(&self) -> String {
        String::from(format!("{}{}{}{}{}", self.piece_format(), self.from_format(), self.capture_format(), self.to_format(), self.promotion_suffix()))
    }

    fn piece_format(&self) -> String {
        let piece_letter = match self.moving_piece_kind {
             PieceKind::Oushou | PieceKind::Gyokushou => "K",
             PieceKind::Fuhyou => "P",
             PieceKind::Kyousha => "L",
             PieceKind::Keima => "N",
             PieceKind::Ginshou => "S",
             PieceKind::Kinshou => "G",
             PieceKind::Hisha => "R",
             PieceKind::Kakugyou => "B",
             PieceKind::Tokin => "+P",
             PieceKind::Narikyou => "+L",
             PieceKind::Narikei => "+N",
             PieceKind::Narigin => "+S",
             PieceKind::Ryuuou => "+R",
             PieceKind::Ryuuma => "+B",
             PieceKind::Empty => ""
        };

        String::from(format!("{}", piece_letter))
    }

    fn capture_format(&self) -> String {
        if self.capture_piece_kind.is_some() {
            String::from("x")
        } else if self.from.is_some() {
            String::from("-")
        } else {
            String::from("*")
        }
    }

    fn from_format(&self) -> String {
        if let Some(f) = self.from {
            if self.disambiguation {
                let x = usize::try_from(f.0).unwrap_or(0);
                let y = usize::try_from(f.1).unwrap_or(0);
                String::from(format!("{}{}", X_FORMAT[x], Y_FORMAT[y]))
            } else {
                String::from("")
            }
        } else {
            String::from("")
        }
    }

    fn to_format(&self) -> String {
        let x = usize::try_from(self.to.0).unwrap_or(0);
        let y = usize::try_from(self.to.1).unwrap_or(0);
        String::from(format!("{}{}", X_FORMAT[x], Y_FORMAT[y]))
    }

    fn promotion_suffix(&self) -> String {
        if self.promote {
            String::from("+")
        } else {
            if self.from.is_some() && self.promotion_rank {
                String::from("=")
            } else {
                String::from("")
            }
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
            promote: self.promote,
            promotion_rank: self.promotion_rank,
            disambiguation: self.disambiguation
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_piece_test() {
        let from = Some((3, 2));
        let to = (4, 3);
        let moving_piece_kind = PieceKind::Kakugyou;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote: false,
            promotion_rank: false,
            disambiguation: false
        };

        assert_eq!("B-54", mov.format());
    }

    #[test]
    fn format_fuhyou_test() {
        let from = Some((4, 6));
        let to = (4, 4);
        let moving_piece_kind = PieceKind::Fuhyou;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote: false,
            promotion_rank: false,
            disambiguation: false
        };

        assert_eq!("P-55", mov.format());
    }

    #[test]
    fn format_drop_test() {
        let from = None;
        let to = (4, 4);
        let moving_piece_kind = PieceKind::Fuhyou;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote: false,
            promotion_rank: false,
            disambiguation: false
        };

        assert_eq!("P*55", mov.format());
    }

    #[test]
    fn format_capture_test() {
        let from = Some((3, 2));
        let to = (4, 3);
        let moving_piece_kind = PieceKind::Kakugyou;
        let capture_piece_kind = Some(PieceKind::Fuhyou);
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind,
            promote: false,
            promotion_rank: false,
            disambiguation: false
        };

        assert_eq!("Bx54", mov.format());
    }

    #[test]
    fn format_disambiguation_file_and_rank_test() {
        let from = Some((7, 4));
        let to = (4, 7);
        let moving_piece_kind = PieceKind::Ryuuma;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote: false,
            promotion_rank: false,
            disambiguation: true
        };

        assert_eq!("+B25-58", mov.format());
    }

    #[test]
    fn promotion_accepted_test() {
        let from = Some((4, 1));
        let to = (4, 0);
        let moving_piece_kind = PieceKind::Fuhyou;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote: true,
            promotion_rank: true,
            disambiguation: false
        };

        assert_eq!("P-51+", mov.format());
    }

    #[test]
    fn promotion_declined_test() {
        let from = Some((4, 1));
        let to = (4, 0);
        let moving_piece_kind = PieceKind::Ginshou;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            capture_piece_kind: None,
            promote: false,
            promotion_rank: true,
            disambiguation: false
        };

        assert_eq!("S-51=", mov.format());
    }
}

