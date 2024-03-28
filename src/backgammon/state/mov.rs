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
    pub die_number: i8,
    pub hit: bool
}

impl MoveStep {
    pub fn format(&self) -> String {
        let hit_string = if self.hit {
            "*"
        } else {
            ""
        };
        String::from(format!("{}/{}{}", self.from.format(), self.to.format(), hit_string))
    }
}

#[derive(Clone)]
pub struct Move {
    pub die_numbers: Vec<i8>,
    pub list: Vec<MoveStep>
}

impl Move {
    // 4-2: 8/4 6/4 - dice 4 and 2, point 8 to 4, 6 to 4
    // * - hit
    // bar - bar
    // off - off board
    pub fn format(&self) -> String {
        let mut die_numbers_copy = self.die_numbers.clone();
        die_numbers_copy.sort();
        let dice = String::from(format!("{}-{}", die_numbers_copy[1], die_numbers_copy[0]));
        let move_steps = self.list.iter().map(|ms| { ms.format() }).collect::<Vec<String>>().join(" ");
        String::from(format!("{}: {}", dice, move_steps))
    }
}

pub fn build_move_step(from_kind: PointKind, from_number: Option<i8>, to_kind: PointKind, to_number: Option<i8>, die_number: i8, hit: bool) -> MoveStep {
    let from = Location { kind: from_kind, number: from_number };
    let to = Location { kind: to_kind, number: to_number };
    MoveStep { from, to, die_number, hit }
}

pub fn bar_move_step(destination_point: Option<&Point>, die_number: i8, player_number: i8) -> Option<MoveStep> {
   if let Some(p) = destination_point {
       if !(p.prime() && p.occupied_by_opponent(player_number)) {
           let hit = p.occupied_by_opponent(player_number) && p.blot();
           Some(build_move_step(
               PointKind::Bar, None,
               PointKind::Point, Some(p.number),
               die_number,
               hit
           ))
       } else {
           None
       }
   } else {
       None
   }
}

pub fn off_board_move_step(origin_point: &Point, die_number: i8) -> Option<MoveStep> {
   Some(build_move_step(
       PointKind::Point, Some(origin_point.number),
       PointKind::OffBoard, None,
       die_number,
       false
   ))
}

pub fn beyond_off_board_move_step(origin_point: &Point, die_number: i8, back_point_number: Option<i8>) -> Option<MoveStep> {
    if let Some(n) = back_point_number {
        if n == origin_point.number {
            off_board_move_step(origin_point, die_number)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn point_to_point_move_step(origin_point: &Point, destination_point: Option<&Point>, die_number: i8, player_number: i8) -> Option<MoveStep> {
   if let Some(p) = destination_point {
       if !(p.prime() && p.occupied_by_opponent(player_number)) {
           let hit = p.occupied_by_opponent(player_number) && p.blot();
           Some(build_move_step(
               PointKind::Point, Some(origin_point.number),
               PointKind::Point, Some(p.number),
               die_number,
               hit
           ))
       } else {
           None
       }
   } else {
       None
   }
}

#[cfg(test)]
mod tests {
    use super::*;

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
       let hit = true;
       let move_step = MoveStep { from, to, die_number, hit };
       assert_eq!(move_step.format(), "1/2*");
    }

    #[test]
    fn move_format_test() {
       let from_a = Location { kind: PointKind::Point, number: Some(1) };
       let to_a = Location { kind: PointKind::Point, number: Some(2) };
       let die_number_a = 1;
       let move_step_a = MoveStep { from: from_a, to: to_a, die_number: die_number_a, hit: false };

       let from_b = Location { kind: PointKind::Point, number: Some(2) };
       let to_b = Location { kind: PointKind::Point, number: Some(4) };
       let die_number_b = 2;
       let move_step_b = MoveStep { from: from_b, to: to_b, die_number: die_number_b, hit: false };

       let mov = Move {
           die_numbers: vec![die_number_a, die_number_b],
           list: vec![move_step_a, move_step_b]
       };

       assert_eq!(mov.format(), "2-1: 1/2 2/4");
    }

    #[test]
    fn build_move_step_test() {
        let from_kind = PointKind::Point;
        let from_number = Some(1);
        let to_kind = PointKind::Point;
        let to_number = Some(2);
        let die_number = 1;

        let move_step = build_move_step(from_kind, from_number, to_kind, to_number, die_number, false);

        assert_eq!(move_step.from.kind, PointKind::Point);
        assert_eq!(move_step.from.number, Some(1));
        assert_eq!(move_step.to.kind, PointKind::Point);
        assert_eq!(move_step.to.number, Some(2));
        assert_eq!(move_step.die_number, 1);
    }

    #[test]
    fn bar_move_step_valid_test() {
        let destination_point = Point {
            number: 2,
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
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
        let destination_point = Point {
            number: 2,
            player_one_piece_count: 0,
            player_two_piece_count: 2
        };
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
        let origin_point = Point {
            number: 24,
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
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
        let origin_point = Point {
            number: 21,
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
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
        let origin_point = Point {
            number: 21,
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let die_number = 6;

        match beyond_off_board_move_step(&origin_point, die_number, None) {
            Some(_) => assert!(false, "expected none"),
            None => assert!(true)
        }
    }

    #[test]
    fn point_to_point_move_step_valid_test() {
        let origin_point = Point {
            number: 1,
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let destination_point = Point {
            number: 4,
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
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
        let origin_point = Point {
            number: 1,
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let destination_point = Point {
            number: 4,
            player_one_piece_count: 0,
            player_two_piece_count: 2
        };
        let die_number = 3;
        let player_number = 1;

        match point_to_point_move_step(&origin_point, Some(&destination_point), die_number, player_number) {
            Some(_) => assert!(false, "expected none"),
            None => assert!(true)
        }
    }

    #[test]
    fn point_to_point_move_step_invalid_test() {
        let origin_point = Point {
            number: 1,
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let die_number = 3;
        let player_number = 1;

        match point_to_point_move_step(&origin_point, None, die_number, player_number) {
            Some(_) => assert!(false, "expected none"),
            None => assert!(true)
        }
    }
}
