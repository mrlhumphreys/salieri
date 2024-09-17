use crate::go::state::stone::Stone;
use crate::go::state::point::Point;
use crate::go::state::point_set::adjacent_to_x_and_y;
use crate::go::state::point_set::players_stones_adjacent_to_x_and_y;
use crate::go::state::point_set::chain_ids;
use crate::go::state::point_set::add_stone;
use crate::go::state::point_set::remove_captured_stones;
use crate::go::state::point_set::filter_by_chain_id;
use crate::go::state::point_set::populate_chains;
use crate::go::state::point_set::simplify;
use crate::go::state::point_set::max_chain_id;
use crate::go::state::point_set::max_stone_id;
use crate::go::state::point_set::mark_territories;
use crate::go::state::point_set::players_territory_count;
use crate::go::state::mov::Move;
use crate::go::state::player_stat::PlayerStat;

#[derive(Clone)]
pub struct GameState {
    pub current_player_number: i8,
    pub points: Vec<Point>,
    pub previous_state: Vec<Vec<i8>>,
    pub player_stats: Vec<PlayerStat>,
    pub max_stone_id: i8
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

        for point in self.points.iter() {
            if point.stone.is_none() {
                let adj = adjacent_to_x_and_y(&self.points, point.x, point.y);

                if adj.iter().any(|p| p.stone.is_none()) { // < 4
                    // point has at least one liberty
                    let mut new_state = self.points.clone();
                    match add_stone(&mut new_state, point.x, point.y, subject_player_number, self.max_stone_id) {
                        Ok(_) => {
                            match remove_captured_stones(&mut new_state, point.x, point.y, self.opposing_player_number()) {
                                Ok(captures) => {
                                    let mov = Move {
                                        x: point.x,
                                        y: point.y,
                                        simplified_game_state: simplify(&self.points),
                                        captures
                                    };
                                    moves.push(mov);
                                },
                                Err(_) => ()
                            }
                        },
                        Err(_) => ()
                    }
                } else {
                    // point technically has no liberties
                    let mut friendly_chain_ids = vec![];

                    for p in adj.iter() {
                        if let Some(s) = &p.stone {
                            // friendly stones
                            if s.player_number == subject_player_number {
                                friendly_chain_ids.push(s.chain_id);
                            }
                        }
                    } // 2-4

                    let friendly_chain_has_two_liberties = friendly_chain_ids.iter().any(|cid| {
                        let chain_points = filter_by_chain_id(&self.points, *cid);
                        let mut liberty_count: i8 = 0;
                        let mut two_liberties = false;
                        for p in chain_points.iter() {
                            let adjacent = adjacent_to_x_and_y(&self.points, p.x, p.y);
                            for a in adjacent.iter() {
                                if a.stone.is_none() {
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

                        match add_stone(&mut new_state, point.x, point.y, subject_player_number, self.max_stone_id) {
                            Ok(_) => {
                                match remove_captured_stones(&mut new_state, point.x, point.y, self.opposing_player_number()) {
                                    Ok(captures) => {
                                        let mov = Move {
                                            x: point.x,
                                            y: point.y,
                                            simplified_game_state: simplify(&self.points),
                                            captures
                                        };
                                        moves.push(mov);
                                    },
                                    Err(_) => ()
                                }
                            },
                            Err(_) => ()
                        }
                    } else {
                        let mut enemy_chain_ids = vec![];

                        for p in adj.iter() {
                            if let Some(s) = &p.stone {
                                // enemy stones
                                if s.player_number != subject_player_number {
                                    enemy_chain_ids.push(s.chain_id);
                                }
                            }
                        } // N

                        let enemy_chain_has_one_liberty = enemy_chain_ids.iter().any(|cid| {
                            let chain_points = filter_by_chain_id(&self.points, *cid); // N
                            let mut one_liberty = false;

                            for p in chain_points.iter() {
                                let adjacent = adjacent_to_x_and_y(&self.points, p.x, p.y);  // N
                                for a in adjacent.iter() {
                                    if a.stone.is_none() {
                                        one_liberty = true;
                                    }
                                } // 1-4
                                if one_liberty {
                                    break;
                                }
                            } // < N

                            one_liberty
                        }); // 0-4

                        let mut new_state = self.points.clone(); // clone

                        match add_stone(&mut new_state, point.x, point.y, subject_player_number, self.max_stone_id) {
                            Ok(_) => {
                                let doesnt_repeat_previous_state = self.previous_state != simplify(&new_state);
                                if enemy_chain_has_one_liberty && doesnt_repeat_previous_state {
                                    match remove_captured_stones(&mut new_state, point.x, point.y, self.opposing_player_number()) {
                                        Ok(captures) => {
                                            // point is adjacent to enemy chain with currently 1 liberties
                                            // && adding stone doesn't repeat previous state
                                            let mov = Move {
                                                x: point.x,
                                                y: point.y,
                                                simplified_game_state: simplify(&self.points),
                                                captures
                                            };
                                            moves.push(mov);
                                        },
                                        Err(_) => ()
                                    }
                                }
                            },
                            Err(_) => ()
                        };
                    }
                }
            }
        }

        moves
    }

    pub fn perform_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        self.previous_state = simplify(&self.points);
        match add_stone(&mut self.points, mov.x, mov.y, self.current_player_number, self.max_stone_id) {
            Ok(chain_id) => {
                if let Err(e) = self.update_joined_chains(mov.x, mov.y, chain_id, self.current_player_number) {
                    return Err(e)
                }
                let opposing_player_number = self.opposing_player_number();
                match remove_captured_stones(&mut self.points, mov.x, mov.y, opposing_player_number) {
                    Ok(stones_captured) => {
                        if let Err(e) = self.update_player_stats(self.current_player_number, stones_captured.len() as i8) {
                            return Err(e)
                        }
                    },
                    Err(e) => return Err(e)
                }
                self.max_stone_id += 1;
                self.current_player_number = self.opposing_player_number();
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    pub fn undo_move(&mut self, mov: &Move) -> Result<(), &'static str> {
        let other_player_number = self.opposing_player_number();
        self.previous_state = mov.simplified_game_state.clone(); // Clone

        // undo move points
        // let mut points = self.points.clone();
        // remove piece at x, y
        match self.points.iter_mut().find(|p| p.x == mov.x && p.y == mov.y ) {
            Some(p) => p.stone = None,
            None => return Err("No point found")
        } // < N
        // add captured stones of current player number (will change player number later)
        let chain_id = max_chain_id(&self.points);
        let mut stone_id = max_stone_id(&self.points);
        for p in self.points.iter_mut() {
            if mov.captures.contains(&(p.x, p.y)) {
                p.stone = Some(Stone { id: stone_id, player_number: self.current_player_number, chain_id });
                stone_id += 1;
            }
        } // N

        // update max stone id
        self.max_stone_id = stone_id;

        // update player stats - reduce prisoner count
        match self.player_stats.iter_mut().find(|ps| ps.player_number == other_player_number) {
            Some(ps) => {
                ps.prisoner_count -= mov.captures.len() as i8;
            },
            None => return Err("No player stat found")
        }

        // change player number
        self.current_player_number = other_player_number;
        Ok(())
    }

    // set stones chain id where current chain id is adjacent to point
    pub fn update_joined_chains(&mut self, x: i8, y: i8, chain_id: i8, player_number: i8) -> Result<(), &'static str> {

        let adj = players_stones_adjacent_to_x_and_y(&self.points, x, y, player_number); // N
        let adjacent_chain_ids = chain_ids(adj);

        self.points.iter_mut().for_each(|p| {
            match &mut p.stone {
                Some(s) => {
                    if adjacent_chain_ids.iter().any(|cid| *cid == s.chain_id) {
                        s.chain_id = chain_id;
                    }
                },
                None => ()
            }
        }); // N
        Ok(())
    }

    pub fn update_player_stats(&mut self, player_number: i8, stones_captured: i8) -> Result<(), &'static str> {
        match self.player_stats.iter_mut().find(|ps| ps.player_number == player_number) {
            Some(ps) => {
                ps.prisoner_count = stones_captured;
                Ok(())
            },
            None => Err("No stats for player")
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
    let mut id: i8 = 0;

    let mut points = vec![];
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
                        stone: Some(Stone { id, player_number: stone_player_number, chain_id: 0 }),
                        territory_id: None
                    };
                    points.push(point);
                    id += 1;

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

    // fill in unoccupied points
    let occupied_point_coordinates = points.iter().map(|p| (p.x, p.y)).collect::<Vec<(i8, i8)>>();

    for x in 0..19 {
        for y in 0..19 {
           if !occupied_point_coordinates.iter().any(|o| *o == (x,y)) {
              let point = Point { x, y, stone: None, territory_id: None };
              points.push(point);
           }
        }
    }

    // build previous state
    let mut previous_state = simplify(&points);
    let other_player_number = match current_player_number {
        1 => 2,
        _ => 1
    };

    // add captured stones (not owned by current player)
    // and remove previous stone
    for x in 0..19 {
        for y in 0..19 {
            previous_captures.iter().for_each(|capture| {
                if x == capture.0 as usize && y == capture.1 as usize {
                    previous_state[y][x] = other_player_number;
                }

                if x == previous_player_last_stone_x as usize && y == previous_player_last_stone_y as usize {
                    previous_state[y][x] = 0;
                }
            });
        }
    }

    let max_stone_id = id;

    let game_state  = GameState {
        current_player_number,
        points,
        previous_state,
        player_stats,
        max_stone_id
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
            Point { x: 0, y: 0, stone: None, territory_id: None },
            Point { x: 0, y: 1, stone: None, territory_id: None },
            Point { x: 0, y: 2, stone: None, territory_id: None },
            Point { x: 0, y: 3, stone: None, territory_id: None },
            Point { x: 0, y: 4, stone: None, territory_id: None },
            Point { x: 0, y: 5, stone: None, territory_id: None },
            Point { x: 0, y: 6, stone: None, territory_id: None },
            Point { x: 0, y: 7, stone: None, territory_id: None },
            Point { x: 0, y: 8, stone: None, territory_id: None },
            Point { x: 0, y: 9, stone: None, territory_id: None },
            Point { x: 0, y: 10, stone: None, territory_id: None },
            Point { x: 0, y: 11, stone: None, territory_id: None },
            Point { x: 0, y: 12, stone: None, territory_id: None },
            Point { x: 0, y: 13, stone: None, territory_id: None },
            Point { x: 0, y: 14, stone: None, territory_id: None },
            Point { x: 0, y: 15, stone: None, territory_id: None },
            Point { x: 0, y: 16, stone: None, territory_id: None },
            Point { x: 0, y: 17, stone: None, territory_id: None },
            Point { x: 0, y: 18, stone: None, territory_id: None },

            Point { x: 1, y: 0, stone: None, territory_id: None },
            Point { x: 1, y: 1, stone: None, territory_id: None },
            Point { x: 1, y: 2, stone: None, territory_id: None },
            Point { x: 1, y: 3, stone: None, territory_id: None },
            Point { x: 1, y: 4, stone: None, territory_id: None },
            Point { x: 1, y: 5, stone: None, territory_id: None },
            Point { x: 1, y: 6, stone: None, territory_id: None },
            Point { x: 1, y: 7, stone: None, territory_id: None },
            Point { x: 1, y: 8, stone: None, territory_id: None },
            Point { x: 1, y: 9, stone: None, territory_id: None },
            Point { x: 1, y: 10, stone: None, territory_id: None },
            Point { x: 1, y: 11, stone: None, territory_id: None },
            Point { x: 1, y: 12, stone: None, territory_id: None },
            Point { x: 1, y: 13, stone: None, territory_id: None },
            Point { x: 1, y: 14, stone: None, territory_id: None },
            Point { x: 1, y: 15, stone: None, territory_id: None },
            Point { x: 1, y: 16, stone: None, territory_id: None },
            Point { x: 1, y: 17, stone: None, territory_id: None },
            Point { x: 1, y: 18, stone: None, territory_id: None },

            Point { x: 2, y: 0, stone: None, territory_id: None },
            Point { x: 2, y: 1, stone: None, territory_id: None },
            Point { x: 2, y: 2, stone: None, territory_id: None },
            Point { x: 2, y: 3, stone: None, territory_id: None },
            Point { x: 2, y: 4, stone: None, territory_id: None },
            Point { x: 2, y: 5, stone: None, territory_id: None },
            Point { x: 2, y: 6, stone: None, territory_id: None },
            Point { x: 2, y: 7, stone: None, territory_id: None },
            Point { x: 2, y: 8, stone: None, territory_id: None },
            Point { x: 2, y: 9, stone: None, territory_id: None },
            Point { x: 2, y: 10, stone: None, territory_id: None },
            Point { x: 2, y: 11, stone: None, territory_id: None },
            Point { x: 2, y: 12, stone: None, territory_id: None },
            Point { x: 2, y: 13, stone: None, territory_id: None },
            Point { x: 2, y: 14, stone: None, territory_id: None },
            Point { x: 2, y: 15, stone: None, territory_id: None },
            Point { x: 2, y: 16, stone: None, territory_id: None },
            Point { x: 2, y: 17, stone: None, territory_id: None },
            Point { x: 2, y: 18, stone: None, territory_id: None },

            Point { x: 3, y: 0, stone: None, territory_id: None },
            Point { x: 3, y: 1, stone: None, territory_id: None },
            Point { x: 3, y: 2, stone: None, territory_id: None },
            Point { x: 3, y: 3, stone: None, territory_id: None },
            Point { x: 3, y: 4, stone: None, territory_id: None },
            Point { x: 3, y: 5, stone: None, territory_id: None },
            Point { x: 3, y: 6, stone: None, territory_id: None },
            Point { x: 3, y: 7, stone: None, territory_id: None },
            Point { x: 3, y: 8, stone: None, territory_id: None },
            Point { x: 3, y: 9, stone: None, territory_id: None },
            Point { x: 3, y: 10, stone: None, territory_id: None },
            Point { x: 3, y: 11, stone: None, territory_id: None },
            Point { x: 3, y: 12, stone: None, territory_id: None },
            Point { x: 3, y: 13, stone: None, territory_id: None },
            Point { x: 3, y: 14, stone: None, territory_id: None },
            Point { x: 3, y: 15, stone: None, territory_id: None },
            Point { x: 3, y: 16, stone: None, territory_id: None },
            Point { x: 3, y: 17, stone: None, territory_id: None },
            Point { x: 3, y: 18, stone: None, territory_id: None },

            Point { x: 4, y: 0, stone: None, territory_id: None },
            Point { x: 4, y: 1, stone: None, territory_id: None },
            Point { x: 4, y: 2, stone: None, territory_id: None },
            Point { x: 4, y: 3, stone: None, territory_id: None },
            Point { x: 4, y: 4, stone: None, territory_id: None },
            Point { x: 4, y: 5, stone: None, territory_id: None },
            Point { x: 4, y: 6, stone: None, territory_id: None },
            Point { x: 4, y: 7, stone: None, territory_id: None },
            Point { x: 4, y: 8, stone: None, territory_id: None },
            Point { x: 4, y: 9, stone: None, territory_id: None },
            Point { x: 4, y: 10, stone: None, territory_id: None },
            Point { x: 4, y: 11, stone: None, territory_id: None },
            Point { x: 4, y: 12, stone: None, territory_id: None },
            Point { x: 4, y: 13, stone: None, territory_id: None },
            Point { x: 4, y: 14, stone: None, territory_id: None },
            Point { x: 4, y: 15, stone: None, territory_id: None },
            Point { x: 4, y: 16, stone: None, territory_id: None },
            Point { x: 4, y: 17, stone: None, territory_id: None },
            Point { x: 4, y: 18, stone: None, territory_id: None },

            Point { x: 5, y: 0, stone: None, territory_id: None },
            Point { x: 5, y: 1, stone: None, territory_id: None },
            Point { x: 5, y: 2, stone: None, territory_id: None },
            Point { x: 5, y: 3, stone: None, territory_id: None },
            Point { x: 5, y: 4, stone: None, territory_id: None },
            Point { x: 5, y: 5, stone: None, territory_id: None },
            Point { x: 5, y: 6, stone: None, territory_id: None },
            Point { x: 5, y: 7, stone: None, territory_id: None },
            Point { x: 5, y: 8, stone: None, territory_id: None },
            Point { x: 5, y: 9, stone: None, territory_id: None },
            Point { x: 5, y: 10, stone: None, territory_id: None },
            Point { x: 5, y: 11, stone: None, territory_id: None },
            Point { x: 5, y: 12, stone: None, territory_id: None },
            Point { x: 5, y: 13, stone: None, territory_id: None },
            Point { x: 5, y: 14, stone: None, territory_id: None },
            Point { x: 5, y: 15, stone: None, territory_id: None },
            Point { x: 5, y: 16, stone: None, territory_id: None },
            Point { x: 5, y: 17, stone: None, territory_id: None },
            Point { x: 5, y: 18, stone: None, territory_id: None },

            Point { x: 6, y: 0, stone: None, territory_id: None },
            Point { x: 6, y: 1, stone: None, territory_id: None },
            Point { x: 6, y: 2, stone: None, territory_id: None },
            Point { x: 6, y: 3, stone: None, territory_id: None },
            Point { x: 6, y: 4, stone: None, territory_id: None },
            Point { x: 6, y: 5, stone: None, territory_id: None },
            Point { x: 6, y: 6, stone: None, territory_id: None },
            Point { x: 6, y: 7, stone: None, territory_id: None },
            Point { x: 6, y: 8, stone: None, territory_id: None },
            Point { x: 6, y: 9, stone: None, territory_id: None },
            Point { x: 6, y: 10, stone: None, territory_id: None },
            Point { x: 6, y: 11, stone: None, territory_id: None },
            Point { x: 6, y: 12, stone: None, territory_id: None },
            Point { x: 6, y: 13, stone: None, territory_id: None },
            Point { x: 6, y: 14, stone: None, territory_id: None },
            Point { x: 6, y: 15, stone: None, territory_id: None },
            Point { x: 6, y: 16, stone: None, territory_id: None },
            Point { x: 6, y: 17, stone: None, territory_id: None },
            Point { x: 6, y: 18, stone: None, territory_id: None },

            Point { x: 7, y: 0, stone: None, territory_id: None },
            Point { x: 7, y: 1, stone: None, territory_id: None },
            Point { x: 7, y: 2, stone: None, territory_id: None },
            Point { x: 7, y: 3, stone: None, territory_id: None },
            Point { x: 7, y: 4, stone: None, territory_id: None },
            Point { x: 7, y: 5, stone: None, territory_id: None },
            Point { x: 7, y: 6, stone: None, territory_id: None },
            Point { x: 7, y: 7, stone: None, territory_id: None },
            Point { x: 7, y: 8, stone: None, territory_id: None },
            Point { x: 7, y: 9, stone: None, territory_id: None },
            Point { x: 7, y: 10, stone: None, territory_id: None },
            Point { x: 7, y: 11, stone: None, territory_id: None },
            Point { x: 7, y: 12, stone: None, territory_id: None },
            Point { x: 7, y: 13, stone: None, territory_id: None },
            Point { x: 7, y: 14, stone: None, territory_id: None },
            Point { x: 7, y: 15, stone: None, territory_id: None },
            Point { x: 7, y: 16, stone: None, territory_id: None },
            Point { x: 7, y: 17, stone: None, territory_id: None },
            Point { x: 7, y: 18, stone: None, territory_id: None },

            Point { x: 8, y: 0, stone: None, territory_id: None },
            Point { x: 8, y: 1, stone: None, territory_id: None },
            Point { x: 8, y: 2, stone: None, territory_id: None },
            Point { x: 8, y: 3, stone: None, territory_id: None },
            Point { x: 8, y: 4, stone: None, territory_id: None },
            Point { x: 8, y: 5, stone: None, territory_id: None },
            Point { x: 8, y: 6, stone: None, territory_id: None },
            Point { x: 8, y: 7, stone: None, territory_id: None },
            Point { x: 8, y: 8, stone: None, territory_id: None },
            Point { x: 8, y: 9, stone: None, territory_id: None },
            Point { x: 8, y: 10, stone: None, territory_id: None },
            Point { x: 8, y: 11, stone: None, territory_id: None },
            Point { x: 8, y: 12, stone: None, territory_id: None },
            Point { x: 8, y: 13, stone: None, territory_id: None },
            Point { x: 8, y: 14, stone: None, territory_id: None },
            Point { x: 8, y: 15, stone: None, territory_id: None },
            Point { x: 8, y: 16, stone: None, territory_id: None },
            Point { x: 8, y: 17, stone: None, territory_id: None },
            Point { x: 8, y: 18, stone: None, territory_id: None },

            Point { x: 9, y: 0, stone: None, territory_id: None },
            Point { x: 9, y: 1, stone: None, territory_id: None },
            Point { x: 9, y: 2, stone: None, territory_id: None },
            Point { x: 9, y: 3, stone: None, territory_id: None },
            Point { x: 9, y: 4, stone: None, territory_id: None },
            Point { x: 9, y: 5, stone: None, territory_id: None },
            Point { x: 9, y: 6, stone: None, territory_id: None },
            Point { x: 9, y: 7, stone: None, territory_id: None },
            Point { x: 9, y: 8, stone: None, territory_id: None },
            Point { x: 9, y: 9, stone: None, territory_id: None },
            Point { x: 9, y: 10, stone: None, territory_id: None },
            Point { x: 9, y: 11, stone: None, territory_id: None },
            Point { x: 9, y: 12, stone: None, territory_id: None },
            Point { x: 9, y: 13, stone: None, territory_id: None },
            Point { x: 9, y: 14, stone: None, territory_id: None },
            Point { x: 9, y: 15, stone: None, territory_id: None },
            Point { x: 9, y: 16, stone: None, territory_id: None },
            Point { x: 9, y: 17, stone: None, territory_id: None },
            Point { x: 9, y: 18, stone: None, territory_id: None },

            Point { x: 10, y: 0, stone: None, territory_id: None },
            Point { x: 10, y: 1, stone: None, territory_id: None },
            Point { x: 10, y: 2, stone: None, territory_id: None },
            Point { x: 10, y: 3, stone: None, territory_id: None },
            Point { x: 10, y: 4, stone: None, territory_id: None },
            Point { x: 10, y: 5, stone: None, territory_id: None },
            Point { x: 10, y: 6, stone: None, territory_id: None },
            Point { x: 10, y: 7, stone: None, territory_id: None },
            Point { x: 10, y: 8, stone: None, territory_id: None },
            Point { x: 10, y: 9, stone: None, territory_id: None },
            Point { x: 10, y: 10, stone: None, territory_id: None },
            Point { x: 10, y: 11, stone: None, territory_id: None },
            Point { x: 10, y: 12, stone: None, territory_id: None },
            Point { x: 10, y: 13, stone: None, territory_id: None },
            Point { x: 10, y: 14, stone: None, territory_id: None },
            Point { x: 10, y: 15, stone: None, territory_id: None },
            Point { x: 10, y: 16, stone: None, territory_id: None },
            Point { x: 10, y: 17, stone: None, territory_id: None },
            Point { x: 10, y: 18, stone: None, territory_id: None },

            Point { x: 11, y: 0, stone: None, territory_id: None },
            Point { x: 11, y: 1, stone: None, territory_id: None },
            Point { x: 11, y: 2, stone: None, territory_id: None },
            Point { x: 11, y: 3, stone: None, territory_id: None },
            Point { x: 11, y: 4, stone: None, territory_id: None },
            Point { x: 11, y: 5, stone: None, territory_id: None },
            Point { x: 11, y: 6, stone: None, territory_id: None },
            Point { x: 11, y: 7, stone: None, territory_id: None },
            Point { x: 11, y: 8, stone: None, territory_id: None },
            Point { x: 11, y: 9, stone: None, territory_id: None },
            Point { x: 11, y: 10, stone: None, territory_id: None },
            Point { x: 11, y: 11, stone: None, territory_id: None },
            Point { x: 11, y: 12, stone: None, territory_id: None },
            Point { x: 11, y: 13, stone: None, territory_id: None },
            Point { x: 11, y: 14, stone: None, territory_id: None },
            Point { x: 11, y: 15, stone: None, territory_id: None },
            Point { x: 11, y: 16, stone: None, territory_id: None },
            Point { x: 11, y: 17, stone: None, territory_id: None },
            Point { x: 11, y: 18, stone: None, territory_id: None },

            Point { x: 12, y: 0, stone: None, territory_id: None },
            Point { x: 12, y: 1, stone: None, territory_id: None },
            Point { x: 12, y: 2, stone: None, territory_id: None },
            Point { x: 12, y: 3, stone: None, territory_id: None },
            Point { x: 12, y: 4, stone: None, territory_id: None },
            Point { x: 12, y: 5, stone: None, territory_id: None },
            Point { x: 12, y: 6, stone: None, territory_id: None },
            Point { x: 12, y: 7, stone: None, territory_id: None },
            Point { x: 12, y: 8, stone: None, territory_id: None },
            Point { x: 12, y: 9, stone: None, territory_id: None },
            Point { x: 12, y: 10, stone: None, territory_id: None },
            Point { x: 12, y: 11, stone: None, territory_id: None },
            Point { x: 12, y: 12, stone: None, territory_id: None },
            Point { x: 12, y: 13, stone: None, territory_id: None },
            Point { x: 12, y: 14, stone: None, territory_id: None },
            Point { x: 12, y: 15, stone: None, territory_id: None },
            Point { x: 12, y: 16, stone: None, territory_id: None },
            Point { x: 12, y: 17, stone: None, territory_id: None },
            Point { x: 12, y: 18, stone: None, territory_id: None },

            Point { x: 13, y: 0, stone: None, territory_id: None },
            Point { x: 13, y: 1, stone: None, territory_id: None },
            Point { x: 13, y: 2, stone: None, territory_id: None },
            Point { x: 13, y: 3, stone: None, territory_id: None },
            Point { x: 13, y: 4, stone: None, territory_id: None },
            Point { x: 13, y: 5, stone: None, territory_id: None },
            Point { x: 13, y: 6, stone: None, territory_id: None },
            Point { x: 13, y: 7, stone: None, territory_id: None },
            Point { x: 13, y: 8, stone: None, territory_id: None },
            Point { x: 13, y: 9, stone: None, territory_id: None },
            Point { x: 13, y: 10, stone: None, territory_id: None },
            Point { x: 13, y: 11, stone: None, territory_id: None },
            Point { x: 13, y: 12, stone: None, territory_id: None },
            Point { x: 13, y: 13, stone: None, territory_id: None },
            Point { x: 13, y: 14, stone: None, territory_id: None },
            Point { x: 13, y: 15, stone: None, territory_id: None },
            Point { x: 13, y: 16, stone: None, territory_id: None },
            Point { x: 13, y: 17, stone: None, territory_id: None },
            Point { x: 13, y: 18, stone: None, territory_id: None },

            Point { x: 14, y: 0, stone: None, territory_id: None },
            Point { x: 14, y: 1, stone: None, territory_id: None },
            Point { x: 14, y: 2, stone: None, territory_id: None },
            Point { x: 14, y: 3, stone: None, territory_id: None },
            Point { x: 14, y: 4, stone: None, territory_id: None },
            Point { x: 14, y: 5, stone: None, territory_id: None },
            Point { x: 14, y: 6, stone: None, territory_id: None },
            Point { x: 14, y: 7, stone: None, territory_id: None },
            Point { x: 14, y: 8, stone: None, territory_id: None },
            Point { x: 14, y: 9, stone: None, territory_id: None },
            Point { x: 14, y: 10, stone: None, territory_id: None },
            Point { x: 14, y: 11, stone: None, territory_id: None },
            Point { x: 14, y: 12, stone: None, territory_id: None },
            Point { x: 14, y: 13, stone: None, territory_id: None },
            Point { x: 14, y: 14, stone: None, territory_id: None },
            Point { x: 14, y: 15, stone: None, territory_id: None },
            Point { x: 14, y: 16, stone: None, territory_id: None },
            Point { x: 14, y: 17, stone: None, territory_id: None },
            Point { x: 14, y: 18, stone: None, territory_id: None },

            Point { x: 15, y: 0, stone: None, territory_id: None },
            Point { x: 15, y: 1, stone: None, territory_id: None },
            Point { x: 15, y: 2, stone: None, territory_id: None },
            Point { x: 15, y: 3, stone: None, territory_id: None },
            Point { x: 15, y: 4, stone: None, territory_id: None },
            Point { x: 15, y: 5, stone: None, territory_id: None },
            Point { x: 15, y: 6, stone: None, territory_id: None },
            Point { x: 15, y: 7, stone: None, territory_id: None },
            Point { x: 15, y: 8, stone: None, territory_id: None },
            Point { x: 15, y: 9, stone: None, territory_id: None },
            Point { x: 15, y: 10, stone: None, territory_id: None },
            Point { x: 15, y: 11, stone: None, territory_id: None },
            Point { x: 15, y: 12, stone: None, territory_id: None },
            Point { x: 15, y: 13, stone: None, territory_id: None },
            Point { x: 15, y: 14, stone: None, territory_id: None },
            Point { x: 15, y: 15, stone: None, territory_id: None },
            Point { x: 15, y: 16, stone: None, territory_id: None },
            Point { x: 15, y: 17, stone: None, territory_id: None },
            Point { x: 15, y: 18, stone: None, territory_id: None },

            Point { x: 16, y: 0, stone: None, territory_id: None },
            Point { x: 16, y: 1, stone: None, territory_id: None },
            Point { x: 16, y: 2, stone: None, territory_id: None },
            Point { x: 16, y: 3, stone: None, territory_id: None },
            Point { x: 16, y: 4, stone: None, territory_id: None },
            Point { x: 16, y: 5, stone: None, territory_id: None },
            Point { x: 16, y: 6, stone: None, territory_id: None },
            Point { x: 16, y: 7, stone: None, territory_id: None },
            Point { x: 16, y: 8, stone: None, territory_id: None },
            Point { x: 16, y: 9, stone: None, territory_id: None },
            Point { x: 16, y: 10, stone: None, territory_id: None },
            Point { x: 16, y: 11, stone: None, territory_id: None },
            Point { x: 16, y: 12, stone: None, territory_id: None },
            Point { x: 16, y: 13, stone: None, territory_id: None },
            Point { x: 16, y: 14, stone: None, territory_id: None },
            Point { x: 16, y: 15, stone: None, territory_id: None },
            Point { x: 16, y: 16, stone: None, territory_id: None },
            Point { x: 16, y: 17, stone: None, territory_id: None },
            Point { x: 16, y: 18, stone: None, territory_id: None },

            Point { x: 17, y: 0, stone: None, territory_id: None },
            Point { x: 17, y: 1, stone: None, territory_id: None },
            Point { x: 17, y: 2, stone: None, territory_id: None },
            Point { x: 17, y: 3, stone: None, territory_id: None },
            Point { x: 17, y: 4, stone: None, territory_id: None },
            Point { x: 17, y: 5, stone: None, territory_id: None },
            Point { x: 17, y: 6, stone: None, territory_id: None },
            Point { x: 17, y: 7, stone: None, territory_id: None },
            Point { x: 17, y: 8, stone: None, territory_id: None },
            Point { x: 17, y: 9, stone: None, territory_id: None },
            Point { x: 17, y: 10, stone: None, territory_id: None },
            Point { x: 17, y: 11, stone: None, territory_id: None },
            Point { x: 17, y: 12, stone: None, territory_id: None },
            Point { x: 17, y: 13, stone: None, territory_id: None },
            Point { x: 17, y: 14, stone: None, territory_id: None },
            Point { x: 17, y: 15, stone: None, territory_id: None },
            Point { x: 17, y: 16, stone: None, territory_id: None },
            Point { x: 17, y: 17, stone: None, territory_id: None },
            Point { x: 17, y: 18, stone: None, territory_id: None },

            Point { x: 18, y: 0, stone: None, territory_id: None },
            Point { x: 18, y: 1, stone: None, territory_id: None },
            Point { x: 18, y: 2, stone: None, territory_id: None },
            Point { x: 18, y: 3, stone: None, territory_id: None },
            Point { x: 18, y: 4, stone: None, territory_id: None },
            Point { x: 18, y: 5, stone: None, territory_id: None },
            Point { x: 18, y: 6, stone: None, territory_id: None },
            Point { x: 18, y: 7, stone: None, territory_id: None },
            Point { x: 18, y: 8, stone: None, territory_id: None },
            Point { x: 18, y: 9, stone: None, territory_id: None },
            Point { x: 18, y: 10, stone: None, territory_id: None },
            Point { x: 18, y: 11, stone: None, territory_id: None },
            Point { x: 18, y: 12, stone: None, territory_id: None },
            Point { x: 18, y: 13, stone: None, territory_id: None },
            Point { x: 18, y: 14, stone: None, territory_id: None },
            Point { x: 18, y: 15, stone: None, territory_id: None },
            Point { x: 18, y: 16, stone: None, territory_id: None },
            Point { x: 18, y: 17, stone: None, territory_id: None },
            Point { x: 18, y: 18, stone: None, territory_id: None }
        ];

        assert_eq!(result.current_player_number, 1);
        assert_eq!(result.points, expected_points);
        assert_eq!(result.player_stats, expected_player_stats);
        assert_eq!(result.max_stone_id, 0);
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
            Point { x: 2, y: 1, stone: Some(Stone { id: 0, player_number: 1, chain_id: 1 }), territory_id: None },
            Point { x: 3, y: 4, stone: Some(Stone { id: 1, player_number: 2, chain_id: 2 }), territory_id: None },
            Point { x: 0, y: 0, stone: None, territory_id: None },
            Point { x: 0, y: 1, stone: None, territory_id: None },
            Point { x: 0, y: 2, stone: None, territory_id: None },
            Point { x: 0, y: 3, stone: None, territory_id: None },
            Point { x: 0, y: 4, stone: None, territory_id: None },
            Point { x: 0, y: 5, stone: None, territory_id: None },
            Point { x: 0, y: 6, stone: None, territory_id: None },
            Point { x: 0, y: 7, stone: None, territory_id: None },
            Point { x: 0, y: 8, stone: None, territory_id: None },
            Point { x: 0, y: 9, stone: None, territory_id: None },
            Point { x: 0, y: 10, stone: None, territory_id: None },
            Point { x: 0, y: 11, stone: None, territory_id: None },
            Point { x: 0, y: 12, stone: None, territory_id: None },
            Point { x: 0, y: 13, stone: None, territory_id: None },
            Point { x: 0, y: 14, stone: None, territory_id: None },
            Point { x: 0, y: 15, stone: None, territory_id: None },
            Point { x: 0, y: 16, stone: None, territory_id: None },
            Point { x: 0, y: 17, stone: None, territory_id: None },
            Point { x: 0, y: 18, stone: None, territory_id: None },

            Point { x: 1, y: 0, stone: None, territory_id: None },
            Point { x: 1, y: 1, stone: None, territory_id: None },
            Point { x: 1, y: 2, stone: None, territory_id: None },
            Point { x: 1, y: 3, stone: None, territory_id: None },
            Point { x: 1, y: 4, stone: None, territory_id: None },
            Point { x: 1, y: 5, stone: None, territory_id: None },
            Point { x: 1, y: 6, stone: None, territory_id: None },
            Point { x: 1, y: 7, stone: None, territory_id: None },
            Point { x: 1, y: 8, stone: None, territory_id: None },
            Point { x: 1, y: 9, stone: None, territory_id: None },
            Point { x: 1, y: 10, stone: None, territory_id: None },
            Point { x: 1, y: 11, stone: None, territory_id: None },
            Point { x: 1, y: 12, stone: None, territory_id: None },
            Point { x: 1, y: 13, stone: None, territory_id: None },
            Point { x: 1, y: 14, stone: None, territory_id: None },
            Point { x: 1, y: 15, stone: None, territory_id: None },
            Point { x: 1, y: 16, stone: None, territory_id: None },
            Point { x: 1, y: 17, stone: None, territory_id: None },
            Point { x: 1, y: 18, stone: None, territory_id: None },

            Point { x: 2, y: 0, stone: None, territory_id: None },
            Point { x: 2, y: 2, stone: None, territory_id: None },
            Point { x: 2, y: 3, stone: None, territory_id: None },
            Point { x: 2, y: 4, stone: None, territory_id: None },
            Point { x: 2, y: 5, stone: None, territory_id: None },
            Point { x: 2, y: 6, stone: None, territory_id: None },
            Point { x: 2, y: 7, stone: None, territory_id: None },
            Point { x: 2, y: 8, stone: None, territory_id: None },
            Point { x: 2, y: 9, stone: None, territory_id: None },
            Point { x: 2, y: 10, stone: None, territory_id: None },
            Point { x: 2, y: 11, stone: None, territory_id: None },
            Point { x: 2, y: 12, stone: None, territory_id: None },
            Point { x: 2, y: 13, stone: None, territory_id: None },
            Point { x: 2, y: 14, stone: None, territory_id: None },
            Point { x: 2, y: 15, stone: None, territory_id: None },
            Point { x: 2, y: 16, stone: None, territory_id: None },
            Point { x: 2, y: 17, stone: None, territory_id: None },
            Point { x: 2, y: 18, stone: None, territory_id: None },

            Point { x: 3, y: 0, stone: None, territory_id: None },
            Point { x: 3, y: 1, stone: None, territory_id: None },
            Point { x: 3, y: 2, stone: None, territory_id: None },
            Point { x: 3, y: 3, stone: None, territory_id: None },
            Point { x: 3, y: 5, stone: None, territory_id: None },
            Point { x: 3, y: 6, stone: None, territory_id: None },
            Point { x: 3, y: 7, stone: None, territory_id: None },
            Point { x: 3, y: 8, stone: None, territory_id: None },
            Point { x: 3, y: 9, stone: None, territory_id: None },
            Point { x: 3, y: 10, stone: None, territory_id: None },
            Point { x: 3, y: 11, stone: None, territory_id: None },
            Point { x: 3, y: 12, stone: None, territory_id: None },
            Point { x: 3, y: 13, stone: None, territory_id: None },
            Point { x: 3, y: 14, stone: None, territory_id: None },
            Point { x: 3, y: 15, stone: None, territory_id: None },
            Point { x: 3, y: 16, stone: None, territory_id: None },
            Point { x: 3, y: 17, stone: None, territory_id: None },
            Point { x: 3, y: 18, stone: None, territory_id: None },

            Point { x: 4, y: 0, stone: None, territory_id: None },
            Point { x: 4, y: 1, stone: None, territory_id: None },
            Point { x: 4, y: 2, stone: None, territory_id: None },
            Point { x: 4, y: 3, stone: None, territory_id: None },
            Point { x: 4, y: 4, stone: None, territory_id: None },
            Point { x: 4, y: 5, stone: None, territory_id: None },
            Point { x: 4, y: 6, stone: None, territory_id: None },
            Point { x: 4, y: 7, stone: None, territory_id: None },
            Point { x: 4, y: 8, stone: None, territory_id: None },
            Point { x: 4, y: 9, stone: None, territory_id: None },
            Point { x: 4, y: 10, stone: None, territory_id: None },
            Point { x: 4, y: 11, stone: None, territory_id: None },
            Point { x: 4, y: 12, stone: None, territory_id: None },
            Point { x: 4, y: 13, stone: None, territory_id: None },
            Point { x: 4, y: 14, stone: None, territory_id: None },
            Point { x: 4, y: 15, stone: None, territory_id: None },
            Point { x: 4, y: 16, stone: None, territory_id: None },
            Point { x: 4, y: 17, stone: None, territory_id: None },
            Point { x: 4, y: 18, stone: None, territory_id: None },

            Point { x: 5, y: 0, stone: None, territory_id: None },
            Point { x: 5, y: 1, stone: None, territory_id: None },
            Point { x: 5, y: 2, stone: None, territory_id: None },
            Point { x: 5, y: 3, stone: None, territory_id: None },
            Point { x: 5, y: 4, stone: None, territory_id: None },
            Point { x: 5, y: 5, stone: None, territory_id: None },
            Point { x: 5, y: 6, stone: None, territory_id: None },
            Point { x: 5, y: 7, stone: None, territory_id: None },
            Point { x: 5, y: 8, stone: None, territory_id: None },
            Point { x: 5, y: 9, stone: None, territory_id: None },
            Point { x: 5, y: 10, stone: None, territory_id: None },
            Point { x: 5, y: 11, stone: None, territory_id: None },
            Point { x: 5, y: 12, stone: None, territory_id: None },
            Point { x: 5, y: 13, stone: None, territory_id: None },
            Point { x: 5, y: 14, stone: None, territory_id: None },
            Point { x: 5, y: 15, stone: None, territory_id: None },
            Point { x: 5, y: 16, stone: None, territory_id: None },
            Point { x: 5, y: 17, stone: None, territory_id: None },
            Point { x: 5, y: 18, stone: None, territory_id: None },

            Point { x: 6, y: 0, stone: None, territory_id: None },
            Point { x: 6, y: 1, stone: None, territory_id: None },
            Point { x: 6, y: 2, stone: None, territory_id: None },
            Point { x: 6, y: 3, stone: None, territory_id: None },
            Point { x: 6, y: 4, stone: None, territory_id: None },
            Point { x: 6, y: 5, stone: None, territory_id: None },
            Point { x: 6, y: 6, stone: None, territory_id: None },
            Point { x: 6, y: 7, stone: None, territory_id: None },
            Point { x: 6, y: 8, stone: None, territory_id: None },
            Point { x: 6, y: 9, stone: None, territory_id: None },
            Point { x: 6, y: 10, stone: None, territory_id: None },
            Point { x: 6, y: 11, stone: None, territory_id: None },
            Point { x: 6, y: 12, stone: None, territory_id: None },
            Point { x: 6, y: 13, stone: None, territory_id: None },
            Point { x: 6, y: 14, stone: None, territory_id: None },
            Point { x: 6, y: 15, stone: None, territory_id: None },
            Point { x: 6, y: 16, stone: None, territory_id: None },
            Point { x: 6, y: 17, stone: None, territory_id: None },
            Point { x: 6, y: 18, stone: None, territory_id: None },

            Point { x: 7, y: 0, stone: None, territory_id: None },
            Point { x: 7, y: 1, stone: None, territory_id: None },
            Point { x: 7, y: 2, stone: None, territory_id: None },
            Point { x: 7, y: 3, stone: None, territory_id: None },
            Point { x: 7, y: 4, stone: None, territory_id: None },
            Point { x: 7, y: 5, stone: None, territory_id: None },
            Point { x: 7, y: 6, stone: None, territory_id: None },
            Point { x: 7, y: 7, stone: None, territory_id: None },
            Point { x: 7, y: 8, stone: None, territory_id: None },
            Point { x: 7, y: 9, stone: None, territory_id: None },
            Point { x: 7, y: 10, stone: None, territory_id: None },
            Point { x: 7, y: 11, stone: None, territory_id: None },
            Point { x: 7, y: 12, stone: None, territory_id: None },
            Point { x: 7, y: 13, stone: None, territory_id: None },
            Point { x: 7, y: 14, stone: None, territory_id: None },
            Point { x: 7, y: 15, stone: None, territory_id: None },
            Point { x: 7, y: 16, stone: None, territory_id: None },
            Point { x: 7, y: 17, stone: None, territory_id: None },
            Point { x: 7, y: 18, stone: None, territory_id: None },

            Point { x: 8, y: 0, stone: None, territory_id: None },
            Point { x: 8, y: 1, stone: None, territory_id: None },
            Point { x: 8, y: 2, stone: None, territory_id: None },
            Point { x: 8, y: 3, stone: None, territory_id: None },
            Point { x: 8, y: 4, stone: None, territory_id: None },
            Point { x: 8, y: 5, stone: None, territory_id: None },
            Point { x: 8, y: 6, stone: None, territory_id: None },
            Point { x: 8, y: 7, stone: None, territory_id: None },
            Point { x: 8, y: 8, stone: None, territory_id: None },
            Point { x: 8, y: 9, stone: None, territory_id: None },
            Point { x: 8, y: 10, stone: None, territory_id: None },
            Point { x: 8, y: 11, stone: None, territory_id: None },
            Point { x: 8, y: 12, stone: None, territory_id: None },
            Point { x: 8, y: 13, stone: None, territory_id: None },
            Point { x: 8, y: 14, stone: None, territory_id: None },
            Point { x: 8, y: 15, stone: None, territory_id: None },
            Point { x: 8, y: 16, stone: None, territory_id: None },
            Point { x: 8, y: 17, stone: None, territory_id: None },
            Point { x: 8, y: 18, stone: None, territory_id: None },

            Point { x: 9, y: 0, stone: None, territory_id: None },
            Point { x: 9, y: 1, stone: None, territory_id: None },
            Point { x: 9, y: 2, stone: None, territory_id: None },
            Point { x: 9, y: 3, stone: None, territory_id: None },
            Point { x: 9, y: 4, stone: None, territory_id: None },
            Point { x: 9, y: 5, stone: None, territory_id: None },
            Point { x: 9, y: 6, stone: None, territory_id: None },
            Point { x: 9, y: 7, stone: None, territory_id: None },
            Point { x: 9, y: 8, stone: None, territory_id: None },
            Point { x: 9, y: 9, stone: None, territory_id: None },
            Point { x: 9, y: 10, stone: None, territory_id: None },
            Point { x: 9, y: 11, stone: None, territory_id: None },
            Point { x: 9, y: 12, stone: None, territory_id: None },
            Point { x: 9, y: 13, stone: None, territory_id: None },
            Point { x: 9, y: 14, stone: None, territory_id: None },
            Point { x: 9, y: 15, stone: None, territory_id: None },
            Point { x: 9, y: 16, stone: None, territory_id: None },
            Point { x: 9, y: 17, stone: None, territory_id: None },
            Point { x: 9, y: 18, stone: None, territory_id: None },

            Point { x: 10, y: 0, stone: None, territory_id: None },
            Point { x: 10, y: 1, stone: None, territory_id: None },
            Point { x: 10, y: 2, stone: None, territory_id: None },
            Point { x: 10, y: 3, stone: None, territory_id: None },
            Point { x: 10, y: 4, stone: None, territory_id: None },
            Point { x: 10, y: 5, stone: None, territory_id: None },
            Point { x: 10, y: 6, stone: None, territory_id: None },
            Point { x: 10, y: 7, stone: None, territory_id: None },
            Point { x: 10, y: 8, stone: None, territory_id: None },
            Point { x: 10, y: 9, stone: None, territory_id: None },
            Point { x: 10, y: 10, stone: None, territory_id: None },
            Point { x: 10, y: 11, stone: None, territory_id: None },
            Point { x: 10, y: 12, stone: None, territory_id: None },
            Point { x: 10, y: 13, stone: None, territory_id: None },
            Point { x: 10, y: 14, stone: None, territory_id: None },
            Point { x: 10, y: 15, stone: None, territory_id: None },
            Point { x: 10, y: 16, stone: None, territory_id: None },
            Point { x: 10, y: 17, stone: None, territory_id: None },
            Point { x: 10, y: 18, stone: None, territory_id: None },

            Point { x: 11, y: 0, stone: None, territory_id: None },
            Point { x: 11, y: 1, stone: None, territory_id: None },
            Point { x: 11, y: 2, stone: None, territory_id: None },
            Point { x: 11, y: 3, stone: None, territory_id: None },
            Point { x: 11, y: 4, stone: None, territory_id: None },
            Point { x: 11, y: 5, stone: None, territory_id: None },
            Point { x: 11, y: 6, stone: None, territory_id: None },
            Point { x: 11, y: 7, stone: None, territory_id: None },
            Point { x: 11, y: 8, stone: None, territory_id: None },
            Point { x: 11, y: 9, stone: None, territory_id: None },
            Point { x: 11, y: 10, stone: None, territory_id: None },
            Point { x: 11, y: 11, stone: None, territory_id: None },
            Point { x: 11, y: 12, stone: None, territory_id: None },
            Point { x: 11, y: 13, stone: None, territory_id: None },
            Point { x: 11, y: 14, stone: None, territory_id: None },
            Point { x: 11, y: 15, stone: None, territory_id: None },
            Point { x: 11, y: 16, stone: None, territory_id: None },
            Point { x: 11, y: 17, stone: None, territory_id: None },
            Point { x: 11, y: 18, stone: None, territory_id: None },

            Point { x: 12, y: 0, stone: None, territory_id: None },
            Point { x: 12, y: 1, stone: None, territory_id: None },
            Point { x: 12, y: 2, stone: None, territory_id: None },
            Point { x: 12, y: 3, stone: None, territory_id: None },
            Point { x: 12, y: 4, stone: None, territory_id: None },
            Point { x: 12, y: 5, stone: None, territory_id: None },
            Point { x: 12, y: 6, stone: None, territory_id: None },
            Point { x: 12, y: 7, stone: None, territory_id: None },
            Point { x: 12, y: 8, stone: None, territory_id: None },
            Point { x: 12, y: 9, stone: None, territory_id: None },
            Point { x: 12, y: 10, stone: None, territory_id: None },
            Point { x: 12, y: 11, stone: None, territory_id: None },
            Point { x: 12, y: 12, stone: None, territory_id: None },
            Point { x: 12, y: 13, stone: None, territory_id: None },
            Point { x: 12, y: 14, stone: None, territory_id: None },
            Point { x: 12, y: 15, stone: None, territory_id: None },
            Point { x: 12, y: 16, stone: None, territory_id: None },
            Point { x: 12, y: 17, stone: None, territory_id: None },
            Point { x: 12, y: 18, stone: None, territory_id: None },

            Point { x: 13, y: 0, stone: None, territory_id: None },
            Point { x: 13, y: 1, stone: None, territory_id: None },
            Point { x: 13, y: 2, stone: None, territory_id: None },
            Point { x: 13, y: 3, stone: None, territory_id: None },
            Point { x: 13, y: 4, stone: None, territory_id: None },
            Point { x: 13, y: 5, stone: None, territory_id: None },
            Point { x: 13, y: 6, stone: None, territory_id: None },
            Point { x: 13, y: 7, stone: None, territory_id: None },
            Point { x: 13, y: 8, stone: None, territory_id: None },
            Point { x: 13, y: 9, stone: None, territory_id: None },
            Point { x: 13, y: 10, stone: None, territory_id: None },
            Point { x: 13, y: 11, stone: None, territory_id: None },
            Point { x: 13, y: 12, stone: None, territory_id: None },
            Point { x: 13, y: 13, stone: None, territory_id: None },
            Point { x: 13, y: 14, stone: None, territory_id: None },
            Point { x: 13, y: 15, stone: None, territory_id: None },
            Point { x: 13, y: 16, stone: None, territory_id: None },
            Point { x: 13, y: 17, stone: None, territory_id: None },
            Point { x: 13, y: 18, stone: None, territory_id: None },

            Point { x: 14, y: 0, stone: None, territory_id: None },
            Point { x: 14, y: 1, stone: None, territory_id: None },
            Point { x: 14, y: 2, stone: None, territory_id: None },
            Point { x: 14, y: 3, stone: None, territory_id: None },
            Point { x: 14, y: 4, stone: None, territory_id: None },
            Point { x: 14, y: 5, stone: None, territory_id: None },
            Point { x: 14, y: 6, stone: None, territory_id: None },
            Point { x: 14, y: 7, stone: None, territory_id: None },
            Point { x: 14, y: 8, stone: None, territory_id: None },
            Point { x: 14, y: 9, stone: None, territory_id: None },
            Point { x: 14, y: 10, stone: None, territory_id: None },
            Point { x: 14, y: 11, stone: None, territory_id: None },
            Point { x: 14, y: 12, stone: None, territory_id: None },
            Point { x: 14, y: 13, stone: None, territory_id: None },
            Point { x: 14, y: 14, stone: None, territory_id: None },
            Point { x: 14, y: 15, stone: None, territory_id: None },
            Point { x: 14, y: 16, stone: None, territory_id: None },
            Point { x: 14, y: 17, stone: None, territory_id: None },
            Point { x: 14, y: 18, stone: None, territory_id: None },
            Point { x: 15, y: 0, stone: None, territory_id: None },

            Point { x: 15, y: 1, stone: None, territory_id: None },
            Point { x: 15, y: 2, stone: None, territory_id: None },
            Point { x: 15, y: 3, stone: None, territory_id: None },
            Point { x: 15, y: 4, stone: None, territory_id: None },
            Point { x: 15, y: 5, stone: None, territory_id: None },
            Point { x: 15, y: 6, stone: None, territory_id: None },
            Point { x: 15, y: 7, stone: None, territory_id: None },
            Point { x: 15, y: 8, stone: None, territory_id: None },
            Point { x: 15, y: 9, stone: None, territory_id: None },
            Point { x: 15, y: 10, stone: None, territory_id: None },
            Point { x: 15, y: 11, stone: None, territory_id: None },
            Point { x: 15, y: 12, stone: None, territory_id: None },
            Point { x: 15, y: 13, stone: None, territory_id: None },
            Point { x: 15, y: 14, stone: None, territory_id: None },
            Point { x: 15, y: 15, stone: None, territory_id: None },
            Point { x: 15, y: 16, stone: None, territory_id: None },
            Point { x: 15, y: 17, stone: None, territory_id: None },
            Point { x: 15, y: 18, stone: None, territory_id: None },

            Point { x: 16, y: 0, stone: None, territory_id: None },
            Point { x: 16, y: 1, stone: None, territory_id: None },
            Point { x: 16, y: 2, stone: None, territory_id: None },
            Point { x: 16, y: 3, stone: None, territory_id: None },
            Point { x: 16, y: 4, stone: None, territory_id: None },
            Point { x: 16, y: 5, stone: None, territory_id: None },
            Point { x: 16, y: 6, stone: None, territory_id: None },
            Point { x: 16, y: 7, stone: None, territory_id: None },
            Point { x: 16, y: 8, stone: None, territory_id: None },
            Point { x: 16, y: 9, stone: None, territory_id: None },
            Point { x: 16, y: 10, stone: None, territory_id: None },
            Point { x: 16, y: 11, stone: None, territory_id: None },
            Point { x: 16, y: 12, stone: None, territory_id: None },
            Point { x: 16, y: 13, stone: None, territory_id: None },
            Point { x: 16, y: 14, stone: None, territory_id: None },
            Point { x: 16, y: 15, stone: None, territory_id: None },
            Point { x: 16, y: 16, stone: None, territory_id: None },
            Point { x: 16, y: 17, stone: None, territory_id: None },
            Point { x: 16, y: 18, stone: None, territory_id: None },

            Point { x: 17, y: 0, stone: None, territory_id: None },
            Point { x: 17, y: 1, stone: None, territory_id: None },
            Point { x: 17, y: 2, stone: None, territory_id: None },
            Point { x: 17, y: 3, stone: None, territory_id: None },
            Point { x: 17, y: 4, stone: None, territory_id: None },
            Point { x: 17, y: 5, stone: None, territory_id: None },
            Point { x: 17, y: 6, stone: None, territory_id: None },
            Point { x: 17, y: 7, stone: None, territory_id: None },
            Point { x: 17, y: 8, stone: None, territory_id: None },
            Point { x: 17, y: 9, stone: None, territory_id: None },
            Point { x: 17, y: 10, stone: None, territory_id: None },
            Point { x: 17, y: 11, stone: None, territory_id: None },
            Point { x: 17, y: 12, stone: None, territory_id: None },
            Point { x: 17, y: 13, stone: None, territory_id: None },
            Point { x: 17, y: 14, stone: None, territory_id: None },
            Point { x: 17, y: 15, stone: None, territory_id: None },
            Point { x: 17, y: 16, stone: None, territory_id: None },
            Point { x: 17, y: 17, stone: None, territory_id: None },
            Point { x: 17, y: 18, stone: None, territory_id: None },

            Point { x: 18, y: 0, stone: None, territory_id: None },
            Point { x: 18, y: 1, stone: None, territory_id: None },
            Point { x: 18, y: 2, stone: None, territory_id: None },
            Point { x: 18, y: 3, stone: None, territory_id: None },
            Point { x: 18, y: 4, stone: None, territory_id: None },
            Point { x: 18, y: 5, stone: None, territory_id: None },
            Point { x: 18, y: 6, stone: None, territory_id: None },
            Point { x: 18, y: 7, stone: None, territory_id: None },
            Point { x: 18, y: 8, stone: None, territory_id: None },
            Point { x: 18, y: 9, stone: None, territory_id: None },
            Point { x: 18, y: 10, stone: None, territory_id: None },
            Point { x: 18, y: 11, stone: None, territory_id: None },
            Point { x: 18, y: 12, stone: None, territory_id: None },
            Point { x: 18, y: 13, stone: None, territory_id: None },
            Point { x: 18, y: 14, stone: None, territory_id: None },
            Point { x: 18, y: 15, stone: None, territory_id: None },
            Point { x: 18, y: 16, stone: None, territory_id: None },
            Point { x: 18, y: 17, stone: None, territory_id: None },
            Point { x: 18, y: 18, stone: None, territory_id: None }
        ];
        assert_eq!(result.current_player_number, 1);
        assert_eq!(result.points, expected_points);
        assert_eq!(result.player_stats, expected_player_stats);
        assert_eq!(result.max_stone_id, 2);
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
        assert_eq!(result.max_stone_id, 2);
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
            [ 0,0,0,0,2, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],

            [ 0,0,0,0,2, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0 ],
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
                match game_state.points.iter().find(|p| p.x == x && p.y == y) {
                    Some(p) => {
                        match &p.stone {
                            Some(s) => {
                                let expected_stone = Stone { id: 1, player_number: 1, chain_id: 1 };
                                assert_eq!(s, &expected_stone);
                                assert_eq!(game_state.current_player_number, 2);
                            },
                            None => assert!(false, "expected stone")
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
                match game_state.points.iter().find(|p| p.x == x && p.y == y) {
                    Some(p) => {
                        match &p.stone {
                            Some(s) => {
                                let expected_stone = Stone { id: 6, player_number: 1, chain_id: 6 };
                                assert_eq!(s, &expected_stone);
                                assert_eq!(game_state.current_player_number, 2);
                            },
                            None => assert!(false, "expected stone")
                        }
                    },
                    None => assert!(false, "expected point")
                }

                // capotured stone
                match game_state.points.iter().find(|p| p.x == 1 && p.y == 1) {
                    Some(p) => assert_eq!(p.stone, None),
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
                match game_state.points.iter().find(|p| p.x == x && p.y == y) {
                    Some(p) => assert_eq!(p.stone, None),
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
                match game_state.points.iter().find(|p| p.x == x && p.y == y) {
                    Some(p) => assert_eq!(p.stone, None),
                    None => assert!(false, "expected point")
                }

                // add last captured stones
                match game_state.points.iter().find(|p| p.x == 1 && p.y == 1) {
                    Some(p) => {
                        match &p.stone {
                            Some(s) => {
                                let expected_stone = Stone { id: 4, player_number: 2, chain_id: 5 };
                                assert_eq!(s, &expected_stone)
                            },
                            None => assert!(false, "expected stone")
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
                match game_state.points.iter().find(|p| p.x == 0 && p.y == 0 ) {
                     Some(p) => {
                         match &p.stone {
                             Some(s) => assert_eq!(s.chain_id, chain_id),
                             None => assert!(false, "expected stone")
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
