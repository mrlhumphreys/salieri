use std::convert::TryFrom;
use crate::xiangqi::state::square::PieceKind;

const X_FORMAT: [char; 9] = [
    '9', '8', '7', '6', '5', '4', '3', '2', '1'
];

pub struct ExternalMove {
    pub from: (i8, i8),
    pub to: (i8, i8),
    pub moving_piece_kind: PieceKind,
    pub player_number: i8,
    pub capture_piece_kind: Option<PieceKind>, // Undo -> place piece back
    pub rank_disambiguation: Option<char>,
    pub pawn_disambiguation: Option<usize>
}

impl ExternalMove {
    // Reference:
    // Piece
    // Disambiguation + forward, - rear,
    //   pawn triple: 1 forward , 2 middle, 3 rear
    // Starting File
    // Direction + - =
    // Number of squares moved vertical || end file horizontally
    pub fn format(&self) -> String {
        String::from(format!("{}{}{}{}", self.piece_format(), self.starting_file_format(), self.direction_format(), self.movement_format()))
    }

    fn piece_format(&self) -> String {
        if let Some(pd) = self.pawn_disambiguation {
            String::from(format!("{}", pd))
        } else {
            let piece_letter = match self.moving_piece_kind {
                 PieceKind::Soldier => "P",
                 PieceKind::Chariot => "R",
                 PieceKind::Horse => "H",
                 PieceKind::Elephant => "E",
                 PieceKind::Advisor => "A",
                 PieceKind::King => "K",
                 PieceKind::Cannon => "C",
                 PieceKind::Empty => ""
            };
            String::from(format!("{}", piece_letter))
        }
    }

    fn starting_file_format(&self) -> String {
        if let Some(fd) = self.rank_disambiguation {
            format!("{}", fd)
        } else {
            let x = usize::try_from(self.from.0).unwrap_or(0);
            format!("{}", X_FORMAT[x])
        }
    }

    fn direction_format(&self) -> String {
       if self.player_number == 1 {
           if self.from.1 > self.to.1 {
             String::from("+")
           } else if self.from.1 < self.to.1 {
             String::from("-")
           } else {
             String::from("=")
           }
       } else {
           if self.from.1 > self.to.1 {
             String::from("-")
           } else if self.from.1 < self.to.1 {
             String::from("+")
           } else {
             String::from("=")
           }
       }
    }

    fn movement_format(&self) -> String {
        // if x is the same
        if self.from.0 == self.to.0 {
            // use the y distance
            let distance = (self.to.1 - self.from.1).abs();
            format!("{}", distance)
        } else {
            // use the to x
            let x = usize::try_from(self.to.0).unwrap_or(0);
            format!("{}", X_FORMAT[x])
        }
    }
}

impl Clone for ExternalMove {
    fn clone(&self) -> ExternalMove {
        ExternalMove {
            from: self.from,
            to: self.to,
            moving_piece_kind: self.moving_piece_kind,
            player_number: self.player_number,
            capture_piece_kind: self.capture_piece_kind,
            rank_disambiguation: self.rank_disambiguation,
            pawn_disambiguation: self.pawn_disambiguation
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_piece_test() {
        let from = (2, 9);
        let to = (0, 7);
        let moving_piece_kind = PieceKind::Elephant;
        let player_number = 1;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            player_number,
            capture_piece_kind: None,
            rank_disambiguation: None,
            pawn_disambiguation: None
        };

        assert_eq!("E7+9", mov.format());
    }

    #[test]
    fn format_soldier_test() {
        let from = (0, 6);
        let to = (0, 5);
        let moving_piece_kind = PieceKind::Soldier;
        let player_number = 1;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            player_number,
            capture_piece_kind: None,
            rank_disambiguation: None,
            pawn_disambiguation: None
        };

        assert_eq!("P9+1", mov.format());
    }

    #[test]
    fn format_disambiguation_test() {
        let from = (8, 9);
        let to = (8, 7);
        let moving_piece_kind = PieceKind::Chariot;
        let player_number = 1;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            player_number,
            capture_piece_kind: None,
            rank_disambiguation: Some('+'),
            pawn_disambiguation: None
        };

        assert_eq!("R++2", mov.format());
    }

    #[test]
    fn format_soldier_disambiguation_test() {
        let from = (0, 6);
        let to = (0, 5);
        let moving_piece_kind = PieceKind::Soldier;
        let player_number = 1;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            player_number,
            capture_piece_kind: None,
            rank_disambiguation: None,
            pawn_disambiguation: Some(1)
        };

        assert_eq!("19+1", mov.format());
    }

    #[test]
    fn format_direction_forward_test() {
        let from = (8, 9);
        let to = (8, 7);
        let moving_piece_kind = PieceKind::Chariot;
        let player_number = 1;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            player_number,
            capture_piece_kind: None,
            rank_disambiguation: None,
            pawn_disambiguation: None
        };

        assert_eq!("R1+2", mov.format());
    }

    #[test]
    fn format_direction_backward_test() {
        let from = (8, 7);
        let to = (8, 9);
        let moving_piece_kind = PieceKind::Chariot;
        let player_number = 1;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            player_number,
            capture_piece_kind: None,
            rank_disambiguation: None,
            pawn_disambiguation: None
        };

        assert_eq!("R1-2", mov.format());
    }

    #[test]
    fn format_move_horizontal_test() {
        let from = (8, 7);
        let to = (6, 7);
        let moving_piece_kind = PieceKind::Chariot;
        let player_number = 1;
        let mov = ExternalMove {
            from,
            to,
            moving_piece_kind,
            player_number,
            capture_piece_kind: None,
            rank_disambiguation: None,
            pawn_disambiguation: None
        };

        assert_eq!("R1=3", mov.format());
    }
}

