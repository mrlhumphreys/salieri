use crate::go::state::game_state::GameState;
use crate::go::state::mov::Move;

// Key: state:move  
// Value: state:wins:simulations 

pub struct Node {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub child_ids: Vec<i32>,
    pub mov: Option<Move>,
    pub state: GameState, 
    pub wins: i32,
    pub simulations: i32,
}

impl Node {
    pub fn leaf(&self) -> bool {
        self.child_ids.len() == 0
    }

    pub fn add_child_ids(&mut self, child_ids: Vec<i32>) -> () {
        self.child_ids = child_ids;
    }

    pub fn add_result(&mut self, result: bool) -> () {
        match result {
            true => self.wins = self.wins + 1,
            false => (),
        }
        self.simulations = self.simulations + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::go::state::game_state;

    #[test]
    fn add_result_win_test() {
        let node_state = game_state::parse(&String::from("PL[B]ABAWXB[0]XW[0]XS")).unwrap();
        let mut node = Node {
            id: 1,
            parent_id: None,
            child_ids: Vec::new(),
            mov: None,
            state: node_state,
            wins: 0,
            simulations: 0,
        };
        node.add_result(true);

        assert_eq!(1, node.wins);
        assert_eq!(1, node.simulations);
    }

    #[test]
    fn add_result_not_win_test() {
        let node_state = game_state::parse(&String::from("PL[B]ABAWXB[0]XW[0]XS")).unwrap();
        let mut node = Node {
            id: 1,
            parent_id: None,
            child_ids: Vec::new(),
            mov: None,
            state: node_state,
            wins: 0,
            simulations: 0
        };
        node.add_result(false);

        assert_eq!(0, node.wins);
        assert_eq!(1, node.simulations);
    }
}
