use crate::backgammon::state::point::Point;
use crate::backgammon::state::point::parse_point;

pub struct PointSet {
    pub points: Vec<Point>
}

impl Clone for PointSet {
    fn clone(&self) -> PointSet {
        PointSet {
            points: self.points.clone()
        }
    }
}

impl PointSet {
    pub fn pop_piece(&mut self, point_number: i8) -> Result<i8, &'static str> {
        match self.points.iter_mut().find(|p| p.number == point_number) {
            Some(p) => p.pop_piece(),
            None => Err("point not found")
        }
    }

    pub fn push_piece(&mut self, piece: i8, point_number: i8) -> Result<Option<i8>, &'static str> {
        match self.points.iter_mut().find(|p| p.number == point_number) {
            Some(p) => p.push_piece(piece),
            None => Err("point not found") 
        }
    }
}

pub fn parse_point_set(encoded: &str) -> Result<PointSet, &'static str> {
    if encoded.len() == 48 {
        let mut points = Vec::new();

        let mut point_counter = 1; 

        while point_counter <= 24 {
            let point_index = (point_counter * 2) - 2;
            let point_component = &encoded[point_index..(point_index + 2)];

            let point = match parse_point(point_counter, point_component) {
                Ok(p) => p,
                Err(e) => return Err(e)
            };

            points.push(point);
            
            point_counter += 1;
        }

        let point_set = PointSet { points };
        Ok(point_set)
    } else {
        Err("invalid point set")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let encoded = String::from("200000000005003000000050020000000050003000000005");
        let result = parse_point_set(&encoded).unwrap(); 
        assert_eq!(result.points.len(), 24);
    }

    #[test]
    fn parse_long_test() {
        let encoded = String::from("20000000000500300000005002000000005000300000000501");
        let result = parse_point_set(&encoded); 
        match result {
            Ok(_) => assert!(false, "must not return point set"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn parse_short_test() {
        let encoded = String::from("2000000000050030000000500200000000500030000000");
        let result = parse_point_set(&encoded); 
        match result {
            Ok(_) => assert!(false, "must not return point set"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn pop_piece_valid_test() {
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 1
        };
        let mut point_set = PointSet { points: vec![point] };
        let result = point_set.pop_piece(1);
        match result {
            Ok(p) => assert_eq!(2, p),
            Err(_) => assert!(false, "expected number")
        }
    }

    #[test]
    fn pop_piece_unknown_point_test() {
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 1
        };
        let mut point_set = PointSet { points: vec![point] };
        let result = point_set.pop_piece(25);
        match result {
            Ok(_) => assert!(false, "expected no number"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn pop_piece_no_piece_test() {
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0 
        };
        let mut point_set = PointSet { points: vec![point] };
        let result = point_set.pop_piece(1);
        match result {
            Ok(_) => assert!(false, "expected no number"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn push_piece_with_point_test() {
        let piece = 1;
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0 
        };
        let mut point_set = PointSet { points: vec![point] };
        let result = point_set.push_piece(piece, 1);
        match result {
            Ok(piece) => {
                match piece {
                    Some(_) => assert!(false, "expected no number"),
                    None => assert!(true)
                }
            }, 
            Err(_) => assert!(false, "expected no error")
        }
    }

    #[test]
    fn push_piece_with_not_point_test() {
        let piece = 1;
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0 
        };
        let mut point_set = PointSet { points: vec![point] };
        let result = point_set.push_piece(piece, 4);
        match result {
            Ok(_) => assert!(false, "expected error"),
            Err(_) => assert!(true)
        }
    }
}
