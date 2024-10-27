use std::cmp::Ordering;
use std::collections::HashSet;
use crate::go::state::point::Point;

pub fn max_chain_id(points: &Vec<Vec<Point>>) -> i8 {
    let point_max = points.iter().flatten().max_by(|a, b| {
        let a_chain_id = if a.player_number != 0 {
            a.chain_id
        } else {
            0
        };

        let b_chain_id = if b.player_number != 0 {
            b.chain_id
        } else {
            0
        };

        match a_chain_id.partial_cmp(&b_chain_id) {
            Some(c) => c,
            None => Ordering::Equal
        }
    });

    match point_max {
        Some(p) => {
            if p.player_number != 0 {
                p.chain_id
            } else {
                0
            }
        },
        None => 0
    }
}

pub fn add_stone(points: &mut Vec<Vec<Point>>, x: usize, y: usize, player_number: i8) -> Result<i8, &'static str> {
    let adj = find_players_stone_adjacent_to_x_and_y(points, x, y, player_number);

    let chain_id = match adj {
        Some(a) => {
            if a.player_number != 0 {
                a.chain_id
            } else {
                max_chain_id(points) + 1
            }
        },
        None => max_chain_id(points) + 1
    };

    let p = &mut points[y][x];
    p.player_number = player_number;
    p.chain_id = chain_id;
    Ok(chain_id)
}

pub fn remove_captured_stones(points: &mut Vec<Vec<Point>>, x: usize, y: usize, opposing_player_number: i8) -> Vec<(usize, usize)> {
    // check if any opponent chains adjacent to point now have zero liberties?
    let adjacent_chain_ids = players_stones_adjacent_to_x_and_y_chain_ids(points, x, y, opposing_player_number);

    let mut chains_to_remove = vec![];

    for cid in adjacent_chain_ids.iter() {
        if chain_has_liberties(points, *cid) == false {
            chains_to_remove.push(cid);
        }
    }

    let mut stones_captured = vec![];

    if !chains_to_remove.is_empty() {
        for (y, row) in points.iter_mut().enumerate() {
            for (x, p) in row.iter_mut().enumerate() {
                let cid = if p.player_number != 0 {
                    p.chain_id
                } else {
                    0
                };

                for remove_id in chains_to_remove.iter() {
                    if cid == **remove_id {
                        p.player_number = 0;
                        stones_captured.push((x, y));
                    }
                }
            }
        }
    }

    stones_captured
}

pub fn chain_has_liberties(points: &Vec<Vec<Point>>, chain_id: i8) -> bool {
    let mut has_liberties = false;

    for (y, row) in points.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if p.player_number != 0 && p.chain_id == chain_id && point_has_liberties(points, x, y) {
               has_liberties = true;
               break;
            }
        }

        if has_liberties {
            break;
        }
    }

    has_liberties
}

pub fn chain_has_two_liberties(points: &Vec<Vec<Point>>, chain_id: i8) -> bool {
    let mut liberty_count = 0;
    let mut two_liberties = false;

    for (y, row) in points.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if p.chain_id == chain_id {
                let adjacent = adjacent_to_x_and_y(points, x, y);

                for a in adjacent.iter() {
                    if a.player_number == 0 {
                        liberty_count += 1;
                        if liberty_count >= 2 {
                            two_liberties = true;
                            break;
                        }
                    }
                }

                if two_liberties {
                    break;
                }
            }
        }
    }

    two_liberties
}

pub fn chain_has_only_one_liberty(points: &Vec<Vec<Point>>, chain_id: i8) -> bool {
    let mut liberty_count = 0;

    for (y, row) in points.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if p.chain_id == chain_id {
                let adjacent = adjacent_to_x_and_y(points, x, y);
                for a in adjacent.iter() {
                    if a.player_number == 0 {
                        liberty_count += 1;
                    }
                }
            }
        }
    }

    liberty_count == 1
}

pub fn adjacent_coordinates(points: &Vec<Vec<Point>>, x: usize, y: usize) -> Vec<(usize,usize)> {
    let max = points.len() - 1;
    if y == 0 {
        if x == 0 {
            vec![
                (x + 1, y),
                (x, y + 1)
            ]
        } else if x < max {
            vec![
                (x - 1, y),
                (x + 1, y),
                (x, y + 1)
            ]
        } else if x == max {
            vec![
                (x - 1, y),
                (x, y + 1)
            ]
        } else {
            vec![]
        }
    } else if y < max {
        if x == 0 {
            vec![
                (x, y - 1),
                (x + 1, y),
                (x, y + 1)
            ]
        } else if x < max {
            vec![
                (x, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x, y + 1)
            ]
        } else if x == max {
            vec![
                (x, y - 1),
                (x - 1, y),
                (x, y + 1)
            ]
        } else {
            vec![]
        }
    } else if y == max {
        if x == 0 {
            vec![
                (x, y - 1),
                (x + 1, y)
            ]
        } else if x < max {
            vec![
                (x, y - 1),
                (x - 1, y),
                (x + 1, y)
            ]
        } else if x == max {
            vec![
                (x, y - 1),
                (x - 1, y)
            ]
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}

pub fn find_players_stone_adjacent_to_x_and_y(points: &Vec<Vec<Point>>, x: usize, y: usize, player_number: i8) -> Option<&Point> {
    let mut found_point = None;

    let coordinates = adjacent_coordinates(points, x, y);
    for coordinate in coordinates {
        let to = &points[coordinate.1][coordinate.0];
        if to.player_number == player_number {
            found_point = Some(to);
            break;
        }
    }

    found_point
}

pub fn adjacent_to_x_and_y(points: &Vec<Vec<Point>>, x: usize, y: usize) -> Vec<&Point> {
    let mut adjacent = vec![];

    let coordinates = adjacent_coordinates(points, x, y);
    for coordinate in coordinates {
        let p = &points[coordinate.1][coordinate.0];
        adjacent.push(p);
    }

    adjacent
}

pub fn point_has_liberties(points: &Vec<Vec<Point>>, x: usize, y: usize) -> bool {
    let mut has_liberties = false;

    let coordinates = adjacent_coordinates(points, x, y);
    for coordinate in coordinates {
        let p = &points[coordinate.1][coordinate.0];
        if p.player_number == 0 {
            has_liberties = true;
            break;
        }
    }

    has_liberties
}

pub fn players_stones_adjacent_to_x_and_y_chain_ids(points: &Vec<Vec<Point>>, x: usize, y: usize, player_number: i8) -> HashSet<i8> {
    let mut chain_ids = HashSet::new();

    let coordinates = adjacent_coordinates(points, x, y);
    for coordinate in coordinates {
        let p = &points[coordinate.1][coordinate.0];
        if p.player_number != 0 && p.chain_id != 0 && p.player_number == player_number {
            chain_ids.insert(p.chain_id);
        }
    }

    chain_ids
}

pub fn adjacent_to_x_and_y_territory_ids(points: &Vec<Vec<Point>>, x: usize, y: usize) -> HashSet<i8> {
    let mut territory_ids = HashSet::new();

    let coordinates = adjacent_coordinates(points, x, y);
    for coordinate in coordinates {
        let p = &points[coordinate.1][coordinate.0];
        if p.player_number == 0 {
            if let Some(tid) = p.territory_id {
                territory_ids.insert(tid);
            }
        }
    }

    territory_ids
}

pub fn populate_chains(points: &mut Vec<Vec<Point>>) -> () {
    let mut chain_id_counter = 1;

    let board_size = points.len();

    for y in 0..board_size {
        for x in 0..board_size {
            let p = &points[y][x];
            if p.player_number != 0 {
                let mut chain_ids = players_stones_adjacent_to_x_and_y_chain_ids(&points, x, y, p.player_number).into_iter().collect::<Vec<i8>>();
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
                        chain_ids.sort();
                        new_chain_id = match chain_ids.first() {
                            Some(cid) => *cid,
                            None => 0
                        };
                        other_chain_ids = chain_ids.as_slice()[1..].to_vec();
                    }
                }

                if other_chain_ids.len() > 0 {
                    for row in &mut *points {
                        for q in row {
                            if q.player_number != 0 {
                                if other_chain_ids.iter().any(|oci| *oci == q.chain_id ) {
                                    q.chain_id = new_chain_id;
                                }
                            }
                        }
                    }
                }

                let q = &mut points[y][x];
                if q.player_number != 0 {
                    q.chain_id = new_chain_id;
                }
            }
        }
    } // N
}

pub fn mark_territories(points: &mut Vec<Vec<Point>>) -> () {
    for row in &mut *points {
        for p in row {
            p.territory_id = None;
        }
    }

    let mut territory_id_counter = 1;
    let board_size = points.len();

    for y in 0..board_size {
        for x in 0..board_size {
            let p = &points[y][x];
            if p.player_number == 0 && p.territory_id == None {
                let mut territory_ids = adjacent_to_x_and_y_territory_ids(&points, x, y).into_iter().collect::<Vec<i8>>();
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
                        territory_ids.sort();
                        new_territory_id = match territory_ids.first() {
                            Some(tid) => *tid,
                            None => 0
                        };
                        other_territory_ids = territory_ids[1..].to_vec();
                    }
                }

                if !other_territory_ids.is_empty() {
                    for row in &mut *points {
                        for q in row {
                            if let Some(tid) = q.territory_id {
                                if other_territory_ids.iter().any(|oti| *oti == tid ) {
                                    q.territory_id = Some(new_territory_id);
                                }
                            }
                        }
                    } // N
                }

                let q = &mut points[y][x];
                q.territory_id = Some(new_territory_id);
            }
        }
    } // N
}

pub fn territory_ids(points: &Vec<Vec<Point>>) -> HashSet<i8> {
    let mut ids = HashSet::new();

    for row in points {
        for p in row {
            if let Some(tid) = p.territory_id {
                ids.insert(tid);
            }
        }
    }

    ids
}

pub fn players_territory_count(points: &Vec<Vec<Point>>, player_number: i8) -> i16 {
    let tids = territory_ids(points);
    let mut point_count: i16 = 0;

    for tid in tids.iter() {
        let mut this_player = false;
        let mut other_player = false;

        for (y, row) in points.iter().enumerate() {
            for (x, p) in row.iter().enumerate() {
                if p.territory_id == Some(*tid) {
                    let adjacent = adjacent_to_x_and_y(points, x, y);
                    for a in adjacent.iter() {
                        if a.player_number != 0 {
                            if a.player_number == player_number {
                                this_player = true;
                            } else {
                                other_player = true;
                            }
                        }
                    }
                    // if territory is next to stone owned  by other player, break;
                    if other_player {
                        break;
                    }
                }
            }

            if other_player {
                break;
            }
        }

        // if territory owned by requested player number
        if this_player && !other_player {
            for row in points {
                for p in row {
                    if p.territory_id == Some(*tid) {
                        point_count += 1;
                    }
                }
            }
        }
    }

    point_count
}

pub fn simplify(points: &Vec<Vec<Point>>) -> Vec<Vec<i8>> {
    points.iter().map(|row| {
        row.iter().map(|p| p.player_number).collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_chain_id_test() {
        let point_set = vec![
            vec![Point { player_number: 1, chain_id: 1, territory_id: None }],
            vec![Point { player_number: 1, chain_id: 1, territory_id: None }]
        ];
        let result = max_chain_id(&point_set);
        assert_eq!(result, 1);
    }

    #[test]
    fn add_stone_test() {
       let mut points = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
       ];
       let x = 0;
       let y = 0;
       let player_number = 1;
       let result = add_stone(&mut points, x, y, player_number);
       match result {
            Ok(chain_id) => {
                assert_eq!(chain_id, 1);
                let p = &points[y as usize][x as usize];

                if p.player_number != 0 {
                    assert_eq!(p.player_number, player_number)
                } else {
                    assert!(false, "expected stone")
                }
            },
            Err(_) => assert!(false, "expected chain_id")
       }
    }

    #[test]
    fn remove_captured_stones_test() {
        let mut points = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 1, chain_id: 2, territory_id: None },
                Point { player_number: 2, chain_id: 3, territory_id: None },
                Point { player_number: 1, chain_id: 4, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 5, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];
        let result = remove_captured_stones(&mut points,1,2,2);
        assert_eq!(result, vec![(1,1)]);

        let p = &points[1][1];
        assert_eq!(p.player_number, 0);
    }

    #[test]
    fn chain_has_liberties_test() {
        let point_set = vec![
            vec![
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];
        let result = chain_has_liberties(&point_set, 1);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn chain_has_no_liberties_test() {
        let point_set = vec![
            vec![
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ],
            vec![
                Point { player_number: 2, chain_id: 2, territory_id: None },
                Point { player_number: 2, chain_id: 2, territory_id: None }
            ]
        ];
        let result = chain_has_liberties(&point_set, 1);
        let expected = false;
        assert_eq!(result, expected);
    }

    #[test]
    fn chain_has_two_liberties_true_test() {
        let point_set = vec![
            vec![
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];
        let result = chain_has_two_liberties(&point_set, 1);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn chain_has_two_liberties_false_test() {
        let point_set = vec![
            vec![
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ],
            vec![
                Point { player_number: 2, chain_id: 2, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];
        let result = chain_has_two_liberties(&point_set, 1);
        let expected = false;
        assert_eq!(result, expected);
    }

    #[test]
    fn chain_has_only_one_liberty_true_test() {
        let point_set = vec![
            vec![
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ],
            vec![
                Point { player_number: 2, chain_id: 2, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];
        let result = chain_has_only_one_liberty(&point_set, 1);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn chain_has_only_one_liberty_false_test() {
        let point_set = vec![
            vec![
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];
        let result = chain_has_only_one_liberty(&point_set, 1);
        let expected = false;
        assert_eq!(result, expected);
    }

    #[test]
    fn adjacent_coordinates_test() {
        let point_set = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ],
            vec![
                Point { player_number: 2, chain_id: 2, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ]
        ];
        let x = 0;
        let y = 0;
        let result = adjacent_coordinates(&point_set, x, y);
        let expected = vec![
            (1,0),
            (0,1)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn find_players_stone_adjacent_to_x_and_y_test() {
        let point_set = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ],
            vec![
                Point { player_number: 2, chain_id: 2, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ]
        ];
        let x = 0;
        let y = 0;
        let player_number = 1;
        let result = find_players_stone_adjacent_to_x_and_y(&point_set, x, y, player_number);
        match result {
            Some(p) => {
                assert_eq!(p.player_number, 1);
                assert_eq!(p.chain_id, 1);
            },
            None => assert!(false, "expected point")
        }
    }

    #[test]
    fn adjacent_to_x_and_y_test() {
        let x = 0;
        let y = 0;
        let point_set = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ],
            vec![
                Point { player_number: 2, chain_id: 2, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ]
        ];
        let result = adjacent_to_x_and_y(&point_set, x, y);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].player_number, 1);
        assert_eq!(result[0].chain_id, 1);
        assert_eq!(result[1].player_number, 2);
        assert_eq!(result[1].chain_id, 2);
    }

    #[test]
    fn players_stones_adjacent_to_x_and_y_chain_ids_test() {
        let point_set = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 3, territory_id: None }
            ],
            vec![
                Point { player_number: 2, chain_id: 2, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None }
            ]
        ];
        let x = 1;
        let y = 1;
        let player_number = 1;
        let result = players_stones_adjacent_to_x_and_y_chain_ids(&point_set, x, y, player_number);
        let expected = HashSet::from([3]);
        assert_eq!(result, expected);
    }

    #[test]
    fn adjacent_to_x_and_y_territory_ids_test() {
        let point_set = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: Some(4) },
                Point { player_number: 0, chain_id: 0, territory_id: Some(3) }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: Some(2) },
                Point { player_number: 0, chain_id: 0, territory_id: Some(1) }
            ]
        ];
        let x = 1;
        let y = 1;
        let result = adjacent_to_x_and_y_territory_ids(&point_set, x, y);
        let expected = HashSet::from([2, 3]);
        assert_eq!(result, expected);
    }

    #[test]
    fn point_has_liberties_test() {
        let point_set = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: Some(1) },
                Point { player_number: 0, chain_id: 0, territory_id: Some(1) }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: Some(1) },
                Point { player_number: 0, chain_id: 0, territory_id: Some(1) }
            ]
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
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: Some(1) },
                Point { player_number: 2, chain_id: 1, territory_id: Some(1) }
            ]
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
            vec![
                Point { player_number: 1, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
            ],
            vec![
                Point { player_number: 1, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];
        populate_chains(&mut point_set);
        let expected = vec![
            vec![
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];
        assert_eq!(point_set, expected);
    }

    // +-B-+
    // B-+-+
    // +-+-+
    #[test]
    fn mark_territories_test() {
        let mut point_set = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 1, chain_id: 2, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];
        mark_territories(&mut point_set);
        let expected = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: Some(1) },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: Some(2) }
            ],
            vec![
                Point { player_number: 1, chain_id: 2, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: Some(2) },
                Point { player_number: 0, chain_id: 0, territory_id: Some(2) }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: Some(2) },
                Point { player_number: 0, chain_id: 0, territory_id: Some(2) },
                Point { player_number: 0, chain_id: 0, territory_id: Some(2) }
            ]
        ];
        assert_eq!(point_set, expected);
    }

    // +-+-B-+-W-W
    // +-+-B-W-+-+
    // B-B-+-W-+-+
    // +-+-+-+-W-W
    // +-+-+-+-+-+
    // +-+-+-+-+-+
    #[test]
    fn players_territory_count_test() {
        let point_set = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: Some(1) },
                Point { player_number: 0, chain_id: 0, territory_id: Some(1) },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: Some(2) },
                Point { player_number: 2, chain_id: 2, territory_id: None },
                Point { player_number: 2, chain_id: 2, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: Some(1) },
                Point { player_number: 0, chain_id: 0, territory_id: Some(1) },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 2, chain_id: 3, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: Some(3) },
                Point { player_number: 0, chain_id: 0, territory_id: Some(3) }
            ],
            vec![
                Point { player_number: 1, chain_id: 4, territory_id: None },
                Point { player_number: 1, chain_id: 4, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: Some(4) },
                Point { player_number: 2, chain_id: 3, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: Some(3) },
                Point { player_number: 0, chain_id: 0, territory_id: Some(3) }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 2, chain_id: 5, territory_id: None },
                Point { player_number: 2, chain_id: 5, territory_id: None }
            ],
            vec![
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
                Point { player_number: 0, chain_id: 0, territory_id: None }
            ]
        ];

        let expected = 4;
        let result = players_territory_count(&point_set, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn simplify_test() {
        let point_set = vec![
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 2, chain_id: 2, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 2, chain_id: 2, territory_id: None }
            ],
            vec![
                Point { player_number: 0, chain_id: 0, territory_id: None },
                Point { player_number: 1, chain_id: 1, territory_id: None },
                Point { player_number: 2, chain_id: 2, territory_id: None }
            ]
        ];
        let expected = vec![
            [0,1,2],
            [0,1,2],
            [0,1,2]
        ];
        let result = simplify(&point_set);

        assert_eq!(result, expected);
    }
}
