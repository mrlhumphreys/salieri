use std::collections::HashSet;
use crate::go::state::point::Point;
use crate::go::state::point_set::adjacent_to_x_and_y;
use crate::go::state::point_set::players_stones_adjacent_to_x_and_y_chain_ids;
use crate::go::state::point_set::add_stone;
use crate::go::state::point_set::remove_captured_stones;
use crate::go::state::point_set::chain_has_two_liberties;
use crate::go::state::point_set::chain_has_only_one_liberty;
use crate::go::state::point_set::populate_chains;
use crate::go::state::point_set::simplify;
use crate::go::state::point_set::max_chain_id;
use crate::go::state::point_set::mark_territories;
use crate::go::state::point_set::players_territory_count;
use crate::go::state::mov::Move;
use crate::go::state::mov::MoveKind;
use crate::go::state::player_stat::PlayerStat;

#[derive(Clone)]
pub struct GameState {
    pub current_player_number: i8,
    pub points: Vec<Vec<Point>>,
    pub previous_state: Vec<Vec<i8>>,
    pub player_stats: Vec<PlayerStat>
}

impl GameState {
    pub fn players_score(&mut self, player_number: i8) -> i16 {
        mark_territories(&mut self.points);
        let territory_count = players_territory_count(&self.points, player_number);
        let prisoner_count = self.players_prisoner_count(player_number) as i16;
        territory_count + prisoner_count
    }

    pub fn players_prisoner_count(&self, player_number: i8) -> i8 {
        match self.player_stats.iter().find(|ps| ps.player_number == player_number) {
            Some(ps) => ps.prisoner_count,
            None => 0
        }
    }

    pub fn possible_moves(&mut self) -> Vec<Move> {
        self.possible_moves_for_player(self.current_player_number)
    }

    pub fn possible_moves_for_player(&mut self, subject_player_number: i8) -> Vec<Move> {
        if self.player_stats.iter().all(|ps| ps.passed) {
            return vec![];
        }

        let mut moves: Vec<Move> = vec![];
        let simplified_game_state = simplify(&self.points);

        for (y, row) in self.points.iter().enumerate() {
            for (x, point) in row.iter().enumerate() {
                if point.player_number == 0 {
                    let adj = adjacent_to_x_and_y(&self.points, x, y);

                    if adj.iter().any(|p| p.player_number == 0) {
                        // point has at least one liberty
                        let mut new_state = self.points.clone();
                        add_stone(&mut new_state, x, y, subject_player_number);
                        let captures = remove_captured_stones(&mut new_state, x, y, self.opposing_player_number());
                        let mov = Move { kind: MoveKind::Place, x, y, simplified_game_state: simplified_game_state.clone(), captures };
                        moves.push(mov);
                    } else {
                        // point technically has no liberties
                        let mut friendly_chain_ids = HashSet::new();

                        for p in &adj {
                            if p.player_number == subject_player_number {
                                friendly_chain_ids.insert(p.chain_id);
                            }
                        }

                        let friendly_chain_has_two_liberties = friendly_chain_ids.iter().any(|cid| {
                            chain_has_two_liberties(&self.points, *cid)
                        });

                        if friendly_chain_has_two_liberties {
                            // point is adjacent to own chain with currently 2 liberties
                            let mut new_state = self.points.clone(); // Clone

                            add_stone(&mut new_state, x, y, subject_player_number);
                            let captures = remove_captured_stones(&mut new_state, x, y, self.opposing_player_number());
                            let mov = Move { kind: MoveKind::Place, x, y, simplified_game_state: simplified_game_state.clone(), captures };
                            moves.push(mov);
                        } else {
                            let mut enemy_chain_ids = HashSet::new();

                            for p in adj.iter() {
                                if p.player_number != 0 && p.player_number != subject_player_number {
                                    enemy_chain_ids.insert(p.chain_id);
                                }
                            }

                            let enemy_chain_has_only_one_liberty = enemy_chain_ids.iter().any(|cid| {
                                chain_has_only_one_liberty(&self.points, *cid)
                            });

                            let mut new_state = self.points.clone(); // clone

                            add_stone(&mut new_state, x, y, subject_player_number);
                            let captures = remove_captured_stones(&mut new_state, x, y, self.opposing_player_number());
                            let doesnt_repeat_previous_state = self.previous_state != simplify(&new_state);
                            if enemy_chain_has_only_one_liberty && doesnt_repeat_previous_state {
                                // point is adjacent to enemy chain with currently 1 liberties
                                // && adding stone doesn't repeat previous state
                                let mov = Move { kind: MoveKind::Place, x, y, simplified_game_state: simplified_game_state.clone(), captures };
                                moves.push(mov);
                            }
                        }
                    }
                }
            }
        }

        let pass = Move { kind: MoveKind::Pass, x: 19, y: 19, simplified_game_state, captures: vec![] };
        moves.push(pass);

        moves
    }

    pub fn perform_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        match mov.kind {
            MoveKind::Place => {
                self.previous_state = simplify(&self.points);
                let chain_id = add_stone(&mut self.points, mov.x, mov.y, self.current_player_number);
                if let Err(e) = self.update_joined_chains(mov.x, mov.y, chain_id, self.current_player_number) {
                    return Err(e)
                }
                let opposing_player_number = self.opposing_player_number();
                let stones_captured = remove_captured_stones(&mut self.points, mov.x, mov.y, opposing_player_number);
                if !stones_captured.is_empty() {
                    if let Err(e) = self.update_player_stats(self.current_player_number, stones_captured.len() as i8) {
                        return Err(e);
                    }
                }
                self.current_player_number = self.opposing_player_number();
                Ok(())
            },
            MoveKind::Pass => {
                match self.mark_as_passed(self.current_player_number, true) {
                    Ok(_) => {
                        self.current_player_number = self.opposing_player_number();
                        Ok(())
                    },
                    Err(e) => Err(e)
                }
            }
        }
    }

    pub fn undo_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        let other_player_number = self.opposing_player_number();
        match mov.kind {
            MoveKind::Place => {
                self.previous_state = mov.simplified_game_state.clone(); // Clone

                // remove piece at x, y
                let p = &mut self.points[mov.y][mov.x];
                p.player_number = 0;
                p.chain_id = 0;

                // if there are captures
                if !mov.captures.is_empty() {
                    let chain_id = max_chain_id(&self.points);
                    // add the captured stones back in
                    for (y, row) in &mut self.points.iter_mut().enumerate() {
                        for (x, p) in row.iter_mut().enumerate() {
                            if mov.captures.contains(&(x, y)) {
                                p.player_number = self.current_player_number;
                                p.chain_id = chain_id;
                            }
                        }
                    }

                    // update player stats - reduce prisoner count
                    match self.player_stats.iter_mut().find(|ps| ps.player_number == other_player_number) {
                        Some(ps) => ps.prisoner_count -= mov.captures.len() as i8,
                        None => return Err("No player stat found")
                    }
                }
                self.current_player_number = other_player_number;
                Ok(())
            },
            MoveKind::Pass => {
                match self.mark_as_passed(other_player_number, false) {
                    Ok(_) => {
                        self.current_player_number = other_player_number;
                        Ok(())
                    },
                    Err(e) => Err(e)
                }
            }
        }

    }

    // set stones chain id where current chain id is adjacent to point
    pub fn update_joined_chains(&mut self, x: usize, y: usize, chain_id: i8, player_number: i8) -> Result<(), &'static str> {

        let adjacent_chain_ids = players_stones_adjacent_to_x_and_y_chain_ids(&self.points, x, y, player_number);

        if !adjacent_chain_ids.is_empty() {
            for row in &mut self.points {
                for p in row {
                    if p.player_number != 0 && adjacent_chain_ids.iter().any(|cid| *cid == p.chain_id) {
                        p.chain_id = chain_id;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn update_player_stats(&mut self, player_number: i8, stones_captured: i8) -> Result<(), &'static str> {
        if let Some(ps) = self.player_stats.iter_mut().find(|ps| ps.player_number == player_number) {
            ps.prisoner_count = stones_captured;
            Ok(())
        } else {
            Err("No stats for player")
        }
    }

    pub fn mark_as_passed(&mut self, player_number: i8, passed: bool) -> Result<(), &'static str> {
        if let Some(ps) = self.player_stats.iter_mut().find(|ps| ps.player_number == player_number) {
            ps.passed = passed;
            Ok(())
        } else {
            Err("No stats for player")
        }
    }

    pub fn opposing_player_number(&self) -> i8 {
        if self.current_player_number == 1 {
            2
        } else {
            1
        }
    }
}

//   a b c d e
// a
// b
// c
// d
// e
// [xy]
// PL[W] or PL[B] - Player Turn
// AB[bb:ee]AW[bb][ee][dc][cd][cb][bc][be][eb][ed][de] - Setup board
// XS[ab][cd]  - Previously Captured Pieces
// XW[0] XB[0] - Number of stones captured
// AB[tt] - Player 1 passed
pub fn parse(encoded: &String) -> Result<GameState, &'static str> {
    let mut read_player = false;
    let mut read_board = false;
    let mut read_white = false;
    let mut read_black = false;
    let mut read_x = false;
    let mut read_y = false;
    let mut read_captures = false;
    let mut read_player_stats = false;
    let mut read_player_stats_or_captures = false;

    let mut x = 0;
    let mut y = 0;

    let mut points = vec![vec![Point { player_number: 0, chain_id: 0, territory_id: None }; 19]; 19];
    let mut previous_captures: Vec<(usize, usize)> = vec![];
    let mut current_player_number: i8 = 0;
    let mut player_stats = vec![]; // XW XB
    let mut black_passed = false;
    let mut white_passed = false;
    let mut raw_prisoner_count = String::from("");
    let mut previous_player_last_stone_x = 0;
    let mut previous_player_last_stone_y = 0;
    let mut error = false;

    for c in encoded.chars() {
        match c {
            'P' => {
                // do nothing
            },
            'L' => {
                read_player = true;
            },
            'A' => {
                read_board = true;
            },
            'B' => {
                if read_player {
                    current_player_number = 1;
                } else if read_board {
                    read_black = true;
                    read_white = false;
                } else if read_player_stats_or_captures {
                    read_player_stats = true;
                    read_black = true;
                    read_white = false;
                } else {
                    error = true;
                }
            },
            'W' => {
                if read_player {
                    current_player_number = 2;
                } else if read_board {
                    read_white = true;
                    read_black = false;
                } else if read_player_stats_or_captures {
                    read_player_stats = true;
                    read_white = true;
                    read_black = false;
                } else {
                    error = true;
                }
            },
            'X' => {
                read_board = false;
                read_player_stats_or_captures = true;
            },
            'S' => {
                read_white = false;
                read_black = false;
                read_captures = true;
            },
            '[' => {
                if read_player {
                    // do nothing
                } else if read_board {
                    read_x = true;
                } else if read_captures {
                    read_x = true;
                } else if read_player_stats {
                    // do nothing
                } else {
                    error = true;
                }
            },
            ']' => {
                if read_player {
                    read_player = false;
                } else if read_board {
                    let stone_player_number = if read_black {
                        1
                    } else {
                        2
                    };

                    read_x = false;
                    read_y = false;
                    let point = Point {
                        player_number: stone_player_number,
                        chain_id: 0,
                        territory_id: None
                    };
                    points[y][x] = point;

                    // used to build previous state
                    if stone_player_number != current_player_number {
                        previous_player_last_stone_x = x;
                        previous_player_last_stone_y = y;
                    }
                } else if read_captures {
                    read_x = false;
                    read_y = false;
                    let coordinate = (x, y);
                    previous_captures.push(coordinate);
                } else if read_player_stats {
                    let player_number = if read_white {
                        2
                    } else {
                        1
                    };

                    let passed = if read_white {
                        white_passed
                    } else {
                        black_passed
                    };

                    let prisoner_count = match raw_prisoner_count.parse::<i8>() {
                        Ok(n) => n,
                        Err(_) => 0
                    };

                    let player_stat = PlayerStat { player_number, prisoner_count, passed };
                    player_stats.push(player_stat);
                    raw_prisoner_count = String::from("");
                } else {
                   error = true;
                }
            },
            'a'..='s' => {
                if read_x {
                    let integer = c as usize; // column/x
                    x = integer - 97;
                    read_x = false;
                    read_y = true;
                } else if read_y {
                    let integer = c as usize; // column/x
                    y = integer - 97;
                } else {
                    error = true;
                }
            },
            't' => {
                if read_x || read_y {
                    if read_white {
                        white_passed = true;
                    } else if read_black {
                        black_passed = true;
                    }
                } else {
                    error = true;
                }
            },
            '0'..='9' => {
                if read_player_stats {
                    raw_prisoner_count.push(c);
                } else {
                    error = true;
                }
            },
            _ => {
                error = true;
            }
        }
    }

    if error {
        return Err("Error parsing state");
    }
    // update chains
    populate_chains(&mut points);

    // build previous state
    let mut previous_state = simplify(&points);

    // add captured stones (owned by current player)
    // and remove previous stone
    for x in 0..19 {
        for y in 0..19 {
            previous_captures.iter().for_each(|capture| {
                if x == capture.0 && y == capture.1 {
                    previous_state[y][x] = current_player_number;
                }

            });
            if x == previous_player_last_stone_x && y == previous_player_last_stone_y {
                previous_state[y][x] = 0;
            }
        }
    }

    let game_state  = GameState {
        current_player_number,
        points,
        previous_state,
        player_stats
    };
    Ok(game_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_start_test() {
        let encoded = String::from("PL[B]XB[0]XW[0]");
        let result = parse(&encoded).unwrap();
        let expected_player_stats = vec![
            PlayerStat { player_number: 1, prisoner_count: 0, passed: false },
            PlayerStat { player_number: 2, prisoner_count: 0, passed: false }
        ];
        let expected_points = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];

        assert_eq!(result.current_player_number, 1);
        assert_eq!(result.points, expected_points);
        assert_eq!(result.player_stats, expected_player_stats);
    }

    #[test]
    fn parse_first_moves_test() {
        let encoded = String::from("PL[B]AB[cb]AW[de]XB[0]XW[0]");
        let result = parse(&encoded).unwrap();
        let expected_player_stats = vec![
            PlayerStat { player_number: 1, prisoner_count: 0, passed: false },
            PlayerStat { player_number: 2, prisoner_count: 0, passed: false }
        ];
        let expected_points = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 2, chain_id: 2, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];
        assert_eq!(result.current_player_number, 1);
        assert_eq!(result.points, expected_points);
        assert_eq!(result.player_stats, expected_player_stats);
    }

    #[test]
    fn parse_player_stats_test() {
        let encoded = String::from("PL[B]AB[cb]AW[de]XB[2]XW[1]");
        let result = parse(&encoded).unwrap();
        let expected_player_stats = vec![
            PlayerStat { player_number: 1, prisoner_count: 2, passed: false },
            PlayerStat { player_number: 2, prisoner_count: 1, passed: false }
        ];
        assert_eq!(result.current_player_number, 1);
        assert_eq!(result.player_stats, expected_player_stats);
    }

    #[test]
    fn parse_passed_test() {
        let encoded = String::from("PL[B]AB[cb]AW[de][tt]XB[2]XW[1]");
        let result = parse(&encoded).unwrap();
        let expected_player_stats = vec![
            PlayerStat { player_number: 1, prisoner_count: 2, passed: false },
            PlayerStat { player_number: 2, prisoner_count: 1, passed: true }
        ];
        assert_eq!(result.current_player_number, 1);
        assert_eq!(result.player_stats, expected_player_stats);
    }

    #[test]
    fn parse_last_capture_test() {
        let encoded = String::from("PL[B]AB[cb]AW[de]XB[2]XW[1]XS[ee][ef]");
        let result = parse(&encoded).unwrap();
        let expected_player_stats = vec![
            PlayerStat { player_number: 1, prisoner_count: 2, passed: false },
            PlayerStat { player_number: 2, prisoner_count: 1, passed: false }
        ];
        let expected_previous_state = vec![
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,1,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,1, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],

            [ 0,0,0,0,1, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],

            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],

            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
            [ 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ]
        ];
        assert_eq!(result.current_player_number, 1);
        assert_eq!(result.player_stats, expected_player_stats);
        assert_eq!(result.previous_state, expected_previous_state);
    }

    #[test]
    fn players_score_test() {
        let encoded = String::from("PL[B]AB[ba][ab]AW[de]XB[4]XW[1]");
        let mut game_state = parse(&encoded).unwrap();
        let expected = 5;
        let result = game_state.players_score(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn players_prisoner_count_test() {
        let encoded = String::from("PL[B]XB[4]XW[0]");
        let game_state = parse(&encoded).unwrap();
        let expected = 4;
        let result = game_state.players_prisoner_count(1);
        assert_eq!(result, expected);
    }

    #[test]
    fn possible_moves_test() {
        let encoded = String::from("PL[B]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves();
        let expected = Move {
            kind: MoveKind::Place,
            x: 0,
            y: 0,
            simplified_game_state: vec![
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0]
            ],
            captures: vec![]
        };
        assert_eq!(result.len(), 362);
        assert_eq!(result[0], expected);
    }

    #[test]
    fn possible_moves_no_liberties_test() {
        // should not include ab (0, 1)
        let encoded = String::from("PL[W]AB[aa][ba][bb][ac][bc][pd][jj]AW[ad][ae][af]XB[3]XW[0]XS[ab]");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves().iter().map(|m| (m.x, m.y)).collect::<Vec<(usize, usize)>>();
        let expected = vec![(2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1), (9, 1), (10, 1), (11, 1), (12, 1), (13, 1), (14, 1), (15, 1), (16, 1), (17, 1), (18, 1), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), (8, 2), (9, 2), (10, 2), (11, 2), (12, 2), (13, 2), (14, 2), (15, 2), (16, 2), (17, 2), (18, 2), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (8, 3), (9, 3), (10, 3), (11, 3), (12, 3), (13, 3), (14, 3), (16, 3), (17, 3), (18, 3), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4), (9, 4), (10, 4), (11, 4), (12, 4), (13, 4), (14, 4), (15, 4), (16, 4), (17, 4), (18, 4), (1, 5), (2, 5), (3, 5), (4, 5), (5, 5), (6, 5), (7, 5), (8, 5), (9, 5), (10, 5), (11, 5), (12, 5), (13, 5), (14, 5), (15, 5), (16, 5), (17, 5), (18, 5), (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (6, 6), (7, 6), (8, 6), (9, 6), (10, 6), (11, 6), (12, 6), (13, 6), (14, 6), (15, 6), (16, 6), (17, 6), (18, 6), (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7), (8, 7), (9, 7), (10, 7), (11, 7), (12, 7), (13, 7), (14, 7), (15, 7), (16, 7), (17, 7), (18, 7), (0, 8), (1, 8), (2, 8), (3, 8), (4, 8), (5, 8), (6, 8), (7, 8), (8, 8), (9, 8), (10, 8), (11, 8), (12, 8), (13, 8), (14, 8), (15, 8), (16, 8), (17, 8), (18, 8), (0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9), (6, 9), (7, 9), (8, 9), (10, 9), (11, 9), (12, 9), (13, 9), (14, 9), (15, 9), (16, 9), (17, 9), (18, 9), (0, 10), (1, 10), (2, 10), (3, 10), (4, 10), (5, 10), (6, 10), (7, 10), (8, 10), (9, 10), (10, 10), (11, 10), (12, 10), (13, 10), (14, 10), (15, 10), (16, 10), (17, 10), (18, 10), (0, 11), (1, 11), (2, 11), (3, 11), (4, 11), (5, 11), (6, 11), (7, 11), (8, 11), (9, 11), (10, 11), (11, 11), (12, 11), (13, 11), (14, 11), (15, 11), (16, 11), (17, 11), (18, 11), (0, 12), (1, 12), (2, 12), (3, 12), (4, 12), (5, 12), (6, 12), (7, 12), (8, 12), (9, 12), (10, 12), (11, 12), (12, 12), (13, 12), (14, 12), (15, 12), (16, 12), (17, 12), (18, 12), (0, 13), (1, 13), (2, 13), (3, 13), (4, 13), (5, 13), (6, 13), (7, 13), (8, 13), (9, 13), (10, 13), (11, 13), (12, 13), (13, 13), (14, 13), (15, 13), (16, 13), (17, 13), (18, 13), (0, 14), (1, 14), (2, 14), (3, 14), (4, 14), (5, 14), (6, 14), (7, 14), (8, 14), (9, 14), (10, 14), (11, 14), (12, 14), (13, 14), (14, 14), (15, 14), (16, 14), (17, 14), (18, 14), (0, 15), (1, 15), (2, 15), (3, 15), (4, 15), (5, 15), (6, 15), (7, 15), (8, 15), (9, 15), (10, 15), (11, 15), (12, 15), (13, 15), (14, 15), (15, 15), (16, 15), (17, 15), (18, 15), (0, 16), (1, 16), (2, 16), (3, 16), (4, 16), (5, 16), (6, 16), (7, 16), (8, 16), (9, 16), (10, 16), (11, 16), (12, 16), (13, 16), (14, 16), (15, 16), (16, 16), (17, 16), (18, 16), (0, 17), (1, 17), (2, 17), (3, 17), (4, 17), (5, 17), (6, 17), (7, 17), (8, 17), (9, 17), (10, 17), (11, 17), (12, 17), (13, 17), (14, 17), (15, 17), (16, 17), (17, 17), (18, 17), (0, 18), (1, 18), (2, 18), (3, 18), (4, 18), (5, 18), (6, 18), (7, 18), (8, 18), (9, 18), (10, 18), (11, 18), (12, 18), (13, 18), (14, 18), (15, 18), (16, 18), (17, 18), (18, 18), (19, 19)];
        assert_eq!(result, expected);
    }

    #[test]
    fn possible_moves_for_player_test() {
        let encoded = String::from("PL[B]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);
        let expected = Move {
            kind: MoveKind::Place,
            x: 0,
            y: 0,
            simplified_game_state: vec![
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0]
            ],
            captures: vec![]
        };
        assert_eq!(result.len(), 362);
        assert_eq!(result[0], expected);
    }

    #[test]
    fn possible_moves_for_player_captures_test() {
        let encoded = String::from("PL[B]AB[ba][ab][cb]AW[bb][cc]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);
        let expected = Move {
            kind: MoveKind::Place,
            x: 1,
            y: 2,
            simplified_game_state: vec![
                vec![0,1,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![1,2,1,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,2,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
                vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0]
            ],
            captures: vec![(1,1)]
        };
        assert_eq!(result.len(), 357);
        match result.iter().find(|m| m.x == 1 && m.y == 2) {
            Some(m) => assert_eq!(m, &expected),
            None => assert!(false, "expected point")
        }
    }

    #[test]
    fn possible_moves_ko_rule_test() {
        // +-1-+
        // 1-+-1
        // 2-1-2
        // +-2-+
        let encoded = String::from("PL[W]AB[ba][ab][cb][bc]AW[ac][cc][bd]XB[0]XW[1]XS[bb]");
        let mut game_state = parse(&encoded).unwrap();
        let moves = game_state.possible_moves();
        let result = moves.iter().find(|m| { m.x == 1 && m.y == 1 }).is_none();
        assert_eq!(result, true);
    }

    #[test]
    fn perform_move_test() {
        let encoded = String::from("PL[B]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let x = 3;
        let y = 3;
        let mov = Move { kind: MoveKind::Place, x, y, simplified_game_state: simplify(&game_state.points), captures: vec![] };
        match game_state.perform_move(&mov) {
            Ok(_) => {
                let p = &game_state.points[y][x];
                if p.player_number != 0 {
                    assert_eq!(p.player_number, 1);
                    assert_eq!(p.chain_id, 1);
                    assert_eq!(game_state.current_player_number, 2);
                } else {
                    assert!(false, "expected stone")
                }
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn perform_move_capture_test() {
        let encoded = String::from("PL[B]AB[ba][ab][cb]AW[bb][cc]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let x = 1;
        let y = 2;
        let mov = Move { kind: MoveKind::Place, x, y, simplified_game_state: simplify(&game_state.points), captures: vec![] };
        match game_state.perform_move(&mov) {
            Ok(_) => {
                // placed stone
                let placed = &game_state.points[y][x];
                if placed.player_number != 0 {
                    assert_eq!(placed.player_number, 1);
                    assert_eq!(placed.chain_id, 6);
                    assert_eq!(game_state.current_player_number, 2);
                } else {
                    assert!(false, "expected stone")
                }

                // capotured stone
                let captured = &game_state.points[1][1];
                assert_eq!(captured.player_number, 0);

                // player stats
                match game_state.player_stats.iter().find(|p| p.player_number == 1) {
                    Some(ps) => assert_eq!(ps.prisoner_count, 1),
                    None => assert!(false, "expected ps")
                }
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn perform_move_pass_test() {
        let encoded = String::from("PL[B]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let x = 19;
        let y = 19;
        let mov = Move { kind: MoveKind::Pass, x, y, simplified_game_state: simplify(&game_state.points), captures: vec![] };
        match game_state.perform_move(&mov) {
            Ok(_) => {
                match game_state.player_stats.iter().find(|p| p.player_number == 1) {
                    Some(ps) => assert_eq!(ps.passed, true),
                    None => assert!(false, "Expected Player Stat")
                }
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn undo_move_test() {
        let encoded = String::from("PL[W]AB[ba]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let x = 2;
        let y = 1;
        let old_encoded = String::from("PL[B]XB[0]XW[0]");
        let old_game_state = parse(&old_encoded).unwrap();
        let mov = Move { kind: MoveKind::Place, x, y, simplified_game_state: simplify(&old_game_state.points), captures: vec![] };
        match game_state.undo_move(&mov) {
            Ok(_) => {
                let p = &game_state.points[y][x];
                assert_eq!(p.player_number, 0);
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn undo_move_capture_test() {
        let encoded = String::from("PL[W]AB[ba][ab][cb][bc]AW[cc]XB[1]XW[0]XS[bb]");
        let mut game_state = parse(&encoded).unwrap();
        let x = 1;
        let y = 2;
        let old_encoded = String::from("PL[B]AB[ba][ab][cb]AW[bb][cc]XB[0]XW[0]");
        let old_game_state = parse(&old_encoded).unwrap();
        let mov = Move { kind: MoveKind::Place, x, y, simplified_game_state: simplify(&old_game_state.points), captures: vec![(1, 1)] };
        match game_state.undo_move(&mov) {
            Ok(_) => {
                // remove last placed stone
                let placed = &game_state.points[y][x];
                assert_eq!(placed.player_number, 0);

                // add last captured stones
                let captured = &game_state.points[1][1];
                if captured.player_number != 0 {
                    assert_eq!(captured.player_number, 2);
                    assert_eq!(captured.chain_id, 5);
                } else {
                    assert!(false, "expected stone")
                }

                // player stats
                match game_state.player_stats.iter().find(|p| p.player_number == 1) {
                    Some(ps) => assert_eq!(ps.prisoner_count, 0),
                    None => assert!(false, "expected ps")
                }
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn mark_as_passed_test() {
        let encoded = String::from("PL[B]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let player_number = 1;
        let passed = true;
        match game_state.mark_as_passed(player_number, passed) {
            Ok(_) => {
                match game_state.player_stats.iter().find(|ps| ps.player_number == player_number) {
                    Some(ps) => assert_eq!(ps.passed, passed),
                    None => assert!(false, "Expected Player Stat")
                }
            },
            Err(e) => assert!(false, "{}", e)
        }
        assert!(true);
    }

    #[test]
    fn update_joined_chains_test() {
        let encoded = String::from("PL[B]AB[aa][ac]AW[bb]XB[0]XW[0]");
        let chain_id = 3;
        let mut game_state = parse(&encoded).unwrap();
        match game_state.update_joined_chains(0, 1, chain_id, 1) {
            Ok(_) => {
                let p = &game_state.points[0][0];
                if p.player_number != 0 {
                    assert_eq!(p.chain_id, chain_id)
                } else {
                    assert!(false, "expected stone")
                }
            },
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn update_player_stats_test() {
        let encoded = String::from("PL[B]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let expected = vec![
            PlayerStat { player_number: 1, prisoner_count: 3, passed: false },
            PlayerStat { player_number: 2, prisoner_count: 0, passed: false }
        ];
        match game_state.update_player_stats(1, 3) {
            Ok(_) => assert_eq!(game_state.player_stats, expected),
            Err(e) =>  assert!(false, "{}", e)
        }
    }
}
