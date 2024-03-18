use rand::prelude::*;

pub fn recommended_move(game_state: &String) -> Option<&'static str> {
    let game_state_string = game_state.as_str();
    match game_state_string {
        "B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12" => {
            // initial
            let mut rng = rand::thread_rng();
            let mut possible_values = ["9-13", "9-14", "10-14", "10-15", "11-15", "11-16", "12-16"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,11,12,13" => {
            // 9-13
            let mut rng = rand::thread_rng();
            let mut possible_values = ["22-18", "23-18", "23-19", "24-20"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W18,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,11,12,13" => {
            // 9-13 22-18
            let mut rng = rand::thread_rng();
            let mut possible_values = ["6-9", "10-14", "10-15", "11-15", "11-16"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W18,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,11,12,13" => {
            // 9-13 23-18
            Some("11-16")
        },
        "B:W19,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,11,12,13" => {
            // 9-13 23-19
            Some("10-15")
        },
        "B:W20,21,22,23,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,11,12,13" => {
            //9-13 24-20
            Some("11-16")
        },
        "W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,11,12,14" => {
            // 9-14
            let mut rng = rand::thread_rng();
            let mut possible_values = ["22-17", "22-18", "23-18", "23-19"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W17,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,11,12,14" => {
            // 9-14 22-17
            Some("11-15")
        },
        "B:W18,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,11,12,14" => {
            // 9-14 22-18
            let mut rng = rand::thread_rng();
            let mut possible_values = ["5-9", "14-17"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W18,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,11,12,14" => {
            // 9-14 23-18
            Some("14-23")
        }
        "B:W19,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,11,12,14" => {
            // 9-14 23-19
            Some("10-15")
        },
        "W:W17,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,12,14,15" => {
            // 9-14 22-17 11-15
            Some("25-22")
        },
        "W:W18,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,6,7,8,9,10,11,12,14" => {
            // 9-14 22-18 5-9
            Some("18-15")
        },
        "B:W17,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,12,14,15" => {
            // 9-14 22-17 11-15 25-22
            Some("15-19")
        },
        "W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,14" => {
            // 10-14
            let mut rng = rand::thread_rng();
            let mut possible_values = ["22-17", "22-18", "23-19", "24-19"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W17,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,14" => {
            // 10-14 22-17
            let mut rng = rand::thread_rng();
            let mut possible_values = ["9-13", "11-15", "11-16"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W18,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,14" => {
            // 10-14 22-18
            Some("7-10")
        },
        "B:W19,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,14" => {
            // 10-14 23-19
            let mut rng = rand::thread_rng();
            let mut possible_values = ["7-10", "11-15"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W19,21,22,23,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,14" => {
            // 10-14 24-19
            let mut rng = rand::thread_rng();
            let mut possible_values = ["6-10", "11-15"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,15" => {
            // 10-15
            let mut rng = rand::thread_rng();
            let mut possible_values = ["21-17", "22-17", "22-18", "23-18", "23-19"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W17,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,15" => {
            // 10-15 21-17
            let mut rng = rand::thread_rng();
            let mut possible_values = ["7-10", "9-13"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W17,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,15" => {
            // 10-15 22-17
            let mut rng = rand::thread_rng();
            let mut possible_values = ["9-14", "15-19"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W18,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,15" => {
            // 10-15 22-18
            Some("15-19")
        },
        "B:W18,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,15" => {
            // 10-15 23-18
            let mut rng = rand::thread_rng();
            let mut possible_values = ["6-10", "7-10"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W19,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,11,12,15" => {
            // 10-15 23-19
            Some("11-16")
        },
        "W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15" => {
            // 11-15
            let mut rng = rand::thread_rng();
            let mut possible_values = ["21-17", "22-17", "22-18", "23-18", "23-19", "24-19", "24-20"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W17,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15" => {
            // 11-15 22-17
            let mut rng = rand::thread_rng();
            let mut possible_values = ["8-11", "9-13", "15-19"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W18,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15" => {
            // 11-15 23-18
            Some("8-11")
        },
        "B:W19,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15" => {
            // 11-15 23-19
            let mut rng = rand::thread_rng();
            let mut possible_values = ["7-11", "8-11", "9-13", "9-14"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "W:W17,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,10,11,12,15" => {
            // 11-15 22-17 8-11
            let mut rng = rand::thread_rng();
            let mut possible_values = ["17-13", "17-14", "25-22"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "W:W17,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,12,13,15" => {
            // 11-15 22-17 9-13
            let mut rng = rand::thread_rng();
            let mut possible_values = ["17-14", "24-20"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "W:W18,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,10,11,12,15" => {
            // 11-15 23-18 8-11
            let mut rng = rand::thread_rng();
            let mut possible_values = ["18-14", "27-23"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "W:W19,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,8,9,10,11,12,15" => {
            // 11-15 23-19 7-11
            let mut rng = rand::thread_rng();
            let mut possible_values = ["22-17", "22-18"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "W:W19,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,10,11,12,15" => {
            // 11-15 23-19 8-11
            let mut rng = rand::thread_rng();
            let mut possible_values = ["22-17", "22-18", "26-23"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "W:W19,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,12,14,15" => {
            // 11-15 23-19 9-14
            let mut rng = rand::thread_rng();
            let mut possible_values = ["22-17", "27-23"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W13,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,10,11,12,15" => {
            // 11-15 22-17 8-11 17-13
            let mut rng = rand::thread_rng();
            let mut possible_values = ["4-8", "15-18"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W18,21,22,23,24,25,26,28,29,30,31,32:B1,2,3,4,5,6,7,9,10,11,12,15" => {
            // 11-15 23-18 8-11 27-23
            Some("11-16")
        },
        "B:W17,19,21,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,8,9,10,11,12,15" => {
            // 11-15 23-19 7-11 22-17
            Some("9-14")
        },
        "B:W17,19,21,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,10,11,12,15" => {
            // 11-15 23-19 8-11 22-17
            let mut rng = rand::thread_rng();
            let mut possible_values = ["3-8", "9-13", "15-18", "11-16", "9-13", "4-8"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W17,19,21,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,10,12,14,15" => {
            // 11-15 23-19 9-14 22-17
            let mut rng = rand::thread_rng();
            let mut possible_values = ["5-9", "6-9", "7-11"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "W:W13,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,5,6,7,8,9,10,11,12,15" => {
            // 11-15 22-17 8-11 17-13 4-8
            Some("25-22")
        },
        "W:W17,19,21,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,10,11,12,13,15" => {
            // 11-15 23-19 8-11 22-17 9-13
            let mut rng = rand::thread_rng();
            let mut possible_values = ["17-14", "25-22"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W14,19,21,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,10,11,12,13,15" => {
            // 11-15 23-19 8-11 22-17 9-13 17-14
            Some("10-17")
        },
        "W:W14,19,21,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,11,12,13,15,17" => {
            // 11-15 23-19 8-11 22-17 9-13 17-14 10-17
            Some("19-10")
        },
        "B:W10,14,21,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,11,12,13,15,17" => {
            // 11-15 23-19 8-11 22-17 9-13 17-14 10-17 19-10
            Some("7-14")
        },
        "W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,16" => {
            // 11-16
            let mut rng = rand::thread_rng();
            let mut possible_values = ["21-17", "22-17", "22-18", "23-18", "23-19", "24-19"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W17,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,16" => {
            // 11-16 21-17
            let mut rng = rand::thread_rng();
            let mut possible_values = ["8-11", "10-14"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W17,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,16" => {
            // 11-16 22-17
            Some("9-13")
        },
        "B:W18,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,16" => {
            // 11-16 22-18
            let mut rng = rand::thread_rng();
            let mut possible_values = ["8-11", "10-14", "10-15"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W18,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,16" => {
            // 11-16 23-18
            let mut rng = rand::thread_rng();
            let mut possible_values = ["7-11", "16-20"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "B:W19,21,22,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,16" => {
            // 11-16 23-19
            Some("16-23")
        },
        "B:W19,21,22,23,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,16" => {
            // 11-16 24-19
            Some("8-11")
        },
        "W:W18,21,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,12,14,16" => {
            // 11-16 22-18 10-14
            Some("25-22")
        },
        "W:W19,21,22,23,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,10,11,12,16" => {
            // 11-16 24-19 8-11
            Some("22-18")
        },
        "B:W18,21,22,23,24,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,12,14,16" => {
            // 11-16 22-18 10-14 25-22
            Some("8-11")
        },
        "B:W18,19,21,23,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,10,11,12,16" => {
            // 11-16 24-19 8-11 22-18
            Some("4-8")
        },
        "W:W18,21,22,23,24,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,11,12,14,16" => {
            // 11-16 22-18 10-14 25-22 8-11
            Some("24-20")
        },
        "W:W18,19,21,23,25,26,27,28,29,30,31,32:B1,2,3,5,6,7,8,9,10,11,12,16" => {
            // 11-16 24-19 8-11 22-18 4-8
            Some("26-22")
        },
        "B:W18,20,21,22,23,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,11,12,14,16" => {
            // 11-16 22-18 10-14 25-22 8-11 24-20
            Some("16-19")
        },
        "B:W18,19,21,22,23,25,27,28,29,30,31,32:B1,2,3,5,6,7,8,9,10,11,12,16" => {
            // 11-16 24-19 8-11 22-18 4-8 26-22
            Some("16-20")
        },
        "W:W18,20,21,22,23,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,11,12,14,19" => {
            // 11-16 22-18 10-14 25-22 8-11 24-20 16-19
            Some("23-16")
        },
        "B:W16,18,20,21,22,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,11,12,14,19" => {
            // 11-16 22-18 10-14 25-22 8-11 24-20 16-19 23-16
            Some("14-23")
        },
        "W:W16,18,20,21,22,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,11,12,19,23" => {
            // 11-16 22-18 10-14 25-22 8-11 24-20 16-19 23-16 14-23
            Some("26-19")
        },
        "W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,16" => {
            // 12-16
            let mut rng = rand::thread_rng();
            let mut possible_values = ["21-17", "24-20"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        }
        "B:W17,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,16" => {
            // 12-16 21-17
            Some("9-14")
        },
        "B:W20,21,22,23,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,16" => {
            // 12-16 24-20
            let mut rng = rand::thread_rng();
            let mut possible_values = ["8-12", "10-15"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch() {
        let game_state = String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12");
        match recommended_move(&game_state)  {
            Some(mov) => {
                // ["9-13", "9-14", "10-14", "10-15", "11-15", "11-16", "12-16"];
                let option_a = mov == "9-13";
                let option_b = mov == "9-14";
                let option_c = mov == "10-14";
                let option_d = mov == "10-15";
                let option_e = mov == "11-15";
                let option_f = mov == "11-16";
                let option_g = mov == "12-16";
                let result = option_a || option_b || option_c || option_d || option_e || option_f || option_g;
                assert_eq!(result, true);
            },
            None => assert!(false, "expected move"),
        }
    }
}
