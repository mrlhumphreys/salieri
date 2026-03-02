use rand::prelude::*;

fn random_move(mut possible_values: Vec<&str>) -> Option<&str> {
    let mut rng = rand::rng();
    possible_values.shuffle(&mut rng);
    Some(possible_values[0])
}

pub fn recommended_move(game_state: &String) -> Option<&'static str> {
    let game_state_string = game_state.as_str();

    match game_state_string {
        // initial state
        "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0" => {
            random_move(vec!["C2=5", "P7+1", "E3+5", "H2+3", "C2=4", "C2=6"])
        },
        // 1. C2=6
        "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C1C5/9/RHEAKAEHR b - - 1 0" => {
            Some("C8=5")
        },
        // 1. C2=6 C8=5
        "rheakaehr/9/4c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C1C5/9/RHEAKAEHR w - - 2 1" => {
            Some("H2+3")
        },
        // 1. C2=6 C8=5
        // 2. H2+3
        "rheakaehr/9/4c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C1C2H2/9/RHEAKAE1R b - - 3 1" => {
            Some("H8+7")
        },
        // 1. C2=6 C8=5
        // 2. H2+3 H8+7
        "r1eakaehr/9/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C1C2H2/9/RHEAKAE1R w - - 4 2" => {
            Some("R1=2")
        },
        // 1. C2=6 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2
        "r1eakaehr/9/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C1C2H2/9/RHEAKAER1 b - - 5 2" => {
            random_move(vec!["R9+1", "P7+1"])
        },
        // 1. C2=6 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        "2eakaehr/r8/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C1C2H2/9/RHEAKAER1 w - - 6 3" => {
            Some("R2+6")
        },
        // 1. C2=6 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. R2+6
        "2eakaehr/r8/2h1c2c1/p1p1p1pRp/9/9/P1P1P1P1P/1C1C2H2/9/RHEAKAE2 b - - 7 3" => {
            Some("R9=4")
        },
        // 1. C2=6 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. R2+6 R9=4
        "2eakaehr/5r3/2h1c2c1/p1p1p1pRp/9/9/P1P1P1P1P/1C1C2H2/9/RHEAKAE2 w - - 8 4" => {
            Some("A4+5")
        },
        // 1. C2=6 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. R2+6 R9=4
        // 5. A4+5
        "2eakaehr/5r3/2h1c2c1/p1p1p1pRp/9/9/P1P1P1P1P/1C1C2H2/4A4/RHEAK1E2 b - - 9 4" => {
            Some("H2+3")
        },
        // 1. C2=4
        "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C3C3/9/RHEAKAEHR b - - 1 0" => {
            Some("C2=5")
        },
        // 1. C2=4 C2=5
        "rheakaehr/9/1c2c4/p1p1p1p1p/9/9/P1P1P1P1P/1C3C3/9/RHEAKAEHR w - - 2 1" => {
            Some("H8+7")
        },
        // 1. C2=4 C2=5
        // 2. H8+7
        "rheakaehr/9/1c2c4/p1p1p1p1p/9/9/P1P1P1P1P/1CH2C3/9/R1EAKAEHR b - - 3 1" => {
            Some("H2+3")
        },
        // 1. C2=4 C2=5
        // 2. H8+7 H2+3
        "rheakae1r/9/1c2c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1CH2C3/9/R1EAKAEHR w - - 4 2" => {
            Some("H2+3")
        },
        // 1. C2=4 C2=5
        // 2. H8+7 H2+3
        // 3. H2+3
        "rheakae1r/9/1c2c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1CH2CH2/9/R1EAKAE1R b - - 5 2" => {
            Some("H8+9")
        },
        // 1. C2=4 C2=5
        // 2. H8+7 H2+3
        // 3. H2+3 H8+9
        "r1eakae1r/9/hc2c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1CH2CH2/9/R1EAKAE1R w - - 6 3" => {
            Some("R1=2")
        },
        // 1. C2=4 C2=5
        // 2. H8+7 H2+3
        // 3. H2+3 H8+9
        // 4. R1=2
        "r1eakae1r/9/hc2c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1CH2CH2/9/R1EAKAER1 b - - 7 3" => {
            Some("R9=8")
        },
        // 1. C2=4 C2=5
        // 2. H8+7 H2+3
        // 3. H2+3 H8+9
        // 4. R1=2 R9=8
        "1reakae1r/9/hc2c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1CH2CH2/9/R1EAKAER1 w - - 8 4" => {
            Some("R9=8")
        },
        // 1. C2=4 C2=5
        // 2. H8+7 H2+3
        // 3. H2+3 H8+9
        // 4. R1=2 R9=8
        // 5. R9=8
        "1reakae1r/9/hc2c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1CH2CH2/9/1REAKAER1 b - - 9 4" => {
            Some("R1=2")
        },
        // 1. C2=4 C2=5
        // 2. H8+7 H2+3
        // 3. H2+3 H8+9
        // 4. R1=2 R9=8
        // 5. R9=8 R1=2
        "1reakaer1/9/hc2c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1CH2CH2/9/1REAKAER1 w - - 10 5" => {
            random_move(vec!["C4+5", "C8+4"])
        },
        // 1. H2+3
        "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C4HC1/9/RHEAKAE1R b - - 1 0" => {
            Some("P7+1")
        },
        // 1. H2+3 P7+1
        "rheakaehr/9/1c5c1/p3p1p1p/2p6/9/P1P1P1P1P/1C4HC1/9/RHEAKAE1R w - - 0 1" => {
            random_move(vec!["P7+1", "C2=1", "C8=6"])
        },
        // 1. H2+3 P7+1
        // 2. C8=6
        "rheakaehr/9/1c5c1/p3p1p1p/2p6/9/P1P1P1P1P/3C2HC1/9/RHEAKAE1R b - - 1 1" => {
            Some("H8+7")
        },
        // 1. H2+3 P7+1
        // 2. C8=6 H8+7
        "r1eakaehr/9/1ch4c1/p3p1p1p/2p6/9/P1P1P1P1P/3C2HC1/9/RHEAKAE1R w - - 2 2" => {
            Some("H8+7")
        },
        // 1. H2+3 P7+1
        // 2. C8=6 H8+7
        // 3. H8+7
        "r1eakaehr/9/1ch4c1/p3p1p1p/2p6/9/P1P1P1P1P/2HC2HC1/9/R1EAKAE1R b - - 3 2" => {
            Some("H2+1")
        },
        // 1. H2+3 P7+1
        // 2. C2=1
        "rheakaehr/9/1c5c1/p3p1p1p/2p6/9/P1P1P1P1P/1C4H1C/9/RHEAKAE1R b - - 1 1" => {
            Some("H8+7")
        },
        // 1. H2+3 P7+1
        // 2. C2=1 H8+7
        "r1eakaehr/9/1ch4c1/p3p1p1p/2p6/9/P1P1P1P1P/1C4H1C/9/RHEAKAE1R w - - 2 2" => {
            Some("R1=2")
        },
        // 1. H2+3 P7+1
        // 2. C2=1 H8+7
        // 3. R1=2
        "r1eakaehr/9/1ch4c1/p3p1p1p/2p6/9/P1P1P1P1P/1C4H1C/9/RHEAKAER1 b - - 3 2" => {
            Some("R9=8")
        },
        // 1. H2+3 P7+1
        // 2. C2=1 H8+7
        // 3. R1=2 R9=8
        "1reakaehr/9/1ch4c1/p3p1p1p/2p6/9/P1P1P1P1P/1C4H1C/9/RHEAKAER1 w - - 4 3" => {
            Some("R2+6")
        },
        // 1. H2+3 P7+1
        // 2. C2=1 H8+7
        // 3. R1=2 R9=8
        // 4. R2+6
        "1reakaehr/9/1ch4c1/p3p1pRp/2p6/9/P1P1P1P1P/1C4H1C/9/RHEAKAE2 b - - 5 3" => {
            Some("C8=9")
        },
        // 1. H2+3 P7+1
        // 2. P7+1
        "rheakaehr/9/1c5c1/p3p1p1p/2p6/2P6/P3P1P1P/1C4HC1/9/RHEAKAE1R b - - 0 1" => {
            Some("H8+7")
        },
        // 1. H2+3 P7+1
        // 2. P7+1 H8+7
        "r1eakaehr/9/1ch4c1/p3p1p1p/2p6/2P6/P3P1P1P/1C4HC1/9/RHEAKAE1R w - - 1 2" => {
            Some("H8+7")
        },
        // 1. H2+3 P7+1
        // 2. P7+1 H8+7
        // 3. H8+7
        "r1eakaehr/9/1ch4c1/p3p1p1p/2p6/2P6/P3P1P1P/1CH3HC1/9/R1EAKAE1R b - - 2 2" => {
            random_move(vec!["R9+1", "H2+3"])
        },
        // 1. E3+5
        "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2E2C1/9/RHEAKA1HR b - - 1 0" => {
            random_move(vec!["P7+1", "C8=5", "C8=4", "C2=4"])
        },
        // 1. E3+5 C2=4
        "rheakaehr/9/1c3c3/p1p1p1p1p/9/9/P1P1P1P1P/1C2E2C1/9/RHEAKA1HR w - - 2 1" => {
            Some("R9+1")
        },
        // 1. E3+5 C2=4
        // 2. R9+1
        "rheakaehr/9/1c3c3/p1p1p1p1p/9/9/P1P1P1P1P/1C2E2C1/R8/1HEAKA1HR b - - 3 1" => {
            Some("H2+3")
        },
        // 1. E3+5 C2=4
        // 2. R9+1 H2+3
        "rheakae1r/9/1c3ch2/p1p1p1p1p/9/9/P1P1P1P1P/1C2E2C1/R8/1HEAKA1HR w - - 4 2" => {
            Some("R9=6")
        },
        // 1. E3+5 C2=4
        // 2. R9+1 H2+3
        // 3. R9=6
        "rheakae1r/9/1c3ch2/p1p1p1p1p/9/9/P1P1P1P1P/1C2E2C1/3R5/1HEAKA1HR b - - 5 2" => {
            Some("H8+7")
        },
        // 1. E3+5 C2=4
        // 2. R9+1 H2+3
        // 3. R9=6 H8+7
        "r1eakae1r/9/1ch2ch2/p1p1p1p1p/9/9/P1P1P1P1P/1C2E2C1/3R5/1HEAKA1HR w - - 6 3" => {
            Some("H8+9")
        },
        // 1. E3+5 C2=4
        // 2. R9+1 H2+3
        // 3. R9=6 H8+7
        // 4. H8+9
        "r1eakae1r/9/1ch2ch2/p1p1p1p1p/9/9/P1P1P1P1P/HC2E2C1/3R5/2EAKA1HR b - - 7 3" => {
            Some("R1=2")
        },
        // 1. E3+5 C2=4
        // 2. R9+1 H2+3
        // 3. R9=6 H8+7
        // 4. H8+9 R1=2
        "r1eakaer1/9/1ch2ch2/p1p1p1p1p/9/9/P1P1P1P1P/HC2E2C1/3R5/2EAKA1HR w - - 8 4" => {
            Some("P9+1")
        },
        // 1. E3+5 C8=4
        "rheakaehr/9/5c1c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2E2C1/9/RHEAKA1HR w - - 2 1" => {
            Some("H2+3")
        },
        // 1. E3+5 C8=4
        // 2. H2+3
        "rheakaehr/9/5c1c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2E1HC1/9/RHEAKA2R b - - 3 1" => {
            Some("H8+7")
        },
        // 1. E3+5 C8=4
        // 2. H2+3 H8+7
        "r1eakaehr/9/2h2c1c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2E1HC1/9/RHEAKA2R w - - 4 2" => {
            Some("R1=2")
        },
        // 1. E3+5 C8=4
        // 2. H2+3 H8+7
        // 3. R1=2
        "r1eakaehr/9/2h2c1c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2E1HC1/9/RHEAKA1R1 b - - 5 2" => {
            random_move(vec!["H2+1", "P7+1"])
        },
        // 1. E3+5 C8=5
        "rheakaehr/9/4c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2E2C1/9/RHEAKA1HR w - - 2 1" => {
            Some("H2+3")
        },
        // 1. E3+5 C8=5
        // 2. H2+3
        "rheakaehr/9/4c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2E1HC1/9/RHEAKA2R b - - 3 1" => {
            Some("H8+7")
        },
        // 1. E3+5 C8=5
        // 2. H2+3 H8+7
        "r1eakaehr/9/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2E1HC1/9/RHEAKA2R w - - 4 2" => {
            Some("R1=2")
        },
        // 1. E3+5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2
        "r1eakaehr/9/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2E1HC1/9/RHEAKA1R1 b - - 5 2" => {
            Some("R9=8")
        },
        // 1. E3+5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9=8
        "1reakaehr/9/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2E1HC1/9/RHEAKA1R1 w - - 6 3" => {
            Some("H8+7")
        },
        // 1. E3+5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9=8
        // 4. H8+7
        "1reakaehr/9/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1CH1E1HC1/9/R1EAKA1R1 b - - 7 3" => {
            random_move(vec!["H2+1", "R8+6", "P7+1"])
        },
        // 1. E3+5 P7+1
        "rheakaehr/9/1c5c1/p3p1p1p/2p6/9/P1P1P1P1P/1C2E2C1/9/RHEAKA1HR w - - 0 1" => {
            random_move(vec!["H8+9", "P7+1"])
        },
        // 1. P7+1
        "rheakaehr/9/1c5c1/p1p1p1p1p/9/2P6/P3P1P1P/1C5C1/9/RHEAKAEHR b - - 0 0" => {
            random_move(vec!["P7+1", "H8+7", "C2=3"])
        },
        // 1. P7+1 C2=3
        "rheakaehr/9/1c4c2/p1p1p1p1p/9/2P6/P3P1P1P/1C5C1/9/RHEAKAEHR w - - 1 1" => {
            Some("C2=5")
        },
        // 1. P7+1 C2=3
        // 2. C2=5
        "rheakaehr/9/1c4c2/p1p1p1p1p/9/2P6/P3P1P1P/1C2C4/9/RHEAKAEHR b - - 2 1" => {
            Some("E3+5")
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        "rheaka1hr/9/1c2e1c2/p1p1p1p1p/9/2P6/P3P1P1P/1C2C4/9/RHEAKAEHR w - - 3 2" => {
            random_move(vec!["A6+5", "H8+9", "H2+3"])
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H2+3
        "rheaka1hr/9/1c2e1c2/p1p1p1p1p/9/2P6/P3P1P1P/1C2C1H2/9/RHEAKAE1R b - - 4 2" => {
            random_move(vec!["R9+1", "P3+1"])
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H2+3 P3+1
        "rheaka1hr/9/1c2e1c2/p1p1p3p/6p2/2P6/P3P1P1P/1C2C1H2/9/RHEAKAE1R w - - 0 3" => {
            Some("R1=2")
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H2+3 P3+1
        // 4. R1=2
        "rheaka1hr/9/1c2e1c2/p1p1p3p/6p2/2P6/P3P1P1P/1C2C1H2/9/RHEAKAER1 b - - 1 3" => {
            Some("P3+1")
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H2+3 P3+1
        // 4. R1=2 P3+1
        "rheaka1hr/9/1c2e1c2/p1p1p3p/9/2P3p2/P3P1P1P/1C2C1H2/9/RHEAKAER1 w - - 0 4" => {
            Some("H8+9")
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H2+3 R9+1
        "1heaka1hr/r8/1c2e1c2/p1p1p1p1p/9/2P6/P3P1P1P/1C2C1H2/9/RHEAKAE1R w - - 5 3" => {
            Some("R1=2")
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H2+3 R9+1
        // 4. R1=2
        "1heaka1hr/r8/1c2e1c2/p1p1p1p1p/9/2P6/P3P1P1P/1C2C1H2/9/RHEAKAER1 b - - 6 3" => {
            Some("R9=2")
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H2+3 R9+1
        // 4. R1=2 R9=2
        "1heaka1hr/7r1/1c2e1c2/p1p1p1p1p/9/2P6/P3P1P1P/1C2C1H2/9/RHEAKAER1 w - - 7 4" => {
            Some("H8+7")
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H2+3 R9+1
        // 4. R1=2 R9=2
        // 5. H8+7
        "1heaka1hr/7r1/1c2e1c2/p1p1p1p1p/9/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 8 4" => {
            Some("H2+4")
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H2+3 R9+1
        // 4. R1=2 R9=2
        // 5. H8+7 H2+4
        "1heaka2r/5h1r1/1c2e1c2/p1p1p1p1p/9/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 w - - 9 5" => {
            Some("C8=9")
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H2+3 R9+1
        // 4. R1=2 R9=2
        // 5. H8+7 H2+4
        // 6. C8=9
        "1heaka2r/5h1r1/1c2e1c2/p1p1p1p1p/9/2P6/P3P1P1P/C1H1C1H2/9/R1EAKAER1 b - - 10 5" => {
            Some("H8+9")
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H2+3 R9+1
        // 4. R1=2 R9=2
        // 5. H8+7 H2+4
        // 6. C8=9 H8+9
        "2eaka2r/5h1r1/hc2e1c2/p1p1p1p1p/9/2P6/P3P1P1P/C1H1C1H2/9/R1EAKAER1 w - - 11 6" => {
            Some("H7+6")
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. H8+9
        "rheaka1hr/9/1c2e1c2/p1p1p1p1p/9/2P6/P3P1P1P/HC2C4/9/R1EAKAEHR b - - 4 2" => {
            random_move(vec!["R9+1", "H8+7"])
        },
        // 1. P7+1 C2=3
        // 2. C2=5 E3+5
        // 3. A6+5
        "rheaka1hr/9/1c2e1c2/p1p1p1p1p/9/2P6/P3P1P1P/1C2C4/4A4/RHE1KAEHR b - - 4 2" => {
            random_move(vec!["P7+1", "H8+7"])
        },
        // 1. P7+1 H8+7
        "r1eakaehr/9/1ch4c1/p1p1p1p1p/9/2P6/P3P1P1P/1C5C1/9/RHEAKAEHR w - - 1 1" => {
            Some("P3+1")
        },
        // 1. P7+1 H8+7
        // 2. P3+1
        "r1eakaehr/9/1ch4c1/p1p1p1p1p/9/2P3P2/P3P3P/1C5C1/9/RHEAKAEHR b - - 0 1" => {
            random_move(vec!["C2=3", "C8=9", "R9+1"])
        },
        // 1. P7+1 P7+1
        "rheakaehr/9/1c5c1/p3p1p1p/2p6/2P6/P3P1P1P/1C5C1/9/RHEAKAEHR w - - 0 1" => {
            random_move(vec!["C2=3", "H8+7"])
        },
        // 1. P7+1 P7+1
        // 2. H8+7
        "rheakaehr/9/1c5c1/p3p1p1p/2p6/2P6/P3P1P1P/1CH4C1/9/R1EAKAEHR b - - 1 1" => {
            Some("H8+7")
        },
        // 1. P7+1 P7+1
        // 2. H8+7 H8+7
        "r1eakaehr/9/1ch4c1/p3p1p1p/2p6/2P6/P3P1P1P/1CH4C1/9/R1EAKAEHR w - - 2 2" => {
           random_move(vec!["R9+1", "C8=9"])
        },
        // 1. P7+1 P7+1
        // 2. C2=3
        "rheakaehr/9/1c5c1/p3p1p1p/2p6/2P6/P3P1P1P/1C4C2/9/RHEAKAEHR b - - 1 1" => {
            Some("C8=5")
        },
        // 1. C2=5
        "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RHEAKAEHR b - - 1 0" => {
            random_move(vec!["H8+7", "H2+3", "C8=5", "C2=5"])
        },
        // 1. C2=5 C2=5
        "rheakaehr/9/1c2c4/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RHEAKAEHR w - - 2 1" => {
            Some("H2+3")
        },
        // 1. C2=5 C2=5
        // 2. H2+3
        "rheakaehr/9/1c2c4/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R b - - 3 1" => {
            random_move(vec!["H2+3", "H8+9"])
        },
        // 1. C2=5 C2=5
        // 2. H2+3 H8+9
        "r1eakaehr/9/hc2c4/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R w - - 4 2" => {
            Some("R1=2")
        },
        // 1. C2=5 C2=5
        // 2. H2+3 H8+9
        // 3. R1=2
        "r1eakaehr/9/hc2c4/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 b - - 5 2" => {
            Some("R9=8")
        },
        // 1. C2=5 C2=5
        // 2. H2+3 H8+9
        // 3. R1=2 R9=8
        "1reakaehr/9/hc2c4/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 6 3" => {
            random_move(vec!["H8+9", "H8+7"])
        },
        // 1. C2=5 C2=5
        // 2. H2+3 H2+3
        "rheakae1r/9/1c2c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R w - - 4 2" => {
            Some("R1=2")
        },
        // 1. C2=5 C2=5
        // 2. H2+3 H2+3
        // 3. R1=2
        "rheakae1r/9/1c2c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 b - - 5 2" => {
            random_move(vec!["R1=2", "H8+7"])
        },
        // 1. C2=5 C2=5
        // 2. H2+3 H2+3
        // 3. R1=2 H8+7
        "r1eakae1r/9/1ch1c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 6 3" => {
            Some("C8=6")
        },
        // 1. C2=5 C2=5
        // 2. H2+3 H2+3
        // 3. R1=2 R1=2
        "rheakaer1/9/1c2c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 6 3" => {
            Some("H8+7")
        },
        // 1. C2=5 C2=5
        // 2. H2+3 H2+3
        // 3. R1=2 R1=2
        // 4. H8+7
        "rheakaer1/9/1c2c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 7 3" => {
            Some("H8+7")
        },
        // 1. C2=5 C2=5
        // 2. H2+3 H2+3
        // 3. R1=2 R1=2
        // 4. H8+7 H8+7
        "r1eakaer1/9/1ch1c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAER1 w - - 8 4" => {
            Some("R9=8")
        },
        // 1. C2=5 C2=5
        // 2. H2+3 H2+3
        // 3. R1=2 R1=2
        // 4. H8+7 H8+7
        // 5. R9=8
        "r1eakaer1/9/1ch1c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1CH1C1H2/9/1REAKAER1 b - - 9 4" => {
            Some("R9=8")
        },
        // 1. C2=5 C2=5
        // 2. H2+3 H2+3
        // 3. R1=2 R1=2
        // 4. H8+7 H8+7
        // 5. R9=8 R9=8
        "1reakaer1/9/1ch1c1h2/p1p1p1p1p/9/9/P1P1P1P1P/1CH1C1H2/9/1REAKAER1 w - - 10 5" => {
            Some("C8+4")
        },
        // 1. C2=5 C8=5
        "rheakaehr/9/4c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RHEAKAEHR w - - 2 1" => {
            Some("H2+3")
        },
        // 1. C2=5 C8=5
        // 2. H2+3
        "rheakaehr/9/4c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R b - - 3 1" => {
            Some("H8+7")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        "r1eakaehr/9/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R w - - 4 2" => {
            random_move(vec!["P3+1", "R1+1", "R1=2"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2
        "r1eakaehr/9/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 b - - 5 2" => {
            random_move(vec!["P7+1", "R9+1"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        "2eakaehr/r8/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 6 3" => {
            random_move(vec!["R2+6", "C8=6", "H8+7"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7
        "2eakaehr/r8/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 7 3" => {
           random_move(vec!["H2+3", "R9=4"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        "2eakaehr/5r3/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAER1 w - - 8 4" => {
            random_move(vec!["P7+1", "P3+1"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P3+1
        "2eakaehr/5r3/2h1c2c1/p1p1p1p1p/9/6P2/P1P1P3P/1CH1C1H2/9/R1EAKAER1 b - - 0 4" => {
            random_move(vec!["R4+4", "P3+1", "H2+1", "H2+3"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P3+1 H2+3
        "2eakae1r/5r3/2h1c1hc1/p1p1p1p1p/9/6P2/P1P1P3P/1CH1C1H2/9/R1EAKAER1 w - - 1 5" => {
            Some("P7+1")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P3+1 H2+3
        // 6. P7+1
        "2eakae1r/5r3/2h1c1hc1/p1p1p1p1p/9/2P3P2/P3P3P/1CH1C1H2/9/R1EAKAER1 b - - 0 5" => {
            Some("R1+1")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P3+1 H2+3
        // 6. P7+1 R1+1
        "2eakae2/5r2r/2h1c1hc1/p1p1p1p1p/9/2P3P2/P3P3P/1CH1C1H2/9/R1EAKAER1 w - - 1 6" => {
            Some("A6+5")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P3+1 H2+3
        // 6. P7+1 R1+1
        // 7. A6+5
        "2eakae2/5r2r/2h1c1hc1/p1p1p1p1p/9/2P3P2/P3P3P/1CH1C1H2/4A4/R1E1KAER1 b - - 2 6" => {
            Some("R4+5")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P3+1 H2+3
        // 6. P7+1 R1+1
        // 7. A6+5 R4+5
        "2eakae2/8r/2h1c1hc1/p1p1p1p1p/9/2P3P2/P3Pr2P/1CH1C1H2/4A4/R1E1KAER1 w - - 3 7" => {
            Some("E7+9")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P3+1 H2+1
        "2eakae1r/5r3/2h1c2ch/p1p1p1p1p/9/6P2/P1P1P3P/1CH1C1H2/9/R1EAKAER1 w - - 1 5" => {
            Some("H3+4")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P3+1 P3+1
        "2eakaehr/5r3/2h1c2c1/p1p1p3p/6p2/6P2/P1P1P3P/1CH1C1H2/9/R1EAKAER1 w - - 0 5" => {
            Some("R2+5")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P3+1 P3+1
        // 6. R2+5
        "2eakaehr/5r3/2h1c2c1/p1p1p3p/6pR1/6P2/P1P1P3P/1CH1C1H2/9/R1EAKAE2 b - - 1 5" => {
            Some("E3+1")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P3+1 P3+1
        // 6. R2+5 E3+1
        "2eaka1hr/5r3/2h1c2ce/p1p1p3p/6pR1/6P2/P1P1P3P/1CH1C1H2/9/R1EAKAE2 w - - 2 6" => {
            Some("C8=9")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P3+1 R4+4
        "2eakaehr/9/2h1c2c1/p1p1p1p1p/9/5rP2/P1P1P3P/1CH1C1H2/9/R1EAKAER1 w - - 1 5" => {
            random_move(vec!["C5=4", "E3+1"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 R9=4
        // 5. P7+1
        "2eakaehr/5r3/2h1c2c1/p1p1p1p1p/9/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 0 4" => {
            random_move(vec!["H2+1", "H2+3", "R4+5"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 H2+3
        "2eakae1r/r8/2h1c1hc1/p1p1p1p1p/9/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAER1 w - - 8 4" => {
            Some("P7+1")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 H2+3
        // 5. P7+1
        "2eakae1r/r8/2h1c1hc1/p1p1p1p1p/9/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 0 4" => {
            Some("R1+1")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. H8+7 H2+3
        // 5. P7+1 R1+1
        "2eakae2/r7r/2h1c1hc1/p1p1p1p1p/9/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 w - - 1 5" => {
            random_move(vec!["P3+1", "C8+1"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6
        "2eakaehr/r8/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/3CC1H2/9/RHEAKAER1 b - - 7 3" => {
            random_move(vec!["H2+3", "R9=4"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 R9=4
        "2eakaehr/5r3/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/3CC1H2/9/RHEAKAER1 w - - 8 4" => {
            Some("A4+5")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 R9=4
        // 5. A4+5
        "2eakaehr/5r3/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/3CC1H2/4A4/RHEAK1ER1 b - - 9 4" => {
            Some("H2+1")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 R9=4
        // 5. A4+5 H2+1
        "2eakae1r/5r3/2h1c2ch/p1p1p1p1p/9/9/P1P1P1P1P/3CC1H2/4A4/RHEAK1ER1 w - - 10 5" => {
            Some("H8+7")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 R9=4
        // 5. A4+5 H2+1
        // 6. H8+7
        "2eakae1r/5r3/2h1c2ch/p1p1p1p1p/9/9/P1P1P1P1P/2HCC1H2/4A4/R1EAK1ER1 b - - 11 5" => {
            Some("R1=2")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 R9=4
        // 5. A4+5 H2+1
        // 6. H8+7 R1=2
        "2eakaer1/5r3/2h1c2ch/p1p1p1p1p/9/9/P1P1P1P1P/2HCC1H2/4A4/R1EAK1ER1 w - - 12 6" => {
            Some("R9=8")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 R9=4
        // 5. A4+5 H2+1
        // 6. H8+7 R1=2
        // 7. R9=8
        "2eakaer1/5r3/2h1c2ch/p1p1p1p1p/9/9/P1P1P1P1P/2HCC1H2/4A4/1REAK1ER1 b - - 13 6" => {
            Some("R4+5")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 R9=4
        // 5. A4+5 H2+1
        // 6. H8+7 R1=2
        // 7. R9=8 R4+5
        "2eakaer1/9/2h1c2ch/p1p1p1p1p/9/9/P1P1PrP1P/2HCC1H2/4A4/1REAK1ER1 w - - 14 7" => {
            Some("R2+6")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 R9=4
        // 5. A4+5 H2+1
        // 6. H8+7 R1=2
        // 7. R9=8 R4+5
        // 8. R2+6
        "2eakaer1/9/2h1c2ch/p1p1p1pRp/9/9/P1P1PrP1P/2HCC1H2/4A4/1REAK1E2 b - - 15 7" => {
            Some("C5=3")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 H2+3
        "2eakae1r/r8/2h1c1hc1/p1p1p1p1p/9/9/P1P1P1P1P/3CC1H2/9/RHEAKAER1 w - - 8 4" => {
            Some("H8+7")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 H2+3
        // 5. H8+7
        "2eakae1r/r8/2h1c1hc1/p1p1p1p1p/9/9/P1P1P1P1P/2HCC1H2/9/R1EAKAER1 b - - 9 4" => {
            Some("R1=2")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 H2+3
        // 5. H8+7 R1=2
        "2eakaer1/r8/2h1c1hc1/p1p1p1p1p/9/9/P1P1P1P1P/2HCC1H2/9/R1EAKAER1 w - - 10 5" => {
            Some("R9=8")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 H2+3
        // 5. H8+7 R1=2
        // 6. R9=8
        "2eakaer1/r8/2h1c1hc1/p1p1p1p1p/9/9/P1P1P1P1P/2HCC1H2/9/1REAKAER1 b - - 11 5" => {
            Some("C2+4")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 H2+3
        // 5. H8+7 R1=2
        // 6. R9=8 C2+4
        "2eakaer1/r8/2h1c1h2/p1p1p1p1p/9/9/P1P1P1PcP/2HCC1H2/9/1REAKAER1 w - - 12 6" => {
            Some("C6+5")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. C8=6 H2+3
        // 5. H8+7 R1=2
        // 6. R9=8 C2+4
        // 7. C6+5
        "2eakaer1/r8/2hCc1h2/p1p1p1p1p/9/9/P1P1P1PcP/2H1C1H2/9/1REAKAER1 b - - 13 6" => {
            Some("R9=7")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. R2+6
        "2eakaehr/r8/2h1c2c1/p1p1p1pRp/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE2 b - - 7 3" => {
            Some("P3+1")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. R2+6 P3+1
        "2eakaehr/r8/2h1c2c1/p1p1p2Rp/6p2/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE2 w - - 0 4" => {
            random_move(vec!["R2=3", "C8=7"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. R2+6 P3+1
        // 5. C8=7
        "2eakaehr/r8/2h1c2c1/p1p1p2Rp/6p2/9/P1P1P1P1P/2C1C1H2/9/RHEAKAE2 b - - 1 4" => {
            Some("H2+3")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. R2+6 P3+1
        // 5. C8=7 H2+3
        "2eakae1r/r8/2h1c1hc1/p1p1p2Rp/6p2/9/P1P1P1P1P/2C1C1H2/9/RHEAKAE2 w - - 2 5" => {
            Some("P7+1")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. R2+6 P3+1
        // 5. C8=7 H2+3
        // 6. P7+1
        "2eakae1r/r8/2h1c1hc1/p1p1p2Rp/6p2/2P6/P3P1P1P/2C1C1H2/9/RHEAKAE2 b - - 0 5" => {
            Some("H3+4")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 R9+1
        // 4. R2+6 P3+1
        // 5. R2=3
        "2eakaehr/r8/2h1c2c1/p1p1p1R1p/6p2/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE2 b - - 1 4" => {
            Some("H2+3")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        "r1eakaehr/9/2h1c2c1/p3p1p1p/2p6/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 0 3" => {
            random_move(vec!["H8+7", "P7+1"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        // 4. P7+1
        "r1eakaehr/9/2h1c2c1/p3p1p1p/2p6/2P6/P3P1P1P/1C2C1H2/9/RHEAKAER1 b - - 0 3" => {
            random_move(vec!["C2+4", "R9+1"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        // 4. P7+1 R9+1
        "2eakaehr/r8/2h1c2c1/p3p1p1p/2p6/2P6/P3P1P1P/1C2C1H2/9/RHEAKAER1 w - - 1 4" => {
            Some("H8+7")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        // 4. P7+1 R9+1
        // 5. H8+7
        "2eakaehr/r8/2h1c2c1/p3p1p1p/2p6/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 2 4" => {
            Some("R9=4")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        // 4. P7+1 R9+1
        // 5. H8+7 R9=4
        "2eakaehr/5r3/2h1c2c1/p3p1p1p/2p6/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 w - - 3 5" => {
            Some("R2+4")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        // 4. P7+1 R9+1
        // 5. H8+7 R9=4
        // 6. R2+4
        "2eakaehr/5r3/2h1c2c1/p3p1p1p/2p6/2P4R1/P3P1P1P/1CH1C1H2/9/R1EAKAE2 b - - 4 5" => {
            Some("H2+3")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        // 4. P7+1 R9+1
        // 5. H8+7 R9=4
        // 6. R2+4 H2+3
        "2eakae1r/5r3/2h1c1hc1/p3p1p1p/2p6/2P4R1/P3P1P1P/1CH1C1H2/9/R1EAKAE2 w - - 5 6" => {
            Some("P3+1")
        },

        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        // 4. P7+1 C2+4
        "r1eakaehr/9/2h1c4/p3p1p1p/2p6/2P6/P3P1PcP/1C2C1H2/9/RHEAKAER1 w - - 1 4" => {
            Some("H8+7")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        // 4. P7+1 C2+4
        // 5. H8+7
        "r1eakaehr/9/2h1c4/p3p1p1p/2p6/2P6/P3P1PcP/1CH1C1H2/9/R1EAKAER1 b - - 2 4" => {
            random_move(vec!["H2+3", "R9+1"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        // 4. H8+7
        "r1eakaehr/9/2h1c2c1/p3p1p1p/2p6/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 1 3" => {
            Some("H2+3")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        // 4. H8+7 H2+3
        "r1eakae1r/9/2h1c1hc1/p3p1p1p/2p6/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAER1 w - - 2 4" => {
            Some("P7+1")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1=2 P7+1
        // 4. H8+7 H2+3
        // 5. P7+1
        "r1eakae1r/9/2h1c1hc1/p3p1p1p/2p6/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 0 4" => {
            Some("R1+1")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1+1
        "r1eakaehr/9/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/8R/RHEAKAE2 b - - 5 2" => {
            Some("R9=8")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1+1 R9=8
        "1reakaehr/9/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/8R/RHEAKAE2 w - - 6 3" => {
            Some("R1=6")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1+1 R9=8
        // 4. R1=6
        "1reakaehr/9/2h1c2c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/3R5/RHEAKAE2 b - - 7 3" => {
            Some("R8+4")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1+1 R9=8
        // 4. R1=6 R8+4
        "2eakaehr/9/2h1c2c1/p1p1p1p1p/1r7/9/P1P1P1P1P/1C2C1H2/3R5/RHEAKAE2 w - - 8 4" => {
            Some("H8+7")
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. R1+1 R9=8
        // 4. R1=6 R8+4
        // 5. H8+7
        "2eakaehr/9/2h1c2c1/p1p1p1p1p/1r7/9/P1P1P1P1P/1CH1C1H2/3R5/R1EAKAE2 b - - 9 4" => {
            random_move(vec!["H2+3", "A6+5"])
        },
        // 1. C2=5 C8=5
        // 2. H2+3 H8+7
        // 3. P3+1
        "r1eakaehr/9/2h1c2c1/p1p1p1p1p/9/6P2/P1P1P3P/1C2C1H2/9/RHEAKAE1R b - - 0 2" => {
            random_move(vec!["R9=8", "R9+1"])
        },
        // 1.C2=5  H2+3
        "rheakae1r/9/1c4hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RHEAKAEHR w - - 2 1" => {
            Some("H2+3")
        },
        // 1. C2=5 H2+3
        // 2. H2+3
        "rheakae1r/9/1c4hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R b - - 3 1" => {
            random_move(vec!["C8=6", "R9+1", "P7+1"])
        },
        // 1. C2=5 H2+3
        // 2. H2+3 P7+1
        "rheakae1r/9/1c4hc1/p3p1p1p/2p6/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R w - - 0 2" => {
            Some("R1=2")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 P7+1
        // 3. R1=2
        "rheakae1r/9/1c4hc1/p3p1p1p/2p6/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 b - - 1 2" => {
            Some("R9+2")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 P7+1
        // 3. R1=2 R9+2
        "1heakae1r/9/rc4hc1/p3p1p1p/2p6/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 2 3" => {
            Some("H8+7")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 P7+1
        // 3. R1=2 R9+2
        // 4. H8+7
        "1heakae1r/9/rc4hc1/p3p1p1p/2p6/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 3 3" => {
            Some("C2-1")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 P7+1
        // 3. R1=2 R9+2
        // 4. H8+7 C2-1
        "1heakae1r/7c1/rc4h2/p3p1p1p/2p6/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAER1 w - - 4 4" => {
            Some("R2+6")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 R9+1
        "1heakae1r/r8/1c4hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R w - - 4 2" => {
            Some("R1=2")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 R9+1
        // 3. R1=2
        "1heakae1r/r8/1c4hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 b - - 5 2" => {
           Some("H8+9")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 R9+1
        // 3. R1=2 H8+9
        "2eakae1r/r8/hc4hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 6 3" => {
            Some("P7+1")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 R9+1
        // 3. R1=2 H8+9
        // 4. P7+1
        "2eakae1r/r8/hc4hc1/p1p1p1p1p/9/2P6/P3P1P1P/1C2C1H2/9/RHEAKAER1 b - - 0 3" => {
            Some("C8=7")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 R9+1
        // 3. R1=2 H8+9
        // 4. P7+1 C8=7
        "2eakae1r/r8/h1c3hc1/p1p1p1p1p/9/2P6/P3P1P1P/1C2C1H2/9/RHEAKAER1 w - - 1 4" => {
            Some("H8+7")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 R9+1
        // 3. R1=2 H8+9
        // 4. P7+1 C8=7
        // 5. H8+7
        "2eakae1r/r8/h1c3hc1/p1p1p1p1p/9/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 2 4" => {
            random_move(vec!["E3+5", "R9=4"])
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        "rheakae1r/9/3c2hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R w - - 4 2" => {
            random_move(vec!["R1+1", "R1=2"])
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2
        "rheakae1r/9/3c2hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 b - - 5 2" => {
            Some("H8+7")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        "r1eakae1r/9/2hc2hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 6 3" => {
            random_move(vec!["C8=6", "P7+1", "P3+1"])
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. P3+1
        "r1eakae1r/9/2hc2hc1/p1p1p1p1p/9/6P2/P1P1P3P/1C2C1H2/9/RHEAKAER1 b - - 0 3" => {
            Some("P3+1")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. P3+1 P3+1
        "r1eakae1r/9/2hc2hc1/p1p1p3p/6p2/6P2/P1P1P3P/1C2C1H2/9/RHEAKAER1 w - - 0 4" => {
            Some("H8+9")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. P3+1 P3+1
        // 5. H8+9
        "r1eakae1r/9/2hc2hc1/p1p1p3p/6p2/6P2/P1P1P3P/HC2C1H2/9/R1EAKAER1 b - - 1 4" => {
            random_move(vec!["E3+5", "E7+5"])
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. P3+1 P3+1
        // 5. H8+9 E7+5
        "r2akae1r/9/2hce1hc1/p1p1p3p/6p2/6P2/P1P1P3P/HC2C1H2/9/R1EAKAER1 w - - 2 5" => {
            random_move(vec!["C8=6", "C8=7"])
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. P3+1 P3+1
        // 5. H8+9 E3+5
        "r1eaka2r/9/2hce1hc1/p1p1p3p/6p2/6P2/P1P1P3P/HC2C1H2/9/R1EAKAER1 w - - 2 5" => {
            random_move(vec!["C8=6", "C8=7"])
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. P7+1
        "r1eakae1r/9/2hc2hc1/p1p1p1p1p/9/2P6/P3P1P1P/1C2C1H2/9/RHEAKAER1 b - - 0 3" => {
            Some("P7+1")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. P7+1 P7+1
        "r1eakae1r/9/2hc2hc1/p3p1p1p/2p6/2P6/P3P1P1P/1C2C1H2/9/RHEAKAER1 w - - 0 4" => {
            Some("R2+6")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. P7+1 P7+1
        // 5. R2+6
        "r1eakae1r/9/2hc2hc1/p3p1pRp/2p6/2P6/P3P1P1P/1C2C1H2/9/RHEAKAE2 b - - 1 4" => {
            random_move(vec!["R9+2", "A4+5"])
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. C8=6
        "r1eakae1r/9/2hc2hc1/p1p1p1p1p/9/9/P1P1P1P1P/3CC1H2/9/RHEAKAER1 b - - 7 3" => {
            Some("R1=2")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. C8=6 R1=2
        "r1eakaer1/9/2hc2hc1/p1p1p1p1p/9/9/P1P1P1P1P/3CC1H2/9/RHEAKAER1 w - - 8 4" => {
            Some("H8+7")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. C8=6 R1=2
        // 5. H8+7
        "r1eakaer1/9/2hc2hc1/p1p1p1p1p/9/9/P1P1P1P1P/2HCC1H2/9/R1EAKAER1 b - - 9 4" => {
            Some("C2=1")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. C8=6 R1=2
        // 5. H8+7 C2=1
        "r1eakaer1/9/2hc2h1c/p1p1p1p1p/9/9/P1P1P1P1P/2HCC1H2/9/R1EAKAER1 w - - 10 5" => {
            Some("P7+1")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. C8=6 R1=2
        // 5. H8+7 C2=1
        // 6. P7+1
        "r1eakaer1/9/2hc2h1c/p1p1p1p1p/9/2P6/P3P1P1P/2HCC1H2/9/R1EAKAER1 b - - 0 5" => {
            Some("P7+1")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. C8=6 R1=2
        // 5. H8+7 C2=1
        // 6. P7+1 P7+1
        "r1eakaer1/9/2hc2h1c/p3p1p1p/2p6/2P6/P3P1P1P/2HCC1H2/9/R1EAKAER1 w - - 0 6" => {
            Some("H7+6")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1=2 H8+7
        // 4. C8=6 R1=2
        // 5. H8+7 C2=1
        // 6. P7+1 P7+1
        // 7. H7+6
        "r1eakaer1/9/2hc2h1c/p3p1p1p/2p6/2PH5/P3P1P1P/3CC1H2/9/R1EAKAER1 b - - 1 6" => {
            Some("A6+5")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1+1
        "rheakae1r/9/3c2hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/8R/RHEAKAE2 b - - 5 2" => {
            Some("H8+7")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1+1 H8+7 *
        "r1eakae1r/9/2hc2hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/8R/RHEAKAE2 w - - 6 3" => {
            Some("R1=4")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1+1 H8+7
        // 4. R1=4
        "r1eakae1r/9/2hc2hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/5R3/RHEAKAE2 b - - 7 3" => {
            Some("R9=8")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1+1 H8+7
        // 4. R1=4 R9=8
        "1reakae1r/9/2hc2hc1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/5R3/RHEAKAE2 w - - 8 4" => {
            Some("H8+7")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1+1 H8+7
        // 4. R1=4 R9=8
        // 5. H8+7
        "1reakae1r/9/2hc2hc1/p1p1p1p1p/9/9/P1P1P1P1P/1CH1C1H2/5R3/R1EAKAE2 b - - 9 4" => {
            Some("A4+5")
        },
        // 1. C2=5 H2+3
        // 2. H2+3 C8=6
        // 3. R1+1 H8+7
        // 4. R1=4 R9=8
        // 5. H8+7 A4+5
        "1reak1e1r/4a4/2hc2hc1/p1p1p1p1p/9/9/P1P1P1P1P/1CH1C1H2/5R3/R1EAKAE2 w - - 10 5" => {
            random_move(vec!["P5+1", "R9+1"])
        },
        // 1. C2=5 H8+7
        "r1eakaehr/9/1ch4c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RHEAKAEHR w - - 2 1" => {
            Some("H2+3")
        },
        // 1. C2=5 H8+7
        // 2. H2+3
        "r1eakaehr/9/1ch4c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R b - - 3 1" => {
            random_move(vec!["R9=8", "R9+1"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9+1
        "2eakaehr/r8/1ch4c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R w - - 4 2" => {
            Some("R1=2")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9+1
        // 3. R1=2
        "2eakaehr/r8/1ch4c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 b - - 5 2" => {
            Some("C8-1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9+1
        // 3. R1=2 C8-1
        "2eakaehr/rc7/2h4c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 6 3" => {
            random_move(vec!["H8+9", "H8+7"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9+1
        // 3. R1=2 C8-1
        // 4. H8+7
        "2eakaehr/rc7/2h4c1/p1p1p1p1p/9/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 7 3" => {
            Some("P3+1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9+1
        // 3. R1=2 C8-1
        // 4. H8+9
        "2eakaehr/rc7/2h4c1/p1p1p1p1p/9/9/P1P1P1P1P/HC2C1H2/9/R1EAKAER1 b - - 7 3" => {
            Some("E3+5")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        "1reakaehr/9/1ch4c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE1R w - - 4 2" => {
            random_move(vec!["P7+1", "R1=2"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. P7+1
        "1reakaehr/9/1ch4c1/p1p1p1p1p/9/2P6/P3P1P1P/1C2C1H2/9/RHEAKAE1R b - - 0 2" => {
            random_move(vec!["P7+1", "C8=9"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. P7+1 C8=9
        "1reakaehr/9/c1h4c1/p1p1p1p1p/9/2P6/P3P1P1P/1C2C1H2/9/RHEAKAE1R w - - 1 3" => {
            Some("H8+7")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. P7+1 C8=9
        // 4. H8+7
        "1reakaehr/9/c1h4c1/p1p1p1p1p/9/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAE1R b - - 2 3" => {
            random_move(vec!["C2=5", "P7+1", "R8+5"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. P7+1 C8=9
        // 4. H8+7 R8+5
        "2eakaehr/9/c1h4c1/p1p1p1p1p/9/1rP6/P3P1P1P/1CH1C1H2/9/R1EAKAE1R w - - 3 4" => {
            random_move(vec!["P5+1", "E7+9"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. P7+1 C8=9
        // 4. H8+7 P7+1
        "1reakaehr/9/c1h4c1/p3p1p1p/2p6/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAE1R w - - 0 4" => {
            random_move(vec!["R1+1", "C8+2", "H7+6"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. P7+1 C8=9
        // 4. H8+7 C2=5
        "1reakaehr/9/c1h1c4/p1p1p1p1p/9/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAE1R w - - 3 4" => {
            Some("R9=8")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. P7+1 C8=9
        // 4. H8+7 C2=5
        // 5. R9=8
        "1reakaehr/9/c1h1c4/p1p1p1p1p/9/2P6/P3P1P1P/1CH1C1H2/9/1REAKAE1R b - - 4 4" => {
            Some("H2+3")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. P7+1 C8=9
        // 4. H8+7 C2=5
        // 5. R9=8 H2+3
        "1reakae1r/9/c1h1c1h2/p1p1p1p1p/9/2P6/P3P1P1P/1CH1C1H2/9/1REAKAE1R w - - 5 5" => {
            random_move(vec!["P3+1", "R1+1"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. P7+1 P7+1
        "1reakaehr/9/1ch4c1/p3p1p1p/2p6/2P6/P3P1P1P/1C2C1H2/9/RHEAKAE1R w - - 0 3" => {
            Some("H8+7")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. P7+1 P7+1
        // 4. H8+7
        "1reakaehr/9/1ch4c1/p3p1p1p/2p6/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAE1R b - - 1 3" => {
            Some("H2+3")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. P7+1 P7+1
        // 4. H8+7 H2+3
        "1reakae1r/9/1ch3hc1/p3p1p1p/2p6/2P6/P3P1P1P/1CH1C1H2/9/R1EAKAE1R w - - 2 4" => {
            random_move(vec!["R1+1", "C8+2", "C8=9"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2
        "1reakaehr/9/1ch4c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 b - - 5 2" => {
            random_move(vec!["P7+1", "C2=5", "C8+4"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        "1reakaehr/9/2h4c1/p1p1p1p1p/9/9/PcP1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 6 3" => {
            Some("P3+1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1
        "1reakaehr/9/2h4c1/p1p1p1p1p/9/6P2/PcP1P3P/1C2C1H2/9/RHEAKAER1 b - - 0 3" => {
            Some("C2=5")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        "1reakaehr/9/2h1c4/p1p1p1p1p/9/6P2/PcP1P3P/1C2C1H2/9/RHEAKAER1 w - - 1 4" => {
            Some("H8+7")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7
        "1reakaehr/9/2h1c4/p1p1p1p1p/9/6P2/PcP1P3P/1CH1C1H2/9/R1EAKAER1 b - - 2 4" => {
            random_move(vec!["R1+1", "H2+3"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 H2+3
        "1reakae1r/9/2h1c1h2/p1p1p1p1p/9/6P2/PcP1P3P/1CH1C1H2/9/R1EAKAER1 w - - 3 5" => {
            random_move(vec!["P7+1", "R9=8"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 H2+3
        // 6. R9=8
        "1reakae1r/9/2h1c1h2/p1p1p1p1p/9/6P2/PcP1P3P/1CH1C1H2/9/1REAKAER1 b - - 4 5" => {
            Some("P3+1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 H2+3
        // 6. R9=8 P3+1
        "1reakae1r/9/2h1c1h2/p1p1p3p/9/6p2/PcP1P3P/1CH1C1H2/9/1REAKAER1 w - - 0 6" => {
            Some("H3+4")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 H2+3
        // 6. R9=8 P3+1
        // 7. H3+4
        "1reakae1r/9/2h1c1h2/p1p1p3p/9/5Hp2/PcP1P3P/1CH1C13/9/1REAKAER1 b - - 1 6" => {
            random_move(vec!["R1=2", "R1+1"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 H2+3
        // 6. P7+1
        "1reakae1r/9/2h1c1h2/p1p1p1p1p/9/2P3P2/Pc2P3P/1CH1C1H2/9/R1EAKAER1 b - - 0 5" => {
            Some("R1=2")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 H2+3
        // 6. P7+1 R1=2
        "1reakaer1/9/2h1c1h2/p1p1p1p1p/9/2P3P2/Pc2P3P/1CH1C1H2/9/R1EAKAER1 w - - 1 6" => {
            Some("R9=8")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 H2+3
        // 6. P7+1 R1=2
        // 7. R9=8
        "1reakaer1/9/2h1c1h2/p1p1p1p1p/9/2P3P2/Pc2P3P/1CH1C1H2/9/1REAKAER1 b - - 2 6" => {
            random_move(vec!["R2+6", "R2+4"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 H2+3
        // 6. P7+1 R1=2
        // 7. R9=8 R2+4
        "1reakae2/9/2h1c1h2/p1p1p1p1p/7r1/2P3P2/Pc2P3P/1CH1C1H2/9/1REAKAER1 w - - 3 7" => {
            Some("C8=9")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 H2+3
        // 6. P7+1 R1=2
        // 7. R9=8 R2+4
        // 8. C8=9
        "1reakae2/9/2h1c1h2/p1p1p1p1p/7r1/2P3P2/Pc2P3P/C1H1C1H2/9/1REAKAER1 b - - 4 7" => {
            Some("R2=8")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 H2+3
        // 6. P7+1 R1=2
        // 7. R9=8 R2+6
        "1reakae2/9/2h1c1h2/p1p1p1p1p/9/2P3P2/Pc2P2rP/1CH1C1H2/9/1REAKAER1 w - - 3 7" => {
            random_move(vec!["H7+6", "C8=9"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 R1+1
        "1reakaeh1/8r/2h1c4/p1p1p1p1p/9/6P2/PcP1P3P/1CH1C1H2/9/R1EAKAER1 w - - 3 5" => {
            Some("R9=8")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 R1+1
        // 6. R9=8
        "1reakaeh1/8r/2h1c4/p1p1p1p1p/9/6P2/PcP1P3P/1CH1C1H2/9/1REAKAER1 b - - 4 5" => {
            Some("R1=8")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C8+4
        // 4. P3+1 C2=5
        // 5. H8+7 R1+1
        // 6. R9=8 R1=8
        "1reakaeh1/7r1/2h1c4/p1p1p1p1p/9/6P2/PcP1P3P/1CH1C1H2/9/1REAKAER1 w - - 5 6" => {
            random_move(vec!["H3+4", "P7+1"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 C2=5
        "1reakaehr/9/1ch1c4/p1p1p1p1p/9/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 6 3" => {
            random_move(vec!["R2+6", "H8+7"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        "1reakaehr/9/1ch4c1/p3p1p1p/2p6/9/P1P1P1P1P/1C2C1H2/9/RHEAKAER1 w - - 0 3" => {
            random_move(vec!["P7+1", "R2+6", "P3+1", "H8+9"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. H8+9
        "1reakaehr/9/1ch4c1/p3p1p1p/2p6/9/P1P1P1P1P/HC2C1H2/9/R1EAKAER1 b - - 1 3" => {
            Some("P7+1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. H8+9 P7+1
        "1reakaehr/9/1ch4c1/p3p1p1p/9/2p6/P1P1P1P1P/HC2C1H2/9/R1EAKAER1 w - - 0 4" => {
            random_move(vec!["C8=6", "C8=7"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P3+1
        "1reakaehr/9/1ch4c1/p3p1p1p/2p6/6P2/P1P1P3P/1C2C1H2/9/RHEAKAER1 b - - 0 3" => {
            Some("P3+1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P3+1 P3+1
        "1reakaehr/9/1ch4c1/p3p3p/2p3p2/6P2/P1P1P3P/1C2C1H2/9/RHEAKAER1 w - - 0 4" => {
            random_move(vec!["C8+4", "H8+9"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P3+1 P3+1
        // 5. H8+9
        "1reakaehr/9/1ch4c1/p3p3p/2p3p2/6P2/P1P1P3P/HC2C1H2/9/R1EAKAER1 b - - 1 4" => {
            random_move(vec!["E3+5", "E7+5", "P1+1"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P3+1 P3+1
        // 5. H8+9 P1+1
        "1reakaehr/9/1ch4c1/p3p4/2p3p1p/6P2/P1P1P3P/HC2C1H2/9/R1EAKAER1 w - - 0 5" => {
            Some("C8=7")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P3+1 P3+1
        // 5. H8+9 E7+5
        "1r1akaehr/9/1ch1e2c1/p3p3p/2p3p2/6P2/P1P1P3P/HC2C1H2/9/R1EAKAER1 w - - 2 5" => {
            Some("R9+1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P3+1 P3+1
        // 5. H8+9 E3+5
        "1reaka1hr/9/1ch1e2c1/p3p3p/2p3p2/6P2/P1P1P3P/HC2C1H2/9/R1EAKAER1 w - - 2 5" => {
            Some("C8+4")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P3+1 P3+1
        // 5. C8+4
        "1reakaehr/9/1ch4c1/pC2p3p/2p3p2/6P2/P1P1P3P/4C1H2/9/RHEAKAER1 b - - 1 4" => {
            random_move(vec!["E7+5"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. R2+6
        "1reakaehr/9/1ch4c1/p3p1pRp/2p6/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE2 b - - 1 3" => {
            Some("H2+3")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. R2+6 H2+3
        "1reakae1r/9/1ch3hc1/p3p1pRp/2p6/9/P1P1P1P1P/1C2C1H2/9/RHEAKAE2 w - - 2 4" => {
            random_move(vec!["P5+1", "C8=6", "H8+7"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. R2+6 H2+3
        // 5. H8+7
        "1reakae1r/9/1ch3hc1/p3p1pRp/2p6/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAE2 b - - 3 4" => {
            Some("P3+1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. R2+6 H2+3
        // 5. H8+7 P3+1
        "1reakae1r/9/1ch3hc1/p3p2Rp/2p3p2/9/P1P1P1P1P/1CH1C1H2/9/R1EAKAE2 w - - 0 5" => {
            Some("R9+1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. R2+6 H2+3
        // 5. C8=6
        "1reakae1r/9/1ch3hc1/p3p1pRp/2p6/9/P1P1P1P1P/3CC1H2/9/RHEAKAE2 b - - 3 4" => {
            Some("R1=2")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. R2+6 H2+3
        // 5. C8=6 R1=2
        "1reakaer1/9/1ch3hc1/p3p1pRp/2p6/9/P1P1P1P1P/3CC1H2/9/RHEAKAE2 w - - 4 5" => {
            Some("H8+7")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. R2+6 H2+3
        // 5. C8=6 R1=2
        // 6. H8+7
        "1reakaer1/9/1ch3hc1/p3p1pRp/2p6/9/P1P1P1P1P/2HCC1H2/9/R1EAKAE2 b - - 5 5" => {
            Some("C2=1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. R2+6 H2+3
        // 5. C8=6 R1=2
        // 6. H8+7 C2=1
        "1reakaer1/9/1ch3h1c/p3p1pRp/2p6/9/P1P1P1P1P/2HCC1H2/9/R1EAKAE2 w - - 6 6" => {
            Some("P5+1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. R2+6 H2+3
        // 5. P5+1
        "1reakae1r/9/1ch3hc1/p3p1pRp/2p6/4P4/P1P3P1P/1C2C1H2/9/RHEAKAE2 b - - 0 4" => {
            Some("A4+5")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1
        "1reakaehr/9/1ch4c1/p3p1p1p/2p6/2P6/P3P1P1P/1C2C1H2/9/RHEAKAER1 b - - 0 3" => {
            random_move(vec!["C8+4", "P7+1"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        "1reakaehr/9/1ch4c1/p3p1p1p/9/2p6/P3P1P1P/1C2C1H2/9/RHEAKAER1 w - - 0 4" => {
            random_move(vec!["C8=7", "H8+7", "R2+6"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6
        "1reakaehr/9/1ch4c1/p3p1pRp/9/2p6/P3P1P1P/1C2C1H2/9/RHEAKAE2 b - - 1 4" => {
            random_move(vec!["R1+1", "H7+6", "A4+5", "C8=9"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 C8=9
        "1reakaehr/9/c1h4c1/p3p1pRp/9/2p6/P3P1P1P/1C2C1H2/9/RHEAKAE2 w - - 2 5" => {
            Some("R2=3")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 C8=9
        // 6. R2=3
        "1reakaehr/9/c1h4c1/p3p1R1p/9/2p6/P3P1P1P/1C2C1H2/9/RHEAKAE2 b - - 0 5" => {
            random_move(vec!["R8+2", "C9-1"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 C8=9
        // 6. R2=3 C9-1
        "1reakaehr/c8/2h4c1/p3p1R1p/9/2p6/P3P1P1P/1C2C1H2/9/RHEAKAE2 w - - 1 6" => {
            random_move(vec!["P5+1", "H8+9", "C8+6", "H8+7"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 C8=9
        // 6. R2=3 C9-1
        // 7. H8+7
        "1reakaehr/c8/2h4c1/p3p1R1p/9/2p6/P3P1P1P/1CH1C1H2/9/R1EAKAE2 b - - 2 6" => {
            random_move(vec!["R1+1", "A4+5"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 C8=9
        // 6. R2=3 C9-1
        // 7. H8+7 A4+5
        "1reak1ehr/c3a4/2h4c1/p3p1R1p/9/2p6/P3P1P1P/1CH1C1H2/9/R1EAKAE2 w - - 3 7" => {
            random_move(vec!["H7+6", "C8=9"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 C8=9
        // 6. R2=3 C9-1
        // 7. H8+7 R1+1
        "1reakaeh1/c7r/2h4c1/p3p1R1p/9/2p6/P3P1P1P/1CH1C1H2/9/R1EAKAE2 w - - 3 7" => {
            Some("C8=9")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 C8=9
        // 6. R2=3 R8+2
        "2eakaehr/9/crh4c1/p3p1R1p/9/2p6/P3P1P1P/1C2C1H2/9/RHEAKAE2 w - - 1 6" => {
            Some("H8+7")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 C8=9
        // 6. R2=3 R8+2
        // 7. H8+7
        "2eakaehr/9/crh4c1/p3p1R1p/9/2p6/P3P1P1P/1CH1C1H2/9/R1EAKAE2 b - - 2 6" => {
            Some("E3+5")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 A4+5
        "1reak1ehr/4a4/1ch4c1/p3p1pRp/9/2p6/P3P1P1P/1C2C1H2/9/RHEAKAE2 w - - 2 5" => {
            random_move(vec!["H8+7", "C8=7"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 H7+6
        "1reakaehr/9/1c5c1/p3p1pRp/3h5/2p6/P3P1P1P/1C2C1H2/9/RHEAKAE2 w - - 2 5" => {
            Some("H8+7")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 H7+6
        // 6. H8+7
        "1reakaehr/9/1c5c1/p3p1pRp/3h5/2p6/P3P1P1P/1CH1C1H2/9/R1EAKAE2 b - - 3 5" => {
            random_move(vec!["E3+5", "R1+1", "P7+1"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. R2+6 R1+1
        "1reakaeh1/8r/1ch4c1/p3p1pRp/9/2p6/P3P1P1P/1C2C1H2/9/RHEAKAE2 w - - 2 5" => {
            random_move(vec!["H8+7", "C8=7"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. H8+7
        "1reakaehr/9/1ch4c1/p3p1p1p/9/2p6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 1 4" => {
            random_move(vec!["E3+5", "C8+2", "C2+4"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. H8+7 C2+4
        "1reakaehr/9/1ch6/p3p1p1p/9/2p6/P3P1PcP/1CH1C1H2/9/R1EAKAER1 w - - 2 5" => {
            Some("P5+1")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. H8+7 C2+4
        // 6. P5+1
        "1reakaehr/9/1ch6/p3p1p1p/9/2p1P4/P5PcP/1CH1C1H2/9/R1EAKAER1 b - - 0 5" => {
            Some("C8+4")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. H8+7 C8+2
        "1reakaehr/9/2h4c1/p3p1p1p/1c7/2p6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 w - - 2 5" => {
            Some("H7+6")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. H8+7 E3+5
        "1reaka1hr/9/1ch1e2c1/p3p1p1p/9/2p6/P3P1P1P/1CH1C1H2/9/R1EAKAER1 w - - 2 5" => {
            Some("C8+2")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. C8=7
        "1reakaehr/9/1ch4c1/p3p1p1p/9/2p6/P3P1P1P/2C1C1H2/9/RHEAKAER1 b - - 1 4" => {
            random_move(vec!["C8+2", "C2+4"])
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. C8=7 C2+4
        "1reakaehr/9/2h4c1/p3p1p1p/9/2p6/Pc2P1P1P/2C1C1H2/9/RHEAKAER1 w - - 2 5" => {
            Some("R2+4")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 P7+1
        // 5. C8=7 C8+2
        "1reakaehr/9/2h4c1/p3p1p1p/1c7/2p6/P3P1P1P/2C1C1H2/9/RHEAKAER1 w - - 2 5" => {
            Some("H8+9")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 C8+4
        "1reakaehr/9/2h4c1/p3p1p1p/2p6/2P6/Pc2P1P1P/1C2C1H2/9/RHEAKAER1 w - - 1 4" => {
            Some("H8+7")
        },
        // 1. C2=5 H8+7
        // 2. H2+3 R9=8
        // 3. R1=2 P7+1
        // 4. P7+1 C8+4
        // 5. H8+7
        "1reakaehr/9/2h4c1/p3p1p1p/2p6/2P6/Pc2P1P1P/1CH1C1H2/9/R1EAKAER1 b - - 2 4" => {
            random_move(vec!["E3+5", "C2=5"])
        },
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch() {
        let game_state = String::from("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0");
        match recommended_move(&game_state)  {
            Some(mov) => {
                let option_a = mov == "C2=5";
                let option_b = mov == "P7+1";
                let option_c = mov == "E3+5";
                let option_d = mov == "H2+3";
                let option_e = mov == "C2=4";
                let option_f = mov == "C2=6";
                let result = option_a || option_b || option_c || option_d || option_e || option_f;
                assert_eq!(result, true);
            },
            None => assert!(false, "expected move"),
        }
    }
}
