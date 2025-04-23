use rand::prelude::*;

pub fn recommended_move(game_state: &String) -> Option<&'static str> {
    let game_state_string = game_state.as_str();

    match game_state_string {
        "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b -" => {
            // starting state
            let mut rng = rand::thread_rng();
            let mut possible_values = ["P-76", "P-26"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "lnsgkgsnl/1r5b1/ppppppppp/9/9/7P1/PPPPPPP1P/1B5R1/LNSGKGSNL w -" => {
            // b: P-2f/P-26
            Some("P-84")
        },
        "lnsgkgsnl/1r5b1/p1ppppppp/1p7/9/7P1/PPPPPPP1P/1B5R1/LNSGKGSNL b -" => {
            // b: P-2f/P-26 w: P-8d/P-84
            Some("P-25")
        },
        "lnsgkgsnl/1r5b1/p1ppppppp/1p7/7P1/9/PPPPPPP1P/1B5R1/LNSGKGSNL w -" => {
            // b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25
            Some("P-85")
        },
        "lnsgkgsnl/1r5b1/p1ppppppp/9/1p5P1/9/PPPPPPP1P/1B5R1/LNSGKGSNL b -" => {
            // b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25 w: P-8e/P-85
            Some("G-78")
        },
        "lnsgkgsnl/1r5b1/p1ppppppp/9/1p5P1/9/PPPPPPP1P/1BG4R1/LNS1KGSNL w -" => {
            // b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25 w: P-8e/P-85 b: G-7h/G-78
            Some("G-32")
        },
        "lnsgkgsnl/1r5b1/ppppppppp/9/9/2P6/PP1PPPPPP/1B5R1/LNSGKGSNL w -" => {
            // b: P-7f/P-76
            let mut rng = rand::thread_rng();
            let mut possible_values = ["P-34", "P-84"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "lnsgkgsnl/1r5b1/pppppp1pp/6p2/9/2P6/PP1PPPPPP/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34
            let mut rng = rand::thread_rng();
            let mut possible_values = ["P-26", "P-66", "P-75"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "lnsgkgsnl/1r5b1/pppppp1pp/6p2/2P6/9/PP1PPPPPP/1B5R1/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75
            let mut rng = rand::thread_rng();
            let mut possible_values = ["K-42", "P-84"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/1p4p2/2P6/9/PP1PPPPPP/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84
            Some("R-78")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/1p4p2/2P6/9/PP1PPPPPP/1BR6/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78
            Some("P-85")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1pP6/9/PP1PPPPPP/1BR6/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            Some("K-48")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1pP6/9/PP1PPPPPP/1BR2K3/LNSG1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48
            Some("S-62")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/6p2/1pP6/9/PP1PPPPPP/1BR2K3/LNSG1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62
            Some("K-38")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/6p2/1pP6/9/PP1PPPPPP/1BR3K2/LNSG1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38
            Some("K-42")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1pP6/9/PP1PPPPPP/1BR3K2/LNSG1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42
            Some("K-28")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1pP6/9/PP1PPPPPP/1BR4K1/LNSG1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42 b: K-2h/K-28
            Some("K-32")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1pP6/9/PP1PPPPPP/1BR4K1/LNSG1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42 b: K-2h/K-28  w: K-3b/K-32
            Some("S-38")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1pP6/9/PP1PPPPPP/1BR3SK1/LNSG1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42 b: K-2h/K-28  w: K-3b/K-32
            // b: S-3h/S-38
            Some("P-64")
        },
        "ln1g1gsnl/1r1s2kb1/p1p1pp1pp/3p2p2/1pP6/9/PP1PPPPPP/1BR3SK1/LNSG1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42 b: K-2h/K-28  w: K-3b/K-32
            // b: S-3h/S-38 w: P-6d/P-64
            Some("R-76")
        },
        "ln1g1gsnl/1r1s2kb1/p1p1pp1pp/3p2p2/1pP6/2R6/PP1PPPPPP/1B4SK1/LNSG1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42 b: K-2h/K-28  w: K-3b/K-32
            // b: S-3h/S-38 w: P-6d/P-64 b: R-7f/R-76
            Some("Bx88+")
        },
        "ln1g1gsnl/1r1s2k2/p1p1pp1pp/3p2p2/1pP6/2R6/PP1PPPPPP/1b+4SK1/LNSG1G1NL b b" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42 b: K-2h/K-28  w: K-3b/K-32
            // b: S-3h/S-38 w: P-6d/P-64 b: R-7f/R-76 w: Bx8h+/Bx88+
            Some("Sx88")
        },
        "ln1g1gsnl/1r1s2k2/p1p1pp1pp/3p2p2/1pP6/2R6/PP1PPPPPP/1S4SK1/LN1G1G1NL w Bb" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42 b: K-2h/K-28  w: K-3b/K-32
            // b: S-3h/S-38 w: P-6d/P-64 b: R-7f/R-76 w: Bx8h+/Bx88+ b: Sx8h/Sx88
            Some("S-22")
        },
        "ln1g1g1nl/1r1s2ks1/p1p1pp1pp/3p2p2/1pP6/2R6/PP1PPPPPP/1S4SK1/LN1G1G1NL b Bb" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42 b: K-2h/K-28  w: K-3b/K-32
            // b: S-3h/S-38 w: P-6d/P-64 b: R-7f/R-76 w: Bx8h+/Bx88+ b: Sx8h/Sx88 w: S-2b/S-22
            Some("G-78")
        },
        "ln1g1g1nl/1r1s2ks1/p1p1pp1pp/3p2p2/1pP6/2R6/PP1PPPPPP/1SG3SK1/LN3G1NL w Bb" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42 b: K-2h/K-28  w: K-3b/K-32
            // b: S-3h/S-38 w: P-6d/P-64 b: R-7f/R-76 w: Bx8h+/Bx88+ b: Sx8h/Sx88 w: S-2b/S-22
            // b: G-7h/G-78
            Some("S-63")
        },
        "ln1g1g1nl/1r4ks1/p1pspp1pp/3p2p2/1pP6/2R6/PP1PPPPPP/1SG3SK1/LN3G1NL b Bb" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42 b: K-2h/K-28  w: K-3b/K-32
            // b: S-3h/S-38 w: P-6d/P-64 b: R-7f/R-76 w: Bx8h+/Bx88+ b: Sx8h/Sx88 w: S-2b/S-22
            // b: G-7h/G-78 w: S-6c/S-63
            Some("P-16")
        },
        "ln1g1g1nl/1r4ks1/p1pspp1pp/3p2p2/1pP6/2R5P/PP1PPPPP1/1SG3SK1/LN3G1NL w Bb" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: P-8d/P-84 b: R-7h/R-78  w: P-8e/P-85
            // b: K-4h/K-48 w: S-6b/S-62 b: K-3h/K-38 w: K-4b/K-42 b: K-2h/K-28  w: K-3b/K-32
            // b: S-3h/S-38 w: P-6d/P-64 b: R-7f/R-76 w: Bx8h+/Bx88+ b: Sx8h/Sx88 w: S-2b/S-22
            // b: G-7h/G-78 w: S-6c/S-63 b: P-1f/P-16
            Some("P-14")
        },
        "lnsg1gsnl/1r3k1b1/pppppp1pp/6p2/2P6/9/PP1PPPPPP/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b
            Some("P-66")
        },
        "lnsg1gsnl/1r3k1b1/pppppp1pp/6p2/2P6/3P5/PP2PPPPP/1B5R1/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66
            Some("P-84")
        },
        "lnsg1gsnl/1r3k1b1/p1pppp1pp/1p4p2/2P6/3P5/PP2PPPPP/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            Some("R-78")
        },
        "lnsg1gsnl/1r3k1b1/p1pppp1pp/1p4p2/2P6/3P5/PP2PPPPP/1BR6/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78
            Some("P-85")
        },
        "lnsg1gsnl/1r3k1b1/p1pppp1pp/6p2/1pP6/3P5/PP2PPPPP/1BR6/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85
            Some("R-76")
        },
        "lnsg1gsnl/1r3k1b1/p1pppp1pp/6p2/1pP6/2RP5/PP2PPPPP/1B7/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76
            Some("S-62")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1pP6/2RP5/PP2PPPPP/1B7/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62
            Some("K-48")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1pP6/2RP5/PP2PPPPP/1B3K3/LNSG1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48
            Some("K-32")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1pP6/2RP5/PP2PPPPP/1B3K3/LNSG1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            Some("K-38")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1pP6/2RP5/PP2PPPPP/1B4K2/LNSG1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38
            Some("G61-52")
        },
        "ln3gsnl/1r1sg1kb1/p1pppp1pp/6p2/1pP6/2RP5/PP2PPPPP/1B4K2/LNSG1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52
            Some("K-28")
        },
        "ln3gsnl/1r1sg1kb1/p1pppp1pp/6p2/1pP6/2RP5/PP2PPPPP/1B5K1/LNSG1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28
            Some("P-54")
        },
        "ln3gsnl/1r1sg1kb1/p1pp1p1pp/4p1p2/1pP6/2RP5/PP2PPPPP/1B5K1/LNSG1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54
            Some("S-38")
        },
        "ln3gsnl/1r1sg1kb1/p1pp1p1pp/4p1p2/1pP6/2RP5/PP2PPPPP/1B4SK1/LNSG1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38
            Some("B-33")
        },
        "ln3gsnl/1r1sg1k2/p1pp1pbpp/4p1p2/1pP6/2RP5/PP2PPPPP/1B4SK1/LNSG1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: B-3c/B-33
            Some("G69-58")
        },
        "ln3gsnl/1r1sg1k2/p1pp1pbpp/4p1p2/1pP6/2RP5/PP2PPPPP/1B2G1SK1/LNS2G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: B-3c/B-33
            // b: G6i-5h/G69-58
            Some("K-22")
        },
        "ln3gsnl/1r1sg2k1/p1pp1pbpp/4p1p2/1pP6/2RP5/PP2PPPPP/1B2G1SK1/LNS2G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: B-3c/B-33
            // b: G6i-5h/G69-58 w: K-2b/K-22
            Some("P-16")
        },
        "ln3gsnl/1r1sg2k1/p1pp1pbpp/4p1p2/1pP6/2RP4P/PP2PPPP1/1B2G1SK1/LNS2G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: B-3c/B-33
            // b: G6i-5h/G69-58 w: K-2b/K-22 b: P-1f/P-16
            Some("P-14")
        },
        "ln3gsnl/1r1sg2k1/p1pp1pbp1/4p1p1p/1pP6/2RP4P/PP2PPPP1/1B2G1SK1/LNS2G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: B-3c/B-33
            // b: G6i-5h/G69-58 w: K-2b/K-22 b: P-1f/P-16 w: P-1d/P-14
            Some("P-96")
        },
        "ln3gsnl/1r1sg2k1/p1pp1pbp1/4p1p1p/1pP6/P1RP4P/1P2PPPP1/1B2G1SK1/LNS2G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: B-3c/B-33
            // b: G6i-5h/G69-58 w: K-2b/K-22 b: P-1f/P-16 w: P-1d/P-14 b: P-9f/P-96
            Some("P-94")
        },
        "ln3gsnl/1r1sg2k1/2pp1pbp1/p3p1p1p/1pP6/P1RP4P/1P2PPPP1/1B2G1SK1/LNS2G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: B-3c/B-33
            // b: G6i-5h/G69-58 w: K-2b/K-22 b: P-1f/P-16 w: P-1d/P-14 b: P-9f/P-96 w: P-9d/P-94
            Some("S-68")
        },
        "ln3gsnl/1r1sg2k1/2pp1pbp1/p3p1p1p/1pP6/P1RP4P/1P2PPPP1/1B1SG1SK1/LN3G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: B-3c/B-33
            // b: G6i-5h/G69-58 w: K-2b/K-22 b: P-1f/P-16 w: P-1d/P-14 b: P-9f/P-96 w: P-9d/P-94
            // b: S-6h/S-68
            Some("S-32")
        },
        "ln3g1nl/1r1sg1sk1/2pp1pbp1/p3p1p1p/1pP6/P1RP4P/1P2PPPP1/1B1SG1SK1/LN3G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: B-3c/B-33
            // b: G6i-5h/G69-58 w: K-2b/K-22 b: P-1f/P-16 w: P-1d/P-14 b: P-9f/P-96 w: P-9d/P-94
            // b: S-6h/S-68 w: S-3b/S-32
            Some("N-77")
        },
        "ln3g1nl/1r1sg1sk1/2pp1pbp1/p3p1p1p/1pP6/P1RP4P/1PN1PPPP1/1B1SG1SK1/L4G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-7e/P-75 w: K-42/K-4b b: P-6f/P-66 w: P-8d/P-84
            // b: R-7h/R-78 w: P-8e/P-85 b: R-7f/R-76 w: S-6b/S-62 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: B-3c/B-33
            // b: G6i-5h/G69-58 w: K-2b/K-22 b: P-1f/P-16 w: P-1d/P-14 b: P-9f/P-96 w: P-9d/P-94
            // b: S-6h/S-68 w: S-3b/S-32 b: N-7g/N-77
            Some("P-44")
        },
        "lnsgkgsnl/1r5b1/pppppp1pp/6p2/9/2P4P1/PP1PPPP1P/1B5R1/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26
            let mut rng = rand::thread_rng();
            let mut possible_values = ["P-84", "P-54"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "lnsgkgsnl/1r5b1/pppp1p1pp/4p1p2/9/2P4P1/PP1PPPP1P/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54
            Some("P-25")
        },
        "lnsgkgsnl/1r5b1/pppp1p1pp/4p1p2/7P1/2P6/PP1PPPP1P/1B5R1/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25
            Some("R-52")
        },
        "lnsgkgsnl/4r2b1/pppp1p1pp/4p1p2/7P1/2P6/PP1PPPP1P/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            Some("S-48")
        },
        "lnsgkgsnl/4r2b1/pppp1p1pp/4p1p2/7P1/2P6/PP1PPPP1P/1B3S1R1/LNSGKG1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48
            Some("P-55")
        },
        "lnsgkgsnl/4r2b1/pppp1p1pp/6p2/4p2P1/2P6/PP1PPPP1P/1B3S1R1/LNSGKG1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55
            Some("K-68")
        },
        "lnsgkgsnl/4r2b1/pppp1p1pp/6p2/4p2P1/2P6/PP1PPPP1P/1B1K1S1R1/LNSG1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68
            Some("B-33")
        },
        "lnsgkgsnl/4r4/pppp1pbpp/6p2/4p2P1/2P6/PP1PPPP1P/1B1K1S1R1/LNSG1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33
            Some("P-36")
        },
        "lnsgkgsnl/4r4/pppp1pbpp/6p2/4p2P1/2P3P2/PP1PPP2P/1B1K1S1R1/LNSG1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36
            Some("S-42")
        },
        "lnsgkg1nl/4rs3/pppp1pbpp/6p2/4p2P1/2P3P2/PP1PPP2P/1B1K1S1R1/LNSG1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            Some("S-37")
        },
        "lnsgkg1nl/4rs3/pppp1pbpp/6p2/4p2P1/2P3P2/PP1PPPS1P/1B1K3R1/LNSG1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37
            Some("S-53")
        },
        "lnsgkg1nl/4r4/ppppspbpp/6p2/4p2P1/2P3P2/PP1PPPS1P/1B1K3R1/LNSG1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53
            Some("S-46")
        },
        "lnsgkg1nl/4r4/ppppspbpp/6p2/4p2P1/2P2SP2/PP1PPP2P/1B1K3R1/LNSG1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46
            Some("S-44")
        },
        "lnsgkg1nl/4r4/pppp1pbpp/5sp2/4p2P1/2P2SP2/PP1PPP2P/1B1K3R1/LNSG1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46 w: S-4d/S-44
            Some("K-78")
        },
        "lnsgkg1nl/4r4/pppp1pbpp/5sp2/4p2P1/2P2SP2/PP1PPP2P/1BK4R1/LNSG1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46 w: S-4d/S-44 b: K-7h/K-78
            Some("K-62")
        },
        "lnsg1g1nl/3kr4/pppp1pbpp/5sp2/4p2P1/2P2SP2/PP1PPP2P/1BK4R1/LNSG1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46 w: S-4d/S-44 b: K-7h/K-78 w: K-6b/K-62
            Some("S-68")
        },
        "lnsg1g1nl/3kr4/pppp1pbpp/5sp2/4p2P1/2P2SP2/PP1PPP2P/1BKS3R1/LN1G1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46 w: S-4d/S-44 b: K-7h/K-78 w: K-6b/K-62
            // b: S-6h/S-68
            Some("K-72")
        },
        "lnsg1g1nl/2k1r4/pppp1pbpp/5sp2/4p2P1/2P2SP2/PP1PPP2P/1BKS3R1/LN1G1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46 w: S-4d/S-44 b: K-7h/K-78 w: K-6b/K-62
            // b: S-6h/S-68 w: K-7b/K-72
            Some("S-77")
        },
        "lnsg1g1nl/2k1r4/pppp1pbpp/5sp2/4p2P1/2P2SP2/PPSPPP2P/1BK4R1/LN1G1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46 w: S-4d/S-44 b: K-7h/K-78 w: K-6b/K-62
            // b: S-6h/S-68 w: K-7b/K-72 b: S-7g/77
            Some("K-82")
        },
        "lnsg1g1nl/1k2r4/pppp1pbpp/5sp2/4p2P1/2P2SP2/PPSPPP2P/1BK4R1/LN1G1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46 w: S-4d/S-44 b: K-7h/K-78 w: K-6b/K-62
            // b: S-6h/S-68 w: K-7b/K-72 b: S-7g/S-77 w: K-8b/K-82
            Some("S-66")
        },
        "lnsg1g1nl/1k2r4/pppp1pbpp/5sp2/4p2P1/2PS1SP2/PP1PPP2P/1BK4R1/LN1G1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46 w: S-4d/S-44 b: K-7h/K-78 w: K-6b/K-62
            // b: S-6h/S-68 w: K-7b/K-72 b: S-7g/S-77 w: K-8b/K-82 b: S-6f/S-66
            Some("S-72")
        },
        "ln1g1g1nl/1ks1r4/pppp1pbpp/5sp2/4p2P1/2PS1SP2/PP1PPP2P/1BK4R1/LN1G1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46 w: S-4d/S-44 b: K-7h/K-78 w: K-6b/K-62
            // b: S-6h/S-68 w: K-7b/K-72 b: S-7g/S-77 w: K-8b/K-82 b: S-6f/S-66 w: S-7b/S-72
            Some("G49-58")
        },
        "ln1g1g1nl/1ks1r4/pppp1pbpp/5sp2/4p2P1/2PS1SP2/PP1PPP2P/1BK1G2R1/LN1G3NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46 w: S-4d/S-44 b: K-7h/K-78 w: K-6b/K-62
            // b: S-6h/S-68 w: K-7b/K-72 b: S-7g/S-77 w: K-8b/K-82 b: S-6f/S-66 w: S-7b/S-72
            // b: G4i-5h/G49-58
            Some("P-94")
        },
        "ln1g1g1nl/1ks1r4/1ppp1pbpp/p4sp2/4p2P1/2PS1SP2/PP1PPP2P/1BK1G2R1/LN1G3NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-5d/P-54 b: P-2e/P-25 w: R-5b/R-52
            // b: S-3h/S-48 w: P-5e/P-55 b: K-6h/K-68 w: B-3c/B-33 b: P-3f/P-36 w: S-4b/S-42
            // b: S-3g/S-37 w: S-5c/S-53 b: S-4f/S-46 w: S-4d/S-44 b: K-7h/K-78 w: K-6b/K-62
            // b: S-6h/S-68 w: K-7b/K-72 b: S-7g/S-77 w: K-8b/K-82 b: S-6f/S-66 w: S-7b/S-72
            // b: G4i-5h/G49-58 w: P-9d/P-94
            Some("P-96")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/1p4p2/9/2P4P1/PP1PPPP1P/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-8d/P-84
            Some("P-25")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1B5R1/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25
            Some("P-85")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1p5P1/2P6/PP1PPPP1P/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25 w: P-8e/P-85
            Some("G-78")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1p5P1/2P6/PP1PPPP1P/1BG4R1/LNS1KGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25 w: P-8e/P-85
            // b: G-7h/G-78
            Some("G-32")
        },
        "lnsgk1snl/1r4gb1/p1pppp1pp/6p2/1p5P1/2P6/PP1PPPP1P/1BG4R1/LNS1KGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25 w: P-8e/P-85
            // b: G-7h/G-78 w: G-3b/G-32
            Some("P-24")
        },
        "lnsgk1snl/1r4gb1/p1pppp1pp/6pP1/1p7/2P6/PP1PPPP1P/1BG4R1/LNS1KGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25 w: P-8e/P-85
            // b: G-7h/G-78 w: G-3b/G-32 b: P-2d/P-24
            Some("Px24")
        },
        "lnsgk1snl/1r4gb1/p1pppp2p/6pp1/1p7/2P6/PP1PPPP1P/1BG4R1/LNS1KGSNL b p" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25 w: P-8e/P-85
            // b: G-7h/G-78 w: G-3b/G-32 b: P-2d/P-24 w: Px2d/Px24
            Some("Rx24")
        },
        "lnsgk1snl/1r4gb1/p1pppp2p/6pR1/1p7/2P6/PP1PPPP1P/1BG6/LNS1KGSNL w Pp" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25 w: P-8e/P-85
            // b: G-7h/G-78 w: G-3b/G-32 b: P-2d/P-24 w: Px2d/Px24 b: Rx2d/Rx24
            Some("P-86")
        },
        "lnsgk1snl/1r4gb1/p1pppp2p/6pR1/9/1pP6/PP1PPPP1P/1BG6/LNS1KGSNL b Pp" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25 w: P-8e/P-85
            // b: G-7h/G-78 w: G-3b/G-32 b: P-2d/P-24 w: Px2d/Px24 b: Rx2d/Rx24 w: P-8f/P-86
            Some("Px86")
        },
        "lnsgk1snl/1r4gb1/p1pppp2p/6pR1/9/1PP6/P2PPPP1P/1BG6/LNS1KGSNL w 2Pp" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25 w: P-8e/P-85
            // b: G-7h/G-78 w: G-3b/G-32 b: P-2d/P-24 w: Px2d/Px24 b: Rx2d/Rx24 w: P-8f/P-86
            // b: Px8f/Px86
            Some("Rx86")
        },
        "lnsgk1snl/6gb1/p1pppp2p/6pR1/9/1rP6/P2PPPP1P/1BG6/LNS1KGSNL b 2P2p" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-2f/P-26 w: P-8d/P-84 b: P-2e/P-25 w: P-8e/P-85
            // b: G-7h/G-78 w: G-3b/G-32 b: P-2d/P-24 w: Px2d/Px24 b: Rx2d/Rx24 w: P-8f/P-86
            // b: Px8f/Px86 w: Rx8f/Rx86
            Some("Rx34")
        },
        "lnsgkgsnl/1r5b1/pppppp1pp/6p2/9/2PP5/PP2PPPPP/1B5R1/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66
            let mut rng = rand::thread_rng();
            let mut possible_values = ["P-84", "R-32"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "lnsgkgsnl/6rb1/pppppp1pp/6p2/9/2PP5/PP2PPPPP/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: R-3b/R-32
            Some("S-78")
        },
        "lnsgkgsnl/6rb1/pppppp1pp/6p2/9/2PP5/PP2PPPPP/1BS4R1/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: R-3b/R-32 b: S-7h/S-78
            Some("K-62")
        },
        "lnsg1gsnl/3k2rb1/pppppp1pp/6p2/9/2PP5/PP2PPPPP/1BS4R1/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: R-3b/R-32 b: S-7h/S-78 w: K-6b/K-62
            Some("S-67")
        },
        "lnsg1gsnl/3k2rb1/pppppp1pp/6p2/9/2PP5/PP1SPPPPP/1B5R1/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: R-3b/R-32 b: S-7h/S-78 w: K-6b/K-62
            // b: S-6g/S-67
            Some("S-72")
        },
        "ln1g1gsnl/2sk2rb1/pppppp1pp/6p2/9/2PP5/PP1SPPPPP/1B5R1/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: R-3b/R-32 b: S-7h/S-78 w: K-6b/K-62
            // b: S-6g/S-67 w: S-7b/S-72
            Some("B-77")
        },
        "ln1g1gsnl/2sk2rb1/pppppp1pp/6p2/9/2PP5/PPBSPPPPP/7R1/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: R-3b/R-32 b: S-7h/S-78 w: K-6b/K-62
            // b: S-6g/S-67 w: S-7b/S-72 b: B-7g/B-77
            Some("K-71")
        },
        "lnkg1gsnl/2s3rb1/pppppp1pp/6p2/9/2PP5/PPBSPPPPP/7R1/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: R-3b/R-32 b: S-7h/S-78 w: K-6b/K-62
            // b: S-6g/S-67 w: S-7b/S-72 b: B-7g/B-77 w: K-7a/K-71
            Some("R-88")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/1p4p2/9/2PP5/PP2PPPPP/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84
            let mut rng = rand::thread_rng();
            let mut possible_values = ["R-78", "R-58", "S-78", "R-68"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/1p4p2/9/2PP5/PP2PPPPP/1B1R5/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68
            Some("P-85")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1p7/2PP5/PP2PPPPP/1B1R5/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            Some("B-77")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/3R5/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77
            Some("S-62")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/3R5/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62
            Some("S-78")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2SR5/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78
            Some("K-42")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2SR5/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42
            Some("K-48")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2SR1K3/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42 b: K-4h/K-48
            Some("K-32")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2SR1K3/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            Some("K-38")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2SR2K2/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38
            Some("G61-52")
        },
        "ln3gsnl/1r1sg1kb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2SR2K2/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52
            Some("K-28")
        },
        "ln3gsnl/1r1sg1kb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2SR3K1/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28
            Some("P-54")
        },
        "ln3gsnl/1r1sg1kb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2SR3K1/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54
            Some("S-38")
        },
        "ln3gsnl/1r1sg1kb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2SR2SK1/LN1G1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38
            Some("S-42")
        },
        "ln3g1nl/1r1sgskb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2SR2SK1/LN1G1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: S-4b/S-42
            Some("G69-58")
        },
        "ln3g1nl/1r1sgskb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2SRG1SK1/LN3G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: S-4b/S-42
            // b: G6i-5h/G69-58
            Some("S42-53")
        },
        "ln3g1nl/1r1sg1kb1/p1ppsp1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2SRG1SK1/LN3G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: S-4b/S-42
            // b: G6i-5h/G69-58 w: S4b-5c/S42-53
            Some("P-16")
        },
        "ln3g1nl/1r1sg1kb1/p1ppsp1pp/4p1p2/1p7/2PP4P/PPB1PPPP1/2SRG1SK1/LN3G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-6h/R-68 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-7h/S-78 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: P-5d/P-54 b: S-3h/S-38 w: S-4b/S-42
            // b: G6i-5h/G69-58 w: S4b-5c/S42-53 b: P-1f/P-16
            Some("P-14")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/1p4p2/9/2PP5/PP2PPPPP/1BS4R1/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78
            Some("P-85")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1p7/2PP5/PP2PPPPP/1BS4R1/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            Some("B-77")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2S4R1/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77
            Some("S-62")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2S4R1/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62
            Some("S-67")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/6p2/1p7/2PP5/PPBSPPPPP/7R1/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67
            Some("K-42")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1p7/2PP5/PPBSPPPPP/7R1/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42
            Some("R-88")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1p7/2PP5/PPBSPPPPP/1R7/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42 b: R-8h/R-88
            Some("K-32")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1p7/2PP5/PPBSPPPPP/1R7/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42 b: R-8h/R-88 w: K-3b/K-32
            Some("K-48")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1p7/2PP5/PPBSPPPPP/1R3K3/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42 b: R-8h/R-88 w: K-3b/K-32
            // b: K-4h/K-48
            Some("G61-52")
        },
        "ln3gsnl/1r1sg1kb1/p1pppp1pp/6p2/1p7/2PP5/PPBSPPPPP/1R3K3/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42 b: R-8h/R-88 w: K-3b/K-32
            // b: K-4h/K-48 w: G6a-5b/G61-52
            Some("K-38")
        },
        "ln3gsnl/1r1sg1kb1/p1pppp1pp/6p2/1p7/2PP5/PPBSPPPPP/1R4K2/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42 b: R-8h/R-88 w: K-3b/K-32
            // b: K-4h/K-48 w: G6a-5b/G61-52 b: K-3h/K-38
            Some("P-54")
        },
        "ln3gsnl/1r1sg1kb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPBSPPPPP/1R4K2/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42 b: R-8h/R-88 w: K-3b/K-32
            // b: K-4h/K-48 w: G6a-5b/G61-52 b: K-3h/K-38 w: P-5d/P-54
            Some("K-28")
        },
        "ln3gsnl/1r1sg1kb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPBSPPPPP/1R5K1/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42 b: R-8h/R-88 w: K-3b/K-32
            // b: K-4h/K-48 w: G6a-5b/G61-52 b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28
            Some("S-42")
        },
        "ln3g1nl/1r1sgskb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPBSPPPPP/1R5K1/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42 b: R-8h/R-88 w: K-3b/K-32
            // b: K-4h/K-48 w: G6a-5b/G61-52 b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28 w: S-4b/S-42
            Some("S-38")
        },
        "ln3g1nl/1r1sgskb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPBSPPPPP/1R4SK1/LN1G1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42 b: R-8h/R-88 w: K-3b/K-32
            // b: K-4h/K-48 w: G6a-5b/G61-52 b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28 w: S-4b/S-42
            // b: S-3h/S-38
            Some("S42-53")
        },
        "ln3g1nl/1r1sg1kb1/p1ppsp1pp/4p1p2/1p7/2PP5/PPBSPPPPP/1R4SK1/LN1G1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42 b: R-8h/R-88 w: K-3b/K-32
            // b: K-4h/K-48 w: G6a-5b/G61-52 b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28 w: S-4b/S-42
            // b: S-3h/S-38 w: S4b-5c/S42-53
            Some("P-16")
        },
        "ln3g1nl/1r1sg1kb1/p1ppsp1pp/4p1p2/1p7/2PP4P/PPBSPPPP1/1R4SK1/LN1G1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: S-7h/S-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6g/S-67 w: K-4b/K-42 b: R-8h/R-88 w: K-3b/K-32
            // b: K-4h/K-48 w: G6a-5b/G61-52 b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28 w: S-4b/S-42
            // b: S-3h/S-38 w: S4b-5c/S42-53 b: P-1f/P-16
            Some("P-14")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/1p4p2/9/2PP5/PP2PPPPP/1B2R4/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58
            Some("P-85")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1p7/2PP5/PP2PPPPP/1B2R4/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            Some("B-77")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/4R4/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77
            Some("S-62")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/4R4/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62
            Some("S-68")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/3SR4/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68
            Some("K-42")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/3SR4/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42
            Some("K-48")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/3SRK3/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/K-48
            Some("K-32")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/3SRK3/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            Some("K-38")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/3SR1K2/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38
            Some("G61-52")
        },
        "ln3gsnl/1r1sg1kb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/3SR1K2/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52
            Some("K-28")
        },
        "ln3gsnl/1r1sg1kb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/3SR2K1/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28
            Some("S-42")
        },
        "ln3g1nl/1r1sgskb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/3SR2K1/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: S-4b/S-42
            Some("S-38")
        },
        "ln3g1nl/1r1sgskb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/3SR1SK1/LN1G1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: S-4b/S-42 b: S-3h/S-38
            Some("P-54")
        },
        "ln3g1nl/1r1sgskb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPB1PPPPP/3SR1SK1/LN1G1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: S-4b/S-42 b: S-3h/S-38 w: P-5d/P-54
            Some("P-56")
        },
        "ln3g1nl/1r1sgskb1/p1pp1p1pp/4p1p2/1p7/2PPP4/PPB2PPPP/3SR1SK1/LN1G1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: S-4b/S-42 b: S-3h/S-38 w: P-5d/P-54
            // b: P-5f/P-56
            Some("S42-53")
        },
        "ln3g1nl/1r1sg1kb1/p1ppsp1pp/4p1p2/1p7/2PPP4/PPB2PPPP/3SR1SK1/LN1G1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: S-4b/S-42 b: S-3h/S-38 w: P-5d/P-54
            // b: P-5f/P-56 w: S4b-5c/S-42-53
            Some("P-16")
        },
        "ln3g1nl/1r1sg1kb1/p1ppsp1pp/4p1p2/1p7/2PPP3P/PPB2PPP1/3SR1SK1/LN1G1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-5h/R-58 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/K-48 w: K-3b/K-32
            // b: K-3h/K-38 w: G6a-5b/G61-52 b: K-2h/K-28 w: S-4b/S-42 b: S-3h/S-38 w: P-5d/P-54
            // b: P-5f/P-56 w: S4b-5c/S-42-53 b: P-1f/P-16
            Some("P-14")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/1p4p2/9/2PP5/PP2PPPPP/1BR6/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78
            Some("P-85")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1p7/2PP5/PP2PPPPP/1BR6/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            Some("B-77")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2R6/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77
            Some("S-62")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2R6/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62
            Some("S-68")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2RS5/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68
            Some("K-42")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2RS5/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42
            Some("K-48")
        },
        "ln1g1gsnl/1r1s1k1b1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2RS1K3/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/k-48
            Some("K-32")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2RS1K3/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/k-48 w: K-3b/K-32
            Some("K-38")
        },
        "ln1g1gsnl/1r1s2kb1/p1pppp1pp/6p2/1p7/2PP5/PPB1PPPPP/2RS2K2/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/k-48 w: K-3b/K-32
            // b: K-3h/K-38
            Some("P-54")
        },
        "ln1g1gsnl/1r1s2kb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2RS2K2/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/k-48 w: K-3b/K-32
            // b: K-3h/K-38 w: P-5d/P-54
            Some("K-28")
        },
        "ln1g1gsnl/1r1s2kb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2RS3K1/LN1G1GSNL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/k-48 w: K-3b/K-32
            // b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28
            Some("G61-52")
        },
        "ln3gsnl/1r1sg1kb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2RS3K1/LN1G1GSNL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/k-48 w: K-3b/K-32
            // b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28 w: G6a-5b/G6a-5b
            Some("S-38")
        },
        "ln3gsnl/1r1sg1kb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2RS2SK1/LN1G1G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/k-48 w: K-3b/K-32
            // b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28 w: G6a-5b/G6a-5b b: S-3h/S-38
            Some("S-42")
        },
        "ln3g1nl/1r1sgskb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2RS2SK1/LN1G1G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/k-48 w: K-3b/K-32
            // b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28 w: G6a-5b/G6a-5b b: S-3h/S-38 w: S-4b/S-42
            Some("G69-58")
        },
        "ln3g1nl/1r1sgskb1/p1pp1p1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2RSG1SK1/LN3G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/k-48 w: K-3b/K-32
            // b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28 w: G6a-5b/G6a-5b b: S-3h/S-38 w: S-4b/S-42
            // b: G6i-5h/G69-58
            Some("S42-53")
        },
        "ln3g1nl/1r1sg1kb1/p1ppsp1pp/4p1p2/1p7/2PP5/PPB1PPPPP/2RSG1SK1/LN3G1NL b -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/k-48 w: K-3b/K-32
            // b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28 w: G6a-5b/G6a-5b b: S-3h/S-38 w: S-4b/S-42
            // b: G6i-5h/G69-58 w: S4b-5c/S42-53
            Some("P-16")
        },
        "ln3g1nl/1r1sg1kb1/p1ppsp1pp/4p1p2/1p7/2PP4P/PPB1PPPP1/2RSG1SK1/LN3G1NL w -" => {
            // b: P-7f/P-76 w: P-3d/P-34 b: P-6f/P-66 w: P-8d/P-84 b: R-7h/R-78 w: P-8e/P-85
            // b: B-7g/B-77 w: S-6b/S-62 b: S-6h/S-68 w: K-4b/K-42 b: K-4h/k-48 w: K-3b/K-32
            // b: K-3h/K-38 w: P-5d/P-54 b: K-2h/K-28 w: G6a-5b/G6a-5b b: S-3h/S-38 w: S-4b/S-42
            // b: G6i-5h/G69-58 w: S4b-5c/S42-53 b: P-1f/P-16
            Some("P-14")
        },
        "lnsgkgsnl/1r5b1/p1ppppppp/1p7/9/2P6/PP1PPPPPP/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84
            let mut rng = rand::thread_rng();
            let mut possible_values = ["P-26", "S-68"];
            possible_values.shuffle(&mut rng);
            Some(possible_values[0])
        },
        "lnsgkgsnl/1r5b1/p1ppppppp/1p7/9/2P6/PP1PPPPPP/1B1S3R1/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68
            Some("P-34")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/1p4p2/9/2P6/PP1PPPPPP/1B1S3R1/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34
            Some("P-66")
        },
        "lnsgkgsnl/1r5b1/p1pppp1pp/1p4p2/9/2PP5/PP2PPPPP/1B1S3R1/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66
            Some("S-62")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/1p4p2/9/2PP5/PP2PPPPP/1B1S3R1/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            Some("P-56")
        },
        "ln1gkgsnl/1r1s3b1/p1pppp1pp/1p4p2/9/2PPP4/PP3PPPP/1B1S3R1/LN1GKGSNL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56
            Some("P-54")
        },
        "ln1gkgsnl/1r1s3b1/p1pp1p1pp/1p2p1p2/9/2PPP4/PP3PPPP/1B1S3R1/LN1GKGSNL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54
            Some("S-48")
        },
        "ln1gkgsnl/1r1s3b1/p1pp1p1pp/1p2p1p2/9/2PPP4/PP3PPPP/1B1S1S1R1/LN1GKG1NL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48
            Some("S-42")
        },
        "ln1gkg1nl/1r1s1s1b1/p1pp1p1pp/1p2p1p2/9/2PPP4/PP3PPPP/1B1S1S1R1/LN1GKG1NL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42
            Some("G49-58")
        },
        "ln1gkg1nl/1r1s1s1b1/p1pp1p1pp/1p2p1p2/9/2PPP4/PP3PPPP/1B1SGS1R1/LN1GK2NL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58
            Some("G-32")
        },
        "ln1gk2nl/1r1s1sgb1/p1pp1p1pp/1p2p1p2/9/2PPP4/PP3PPPP/1B1SGS1R1/LN1GK2NL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            Some("G-78")
        },
        "ln1gk2nl/1r1s1sgb1/p1pp1p1pp/1p2p1p2/9/2PPP4/PP3PPPP/1BGSGS1R1/LN2K2NL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            // b: G-7h/G-78
            Some("K-41")
        },
        "ln1g1k1nl/1r1s1sgb1/p1pp1p1pp/1p2p1p2/9/2PPP4/PP3PPPP/1BGSGS1R1/LN2K2NL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            // b: G-7h/G-78 w: K-4a/K-41
            Some("K-69")
        },
        "ln1g1k1nl/1r1s1sgb1/p1pp1p1pp/1p2p1p2/9/2PPP4/PP3PPPP/1BGSGS1R1/LN1K3NL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            // b: G-7h/G-78 w: K-4a/K-41 b: K-6i/K-69
            Some("P-74")
        },
        "ln1g1k1nl/1r1s1sgb1/p2p1p1pp/1pp1p1p2/9/2PPP4/PP3PPPP/1BGSGS1R1/LN1K3NL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            // b: G-7h/G-78 w: K-4a/K-41 b: K-6i/K-69 w: P-7d/P-74
            Some("G58-67")
        },
        "ln1g1k1nl/1r1s1sgb1/p2p1p1pp/1pp1p1p2/9/2PPP4/PP1G1PPPP/1BGS1S1R1/LN1K3NL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            // b: G-7h/G-78 w: K-4a/K-41 b: K-6i/K-69 w: P-7d/P-74 b: G5h-6g/G-58-67
            Some("G-52")
        },
        "ln3k1nl/1r1sgsgb1/p2p1p1pp/1pp1p1p2/9/2PPP4/PP1G1PPPP/1BGS1S1R1/LN1K3NL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            // b: G-7h/G-78 w: K-4a/K-41 b: K-6i/K-69 w: P-7d/P-74 b: G5h-6g/G-58-67 b: G-5b/G/52
            Some("S-77")
        },
        "ln3k1nl/1r1sgsgb1/p2p1p1pp/1pp1p1p2/9/2PPP4/PPSG1PPPP/1BG2S1R1/LN1K3NL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            // b: G-7h/G-78 w: K-4a/K-41 b: K-6i/K-69 w: P-7d/P-74 b: G5h-6g/G-58-67 w: G-5b/G-52
            // b: S-7g/S-77
            Some("S-33")
        },
        "ln3k1nl/1r1sg1gb1/p2p1pspp/1pp1p1p2/9/2PPP4/PPSG1PPPP/1BG2S1R1/LN1K3NL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            // b: G-7h/G-78 w: K-4a/K-41 b: K-6i/K-69 w: P-7d/P-74 b: G5h-6g/G-58-67 w: G-5b/G-52
            // b: S-7g/S-77 w: S-3c/S-33
            Some("B-79")
        },
        "ln3k1nl/1r1sg1gb1/p2p1pspp/1pp1p1p2/9/2PPP4/PPSG1PPPP/2G2S1R1/LNBK3NL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            // b: G-7h/G-78 w: K-4a/K-41 b: K-6i/K-69 w: P-7d/P-74 b: G5h-6g/G-58-67 w: G-5b/G-52
            // b: S-7g/S-77 w: S-3c/S-33 b: B-7i/B-79
            Some("B-31")
        },
        "ln3kbnl/1r1sg1g2/p2p1pspp/1pp1p1p2/9/2PPP4/PPSG1PPPP/2G2S1R1/LNBK3NL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            // b: G-7h/G-78 w: K-4a/K-41 b: K-6i/K-69 w: P-7d/P-74 b: G5h-6g/G-58-67 w: G-5b/G-52
            // b: S-7g/S-77 w: S-3c/S-33 b: B-7i/B-79 w: B-3a/B-31
            Some("P-36")
        },
        "ln3kbnl/1r1sg1g2/p2p1pspp/1pp1p1p2/9/2PPP1P2/PPSG1P1PP/2G2S1R1/LNBK3NL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: S-6h/S-68 w: P-3d/P-34 b: P-6f/P-66 w: S-6b/S-62
            // b; P-5f/P-56 w: P-5d/P-54 b: S-4h/S-48 w: S-4b/S-42 b: G4i-5h/G49-58 w: G-3b/G-32
            // b: G-7h/G-78 w: K-4a/K-41 b: K-6i/K-69 w: P-7d/P-74 b: G5h-6g/G-58-67 w: G-5b/G-52
            // b: S-7g/S-77 w: S-3c/S-33 b: B-7i/B-79 w: B-3a/B-31 b: P-3f/P-36
            Some("P-44")
        },
        "lnsgkgsnl/1r5b1/p1ppppppp/1p7/9/2P4P1/PP1PPPP1P/1B5R1/LNSGKGSNL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: P-2f/P-26
            Some("G-32")
        },
        "lnsgk1snl/1r4gb1/p1ppppppp/1p7/9/2P4P1/PP1PPPP1P/1B5R1/LNSGKGSNL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: P-2f/P-26 w: G-3b/G-32
            Some("G-78")
        },
        "lnsgk1snl/1r4gb1/p1ppppppp/1p7/9/2P4P1/PP1PPPP1P/1BG4R1/LNS1KGSNL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: P-2f/P-26 w: G-3b/G-32 b: G-7h/G-78
            Some("P-85")
        },
        "lnsgk1snl/1r4gb1/p1ppppppp/9/1p7/2P4P1/PP1PPPP1P/1BG4R1/LNS1KGSNL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: P-2f/P-26 w: G-3b/G-32 b: G-7h/G-78 w: P-8e/P-85
            Some("B-77")
        },
        "lnsgk1snl/1r4gb1/p1ppppppp/9/1p7/2P4P1/PPBPPPP1P/2G4R1/LNS1KGSNL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: P-2f/P-26 w: G-3b/G-32 b: G-7h/G-78 w: P-8e/P-85
            // b: B-7g/B-77
            Some("P-34")
        },
        "lnsgk1snl/1r4gb1/p1pppp1pp/6p2/1p7/2P4P1/PPBPPPP1P/2G4R1/LNS1KGSNL b -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: P-2f/P-26 w: G-3b/G-32 b: G-7h/G-78 w: P-8e/P-85
            // b: B-7g/B-77 w: P-3d/P-34
            Some("S-88")
        },
        "lnsgk1snl/1r4gb1/p1pppp1pp/6p2/1p7/2P4P1/PPBPPPP1P/1SG4R1/LN2KGSNL w -" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: P-2f/P-26 w: G-3b/G-32 b: G-7h/G-78 w: P-8e/P-85
            // b: B-7g/B-77 w: P-3d/P-34 b: S-8h/S-88
            Some("Bx77+")
        },
        "lnsgk1snl/1r4g2/p1pppp1pp/6p2/1p7/2P4P1/PPbPPPP1P/1SG4R1/LN2KGSNL b b" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: P-2f/P-26 w: G-3b/G-32 b: G-7h/G-78 w: P-8e/P-85
            // b: B-7g/B-77 w: P-3d/P-34 b: S-8h/S-88 w: Bx7g+/Bx77+
            Some("Sx77+")
        },
        "lnsgk1snl/1r4g2/p1pppp1pp/6p2/1p7/2P4P1/PPSPPPP1P/2G4R1/LN2KGSNL w Bb" => {
            // b: P-7f/P-76 w: P-8d/P-84 b: P-2f/P-26 w: G-3b/G-32 b: G-7h/G-78 w: P-8e/P-85
            // b: B-7g/B-77 w: P-3d/P-34 b: S-8h/S-88 w: Bx7g+/Bx77+ b: Sx7g+/Sx77+
            Some("S-42")
        },
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch() {
        let game_state = String::from("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b -");
        match recommended_move(&game_state)  {
            Some(mov) => {
                let option_a = mov == "P-76";
                let option_b = mov == "P-26";
                let result = option_a || option_b;
                assert_eq!(result, true);
            },
            None => assert!(false, "expected move"),
        }
    }
}
