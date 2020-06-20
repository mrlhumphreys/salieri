use rand::prelude::*;

pub fn recommended_move(game_state: &String) -> Option<&'static str> {
    let game_state_string = game_state.as_str();
    match game_state_string {
        "bbbbbbb-bbbb--b---w-ww-wwwwwwwwww" => Some("22-17"),
        "bbbbbbb-bbbb--b-w---w-wwwwwwwwwww" => Some("17-13"),
        "bbbbbbb-bbbb--b-w-w-w--wwwwwwwwwb" => {
            let mut rng = rand::thread_rng();
            let mut possible_values = ["11-16", "4-8", "9-13"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "bbbbbbb-bbbbw-b-----w-wwwwwwwwwwb" => Some("15-18"),
        "bbbbbbbb-b-b-bb---w-ww-wwwwwwwwww" => {
            let mut rng = rand::thread_rng();
            let mut possible_values = ["22-17", "27-23"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "bbbbbbbb-b-b-bb-w-w-w--wwwwwwwwwb" => {
            let mut rng = rand::thread_rng();
            let mut possible_values = ["5-9", "6-9", "7-11"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        }
        "bbbbbbbbbb-b---b----wwwwwwwwwwwww" => Some("24-20"),
        "bbbbbbbbbb-b---b---wwww-wwwwwwwwb" => Some("16-19"),
        "bbbbbbbbbb-b--b-----wwwwwwwwwwwww" => {
            let mut rng = rand::thread_rng();
            let mut possible_values = ["22-17", "22-18", "23-19", "24-19", "24-20"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "bbbbbbbbbb-b--b----wwww-wwwwwwwwb" => Some("8-11"),
        "bbbbbbbbbb-b--b---w-ww-wwwwwwwwwb" => {
            let mut rng = rand::thread_rng();
            let mut possible_values = ["8-11", "9-13", "9-14"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "bbbbbbbbbb-b--b-w---w-wwwwwwwwwwb" => {
            let mut rng = rand::thread_rng();
            let mut possible_values = ["15-19", "8-11"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "bbbbbbbbbbbb--------wwwwwwwwwwwwb" => {
            let mut rng = rand::thread_rng();
            let mut possible_values = ["11-15", "11-16"];
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
        let game_state = String::from("bbbbbbbbbbbb--------wwwwwwwwwwwwb");
        match recommended_move(&game_state)  {
            Some(mov) => {
                let option_a = mov == "11-15";
                let option_b = mov == "11-16";
                let result = option_a || option_b;
                assert_eq!(result, true);
            },
            None => assert!(false, "expected move"),
        }
    }
}
