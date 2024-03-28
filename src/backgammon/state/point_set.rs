use crate::backgammon::state::point::Point;
use crate::backgammon::state::point::parse_point;

pub fn parse_point_set(encoded: &str) -> Result<Vec<Point>, &'static str> {
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

        Ok(points)
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
        assert_eq!(result.len(), 24);
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
}
