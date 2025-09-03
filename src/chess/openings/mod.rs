use rand::prelude::*;

pub fn recommended_move(game_state: &String) -> Option<&'static str> {
    let game_state_string = game_state.as_str();

    match game_state_string {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" => {
            // initial state
            let mut rng = rand::rng();
            let mut possible_values = ["e4", "d4"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1" => {
            // e4
            let mut rng = rand::rng();
            let mut possible_values = ["c5", "c6", "d5", "d6", "e5", "e6", "Nf6"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1" => {
            // d4
            let mut rng = rand::rng();
            let mut possible_values = ["d5", "Nf6", "f5"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2" => {
            // e4 e5
            let mut rng = rand::rng();
            let mut possible_values = ["Nf3", "f4", "Nc3"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq d6 0 2" => {
            let mut rng = rand::rng();
            let mut possible_values = ["c4", "Nf3"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "rnbqkb1r/pppppppp/5n2/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 1 2" => {
            let mut rng = rand::rng();
            let mut possible_values = ["c4", "Bg5"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "rnbqkbnr/ppp1pppp/3p4/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2" => {
            // e4 d6
            Some("d4")
        },
        "rnbqkbnr/ppp1pppp/8/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 1 2" => {
            // d4 d5 Nf3
            Some("Nf6")
        },
        "rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2" => {
            // e4 e5 Nf3
            Some("Nc6")
        },
        "rnbqkbnr/ppp1pppp/8/3p4/2PP4/8/PP2PPPP/RNBQKBNR b KQkq c3 0 2" => {
            // d4 d5 c4
            Some("c6")
        },
        "rnbqkbnr/ppp1pppp/3p4/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq d3 0 2" => {
            // e4 d6 d4
            Some("Nf6")
        },
        "rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq c3 0 2" => {
            // d4 Nf6 c4
            let mut rng = rand::rng();
            let mut possible_values = ["g6", "e6", "c5"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 3" => {
            // d4 Nf6 c4 e6
            let mut rng = rand::rng();
            let mut possible_values = ["Nc3", "Nf3", "g3"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "rnbqkb1r/pppppp1p/5np1/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 3" => {
            // d4 Nf6 c4  g6
            Some("Nc3")
        },
        "rnbqkb1r/pp1ppppp/5n2/2p5/2PP4/8/PP2PPPP/RNBQKBNR w KQkq c6 0 3" => {
            // d4 Nf6 c4 c5
            Some("d5")
        },
        "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3" => {
            // e4 e5 Nf3 Nc6
            let mut rng = rand::rng();
            let mut possible_values = ["Bb5", "Bc4", "d4"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 2 3" => {
            // d4 d5 Nf3 Nf6
            Some("Bf4")
        },
        "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 1 3" => {
            // d4 Nf6 c4  e6  Nc3
            Some("Bb4")
        },
        "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 1 3" => {
            // d4 Nf6 c4  e6  Nf3
            let mut rng = rand::rng();
            let mut possible_values = ["b6", "Bb4+"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "rnbqkb1r/pppppp1p/5np1/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 1 3" => {
            // d4 Nf6 c4  g6  Nc3
            Some("d5")
        },
        "rnbqkb1r/pp1ppppp/5n2/2pP4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 3" => {
            // d4 Nf6 c4 c5 d5
            let mut rng = rand::rng();
            let mut possible_values = ["b5", "e6"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 4" => {
            // d4 Nf6 c4  c5  d5  e6
            Some("Nc3")
        },
        "rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 1 4" => {
            // d4 Nf6 c4  c5  d5  e6   Nc3
            Some("exd5")
        },
        "rnbqkb1r/pp1p1ppp/5n2/2pp4/2P5/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 5" => {
            // d4 Nf6 c4  c5  d5  e6   Nc3 exd5
            Some("cxd5")
        },
        "rnbqkb1r/pp1p1ppp/5n2/2pP4/8/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 5" => {
            // d4 Nf6 c4  c5  d5  e6   Nc3 exd5 cxd5
            Some("d6")
        }
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch() {
        let game_state = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        match recommended_move(&game_state)  {
            Some(mov) => {
                let option_a = mov == "e4";
                let option_b = mov == "d4";
                let result = option_a || option_b;
                assert_eq!(result, true);
            },
            None => assert!(false, "expected move"),
        }
    }
}
