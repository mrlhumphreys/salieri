#[derive(Clone, Copy)]
pub enum MoveKind {
    Mov,
    Jump,
}

pub struct Move {
    pub kind: MoveKind, 
    pub from: i8,
    pub to: Vec<i8>,
}

impl Move {
    pub fn format(&self) -> String {
        let separator = match self.kind {
            MoveKind::Mov => "-",
            MoveKind::Jump => "x",
        };

        let to_string = self.to.iter().map(|i| { i.to_string() }).collect::<Vec<String>>().join(separator); 
        String::from(format!("{}{}{}", self.from, separator, to_string))
    }

    pub fn legs(&self) -> Vec<(i8, i8)> {
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
        let mov = Move { kind: MoveKind::Mov, from: 1, to: vec![3, 5, 7] };    
        let legs = mov.legs();
        assert_eq!(legs, vec![(1,3), (3,5), (5, 7)]); 
    }

    #[test]
    fn format_move() {
        let mov = Move { kind: MoveKind::Mov, from: 1, to: vec![3] };    
        let result = mov.format();
        assert_eq!(result, "1-3");
    }

    #[test]
    fn format_jump() {
        let mov = Move { kind: MoveKind::Jump, from: 1, to: vec![3, 5, 7] };    
        let result = mov.format();
        assert_eq!(result, "1x3x5x7");
    }
}
