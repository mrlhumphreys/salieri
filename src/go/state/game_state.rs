use std::collections::HashSet;
use crate::go::state::point::Point;
use crate::go::state::point_set::adjacent_to_x_and_y;
use crate::go::state::point_set::players_stones_adjacent_to_x_and_y_chain_ids;
use crate::go::state::point_set::add_stone;
use crate::go::state::point_set::remove_captured_stones;
use crate::go::state::point_set::filter_by_chain_id;
use crate::go::state::point_set::populate_chains;
use crate::go::state::point_set::simplify;
use crate::go::state::point_set::max_chain_id;
use crate::go::state::point_set::mark_territories;
use crate::go::state::point_set::players_territory_count;
use crate::go::state::mov::Move;
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
        let mut moves: Vec<Move> = vec![];
        // points that are emppty

        for point in self.points.iter().flatten() {
            if point.player_number == 0 {
                let adj = adjacent_to_x_and_y(&self.points, point.x, point.y);

                if adj.iter().any(|p| p.player_number == 0) { // < 4
                    // point has at least one liberty
                    let mut new_state = self.points.clone();
                    if add_stone(&mut new_state, point.x, point.y, subject_player_number).is_ok() {
                        let captures = remove_captured_stones(&mut new_state, point.x, point.y, self.opposing_player_number());
                        let mov = Move {
                            x: point.x,
                            y: point.y,
                            simplified_game_state: simplify(&self.points),
                            captures
                        };
                        moves.push(mov);
                    }
                } else {
                    // point technically has no liberties
                    let mut friendly_chain_ids = HashSet::new();

                    for p in adj.iter() {
                        // friendly stones
                        if p.player_number != 0 && p.player_number == subject_player_number {
                            friendly_chain_ids.insert(p.chain_id);
                        }
                    } // 2-4

                    let friendly_chain_has_two_liberties = friendly_chain_ids.iter().any(|cid| {
                        let chain_points = filter_by_chain_id(&self.points, *cid);
                        let mut liberty_count: i8 = 0;
                        let mut two_liberties = false;
                        for p in chain_points.iter() {
                            let adjacent = adjacent_to_x_and_y(&self.points, p.x, p.y);
                            for a in adjacent.iter() {
                                if a.player_number == 0 {
                                    liberty_count += 1;
                                    if liberty_count >= 2 {
                                        two_liberties = true;
                                        break;
                                    }
                                }
                            } // < 4
                            if two_liberties {
                                break;
                            }
                        } // < N
                        two_liberties
                    }); // 0-4

                    if friendly_chain_has_two_liberties {
                        // point is adjacent to own chain with currently 2 liberties
                        let mut new_state = self.points.clone(); // Clone

                        if add_stone(&mut new_state, point.x, point.y, subject_player_number).is_ok() {
                            let captures = remove_captured_stones(&mut new_state, point.x, point.y, self.opposing_player_number());
                            let mov = Move {
                                x: point.x,
                                y: point.y,
                                simplified_game_state: simplify(&self.points),
                                captures
                            };
                            moves.push(mov);
                        }
                    } else {
                        let mut enemy_chain_ids = HashSet::new();

                        for p in adj.iter() {
                            // enemy stones
                            if p.player_number != 0 && p.player_number != subject_player_number {
                                enemy_chain_ids.insert(p.chain_id);
                            }
                        } // N

                        let enemy_chain_has_only_one_liberty = enemy_chain_ids.iter().any(|cid| {
                            let chain_points = filter_by_chain_id(&self.points, *cid); // N
                            let mut liberties = 0;

                            for p in chain_points.iter() {
                                let adjacent = adjacent_to_x_and_y(&self.points, p.x, p.y);  // N
                                for a in adjacent.iter() {
                                    if a.player_number == 0 {
                                        liberties += 1;
                                    }
                                } // 1-4
                            } // < N

                            liberties == 1
                        }); // 0-4

                        let mut new_state = self.points.clone(); // clone

                        if add_stone(&mut new_state, point.x, point.y, subject_player_number).is_ok() {
                            let captures = remove_captured_stones(&mut new_state, point.x, point.y, self.opposing_player_number());
                            let doesnt_repeat_previous_state = self.previous_state != simplify(&new_state);
                            if enemy_chain_has_only_one_liberty && doesnt_repeat_previous_state {
                                // point is adjacent to enemy chain with currently 1 liberties
                                // && adding stone doesn't repeat previous state
                                let mov = Move {
                                    x: point.x,
                                    y: point.y,
                                    simplified_game_state: simplify(&self.points),
                                    captures
                                };
                                moves.push(mov);
                            }
                        }
                    }
                }
            }
        }

        moves
    }

    pub fn perform_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        self.previous_state = simplify(&self.points);
        match add_stone(&mut self.points, mov.x, mov.y, self.current_player_number) {
            Ok(chain_id) => {
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
            Err(e) => Err(e)
        }
    }

    pub fn undo_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        let other_player_number = self.opposing_player_number();
        self.previous_state = mov.simplified_game_state.clone(); // Clone

        // remove piece at x, y
        match self.points.iter_mut().flatten().find(|p| p.x == mov.x && p.y == mov.y ) {
            Some(p) => {
                p.player_number = 0;
                p.chain_id = 0;
            },
            None => return Err("No point found")
        } // < N
        // add captured stones of current player number (will change player number later)
        let chain_id = max_chain_id(&self.points);

        // if there are captures
        if !mov.captures.is_empty() {
            // add the captured stones back in
            for p in self.points.iter_mut().flatten() {
                if mov.captures.contains(&(p.x, p.y)) {
                    p.player_number = self.current_player_number;
                    p.chain_id = chain_id;
                }
            } // N

            // update player stats - reduce prisoner count
            match self.player_stats.iter_mut().find(|ps| ps.player_number == other_player_number) {
                Some(ps) => ps.prisoner_count -= mov.captures.len() as i8,
                None => return Err("No player stat found")
            }
        }

        // change player number
        self.current_player_number = other_player_number;
        Ok(())
    }

    // set stones chain id where current chain id is adjacent to point
    pub fn update_joined_chains(&mut self, x: i8, y: i8, chain_id: i8, player_number: i8) -> Result<(), &'static str> {

        let adjacent_chain_ids = players_stones_adjacent_to_x_and_y_chain_ids(&self.points, x, y, player_number); // N

        if !adjacent_chain_ids.is_empty() {
            self.points.iter_mut().flatten().for_each(|p| {
                if p.player_number != 0 && adjacent_chain_ids.iter().any(|cid| *cid == p.chain_id) {
                    p.chain_id = chain_id;
                }
            }); // N
        }

        Ok(())
    }

    pub fn update_player_stats(&mut self, player_number: i8, stones_captured: i8) -> Result<(), &'static str> {
        if let Some(ps) = self.player_stats.iter_mut().find(|ps| ps.player_number == player_number) {
            ps.prisoner_count = stones_captured;
            Ok(())
        } else {
            Err("No stats for player")
        } // < N
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

    let mut x: i8 = 0;
    let mut y: i8 = 0;

    let mut points: Vec<Vec<Point>> = vec![];
    for y in 0..19 {
        let mut row = vec![];
        for x in 0..19 {
            let point = Point { x, y, player_number: 0, chain_id: 0, territory_id: None };
            row.push(point);
        }
        points.push(row);
    }
    let mut previous_captures: Vec<(i8, i8)> = vec![];
    let mut current_player_number: i8 = 0;
    let mut player_stats = vec![]; // XW XB
    let mut raw_prisoner_count = String::from("");
    let mut previous_player_last_stone_x = -1;
    let mut previous_player_last_stone_y = -1;
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
                        x,
                        y,
                        player_number: stone_player_number,
                        chain_id: 0,
                        territory_id: None
                    };
                    points[y as usize][x as usize] = point;

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

                    let prisoner_count = match raw_prisoner_count.parse::<i8>() {
                        Ok(n) => n,
                        Err(_) => 0
                    };

                    let player_stat = PlayerStat { player_number, prisoner_count, passed: false };
                    player_stats.push(player_stat);
                    raw_prisoner_count = String::from("");
                } else {
                   error = true;
                }
            },
            'a'..='s' => {
                if read_x {
                    let integer = c as i8; // column/x
                    x = integer - 97;
                    read_x = false;
                    read_y = true;
                } else if read_y {
                    let integer = c as i8; // column/x
                    y = integer - 97;
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
                if x == capture.0 as usize && y == capture.1 as usize {
                    previous_state[y][x] = current_player_number;
                }

            });
            if x == previous_player_last_stone_x as usize && y == previous_player_last_stone_y as usize {
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
                Point { y: 0, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 1, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 2, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 3, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 4, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 5, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 6, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 7, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 8, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 9, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 10, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 11, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 12, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 13, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 14, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 15, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 16, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 17, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 18, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 18, player_number: 0, chain_id: 0, territory_id: None }
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
                Point { y: 0, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 0, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 1, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 2, player_number: 1, chain_id: 1, territory_id: None },
                Point { y: 1, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 1, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 2, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 2, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 3, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 3, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 4, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 3, player_number: 2, chain_id: 2, territory_id: None },
                Point { y: 4, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 4, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 5, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 5, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 6, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 6, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 7, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 7, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 8, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 8, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 9, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 9, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 10, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 10, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 11, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 11, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 12, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 12, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 13, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 13, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 14, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 14, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 15, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 15, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 16, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 16, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 17, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 17, x: 18, player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { y: 18, x: 0, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 1, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 2, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 3, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 4, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 5, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 6, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 7, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 8, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 9, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 10, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 11, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 12, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 13, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 14, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 15, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 16, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 17, player_number: 0, chain_id: 0, territory_id: None },
                Point { y: 18, x: 18, player_number: 0, chain_id: 0, territory_id: None }
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
        assert_eq!(result.len(), 361);
        assert_eq!(result[0], expected);
    }

    #[test]
    fn possible_moves_no_liberties_test() {
        // should not include ab (0, 1)
        let encoded = String::from("PL[W]AB[aa][ba][bb][ac][bc][pd][jj]AW[ad][ae][af]XB[3]XW[0]XS[ab]");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves().iter().map(|m| (m.x, m.y)).collect::<Vec<(i8, i8)>>();
        let expected = vec![(2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1), (9, 1), (10, 1), (11, 1), (12, 1), (13, 1), (14, 1), (15, 1), (16, 1), (17, 1), (18, 1), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), (8, 2), (9, 2), (10, 2), (11, 2), (12, 2), (13, 2), (14, 2), (15, 2), (16, 2), (17, 2), (18, 2), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (8, 3), (9, 3), (10, 3), (11, 3), (12, 3), (13, 3), (14, 3), (16, 3), (17, 3), (18, 3), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4), (9, 4), (10, 4), (11, 4), (12, 4), (13, 4), (14, 4), (15, 4), (16, 4), (17, 4), (18, 4), (1, 5), (2, 5), (3, 5), (4, 5), (5, 5), (6, 5), (7, 5), (8, 5), (9, 5), (10, 5), (11, 5), (12, 5), (13, 5), (14, 5), (15, 5), (16, 5), (17, 5), (18, 5), (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (6, 6), (7, 6), (8, 6), (9, 6), (10, 6), (11, 6), (12, 6), (13, 6), (14, 6), (15, 6), (16, 6), (17, 6), (18, 6), (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7), (8, 7), (9, 7), (10, 7), (11, 7), (12, 7), (13, 7), (14, 7), (15, 7), (16, 7), (17, 7), (18, 7), (0, 8), (1, 8), (2, 8), (3, 8), (4, 8), (5, 8), (6, 8), (7, 8), (8, 8), (9, 8), (10, 8), (11, 8), (12, 8), (13, 8), (14, 8), (15, 8), (16, 8), (17, 8), (18, 8), (0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9), (6, 9), (7, 9), (8, 9), (10, 9), (11, 9), (12, 9), (13, 9), (14, 9), (15, 9), (16, 9), (17, 9), (18, 9), (0, 10), (1, 10), (2, 10), (3, 10), (4, 10), (5, 10), (6, 10), (7, 10), (8, 10), (9, 10), (10, 10), (11, 10), (12, 10), (13, 10), (14, 10), (15, 10), (16, 10), (17, 10), (18, 10), (0, 11), (1, 11), (2, 11), (3, 11), (4, 11), (5, 11), (6, 11), (7, 11), (8, 11), (9, 11), (10, 11), (11, 11), (12, 11), (13, 11), (14, 11), (15, 11), (16, 11), (17, 11), (18, 11), (0, 12), (1, 12), (2, 12), (3, 12), (4, 12), (5, 12), (6, 12), (7, 12), (8, 12), (9, 12), (10, 12), (11, 12), (12, 12), (13, 12), (14, 12), (15, 12), (16, 12), (17, 12), (18, 12), (0, 13), (1, 13), (2, 13), (3, 13), (4, 13), (5, 13), (6, 13), (7, 13), (8, 13), (9, 13), (10, 13), (11, 13), (12, 13), (13, 13), (14, 13), (15, 13), (16, 13), (17, 13), (18, 13), (0, 14), (1, 14), (2, 14), (3, 14), (4, 14), (5, 14), (6, 14), (7, 14), (8, 14), (9, 14), (10, 14), (11, 14), (12, 14), (13, 14), (14, 14), (15, 14), (16, 14), (17, 14), (18, 14), (0, 15), (1, 15), (2, 15), (3, 15), (4, 15), (5, 15), (6, 15), (7, 15), (8, 15), (9, 15), (10, 15), (11, 15), (12, 15), (13, 15), (14, 15), (15, 15), (16, 15), (17, 15), (18, 15), (0, 16), (1, 16), (2, 16), (3, 16), (4, 16), (5, 16), (6, 16), (7, 16), (8, 16), (9, 16), (10, 16), (11, 16), (12, 16), (13, 16), (14, 16), (15, 16), (16, 16), (17, 16), (18, 16), (0, 17), (1, 17), (2, 17), (3, 17), (4, 17), (5, 17), (6, 17), (7, 17), (8, 17), (9, 17), (10, 17), (11, 17), (12, 17), (13, 17), (14, 17), (15, 17), (16, 17), (17, 17), (18, 17), (0, 18), (1, 18), (2, 18), (3, 18), (4, 18), (5, 18), (6, 18), (7, 18), (8, 18), (9, 18), (10, 18), (11, 18), (12, 18), (13, 18), (14, 18), (15, 18), (16, 18), (17, 18), (18, 18)];
        assert_eq!(result, expected);
    }

    #[test]
    fn possible_moves_for_player_test() {
        let encoded = String::from("PL[B]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);
        let expected = Move {
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
        assert_eq!(result.len(), 361);
        assert_eq!(result[0], expected);
    }

    #[test]
    fn possible_moves_for_player_captures_test() {
        let encoded = String::from("PL[B]AB[ba][ab][cb]AW[bb][cc]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let result = game_state.possible_moves_for_player(1);
        let expected = Move {
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
        assert_eq!(result.len(), 356);
        match result.iter().find(|m| m.x == 1 && m.y == 2) {
            Some(m) => assert_eq!(m, &expected),
            None => assert!(false, "expected point")
        }
    }

    #[test]
    fn perform_move_test() {
        let encoded = String::from("PL[B]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let x = 3;
        let y = 3;
        let mov = Move { x, y, simplified_game_state: simplify(&game_state.points), captures: vec![] };
        match game_state.perform_move(&mov) {
            Ok(_) => {
                match game_state.points.iter().flatten().find(|p| p.x == x && p.y == y) {
                    Some(p) => {
                        if p.player_number != 0 {
                            assert_eq!(p.player_number, 1);
                            assert_eq!(p.chain_id, 1);
                            assert_eq!(game_state.current_player_number, 2);
                        } else {
                            assert!(false, "expected stone")
                        }
                    },
                    None => assert!(false, "expected point")
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
        let mov = Move { x, y, simplified_game_state: simplify(&game_state.points), captures: vec![] };
        match game_state.perform_move(&mov) {
            Ok(_) => {
                // placed stone
                match game_state.points.iter().flatten().find(|p| p.x == x && p.y == y) {
                    Some(p) => {
                        if p.player_number != 0 {
                            assert_eq!(p.player_number, 1);
                            assert_eq!(p.chain_id, 6);
                            assert_eq!(game_state.current_player_number, 2);
                        } else {
                            assert!(false, "expected stone")
                        }
                    },
                    None => assert!(false, "expected point")
                }

                // capotured stone
                match game_state.points.iter().flatten().find(|p| p.x == 1 && p.y == 1) {
                    Some(p) => assert_eq!(p.player_number, 0),
                    None => assert!(false, "expected point")
                }

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
    fn undo_move_test() {
        let encoded = String::from("PL[W]AB[ba]XB[0]XW[0]");
        let mut game_state = parse(&encoded).unwrap();
        let x = 2;
        let y = 1;
        let old_encoded = String::from("PL[B]XB[0]XW[0]");
        let old_game_state = parse(&old_encoded).unwrap();
        let mov = Move { x, y, simplified_game_state: simplify(&old_game_state.points), captures: vec![] };
        match game_state.undo_move(&mov) {
            Ok(_) => {
                match game_state.points.iter().flatten().find(|p| p.x == x && p.y == y) {
                    Some(p) => assert_eq!(p.player_number, 0),
                    None => assert!(false, "expected point")
                }
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
        let mov = Move { x, y, simplified_game_state: simplify(&old_game_state.points), captures: vec![(1, 1)] };
        match game_state.undo_move(&mov) {
            Ok(_) => {
                // remove last placed stone
                match game_state.points.iter().flatten().find(|p| p.x == x && p.y == y) {
                    Some(p) => assert_eq!(p.player_number, 0),
                    None => assert!(false, "expected point")
                }

                // add last captured stones
                match game_state.points.iter().flatten().find(|p| p.x == 1 && p.y == 1) {
                    Some(p) => {
                        if p.player_number != 0 {
                            assert_eq!(p.player_number, 2);
                            assert_eq!(p.chain_id, 5);
                        } else {
                            assert!(false, "expected stone")
                        }
                    },
                    None => assert!(false, "expected point")
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
    fn update_joined_chains_test() {
        let encoded = String::from("PL[B]AB[aa][ac]AW[bb]XB[0]XW[0]");
        let chain_id = 3;
        let mut game_state = parse(&encoded).unwrap();
        match game_state.update_joined_chains(0, 1, chain_id, 1) {
            Ok(_) => {
                match game_state.points.iter().flatten().find(|p| p.x == 0 && p.y == 0 ) {
                     Some(p) => {
                         if p.player_number != 0 {
                             assert_eq!(p.chain_id, chain_id)
                         } else {
                             assert!(false, "expected stone")
                         }
                     },
                     None => assert!(false, "expected point")
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
