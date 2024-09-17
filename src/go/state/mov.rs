use std::convert::TryFrom;

const FORMAT: [char; 19] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's'
];

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub x: i8,
    pub y: i8,
    pub simplified_game_state: Vec<Vec<i8>>,
    pub captures: Vec<(i8, i8)>
}

impl Move {
    pub fn format(&self) -> String {
        let x = usize::try_from(self.x).unwrap_or(0);
        let y = usize::try_from(self.y).unwrap_or(0);
        String::from(format!("{}{}", FORMAT[x], FORMAT[y]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_test() {
        let mov = Move {
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
