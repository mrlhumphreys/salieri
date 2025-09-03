use rand::prelude::*;

pub fn recommended_move(game_state: &String) -> Option<&'static str> {
    let game_state_string = game_state.as_str();
    match game_state_string {
        "0020000000000500030000005005000000300050000000000200121" => {
            let mut rng = rand::rng();
            let mut possible_values = ["1-2: 24/23 13/11", "1-2: 13/11 6/5"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },

        "0020000000000500030000005005000000300050000000000200131" => Some("1-3: 8/5 6/5"),
        "0020000000000500030000005005000000300050000000000200231" => {
            let mut rng = rand::rng();
            let mut possible_values = ["2-3: 24/21 13/11", "2-3: 13/11 13/10"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },

        "0020000000000500030000005005000000300050000000000200141" => Some("1-4: 24/23 13/9"),
        "0020000000000500030000005005000000300050000000000200241" => Some("2-4: 8/4 6/4"),
        "0020000000000500030000005005000000300050000000000200341" => {
            let mut rng = rand::rng();
            let mut possible_values = ["3-4: 24/20 13/10", "3-4: 13/10 13/9", "3-4: 24/21 13/9"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },

        "0020000000000500030000005005000000300050000000000200151" => Some("1-5: 24/23 13/8"),
        "0020000000000500030000005005000000300050000000000200251" => {
            let mut rng = rand::rng();
            let mut possible_values = ["2-5: 13/11 13/8", "2-5: 24/22 13/8"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "0020000000000500030000005005000000300050000000000200351" => Some("3-5: 8/3 6/3"),
        "0020000000000500030000005005000000300050000000000200451" => {
            let mut rng = rand::rng();
            let mut possible_values = ["4-5: 24/20 13/8", "4-5: 13/9 13/8"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },

        "0020000000000500030000005005000000300050000000000200161" => Some("1-6: 13/7 8/7"),
        "0020000000000500030000005005000000300050000000000200261" => Some("2-6: 24/18 13/11"),
        "0020000000000500030000005005000000300050000000000200361" => Some("3-6: 24/18 13/10"),
        "0020000000000500030000005005000000300050000000000200461" => {
            let mut rng = rand::rng();
            let mut possible_values = ["4-6: 24/18 13/9", "4-6: 24/20 20/14", "4-6: 8/2 6/2"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "0020000000000500030000005005000000300050000000000200561" => Some("5-6: 24/18 18/13"),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_test() {
        let game_state = String::from("0020000000000500030000005005000000300050000000000200461");
        match recommended_move(&game_state)  {
            Some(mov) => {
                let option_a = mov == "4-6: 24/18 13/9";
                let option_b = mov == "4-6: 24/20 20/14";
                let option_c = mov == "4-6: 8/2 6/2";
                let result = option_a || option_b || option_c;
                assert_eq!(result, true);
            },
            None => assert!(false, "expected move"),
        }
    }
}
