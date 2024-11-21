const FORMAT: [char; 20] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't'
];

#[derive(Clone, Debug, PartialEq)]
pub enum MoveKind {
    Place,
    Pass
}

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub kind: MoveKind,
    pub x: usize,
    pub y: usize,
    pub simplified_game_state: Vec<Vec<i8>>,
    pub captures: Vec<(usize, usize)>
}

impl Move {
    pub fn format(&self) -> String {
        String::from(format!("{}{}", FORMAT[self.x], FORMAT[self.y]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_test() {
        let mov = Move {
            kind: MoveKind::Place,
            x: 3,
            y: 4,
            simplified_game_state: vec![],
            captures: vec![]
        };
        let expected = "de";
        let result = mov.format();
        assert_eq!(result, expected);
    }
}
