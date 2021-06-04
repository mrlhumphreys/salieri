use crate::backgammon::state::point::Point;

#[derive(Clone,PartialEq,Debug)]
pub enum PointKind {
    Point,
    Bar,
    OffBoard
}

#[derive(Clone)]
pub struct Location {
    pub kind: PointKind,
    pub number: Option<i8>
}

impl Location {
    pub fn format(&self) -> String {
        match self.kind {
           PointKind::Point => {
                match self.number {
                    Some(n) => n.to_string(),
                    None => "-".to_string()
                }
           },
           PointKind::Bar => "bar".to_string(),
           PointKind::OffBoard => "off".to_string()
        }
    }
}

#[derive(Clone)]
pub struct MoveStep {
    pub from: Location,
    pub to: Location,
    pub die_number: i8
}

impl MoveStep {
    pub fn format(&self) -> String {
        String::from(format!("{}/{}", self.from.format(), self.to.format()))
    }
}

#[derive(Clone)]
pub struct Move {
    pub list: Vec<MoveStep>
}

impl Move {
    // 4-2: 8/4 6/4 - dice 4 and 2, point 8 to 4, 6 to 4
    // * - hit
    // bar - bar
    // off - off board
    pub fn format(&self) -> String {
        let dice = String::from(format!("{}-{}", self.list[0].die_number, self.list[1].die_number));
        let move_steps = self.list.iter().map(|ms| { ms.format() }).collect::<Vec<String>>().join(" ");
        String::from(format!("{}: {}", dice, move_steps))
    }
}

pub fn build_move_step(from_kind: PointKind, from_number: Option<i8>, to_kind: PointKind, to_number: Option<i8>, die_number: i8) -> MoveStep {
    let from = Location { kind: from_kind, number: from_number };
    let to = Location { kind: to_kind, number: to_number };
    MoveStep { from, to, die_number }
}

pub fn bar_move_step(destination_point: Option<&Point>, die_number: i8, player_number: i8) -> Option<MoveStep> {
   match destination_point {
       Some(p) => {
           if !(p.prime() && p.occupied_by_opponent(player_number)) {
               Some(build_move_step(
                   PointKind::Bar, None, 
                   PointKind::Point, Some(p.number),
                   die_number
               ))    
           } else {
               None
           }
       },
       None => None 
   }
}

pub fn off_board_move_step(origin_point: &Point, die_number: i8) -> Option<MoveStep> {
   Some(build_move_step(
       PointKind::Point, Some(origin_point.number),
       PointKind::OffBoard, None,
       die_number
   ))
}

pub fn beyond_off_board_move_step(origin_point: &Point, die_number: i8, back_point_number: Option<i8>) -> Option<MoveStep> {
    match back_point_number { 
        Some(n) => {
            if n == origin_point.number {
                off_board_move_step(origin_point, die_number)
            } else {
                None
            }
        },
        None => None 
    }
}

pub fn point_to_point_move_step(origin_point: &Point, destination_point: Option<&Point>, die_number: i8, player_number: i8) -> Option<MoveStep> {
   match destination_point {
       Some(p) => {
           if !(p.prime() && p.occupied_by_opponent(player_number)) {
               Some(build_move_step(
                   PointKind::Point, Some(origin_point.number),
                   PointKind::Point, Some(p.number),
                   die_number
               ))
           } else {
               None
           }
       },
       None => None 
   }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backgammon::state::piece::Piece;

    #[test]
    fn location_format_point_test() {
       let location = Location { kind: PointKind::Point, number: Some(1) }; 
       assert_eq!(location.format(), "1");
    }

    #[test]
    fn location_format_bar_test() {
       let location = Location { kind: PointKind::Bar, number: None }; 
       assert_eq!(location.format(), "bar");
    }

    #[test]
    fn location_format_off_board_test() {
       let location = Location { kind: PointKind::OffBoard, number: None }; 
       assert_eq!(location.format(), "off");
    }

    #[test]
    fn move_step_format_test() {
       let from = Location { kind: PointKind::Point, number: Some(1) }; 
       let to = Location { kind: PointKind::Point, number: Some(2) }; 
       let die_number = 1;
       let move_step = MoveStep { from, to, die_number };
       assert_eq!(move_step.format(), "1/2");
    }

    #[test]
    fn move_format_test() {
       let from_a = Location { kind: PointKind::Point, number: Some(1) }; 
       let to_a = Location { kind: PointKind::Point, number: Some(2) }; 
       let die_number_a = 1;
       let move_step_a = MoveStep { from: from_a, to: to_a, die_number: die_number_a };
       
       let from_b = Location { kind: PointKind::Point, number: Some(2) }; 
       let to_b = Location { kind: PointKind::Point, number: Some(4) }; 
       let die_number_b = 2;
       let move_step_b = MoveStep { from: from_b, to: to_b, die_number: die_number_b };

       let mov = Move { list: vec![move_step_a, move_step_b] };
       
       assert_eq!(mov.format(), "1-2: 1/2 2/4");
    }

    #[test]
    fn build_move_step_test() {
        let from_kind = PointKind::Point;        
        let from_number = Some(1);
        let to_kind = PointKind::Point;
        let to_number = Some(2);
        let die_number = 1;

        let move_step = build_move_step(from_kind, from_number, to_kind, to_number, die_number);

        assert_eq!(move_step.from.kind, PointKind::Point);
        assert_eq!(move_step.from.number, Some(1));
        assert_eq!(move_step.to.kind, PointKind::Point);
        assert_eq!(move_step.to.number, Some(2));
        assert_eq!(move_step.die_number, 1);
    }

    #[test]
    fn bar_move_step_valid_test() {
        let destination_point = Point { number: 2, pieces: vec![] }; 
        let die_number = 2;
        let player_number = 1;

        match bar_move_step(Some(&destination_point), die_number, player_number) {
            Some(move_step) => {
                assert_eq!(move_step.from.kind, PointKind::Bar);
                assert_eq!(move_step.from.number, None);
                assert_eq!(move_step.to.kind, PointKind::Point);
                assert_eq!(move_step.to.number, Some(2));
                assert_eq!(move_step.die_number, 2);
            },
            None => assert!(false, "expected move step")
        }
    }

    #[test]
    fn bar_move_step_prime_test() {
        let piece_a = Piece { player_number: 2 };
        let piece_b = Piece { player_number: 2 };
        let destination_point = Point { number: 2, pieces: vec![piece_a, piece_b] }; 
        let die_number = 2;
        let player_number = 1;

        match bar_move_step(Some(&destination_point), die_number, player_number) {
            Some(_) => assert!(false, "expected none"),
            None => assert!(true) 
        }
    }

    #[test]
    fn bar_move_step_invalid_test() {
        let die_number = 2;
        let player_number = 1;

        match bar_move_step(None, die_number, player_number) {
            Some(_) => assert!(false, "expected none"),
            None => assert!(true) 
        }
    }

    #[test]
    fn off_board_move_step_test() {
        let origin_point = Point { number: 24, pieces: vec![] }; 
        let die_number = 1;

        match off_board_move_step(&origin_point, die_number) {
            Some(move_step) => {
                assert_eq!(move_step.from.kind, PointKind::Point);
                assert_eq!(move_step.from.number, Some(24));
                assert_eq!(move_step.to.kind, PointKind::OffBoard);
                assert_eq!(move_step.to.number, None);
                assert_eq!(move_step.die_number, 1);
            },
            None => assert!(false, "expected move")
        }
    }

    #[test]
    fn beyond_off_board_move_step_valid_test() {
        let origin_point = Point { number: 21, pieces: vec![] }; 
        let die_number = 6;
        let back_point_number = 21;

        match beyond_off_board_move_step(&origin_point, die_number, Some(back_point_number)) {
            Some(move_step) => {
                assert_eq!(move_step.from.kind, PointKind::Point);
                assert_eq!(move_step.from.number, Some(21));
                assert_eq!(move_step.to.kind, PointKind::OffBoard);
                assert_eq!(move_step.to.number, None);
                assert_eq!(move_step.die_number, 6);
            },
            None => assert!(false, "expected move")
        }
    }

    #[test]
    fn beyond_off_board_move_step_invalid_test() {
        let origin_point = Point { number: 21, pieces: vec![] }; 
        let die_number = 6;

        match beyond_off_board_move_step(&origin_point, die_number, None) {
            Some(_) => assert!(false, "expected none"),
            None => assert!(true)
        }
    }

    #[test]
    fn point_to_point_move_step_valid_test() {
        let origin_point = Point { number: 1, pieces: vec![] }; 
        let destination_point = Point { number: 4, pieces: vec![] }; 
        let die_number = 3;
        let player_number = 1;

        match point_to_point_move_step(&origin_point, Some(&destination_point), die_number, player_number) {
            Some(move_step) => {
                assert_eq!(move_step.from.kind, PointKind::Point);
                assert_eq!(move_step.from.number, Some(1));
                assert_eq!(move_step.to.kind, PointKind::Point);
                assert_eq!(move_step.to.number, Some(4));
                assert_eq!(move_step.die_number, 3);
            },
            None => assert!(false, "expected move")
        }
    }

    #[test]
    fn point_to_point_move_step_prime_test() {
        let origin_point = Point { number: 1, pieces: vec![] }; 
        let piece_a = Piece { player_number: 2 };
        let piece_b = Piece { player_number: 2 };
        let destination_point = Point { number: 4, pieces: vec![piece_a, piece_b] }; 
        let die_number = 3;
        let player_number = 1;

        match point_to_point_move_step(&origin_point, Some(&destination_point), die_number, player_number) {
            Some(_) => assert!(false, "expected none"),
            None => assert!(true)
        }
    }

    #[test]
    fn point_to_point_move_step_invalid_test() {
        let origin_point = Point { number: 1, pieces: vec![] }; 
        let die_number = 3;
        let player_number = 1;

        match point_to_point_move_step(&origin_point, None, die_number, player_number) {
            Some(_) => assert!(false, "expected none"),
            None => assert!(true)
        }
    }
}
