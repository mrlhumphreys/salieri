use crate::checkers::state::point::point_to_id;

#[derive(Clone, Copy)]
pub enum MoveKind {
    Mov,
    Jump,
}

pub struct Move {
    pub kind: MoveKind,
    pub from: (i8, i8),
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

    pub fn legs(&self) -> Vec<((i8, i8), (i8, i8))> {
        let mut points = vec![self.from];
        let mut tos = self.to.clone();
        points.append(&mut tos);
        let size = points.len();

        (0..(size-1)).map(|n| {
            (points[n], points[n+1])
        }).collect()
    }
}

impl Clone for Move {
    fn clone(&self) -> Move {
        Move {
            kind: self.kind,
            from: self.from,
            to: self.to.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legs() {
        let mov = Move { kind: MoveKind::Mov, from: (6, 7), to: vec![(2, 7), (7, 6), (3, 6)] };
        let legs = mov.legs();
        assert_eq!(legs, vec![((6, 7), (2, 7)), ((2, 7), (7, 6)), ((7, 6), (3, 6))]);
    }

    #[test]
    fn format_move() {
        let mov = Move { kind: MoveKind::Mov, from: (6, 7), to: vec![(2, 7)] };
        let result = mov.format();
        assert_eq!(result, "1-3");
    }

    #[test]
    fn format_jump() {
        let mov = Move { kind: MoveKind::Jump, from: (6, 7), to: vec![(2, 7), (7, 6), (3, 6)] };
        let result = mov.format();
        assert_eq!(result, "1x3x5x7");
    }
}
