use std::cmp::Ordering;
use crate::go::state::vector::orthogonal;
use crate::go::state::vector::magnitude;
use crate::go::state::stone::Stone;
use crate::go::state::point::Point;

pub fn max_chain_id(points: &Vec<Point>) -> i8 {
    let point_max = points.iter().max_by(|a, b| {
        let a_chain_id = match &a.stone {
            Some(s) => s.chain_id,
            None => 0
        };

        let b_chain_id = match &b.stone {
            Some(s) => s.chain_id,
            None => 0
        };

        match a_chain_id.partial_cmp(&b_chain_id) {
            Some(c) => c,
            None => Ordering::Equal
        }
    });

    match point_max {
        Some(p) => {
            match &p.stone {
                Some(s) => s.chain_id,
                None => 0
            }
        },
        None => 0
    }
}

pub fn add_stone(points: &mut Vec<Point>, x: i8, y: i8, player_number: i8) -> Result<i8, &'static str> {
    let adj = find_players_stone_adjacent_to_x_and_y(points, x, y, player_number);

    let chain_id = match adj {
        Some(a) => {
            match &a.stone {
                Some(s) => s.chain_id,
                None => max_chain_id(points) + 1
            }
        },
        None => max_chain_id(points) + 1
    };

    match points.iter_mut().find(|p| p.x == x && p.y == y) {
        Some(p) => {
            let stone = Stone { player_number, chain_id };
            p.stone = Some(stone);
            Ok(chain_id)
        },
        None => Err("Point does not exist")
    }
}

pub fn remove_captured_stones(points: &mut Vec<Point>, x: i8, y: i8, opposing_player_number: i8) -> Result<Vec<(i8, i8)>, &'static str> {
    // check if any opponent chains adjacent to point now have zero liberties?
    let adj = players_stones_adjacent_to_x_and_y(points, x, y, opposing_player_number); // N

    let adjacent_chain_ids = chain_ids(adj); // 0 - 4

    let mut chains_to_remove = vec![];

    for cid in adjacent_chain_ids.iter() {
        if chain_has_liberties(points, *cid) == false {
            chains_to_remove.push(cid);
        }
    } // 0 - 4

    let mut stones_captured = vec![];

    if chains_to_remove.len() > 0 {
        for p in points.iter_mut() {
            let cid = match &p.stone {
                Some(s) => s.chain_id,
                None => 0
            };

            for remove_id in chains_to_remove.iter() {
                if cid == **remove_id {
                    p.stone = None;
                    stones_captured.push((p.x, p.y));
                }
            }
        } // N
    }

    Ok(stones_captured)
}

pub fn filter_by_chain_id(points: &Vec<Point>, chain_id: i8) -> Vec<&Point> {
    points.iter().filter(|p| {
        match &p.stone {
            Some(s) => s.chain_id == chain_id,
            None => false
        }
    }).collect()
}

pub fn chain_has_liberties(points: &Vec<Point>, chain_id: i8) -> bool {
    let mut has_liberties = false;

    for p in points.iter() {
        let matches_chain = match &p.stone {
            Some(s) => s.chain_id == chain_id,
            None => false
        };
        if matches_chain && point_has_liberties(points, p.x, p.y) {
           has_liberties = true;
           break;
        }
    }

    has_liberties
}

pub fn chain_ids(points: Vec<&Point>) -> Vec<i8> {
   points.iter().map(|a| {
       match &a.stone {
           Some(s) => s.chain_id,
           None => 0
       }
   }).collect()
}

pub fn players_stones_adjacent_to_x_and_y(points: &Vec<Point>, x: i8, y: i8, player_number: i8) -> Vec<&Point> {
    points.iter().filter(|to| {
        if orthogonal(x, y, to.x, to.y) && magnitude(x, y, to.x, to.y) == 1  {
            match &to.stone {
                Some(s) => s.player_number == player_number,
                None => false
            }
        } else {
            false
        }
    }).collect()
}

pub fn find_players_stone_adjacent_to_x_and_y(points: &Vec<Point>, x: i8, y: i8, player_number: i8) -> Option<&Point> {
    points.iter().find(|to| {
        if orthogonal(x, y, to.x, to.y) && magnitude(x, y, to.x, to.y) == 1  {
            match &to.stone {
                Some(s) => s.player_number == player_number,
                None => false
            }
        } else {
            false
        }
    })
}

pub fn adjacent_to_x_and_y(points: &Vec<Point>, x: i8, y: i8) -> Vec<&Point> {
    points.iter().filter(|p| {
        orthogonal(x, y, p.x, p.y) && magnitude(x, y, p.x, p.y) == 1
    }).collect()
}

pub fn point_has_liberties(points: &Vec<Point>, x: i8, y: i8) -> bool {
    let mut has_liberties = false;

    for p in points.iter() {
        if orthogonal(x, y, p.x, p.y) && magnitude(x, y, p.x, p.y) == 1 && p.stone.is_none() {
            has_liberties = true;
            break;
        }
    }

    has_liberties
}

pub fn players_stones_adjacent_to_x_and_y_chain_ids(points: &Vec<Point>, x: i8, y: i8, player_number: i8) -> Vec<i8> {
    let mut chain_ids = vec![];
    for p in points.iter() {
        if orthogonal(x, y, p.x, p.y) && magnitude(x, y, p.x, p.y) == 1 {
            if let Some(stone) = &p.stone {
                if stone.chain_id != 0 && stone.player_number == player_number {
                    chain_ids.push(stone.chain_id);
                }
            }
        }
    }
    chain_ids
}

pub fn adjacent_to_x_and_y_territory_ids(points: &Vec<Point>, x: i8, y: i8) -> Vec<i8> {
    let mut territory_ids = vec![];
    for p in points.iter() {
        if orthogonal(x, y, p.x, p.y) && magnitude(x, y, p.x, p.y) == 1 && p.stone.is_none() {
            if let Some(tid) = p.territory_id {
                territory_ids.push(tid);
            }
        }
    }
    territory_ids
}

pub fn populate_chains(points: &mut Vec<Point>) -> () {
    let mut chain_id_counter = 1;
    for idx in 0..points.len() {
        let p = &points[idx];
        if let Some(stone) = &p.stone {
            let chain_ids = players_stones_adjacent_to_x_and_y_chain_ids(&points, p.x, p.y, stone.player_number);  // N
            let new_chain_id;
            let mut other_chain_ids = vec![];

            match chain_ids.len() {
                0 => {
                    new_chain_id = chain_id_counter;
                    chain_id_counter += 1;
                },
                1 => {
                    new_chain_id = match chain_ids.first() {
                        Some(cid) => *cid,
                        None => 0
                    };
                },
                _ => {
                    new_chain_id = match chain_ids.first() {
                        Some(cid) => *cid,
                        None => 0
                    };
                    other_chain_ids = chain_ids.as_slice()[1..chain_ids.len()].to_vec();
                }
            }

            if other_chain_ids.len() > 0 {
                for q in points.iter_mut() {
                    match &mut q.stone {
                        Some(s) => {
                            if other_chain_ids.iter().any(|oci| *oci == s.chain_id ) {
                                s.chain_id = new_chain_id;
                            }
                        },
                        None => ()
                    }
                } // N
            }

            let q = &mut points[idx];
            match &mut q.stone {
                Some(s) => s.chain_id = new_chain_id,
                None => ()
            }
        }
    } // N
}

pub fn mark_territories(points: &mut Vec<Point>) -> () {
    points.iter_mut().for_each(|p| p.territory_id = None);

    let mut territory_id_counter = 1;
    for idx in 0..points.len() {
        let p = &points[idx];
        if p.stone == None && p.territory_id == None {
            let territory_ids = adjacent_to_x_and_y_territory_ids(&points, p.x, p.y); // N
            let new_territory_id: i8;
            let mut other_territory_ids = vec![];

            match territory_ids.len() {
                0 => {
                    new_territory_id = territory_id_counter;
                    territory_id_counter += 1;
                },
                1 => {
                    new_territory_id = match territory_ids.first() {
                        Some(tid) => *tid,
                        None => 0
                    };
                },
                _ => {
                    new_territory_id = match territory_ids.first() {
                        Some(tid) => *tid,
                        None => 0
                    };
                    other_territory_ids = territory_ids[1..].to_vec();
                }
            }

            if other_territory_ids.len() > 0 {
                for q in points.iter_mut() {
                    if let Some(tid) = q.territory_id {
                        if other_territory_ids.iter().any(|oti| *oti == tid ) {
                            q.territory_id = Some(new_territory_id);
                        }
                    }
                } // N
            }

            let q = &mut points[idx];
            q.territory_id = Some(new_territory_id);
        }
    } // N
}

pub fn territory_ids(points: &Vec<Point>) -> Vec<i8> {
    let mut ids = vec![];
    for p in points.iter() {
        match p.territory_id {
            Some(tid) => ids.push(tid),
            None => ()
        }
    }
    ids.sort();
    ids.dedup();
    ids
}

pub fn players_territory_count(points: &Vec<Point>, player_number: i8) -> i16 {
    let tids = territory_ids(points); // N
    let mut point_count: i16 = 0;

    for tid in tids.iter() {
        // player number
        let mut this_player= false;
        let mut other_player= false;
        for p in points.iter() {
            if p.territory_id == Some(*tid) {
                let adjacent = adjacent_to_x_and_y(points, p.x, p.y); // N
                for a in adjacent.iter() {
                    if let Some(s) = &a.stone {
                        if s.player_number == player_number {
                            this_player = true;
                        } else {
                            other_player = true;
                        }
                    }
                } // 0 - 4
                // if territory is next to stone owned  by other player, break;
                if other_player {
                    break;
                }
            }
        }

        // if territory owned by requested player number
        if this_player && !other_player {
            for p in points.iter() {
                if p.territory_id == Some(*tid) {
                    point_count += 1;
                }
            }
        }
    } // < N

    point_count
}

pub fn simplify(points: &Vec<Point>) -> Vec<Vec<i8>> {
    let mut result = vec![
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
    ];
    points.iter().for_each(|p| {
       match &p.stone {
            Some(s) => {
                result[p.y as usize][p.x as usize] = s.player_number;
            },
            None => ()
       }
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_chain_id_test() {
        let point_set = vec![
            Point {
                x: 0,
                y: 0,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            },
            Point {
                x: 0,
                y: 1,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            }
        ];
        let result = max_chain_id(&point_set);
        assert_eq!(result, 1);
    }

    #[test]
    fn add_stone_test() {
       let mut points = vec![
            Point {
                 x: 0,
                 y: 0,
                 stone: None,
                 territory_id: None
            }
       ];
       let x = 0;
       let y = 0;
       let player_number = 1;
       let result = add_stone(&mut points, x, y, player_number);
       match result {
            Ok(chain_id) => {
                assert_eq!(chain_id, 1);
                match points.iter().find(|p| p.x == x && p.y == y ) {
                    Some(p) => {
                        match &p.stone {
                            Some(s) => assert_eq!(s.player_number, player_number),
                            None => assert!(false, "expected stone")
                        }
                    },
                    None => assert!(false, "expected point")
                };
            },
            Err(_) => assert!(false, "expected chain_id")
       }
    }

    #[test]
    fn remove_captured_stones_test() {
        let mut points = vec![
            Point { x: 0, y: 0, stone: None, territory_id: None },
            Point { x: 1, y: 0, stone: Some(Stone { player_number: 1, chain_id: 1 }), territory_id: None },
            Point { x: 2, y: 0, stone: None, territory_id: None },

            Point { x: 0, y: 1, stone: Some(Stone { player_number: 1, chain_id: 2 }), territory_id: None },
            Point { x: 1, y: 1, stone: Some(Stone { player_number: 2, chain_id: 3 }), territory_id: None },
            Point { x: 2, y: 1, stone: Some(Stone { player_number: 1, chain_id: 4 }), territory_id: None },

            Point { x: 0, y: 2, stone: None, territory_id: None },
            Point { x: 1, y: 2, stone: Some(Stone { player_number: 1, chain_id: 5 }), territory_id: None },
            Point { x: 2, y: 2, stone: None, territory_id: None }
        ];
        match remove_captured_stones(&mut points,1,2,2) {
            Ok(result) => {
                assert_eq!(result, vec![(1,1)]);
                match points.iter().find(|p| p.x == 1 && p.y == 1) {
                     Some(p) => assert_eq!(p.stone, None),
                     None => assert!(false, "expected point")
                }
            },
            Err(e) =>  assert!(false, "{}", e)
        }
    }

    #[test]
    fn filter_by_chain_id_test() {
        let point_set = vec![
            Point {
                x: 0,
                y: 0,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            },
            Point {
                x: 0,
                y: 1,
                stone: Some(Stone { player_number: 1, chain_id: 2 }),
                territory_id: None
            }
        ];
        let result = filter_by_chain_id(&point_set, 1);
        let expected = vec![
            &Point {
                x: 0,
                y: 0,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            }
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn chain_has_liberties_test() {
        let point_set = vec![
            Point {
                x: 0,
                y: 0,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            },
            Point {
                x: 0,
                y: 1,
                stone: None,
                territory_id: None
            }
        ];
        let result = chain_has_liberties(&point_set, 1);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn chain_has_no_liberties_test() {
        let point_set = vec![
            Point {
                x: 0,
                y: 0,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            },
            Point {
                x: 0,
                y: 1,
                stone: Some(Stone { player_number: 2, chain_id: 2 }),
                territory_id: None
            }
        ];
        let result = chain_has_liberties(&point_set, 1);
        let expected = false;
        assert_eq!(result, expected);
    }

    #[test]
    fn chain_ids_test() {
        let point_set = vec![
            &Point {
                x: 0,
                y: 0,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            },
            &Point {
                x: 0,
                y: 1,
                stone: Some(Stone { player_number: 1, chain_id: 2 }),
                territory_id: None
            }
        ];
        let result = chain_ids(point_set);
        assert_eq!(result, vec![1,2]);
    }

    #[test]
    fn players_stones_adjacent_to_x_and_y_test() {
        let point_set = vec![
            Point {
                x: 0,
                y: 0,
                stone: None,
                territory_id: None
            },
            Point {
                x: 1,
                y: 0,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            },
            Point {
                x: 0,
                y: 1,
                stone: Some(Stone { player_number: 2, chain_id: 2 }),
                territory_id: None
            },
            Point {
                x: 1,
                y: 1,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            }
        ];
        let x = 0;
        let y = 0;
        let player_number = 1;
        let result = players_stones_adjacent_to_x_and_y(&point_set, x, y, player_number);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].x, 1);
        assert_eq!(result[0].y, 0);
    }

    #[test]
    fn find_players_stone_adjacent_to_x_and_y_test() {
        let point_set = vec![
            Point {
                x: 0,
                y: 0,
                stone: None,
                territory_id: None
            },
            Point {
                x: 1,
                y: 0,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            },
            Point {
                x: 0,
                y: 1,
                stone: Some(Stone { player_number: 2, chain_id: 2 }),
                territory_id: None
            },
            Point {
                x: 1,
                y: 1,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            }
        ];
        let x = 0;
        let y = 0;
        let player_number = 1;
        let result = find_players_stone_adjacent_to_x_and_y(&point_set, x, y, player_number);
        match result {
            Some(p) => {
                assert_eq!(p.x, 1);
                assert_eq!(p.y, 0);
            },
            None => assert!(false, "expected point")
        }
    }

    #[test]
    fn adjacent_to_x_and_y_test() {
        let point_a = Point {
            x: 0,
            y: 0,
            stone: None,
            territory_id: None
        };
        let point_b = Point {
            x: 1,
            y: 0,
            stone: Some(Stone { player_number: 1, chain_id: 1 }),
            territory_id: None
        };
        let point_c = Point {
            x: 0,
            y: 1,
            stone: Some(Stone { player_number: 2, chain_id: 2 }),
            territory_id: None
        };
        let point_d = Point {
            x: 1,
            y: 1,
            stone: Some(Stone { player_number: 1, chain_id: 1 }),
            territory_id: None
        };
        let x = 0;
        let y = 0;
        let point_set = vec![point_a, point_b, point_c, point_d];
        let result = adjacent_to_x_and_y(&point_set, x, y);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].x, 1);
        assert_eq!(result[0].y, 0);
        assert_eq!(result[1].x, 0);
        assert_eq!(result[1].y, 1);
    }

    #[test]
    fn players_stones_adjacent_to_x_and_y_chain_ids_test() {
        let point_set = vec![
            Point {
                x: 1,
                y: 1,
                stone: Some(Stone { player_number: 1, chain_id: 1 }),
                territory_id: None
            },
            Point {
                x: 0,
                y: 1,
                stone: Some(Stone { player_number: 2, chain_id: 2 }),
                territory_id: None
            },
            Point {
                x: 1,
                y: 0,
                stone: Some(Stone { player_number: 1, chain_id: 3 }),
                territory_id: None
            }
        ];
        let x = 1;
        let y = 1;
        let player_number = 1;
        let result = players_stones_adjacent_to_x_and_y_chain_ids(&point_set, x, y, player_number);
        let expected = vec![3];
        assert_eq!(result, expected);
    }

    #[test]
    fn adjacent_to_x_and_y_territory_ids_test() {
        let point_set = vec![
            Point {
                x: 1,
                y: 1,
                stone: None,
                territory_id: Some(1)
            },
            Point {
                x: 0,
                y: 1,
                stone: None,
                territory_id: Some(2)
            },
            Point {
                x: 1,
                y: 0,
                stone: None,
                territory_id: Some(3)
            }
        ];
        let x = 1;
        let y = 1;
        let result = adjacent_to_x_and_y_territory_ids(&point_set, x, y);
        let expected = vec![2, 3];
        assert_eq!(result, expected);
    }

    #[test]
    fn point_has_liberties_test() {
        let point_set = vec![
            Point {
                x: 1,
                y: 1,
                stone: None,
                territory_id: Some(1)
            },
            Point {
                x: 0,
                y: 1,
                stone: None,
                territory_id: Some(1)
            }
        ];
        let x = 1;
        let y = 1;
        let result = point_has_liberties(&point_set, x, y);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn point_has_no_liberties_test() {
        let point_set = vec![
            Point {
                x: 1,
                y: 1,
                stone: None,
                territory_id: Some(1)
            },
            Point {
                x: 0,
                y: 1,
                stone: Some(Stone { player_number: 2, chain_id: 1 }),
                territory_id: Some(1)
            }
        ];
        let x = 1;
        let y = 1;
        let result = point_has_liberties(&point_set, x, y);
        let expected = false;
        assert_eq!(result, expected);
    }

    #[test]
    fn populate_chains_test() {
        let mut point_set = vec![
            Point {
                x: 0,
                y: 0,
                stone: Some(Stone { player_number: 1, chain_id: 0 }),
                territory_id: None
            },
            Point {
                x: 1,
                y: 0,
                stone: Some(Stone { player_number: 1, chain_id: 0 }),
                territory_id: None
            },
            Point {
                x: 1,
                y: 1,
                stone: Some(Stone { player_number: 1, chain_id: 0 }),
                territory_id: None
            },
            Point {
                x: 0,
                y: 2,
                stone: Some(Stone { player_number: 1, chain_id: 0 }),
                territory_id: None
            },
            Point {
                x: 1,
                y: 2,
                stone: Some(Stone { player_number: 1, chain_id: 0 }),
                territory_id: None
            }
        ];
        populate_chains(&mut point_set);
        assert_eq!(point_set[0].stone.as_ref().unwrap().chain_id, 1);
        assert_eq!(point_set[1].stone.as_ref().unwrap().chain_id, 1);
        assert_eq!(point_set[2].stone.as_ref().unwrap().chain_id, 1);
        assert_eq!(point_set[3].stone.as_ref().unwrap().chain_id, 1);
        assert_eq!(point_set[4].stone.as_ref().unwrap().chain_id, 1);
    }

    // +-B-+
    // B-+-+
    // +-+-+
    #[test]
    fn mark_territories_test() {
        let mut point_set = vec![
            Point { x: 0, y: 0, stone: None, territory_id: None },
            Point { x: 1, y: 0, stone: Some(Stone { player_number: 1, chain_id: 1 }), territory_id: None },
            Point { x: 2, y: 0, stone: None, territory_id: None },

            Point { x: 0, y: 1, stone: Some(Stone { player_number: 1, chain_id: 2}), territory_id: None },
            Point { x: 1, y: 1, stone: None, territory_id: None },
            Point { x: 2, y: 1, stone: None, territory_id: None },

            Point { x: 0, y: 2, stone: None, territory_id: None },
            Point { x: 1, y: 2, stone: None, territory_id: None },
            Point { x: 2, y: 2, stone: None, territory_id: None }
        ];
        mark_territories(&mut point_set);
        assert_eq!(point_set[0].territory_id, Some(1));
        assert_eq!(point_set[1].territory_id, None);
        assert_eq!(point_set[2].territory_id, Some(2));

        assert_eq!(point_set[3].territory_id, None);
        assert_eq!(point_set[4].territory_id, Some(2));
        assert_eq!(point_set[5].territory_id, Some(2));

        assert_eq!(point_set[6].territory_id, Some(2));
        assert_eq!(point_set[7].territory_id, Some(2));
        assert_eq!(point_set[8].territory_id, Some(2));
    }

    // +-+-B-+-W-W
    // +-+-B-W-+-+
    // B-B-+-W-+-+
    #[test]
    fn players_territory_count_test() {
        let point_set = vec![
            Point { x: 0, y: 0, stone: None, territory_id: Some(1) },
            Point { x: 1, y: 0, stone: None, territory_id: Some(1) },
            Point { x: 2, y: 0, stone: Some(Stone { player_number: 1, chain_id: 1}), territory_id: None },
            Point { x: 3, y: 0, stone: None, territory_id: Some(2) },
            Point { x: 4, y: 0, stone: Some(Stone { player_number: 2, chain_id: 2}), territory_id: None },
            Point { x: 5, y: 0, stone: Some(Stone { player_number: 2, chain_id: 2}), territory_id: None },

            Point { x: 0, y: 1, stone: None, territory_id: Some(1) },
            Point { x: 1, y: 1, stone: None, territory_id: Some(1) },
            Point { x: 2, y: 1, stone: Some(Stone { player_number: 1, chain_id: 1}), territory_id: None },
            Point { x: 3, y: 1, stone: Some(Stone { player_number: 2, chain_id: 3}), territory_id: None },
            Point { x: 4, y: 1, stone: None, territory_id: Some(3) },
            Point { x: 5, y: 1, stone: None, territory_id: Some(3) },

            Point { x: 0, y: 2, stone: Some(Stone { player_number: 1, chain_id: 4}), territory_id: None },
            Point { x: 1, y: 2, stone: Some(Stone { player_number: 1, chain_id: 4}), territory_id: None },
            Point { x: 2, y: 2, stone: None, territory_id: Some(4) },
            Point { x: 3, y: 2, stone: Some(Stone { player_number: 2, chain_id: 3}), territory_id: None },
            Point { x: 4, y: 2, stone: None, territory_id: Some(3) },
            Point { x: 5, y: 2, stone: None, territory_id: Some(3) }
        ];

        let expected = 4;
        let result = players_territory_count(&point_set, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn simplify_test() {
        let point_set = vec![
            Point { x: 0, y: 0, stone: None, territory_id: None },
            Point { x: 1, y: 0, stone: Some(Stone { player_number: 1, chain_id: 1 }), territory_id: None },
            Point { x: 2, y: 0, stone: Some(Stone { player_number: 2, chain_id: 2 }), territory_id: None },
        ];
        let expected = vec![
            [0,1,2,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],

            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
            [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0]
        ];
        let result = simplify(&point_set);

        assert_eq!(result, expected);
    }
}
