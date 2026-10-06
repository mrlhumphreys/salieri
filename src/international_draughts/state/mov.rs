use crate::international_draughts::state::point::point_to_id;

#[derive(Clone, Copy)]
pub enum MoveKind {
    Mov,
    Jump,
}

pub struct Move {
    pub kind: MoveKind,
    pub from: (i8, i8),
    pub captures: Vec<(i8, i8)>,
    pub to: Vec<(i8, i8)>
}

impl Move {
    pub fn format(&self) -> String {
        let separator = match self.kind {
            MoveKind::Mov => "-",
            MoveKind::Jump => "x",
        };

        let from_id = point_to_id(self.from);
        let to_ids = self.to.iter().map(|p| point_to_id(*p).to_string()).collect::<Vec<String>>().join(separator);

        String::from(format!("{}{}{}", from_id, separator, to_ids))
    }

    pub fn legs(&self) -> Vec<((i8, i8), Option<(i8, i8)>, (i8, i8))> {
        let mut points = vec![self.from];
        let captures = self.captures.clone();
        let mut tos = self.to.clone();
        points.append(&mut tos);
        let size = points.len();

        (0..(size-1)).map(|n| {
            if captures.is_empty() {
                (points[n], None, points[n+1])
            } else {
                (points[n], Some(captures[n]), points[n+1])
            }
        }).collect()
    }
}

impl Clone for Move {
    fn clone(&self) -> Move {
        Move {
            kind: self.kind,
            from: self.from,
            captures: self.captures.clone(),
            to: self.to.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legs() {
        let mov = Move {
            kind: MoveKind::Mov,
            from: (6, 7),
            captures: vec![(5, 6), (5, 4), (5, 2)],
            to: vec![(4, 5), (6, 3), (4, 1)]
        };
        let legs = mov.legs();
        assert_eq!(legs, vec![
            ((6, 7), Some((5, 6)), (4, 5)),
            ((4, 5), Some((5, 4)), (6, 3)),
            ((6, 3), Some((5, 2)), (4, 1))
        ]);
    }

    #[test]
    fn format_move() {
        let mov = Move {
            kind: MoveKind::Mov,
            from: (6, 7),
            captures: vec![],
            to: vec![(5, 6)]
        };
        let result = mov.format();
        assert_eq!(result, "12-18");
    }

    #[test]
    fn format_jump() {
        let mov = Move {
            kind: MoveKind::Jump,
            from: (6, 7),
            captures: vec![(5, 6), (5, 4), (5, 2)],
            to: vec![(4, 5), (6, 3), (4, 1)]
        };
        let result = mov.format();
        assert_eq!(result, "12x23x32x43");
    }
}
