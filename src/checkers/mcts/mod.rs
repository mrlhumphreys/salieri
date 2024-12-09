mod node;

use std::cmp::Ordering;

use rand::prelude::*;
use crate::checkers::state::game_state::GameState;
use crate::checkers::state::mov::Move;
use crate::checkers::mcts::node::Node;

const EXPLORATION: f32 = 1.4142135623730950; // square root 2

pub fn recommended_move(game_state: GameState, simulation_count: i16, max_simulation_depth: i16) -> Result<Move, &'static str> {
    let moves = game_state.possible_moves();
    match moves.len() {
        0 => Err("No moves possible"),
        1 => match moves.first() {
            Some(s) => Ok(s.clone()),
            None => Err("No moves possible")
        },
        _ => {
            let root_node = Node {
                id: 1,
                parent_id: None,
                child_ids: Vec::new(),
                mov: None,
                state: game_state,
                wins: 0,
                simulations: 0
            };

            let mut nodes = vec![root_node];

            for _i in 1..simulation_count {
                // 1) selection
                match selection(&nodes) {
                    Ok(selected_node_id) => {
                        // 2) expansion
                        expansion(&mut nodes, selected_node_id)?;

                        // 3) simulation - pick one child nodes of previously expanded
                        match simulate(&nodes, selected_node_id, max_simulation_depth) {
                            // 4) backpropagation
                            Ok(result) => backpropagation(&mut nodes, selected_node_id, result)?,
                            Err(e) => return Err(e)
                        }
                    },
                    Err(e) => return Err(e)
                }
            };

            let best_node = nodes.iter().filter(|n| n.parent_id == Some(1)).max_by(|a,b| a.wins.cmp(&b.wins));
            if let Some(n) = best_node {
                match n.mov.clone() {
                    Some(m) => Ok(m),
                    None => Err("No best move")
                }
            } else {
                Err("No best node")
            }
        }
    }
}

fn selection(nodes: &Vec<Node>) -> Result<i32, &'static str> {
    let leaf_nodes = nodes.iter().filter(|n| n.child_ids.len() == 0 );
    let node_scores = leaf_nodes.map(|n| {
        match nodes.iter().find(|p| Some(p.id) == n.parent_id) {
            Some(parent) => (n.id, upper_confidence_bound(parent, n)),
            None => (n.id, 0.0)
        }
    });

    let max_node_score = node_scores.max_by(|a,b| {
        match (a.1).partial_cmp(&b.1) {
            Some(c) => c,
            None => Ordering::Equal
        }
    });

    match max_node_score {
        Some(ns) => Ok(ns.0),
        None => Err("mcts::selection - No nodes")
    }
}

fn expansion(nodes: &mut Vec<Node>, id: i32) -> Result<(), &'static str> {
   let mut counter_id = match nodes.iter().map(|n| n.id).max() {
       Some(max_id) => max_id,
       None => 1
   };

   if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
       if node.leaf() {
            let mut child_nodes: Vec<Node> = Vec::new();
            for mov in node.state.possible_moves() {
                counter_id = counter_id + 1;
                let mut new_game_state = node.state.clone();
                new_game_state.perform_move(&mov)?;
                let child_node = Node {
                    id: counter_id,
                    parent_id: Some(node.id),
                    child_ids: Vec::new(),
                    mov: Some(mov),
                    state: new_game_state,
                    wins: 0,
                    simulations: 0
                };
                child_nodes.push(child_node);
            }
            node.add_child_ids(child_nodes.iter().map(|n| n.id).collect());
            nodes.extend(child_nodes);
            Ok(())
        } else {
            Err("mcts::expansion - Node already has child nodes.")
        }
    } else {
        Err("mcts::expansion - Can't find node")
    }
}

fn simulate(nodes: &Vec<Node>, id: i32, max_simulation_depth: i16) -> Result<bool, &'static str> {
    if let Some(node) = nodes.iter().find(|n| n.id == id) {
        let mut end_game = false;
        let mut winner: Option<i8> = None;
        let mut simulation_depth: i16 = 0;
        let mut current_game_state = node.state.clone();

        while !end_game && simulation_depth <= max_simulation_depth {
            let mut moves = current_game_state.possible_moves();

            match moves.len() {
                0 => {
                    end_game = true;
                    winner = current_game_state.winner();
                },
                1 => {
                    let selected_move = &moves[0];
                    match current_game_state.perform_move(&selected_move) {
                        Ok(_) => (),
                        Err(_) => end_game = true
                    }
                },
                _ => {
                    let mut rng = rand::thread_rng();
                    moves.shuffle(&mut rng);
                    let selected_move = &moves[0];
                    match current_game_state.perform_move(&selected_move) {
                        Ok(_) => (),
                        Err(_) => end_game = true
                    }
                }
            }

            if let Some(w) = current_game_state.winner() {
                end_game = true;
                winner = Some(w);
            } else {
                simulation_depth = simulation_depth + 1;
            }
        }

        match winner {
            Some(w) => Ok(w == node.state.current_player_number),
            None => Ok(false)
        }
    } else {
        Err("Node not found")
    }
}

fn backpropagation(nodes: &mut Vec<Node>, selected_node_id: i32, result: bool) -> Result<(), &'static str> {
    if let Some(node) = nodes.iter_mut().find(|n| n.id == selected_node_id) {
        node.add_result(result);
        match node.parent_id {
            Some(p_id) => backpropagation(nodes, p_id, result),
            None => Ok(())
        }
    } else {
        Err("Node not found")
    }
}

fn upper_confidence_bound(parent_node: &Node, node: &Node) -> f32 {
    if node.simulations == 0 {
        f32::INFINITY
    } else {
        ( node.wins as f32 / node.simulations as f32 ) + EXPLORATION * ((parent_node.simulations as f32).ln() / node.simulations as f32).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkers::state::mov::MoveKind;
    use crate::checkers::state::game_state;

    #[test]
    fn recommended_move_test() {
       let game_state = game_state::parse(&String::from("W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15")).unwrap();
       let simulation_count: i16 = 10;
       let max_simulation_depth: i16 = 30;

       match recommended_move(game_state, simulation_count, max_simulation_depth) {
           Ok(mov) => {
               assert_eq!((7, 2), mov.from);
               assert_eq!(vec![(6, 3)], mov.to);
           },
           Err(e) => assert!(false, "{}", e)
       }
    }

    #[test]
    fn selection_test() {
        let child_node_a_state = game_state::parse(&String::from("W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15")).unwrap();
        let mov_a = Move {
            kind: MoveKind::Mov,
            from: (4, 5),
            to: vec![(3, 4)]
        };
        let child_node_a = Node {
            id: 2,
            parent_id: Some(1),
            child_ids: Vec::new(),
            mov: Some(mov_a),
            state: child_node_a_state,
            wins: 1,
            simulations: 10
        };

        let child_node_b_state = game_state::parse(&String::from("W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,16")).unwrap();
        let mov_b = Move {
            kind: MoveKind::Mov,
            from: (2, 5),
            to: vec![(1, 4)]
        };
        let child_node_b = Node {
            id: 3,
            parent_id: Some(1),
            child_ids: Vec::new(),
            mov: Some(mov_b),
            state: child_node_b_state,
            wins: 15,
            simulations: 30
        };

        let parent_node_state = game_state::parse(&String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12")).unwrap();

        let parent_node = Node {
            id: 1,
            parent_id: None,
            child_ids: vec![2, 3],
            mov: None,
            state: parent_node_state,
            wins: 16,
            simulations: 40
        };

        let nodes = vec![parent_node, child_node_a, child_node_b];

        let result = selection(&nodes);
        match result {
            Ok(r) => assert_eq!(r, 3),
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    fn upper_confidence_bound_test() {
        let node_state = game_state::parse(&String::from("W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15")).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: (4, 5),
            to: vec![(3, 4)]
        };
        let node = Node {
            id: 2,
            parent_id: Some(1),
            child_ids: Vec::new(),
            mov: Some(mov),
            state: node_state,
            wins: 11,
            simulations: 21
        };

        let parent_node_state = game_state::parse(&String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12")).unwrap();

        let parent_node = Node {
            id: 1,
            parent_id: None,
            child_ids: vec![2],
            mov: None,
            state: parent_node_state,
            wins: 7,
            simulations: 10
        };

        let result = upper_confidence_bound(&parent_node, &node);
        assert_eq!(result, 0.99209774);
    }

    #[test]
    fn expansion_test() {
        let node_state = game_state::parse(&String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12")).unwrap();
        let node = Node {
            id: 1,
            parent_id: None,
            child_ids: Vec::new(),
            mov: None,
            state: node_state,
            wins: 0,
            simulations: 0,
        };

        let mut nodes = vec![node];

        match expansion(&mut nodes, 1) {
            Ok(_) => assert_eq!(8, nodes.len()),
            Err(e) => assert!(false, "{}", e)
        }
    }


    #[test]
    fn simulate_test() {
        let node_state = game_state::parse(&String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12")).unwrap();
        let node = Node {
            id: 1,
            parent_id: None,
            child_ids: Vec::new(),
            mov: None,
            state: node_state,
            wins: 0,
            simulations: 0
        };
        let nodes = vec![node];
        let max_simulation_depth = 5;
        match simulate(&nodes, 1, max_simulation_depth) {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, "{}", e)
        }
    }

    #[test]
    fn backpropagation_test() {
        let node_state = game_state::parse(&String::from("W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,12,15")).unwrap();
        let mov = Move {
            kind: MoveKind::Mov,
            from: (4, 5),
            to: vec![(3, 4)]
        };
        let node = Node {
            id: 2,
            parent_id: Some(1),
            child_ids: Vec::new(),
            mov: Some(mov),
            state: node_state,
            wins: 11,
            simulations: 21
        };

        let parent_node_state = game_state::parse(&String::from("B:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12")).unwrap();

        let parent_node = Node {
            id: 1,
            parent_id: None,
            child_ids: vec![2],
            mov: None,
            state: parent_node_state,
            wins: 7,
            simulations: 10
        };

        let mut nodes = vec![parent_node, node];

        match backpropagation(&mut nodes, 2, true) {
            Ok(_) => {
               match nodes.iter().find(|n| n.id == 1) {
                    Some(n) => {
                        assert_eq!(8, n.wins);
                        assert_eq!(11, n.simulations);
                    },
                    None => assert!(false, "Can't find node")
               }
               match nodes.iter().find(|n| n.id == 2) {
                    Some(n) => {
                        assert_eq!(12, n.wins);
                        assert_eq!(22, n.simulations);
                    },
                    None => assert!(false, "Can't find node")
               }
            },
            Err(e) => assert!(false, "{}", e)
        }
    }
}
