use rand::prelude::*;

pub fn recommended_move(game_state: &String) -> Option<&'static str> {
    let game_state_string = game_state.as_str();
    match game_state_string {
        "PL[B]ABAWXB[0]XW[0]XS" => {
            // initial
            let mut rng = rand::rng();
            let mut possible_values = ["dd", "pd", "dp", "pp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },

        "PL[W]AB[dd]AWXB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["pd", "dp", "pp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "PL[W]AB[pd]AWXB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["dd", "dp", "pp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "PL[W]AB[dp]AWXB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["dd", "pd", "pp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "PL[W]AB[pp]AWXB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["dd", "pd", "dp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },

        "PL[B]AB[dd]AW[pd]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["dp", "pp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "PL[B]AB[dd]AW[dp]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["pd", "pp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "PL[B]AB[dd]AW[pp]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["pd", "dp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },

        "PL[B]AB[pd]AW[dd]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["dp", "pp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "PL[B]AB[pd]AW[dp]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["dd", "pp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "PL[B]AB[pd]AW[pp]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["dd", "dp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },

        "PL[B]AB[dp]AW[dd]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["pd", "pp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "PL[B]AB[dp]AW[pd]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["dd", "pp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "PL[B]AB[dp]AW[pp]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["dd", "dp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },

        "PL[B]AB[pp]AW[dd]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["pd", "dp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "PL[B]AB[pp]AW[dp]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["dd", "pd"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "PL[B]AB[pp]AW[pd]XB[0]XW[0]XS" => {
            let mut rng = rand::rng();
            let mut possible_values = ["dd", "dp"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },

        "PL[W]AB[dd][pd]AW[dp]XB[0]XW[0]XS" => {
            Some("pp")
        },
        "PL[W]AB[dd][dp]AW[pd]XB[0]XW[0]XS" => {
            Some("pp")
        },
        "PL[W]AB[pd][dd]AW[dp]XB[0]XW[0]XS" => {
            Some("pp")
        },
        "PL[W]AB[pd][dp]AW[dd]XB[0]XW[0]XS" => {
            Some("pp")
        },
        "PL[W]AB[dp][dd]AW[pd]XB[0]XW[0]XS" => {
            Some("pp")
        },
        "PL[W]AB[dp][pd]AW[dd]XB[0]XW[0]XS" => {
            Some("pp")
        },

        "PL[W]AB[dd][pp]AW[dp]XB[0]XW[0]XS" => {
            Some("pd")
        },
        "PL[W]AB[dd][dp]AW[pp]XB[0]XW[0]XS" => {
            Some("pd")
        },
        "PL[W]AB[pp][dd]AW[dp]XB[0]XW[0]XS" => {
            Some("pd")
        },
        "PL[W]AB[pp][dp]AW[dd]XB[0]XW[0]XS" => {
            Some("pd")
        },
        "PL[W]AB[dp][dd]AW[pp]XB[0]XW[0]XS" => {
            Some("pd")
        },
        "PL[W]AB[dp][pp]AW[dd]XB[0]XW[0]XS" => {
            Some("pd")
        },

        "PL[W]AB[dd][pp]AW[pd]XB[0]XW[0]XS" => {
            Some("dp")
        },
        "PL[W]AB[dd][pd]AW[pp]XB[0]XW[0]XS" => {
            Some("dp")
        },
        "PL[W]AB[pp][dd]AW[pd]XB[0]XW[0]XS" => {
            Some("dp")
        },
        "PL[W]AB[pp][pd]AW[dd]XB[0]XW[0]XS" => {
            Some("dp")
        },
        "PL[W]AB[pd][dd]AW[pp]XB[0]XW[0]XS" => {
            Some("dp")
        },
        "PL[W]AB[pd][pp]AW[dd]XB[0]XW[0]XS" => {
            Some("dp")
        },

        "PL[W]AB[dp][pp]AW[pd]XB[0]XW[0]XS" => {
            Some("dd")
        },
        "PL[W]AB[dp][pd]AW[pp]XB[0]XW[0]XS" => {
            Some("dd")
        },
        "PL[W]AB[pp][dp]AW[pd]XB[0]XW[0]XS" => {
            Some("dd")
        },
        "PL[W]AB[pp][pd]AW[dp]XB[0]XW[0]XS" => {
            Some("dd")
        },
        "PL[W]AB[pd][dp]AW[pp]XB[0]XW[0]XS" => {
            Some("dd")
        },
        "PL[W]AB[pd][pp]AW[dp]XB[0]XW[0]XS" => {
            Some("dd")
        },

        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch() {
        let game_state = String::from("PL[B]ABAWXB[0]XW[0]XS");
        match recommended_move(&game_state)  {
            Some(mov) => {
                let option_a = mov == "dd";
                let option_b = mov == "pd";
                let option_c = mov == "dp";
                let option_d = mov == "pp";
                let result = option_a || option_b || option_c || option_d;
                assert_eq!(result, true);
            },
            None => assert!(false, "expected move"),
        }
    }
}
