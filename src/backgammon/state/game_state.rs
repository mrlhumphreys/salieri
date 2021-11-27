use std::cmp::Ordering;

use crate::backgammon::state::die::Die;
use crate::backgammon::state::dice_set::parse_dice_set;
use crate::backgammon::state::bar::Bar;
use crate::backgammon::state::bar::parse_bar;
use crate::backgammon::state::point_set::PointSet;
use crate::backgammon::state::point_set::parse_point_set;
use crate::backgammon::state::point::Point;
use crate::backgammon::state::off_board::OffBoard;
use crate::backgammon::state::off_board::parse_off_board;
use crate::backgammon::state::mov::Move;
use crate::backgammon::state::mov::MoveStep;
use crate::backgammon::state::mov::Location;
use crate::backgammon::state::mov::PointKind;
use crate::backgammon::state::mov::bar_move_step;
use crate::backgammon::state::mov::off_board_move_step;
use crate::backgammon::state::mov::beyond_off_board_move_step;
use crate::backgammon::state::mov::point_to_point_move_step;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    MovePhase,
    RollPhase
}

pub struct GameState {
    pub current_player_number: i8,
    pub current_phase: Phase,
    pub dice: Vec<Die>,
    pub bar: Bar,
    pub points: PointSet,
    pub off_board: OffBoard
}

impl Clone for GameState {
    fn clone(&self) -> GameState {
        GameState {
            current_player_number: self.current_player_number,
            current_phase: self.current_phase,
            dice: self.dice.clone(),
            bar: self.bar.clone(),
            points: self.points.clone(),
            off_board: self.off_board.clone()
        }
    }
}

impl GameState {
    pub fn winner(&self) -> Option<i8> {
        if self.off_board.player_one_piece_count == 15 {
            Some(1)
        } else if self.off_board.player_two_piece_count == 15 {
            Some(2)
        } else {
            None
        }
    }

    fn on_bar(&self) -> bool {
        match self.current_player_number {
            1 => self.bar.player_one_piece_count > 0,
            2 => self.bar.player_two_piece_count > 0,
            _ => false
        }
    }

    fn bearing_off(&self) -> bool {
        !self.points.points.iter().any(|p| p.occupied_by_player(self.current_player_number) && !p.home(self.current_player_number))
    }

    fn back_point_number(&self) -> Option<i8> {
        let players_points = self.points.points.iter().filter(|p| p.occupied_by_player(self.current_player_number));
        let point = match self.current_player_number {
            2 => {
                players_points.max_by(|a,b| {
                    match (a.number).partial_cmp(&b.number) {
                        Some(c) => c,
                        None => Ordering::Equal
                    }
                })
            }, 
            _ => {
                players_points.min_by(|a,b| {
                    match (a.number).partial_cmp(&b.number) {
                        Some(c) => c,
                        None => Ordering::Equal
                    }
                })
            } 
        };
        match point {
            Some(p) => Some(p.number),
            None => None
        }
    }

    fn possible_steps(&self) -> Vec<MoveStep> {
        let mut move_steps: Vec<MoveStep> = Vec::new();

        for die in self.dice.iter().filter(|d| !d.used) {
            let die_number = match die.number {
                Some(d) => d,
                None => 0
            };

            
            if self.on_bar() {
                let destination_point_number = self.bar_destination_point_number(die_number); 
                let destination_point = self.find_point_by_number(destination_point_number);
                let move_step = bar_move_step(destination_point, die_number, self.current_player_number);
                match move_step {
                    Some(ms) => {
                        move_steps.push(ms)
                    },
                    None => ()
                }
            } else {
                let mut move_step: Option<MoveStep>;
                for point in self.points.points.iter().filter(|p| p.occupied_by_player(self.current_player_number)) {
                    let destination_point_number = self.point_destination_point_number(point.number, die_number);

                    if self.bearing_off() {
                        if destination_point_number == 25 || destination_point_number == 1 { 
                            move_step = off_board_move_step(point, die_number);
                        } else if destination_point_number > 25 || destination_point_number < 1 {
                            move_step = beyond_off_board_move_step(point, die_number, self.back_point_number());
                        } else {
                            let destination_point = self.find_point_by_number(destination_point_number);
                            move_step = point_to_point_move_step(point, destination_point, die_number, self.current_player_number);
                        }
                    } else {
                        let destination_point = self.find_point_by_number(destination_point_number);
                        move_step = point_to_point_move_step(point, destination_point, die_number, self.current_player_number);
                    }

                    match move_step {
                        Some(ms) => {
                            move_steps.push(ms)
                        },
                        None => ()
                    }
                }
            }

        }
        move_steps 
    }

    fn find_moves(&self, step_list: Vec<MoveStep>, mut moves: Vec<Move>) -> Result<Vec<Move>, &'static str> {
        let steps = self.possible_steps(); 
        if steps.len() == 0 {
            // generate move and push to list
            let mov = Move { list: step_list };
            moves.push(mov);
        } else {
            for step in steps {
                let mut new_step_list = step_list.clone();
                new_step_list.push(step.clone());
                let mut new_game_state = self.clone(); 
                match new_game_state.perform_move_step(&step) {
                   Ok(_) => (),
                   Err(e) => return Err(e) 
                };

                match new_game_state.find_moves(new_step_list, moves) {
                    Ok(m) => moves = m,
                    Err(e) => return Err(e)
                }
            }
        }
        Ok(moves)
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        let moves: Vec<Move> = vec![];
        let step_list: Vec<MoveStep> = vec![];
        match self.find_moves(step_list, moves) {
            Ok(m) => return m,
            Err(_) => return vec![]
        }
    }

    fn bar_destination_point_number(&self, die_number: i8) -> i8 {
        match self.current_player_number {
            2 => 25 - die_number,
            _ => die_number
        }
    }

    fn point_destination_point_number(&self, point_number: i8, die_number: i8) -> i8 {
        match self.current_player_number {
            2 => point_number - die_number,
            _ => point_number + die_number
        }
    }

    fn find_point_by_number(&self, point_number: i8) -> Option<&Point> {
        self.points.points.iter().find(|p| p.number == point_number)
    }

    fn perform_move_step(&mut self, move_step: &MoveStep) -> Result<(), &'static str> {
        match self.dice.iter_mut().find(|d| !d.used && d.number == Some(move_step.die_number)) {
            Some(d) => d.mark_used(),
            None => return Err("no unused die matching number")
        };

        match self.pop_piece(self.current_player_number, &move_step.from) {
            Ok(popped_piece) => self.push_piece(&move_step.to, popped_piece),
            Err(e) => Err(e)
        }
    }
    
    pub fn perform_set_roll(&self, die_a: i8, die_b: i8) -> GameState {
        let mut new_game_state = self.clone();

        if die_a == die_b {
            new_game_state.dice = vec![
                Die { number: Some(die_a), used: false },
                Die { number: Some(die_b), used: false },
                Die { number: Some(die_a), used: false },
                Die { number: Some(die_b), used: false }
            ];
        } else {
            new_game_state.dice = vec![
                Die { number: Some(die_a), used: false },
                Die { number: Some(die_b), used: false }
            ];
        }

        new_game_state.current_phase = Phase::MovePhase;

        new_game_state
    }

    pub fn perform_move(&self, mov: &Move) -> Result<GameState, &'static str> {
        let mut new_game_state = self.clone();

        let items: Result<Vec<_>, _> = mov.list.iter().map(|step| {
            new_game_state.perform_move_step(step)
        }).collect();

        match items {
            Ok(_) => (),
            Err(e) => return Err(e)
        };

        new_game_state.dice = vec![
            Die { number: None, used: false },
            Die { number: None, used: false }
        ];

        match self.current_player_number { 
            1 => new_game_state.current_player_number = 2,
            2 => new_game_state.current_player_number = 1,
            _ => return Err("invalid player number")
        };

        new_game_state.current_phase = Phase::RollPhase;

        Ok(new_game_state)
    }
    
    fn pop_piece(&mut self, player_number: i8, location: &Location) -> Result<i8, &'static str> {
        match location.kind {
            PointKind::Point => {
                match location.number {
                    Some(n) => self.points.pop_piece(n),
                    None => Err("point number must be specified")
                }
            }
            PointKind::Bar => self.bar.pop_piece(player_number),
            PointKind::OffBoard => Err("Cannot move piece from OffBoard") 
        }
    }

    fn push_piece(&mut self, location: &Location, piece: i8) -> Result<(), &'static str> {
        match location.kind {
            PointKind::Point => {
                match location.number {
                    Some(n) => {
                        match self.points.push_piece(piece, n) {
                            Ok(res) => {
                                match res {
                                    Some(hit_piece) => self.bar.push_piece(hit_piece),
                                    None => Ok(())
                                }
                            },
                            Err(e) => Err(e)
                        }
                    },
                    None => Err("point number must be specified")
                }
            },
            PointKind::Bar => self.bar.push_piece(piece),
            PointKind::OffBoard => self.off_board.push_piece(piece) 
        }
    }
}

pub fn parse(encoded: &String) -> Result<GameState, &'static str> {
    let bar_component = &encoded[0..2];
    let points_component = &encoded[2..50];
    let off_board_component = &encoded[50..52];
    let dice_component = &encoded[52..54];
    let player_component = &encoded[54..55];

    let current_player_number = match player_component {
        "1" => 1,
        "2" => 2,
        _ => return Err("Invalid State")
    };

    let current_phase = Phase::RollPhase;

    let dice = match parse_dice_set(dice_component) {
        Ok(d) => d,
        Err(e) => return Err(e)
    };

    let bar = match parse_bar(bar_component) {
        Ok(b) => b,
        Err(e) => return Err(e)
    };

    let points = match parse_point_set(points_component) {
        Ok(p) => p,
        Err(e) => return Err(e)
    };

    let off_board = match parse_off_board(off_board_component) {
        Ok(o) => o,
        Err(e) => return Err(e)
    };
    
    let game_state = GameState {
        current_player_number,
        current_phase,
        dice,
        bar,
        points,
        off_board
    };

    Ok(game_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backgammon::state::die::Die;
    use crate::backgammon::state::point::Point;

    #[test]
    fn parse_test() {
        let encoded = String::from("0020000000000500300000005002000000005000300000000500121");
        let result = parse(&encoded).unwrap();

        assert_eq!(result.current_player_number, 1);

        match result.current_phase {
            Phase::RollPhase => assert!(true),
            Phase::MovePhase => assert!(false, "must be roll phase")
        }

        assert_eq!(result.dice.len(), 2);
        assert_eq!(result.bar.player_one_piece_count, 0);
        assert_eq!(result.bar.player_two_piece_count, 0);
        assert_eq!(result.points.points.len(), 24);
        assert_eq!(result.off_board.player_one_piece_count, 0);
        assert_eq!(result.off_board.player_two_piece_count, 0);
    }

    #[test]
    fn winner_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die_a = Die { number: Some(1), used: true };
        let die_b = Die { number: Some(2), used: true };
        let dice = vec![die_a, die_b];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point_a = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 2 
        };
        let point_b = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point_c = Point { 
            number: 3, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point_d = Point { 
            number: 4, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point_a, point_b, point_c, point_d] };
        let off_board = OffBoard { 
            player_one_piece_count: 15,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };
        
        assert_eq!(game_state.winner(), Some(1)); 
    }

    #[test]
    fn no_winner_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die_a = Die { number: Some(1), used: true };
        let die_b = Die { number: Some(2), used: true };
        let dice = vec![die_a, die_b];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point_a = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 2
        };
        let point_b = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point_c = Point { 
            number: 3, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point_d = Point { 
            number: 4, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point_a, point_b, point_c, point_d] };
        let off_board = OffBoard { 
            player_one_piece_count: 14,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };
        
        assert_eq!(game_state.winner(), None);
    }

    #[test]
    fn possible_moves_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die_a = Die { number: Some(1), used: false };
        let die_b = Die { number: Some(2), used: false };
        let dice = vec![die_a, die_b];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point_a = Point { 
            number: 1, 
            player_one_piece_count: 2,
            player_two_piece_count: 0
        };
        let point_b = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point_c = Point { 
            number: 3, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point_d = Point { 
            number: 4, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point_a, point_b, point_c, point_d] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let possible_moves = game_state.possible_moves();

        assert_eq!(possible_moves.len(), 4);

        let move_a_step_a = &possible_moves[0].list[0];

        assert_eq!(move_a_step_a.from.kind, PointKind::Point); 
        assert_eq!(move_a_step_a.from.number, Some(1)); 
        assert_eq!(move_a_step_a.to.kind, PointKind::Point); 
        assert_eq!(move_a_step_a.to.number, Some(2)); 
        assert_eq!(move_a_step_a.die_number, 1); 

        let move_a_step_b = &possible_moves[0].list[1];

        assert_eq!(move_a_step_b.from.kind, PointKind::Point); 
        assert_eq!(move_a_step_b.from.number, Some(1)); 
        assert_eq!(move_a_step_b.to.kind, PointKind::Point); 
        assert_eq!(move_a_step_b.to.number, Some(3)); 
        assert_eq!(move_a_step_b.die_number, 2); 

        let move_b_step_a = &possible_moves[1].list[0];

        assert_eq!(move_b_step_a.from.kind, PointKind::Point); 
        assert_eq!(move_b_step_a.from.number, Some(1)); 
        assert_eq!(move_b_step_a.to.kind, PointKind::Point); 
        assert_eq!(move_b_step_a.to.number, Some(2)); 
        assert_eq!(move_b_step_a.die_number, 1); 

        let move_b_step_b = &possible_moves[1].list[1];

        assert_eq!(move_b_step_b.from.kind, PointKind::Point); 
        assert_eq!(move_b_step_b.from.number, Some(2)); 
        assert_eq!(move_b_step_b.to.kind, PointKind::Point); 
        assert_eq!(move_b_step_b.to.number, Some(4)); 
        assert_eq!(move_b_step_b.die_number, 2); 

        let move_c_step_a = &possible_moves[2].list[0];

        assert_eq!(move_c_step_a.from.kind, PointKind::Point); 
        assert_eq!(move_c_step_a.from.number, Some(1)); 
        assert_eq!(move_c_step_a.to.kind, PointKind::Point); 
        assert_eq!(move_c_step_a.to.number, Some(3)); 
        assert_eq!(move_c_step_a.die_number, 2); 

        let move_c_step_b = &possible_moves[2].list[1];

        assert_eq!(move_c_step_b.from.kind, PointKind::Point); 
        assert_eq!(move_c_step_b.from.number, Some(1)); 
        assert_eq!(move_c_step_b.to.kind, PointKind::Point); 
        assert_eq!(move_c_step_b.to.number, Some(2)); 
        assert_eq!(move_c_step_b.die_number, 1); 

        let move_d_step_a = &possible_moves[3].list[0];

        assert_eq!(move_d_step_a.from.kind, PointKind::Point); 
        assert_eq!(move_d_step_a.from.number, Some(1)); 
        assert_eq!(move_d_step_a.to.kind, PointKind::Point); 
        assert_eq!(move_d_step_a.to.number, Some(3)); 
        assert_eq!(move_d_step_a.die_number, 2); 

        let move_d_step_b = &possible_moves[3].list[1];

        assert_eq!(move_d_step_b.from.kind, PointKind::Point); 
        assert_eq!(move_d_step_b.from.number, Some(3)); 
        assert_eq!(move_d_step_b.to.kind, PointKind::Point); 
        assert_eq!(move_d_step_b.to.number, Some(4)); 
        assert_eq!(move_d_step_b.die_number, 1); 
    }

    #[test]
    fn on_bar_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let point = Point { 
            number: 19, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let result = game_state.on_bar();
        assert!(result)
    }

    #[test]
    fn not_on_bar_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point = Point { 
            number: 19, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let result = game_state.on_bar();
        assert!(!result)
    }

    #[test]
    fn not_bearing_off_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let non_home_point = Point { 
            number: 18, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let home_point = Point { 
            number: 19, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![non_home_point, home_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let result = game_state.bearing_off();
        assert!(!result)
    }

    #[test]
    fn bearing_off_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let non_home_point = Point { 
            number: 18, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let home_point = Point { 
            number: 19, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![non_home_point, home_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let result = game_state.bearing_off();
        assert!(result)
    }

    #[test]
    fn back_point_number_player_1_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let front_point = Point { 
            number: 24, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let back_point = Point { 
            number: 18, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![front_point, back_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let result = game_state.back_point_number();

        match result {
            Some(n) => assert_eq!(n, 18),
            None => assert!(false, "expected number")
        }
    }

    #[test]
    fn back_point_number_player_2_test() {
        let current_player_number = 2;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let front_point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 1 
        };
        let back_point = Point { 
            number: 6, 
            player_one_piece_count: 0,
            player_two_piece_count: 1 
        };
        let points = PointSet { points: vec![front_point, back_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let result = game_state.back_point_number();

        match result {
            Some(n) => assert_eq!(n, 6),
            None => assert!(false, "expected number")
        }
    }

    #[test]
    fn back_point_number_none_test() {
        let current_player_number = 2;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let front_point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let back_point = Point { 
            number: 6, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![front_point, back_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let result = game_state.back_point_number(); 

        match result {
            Some(_) => assert!(false, "expected no back point"),
            None => assert!(true)
        }
    }

    #[test]
    fn point_to_point_move_step_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 1, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let to_point = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let beyond_point = Point { 
            number: 3, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![from_point, to_point, beyond_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let steps = game_state.possible_steps();

        assert_eq!(steps.len(), 1);

        match steps.first() {
            Some(s) => {
                assert_eq!(s.from.kind, PointKind::Point);  
                assert_eq!(s.from.number, Some(1));  
                assert_eq!(s.to.kind, PointKind::Point);  
                assert_eq!(s.to.number, Some(2));  
                assert_eq!(s.die_number, 1);
            },
            None => assert!(false, "expected step")
        }
    }

    #[test]
    fn bar_move_step_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let to_point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let beyond_point = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![to_point, beyond_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let steps = game_state.possible_steps();

        assert_eq!(steps.len(), 1);

        match steps.first() {
            Some(s) => {
                assert_eq!(s.from.kind, PointKind::Bar);  
                assert_eq!(s.from.number, None);  
                assert_eq!(s.to.kind, PointKind::Point);  
                assert_eq!(s.to.number, Some(1));  
                assert_eq!(s.die_number, 1);
            },
            None => assert!(false, "expected step")
        }
    }

    #[test]
    fn off_board_move_step_bearing_off_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 24, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![from_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let steps = game_state.possible_steps();

        assert_eq!(steps.len(), 1);

        match steps.first() {
            Some(s) => {
                assert_eq!(s.from.kind, PointKind::Point);  
                assert_eq!(s.from.number, Some(24));  
                assert_eq!(s.to.kind, PointKind::OffBoard);  
                assert_eq!(s.to.number, None);  
                assert_eq!(s.die_number, 1);
            },
            None => assert!(false, "expected step")
        }
    }

    #[test]
    fn off_board_move_step_not_bearing_off_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let not_home_point = Point { 
            number: 18, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 24, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![from_point, not_home_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let steps = game_state.possible_steps();

        assert_eq!(steps.len(), 0);
    }

    #[test]
    fn beyond_off_board_move_step_back_point_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(6), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let back_point = Point { 
            number: 19, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 24, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![back_point, from_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let steps = game_state.possible_steps();

        assert_eq!(steps.len(), 1);

        match steps.first() {
            Some(s) => {
                assert_eq!(s.from.kind, PointKind::Point);  
                assert_eq!(s.from.number, Some(24));  
                assert_eq!(s.to.kind, PointKind::OffBoard);  
                assert_eq!(s.to.number, None);  
                assert_eq!(s.die_number, 6);
            },
            None => assert!(false, "expected step")
        }
    }

    #[test]
    fn beyond_off_board_move_step_not_back_point_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(6), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let back_point = Point { 
            number: 19, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 24, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![back_point, from_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let steps = game_state.possible_steps();

        assert_eq!(steps.len(), 1);

        match steps.first() {
            Some(s) => {
                assert_eq!(s.from.kind, PointKind::Point);  
                assert_eq!(s.from.number, Some(19));  
                assert_eq!(s.to.kind, PointKind::OffBoard);  
                assert_eq!(s.to.number, None);  
                assert_eq!(s.die_number, 6);
            },
            None => assert!(false, "expected step")
        }
    }

    #[test]
    fn perform_set_roll_test() {
        let current_player_number = 1;
        let current_phase = Phase::RollPhase;
        let die_a = Die { number: None, used: false };
        let die_b = Die { number: None, used: false };
        let dice = vec![die_a, die_b];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 1, 
            player_one_piece_count: 2,
            player_two_piece_count: 0
        };
        let to_point_a = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let to_point_b = Point { 
            number: 3, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![from_point, to_point_a, to_point_b] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let new_game_state = game_state.perform_set_roll(1, 2);

        match new_game_state.dice.iter().find(|p| p.number == Some(1)) {
            Some(_) => assert!(true),
            None => assert!(false, "expected dice")
        }

        match new_game_state.dice.iter().find(|p| p.number == Some(2)) {
            Some(_) => assert!(true),
            None => assert!(false, "expected dice")
        }

        assert_eq!(new_game_state.current_phase, Phase::MovePhase);
    }

    #[test]
    fn peform_set_roll_double_test() {
        let current_player_number = 1;
        let current_phase = Phase::RollPhase;
        let die_a = Die { number: None, used: false };
        let die_b = Die { number: None, used: false };
        let dice = vec![die_a, die_b];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 1, 
            player_one_piece_count: 2,
            player_two_piece_count: 0
        };
        let to_point_a = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let to_point_b = Point { 
            number: 3, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![from_point, to_point_a, to_point_b] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let new_game_state = game_state.perform_set_roll(2, 2);

        match new_game_state.dice.iter().find(|p| p.number == Some(2)) {
            Some(_) => assert!(true),
            None => assert!(false, "expected dice")
        }

        assert_eq!(new_game_state.dice.len(), 4);
        assert_eq!(new_game_state.current_phase, Phase::MovePhase);
    }

    #[test]
    fn perform_move_valid_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die_a = Die { number: Some(1), used: false };
        let die_b = Die { number: Some(2), used: false };
        let dice = vec![die_a, die_b];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 1, 
            player_one_piece_count: 2,
            player_two_piece_count: 0
        };
        let to_point_a = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let to_point_b = Point { 
            number: 3, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![from_point, to_point_a, to_point_b] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let from_a = Location { kind: PointKind::Point, number: Some(1) };
        let to_a = Location { kind: PointKind::Point, number: Some(2) };
        let die_number_a = 1;
        let move_step_a = MoveStep { from: from_a, to: to_a, die_number: die_number_a };

        let from_b = Location { kind: PointKind::Point, number: Some(1) };
        let to_b = Location { kind: PointKind::Point, number: Some(3) };
        let die_number_b = 2;
        let move_step_b = MoveStep { from: from_b, to: to_b, die_number: die_number_b };

        let mov = Move { list: vec![move_step_a, move_step_b] };

        match game_state.perform_move(&mov) {
            Ok(new_game_state) => {
                match new_game_state.points.points.iter().find(|p| p.number == 1) {
                    Some(p) => assert_eq!(p.player_one_piece_count, 0),
                    None => assert!(false, "expected piece")
                }

                match new_game_state.points.points.iter().find(|p| p.number == 2) {
                    Some(p) => assert_eq!(p.player_one_piece_count, 1),
                    None => assert!(false, "expected piece")
                }

                match new_game_state.points.points.iter().find(|p| p.number == 3) {
                    Some(p) => assert_eq!(p.player_one_piece_count, 1),
                    None => assert!(false, "expected piece")
                }

                assert!(new_game_state.dice.iter().all(|d| d.number == None && d.used == false));

                assert_eq!(new_game_state.current_phase, Phase::RollPhase);
                assert_eq!(new_game_state.current_player_number, 2);
            },
            Err(_) => assert!(false, "expected no error") 
        }
    }

    #[test]
    fn perform_move_invalid_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die_a = Die { number: Some(1), used: false };
        let die_b = Die { number: Some(2), used: false };
        let dice = vec![die_a, die_b];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 1, 
            player_one_piece_count: 2,
            player_two_piece_count: 0
        };
        let to_point_a = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let to_point_b = Point { 
            number: 3, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![from_point, to_point_a, to_point_b] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let from_a = Location { kind: PointKind::Point, number: Some(1) };
        let to_a = Location { kind: PointKind::Point, number: Some(2) };
        let die_number_a = 1;
        let move_step_a = MoveStep { from: from_a, to: to_a, die_number: die_number_a };

        let from_b = Location { kind: PointKind::Point, number: Some(1) };
        let to_b = Location { kind: PointKind::Point, number: Some(3) };
        let die_number_b = 7;
        let move_step_b = MoveStep { from: from_b, to: to_b, die_number: die_number_b };

        let mov = Move { list: vec![move_step_a, move_step_b] };

        match game_state.perform_move(&mov) {
            Ok(_) => assert!(false, "expected error"),
            Err(_) => assert!(true)
        };
    }

    #[test]
    fn perform_move_step_valid_test() {
        let current_player_number = 1;
        let current_phase = Phase::RollPhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 1, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let to_point = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![from_point, to_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let from = Location { kind: PointKind::Point, number: Some(1) };
        let to = Location { kind: PointKind::Point, number: Some(2) };
        let die_number = 1;
        let move_step = MoveStep { from, to, die_number };
        
        let result = game_state.perform_move_step(&move_step);

        match result {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, e)
        }

        match game_state.points.points.iter().find(|p| p.number == 1) {
            Some(p) => assert_eq!(p.player_one_piece_count, 0),
            None => assert!(false, "expected point") 
        }

        match game_state.points.points.iter().find(|p| p.number == 2) {
            Some(p) => assert_eq!(p.player_one_piece_count, 1),
            None => assert!(false, "expected point") 
        }

        match game_state.dice.iter().find(|d| d.number == Some(1)) {
            Some(d) => assert!(d.used), 
            None => assert!(false, "expected die")
        }
    }

    #[test]
    fn perform_move_step_invalid_test() {
        let current_player_number = 1;
        let current_phase = Phase::RollPhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let to_point = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { 
            points: vec![from_point, to_point] 
        };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let from = Location { kind: PointKind::Point, number: Some(1) };
        let to = Location { kind: PointKind::Point, number: Some(2) };
        let die_number = 1;
        let move_step = MoveStep { from, to, die_number };

        match game_state.perform_move_step(&move_step) {
            Ok(_) => assert!(false, "expected error"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn perform_move_step_invalid_die_number_test() {
        let current_player_number = 1;
        let current_phase = Phase::RollPhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let from_point = Point { 
            number: 1, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let to_point = Point { 
            number: 2, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![from_point, to_point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let from = Location { kind: PointKind::Point, number: Some(1) };
        let to = Location { kind: PointKind::Point, number: Some(2) };
        let die_number = 3;
        let move_step = MoveStep { from, to, die_number };

        match game_state.perform_move_step(&move_step) {
            Ok(_) => assert!(false, "expected error"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn pop_piece_from_point_test() {
        let current_player_number = 1;
        let current_phase = Phase::RollPhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point = Point { 
            number: 1, 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let location = Location {
           kind: PointKind::Point,
           number: Some(1),
        };

        match game_state.pop_piece(1, &location) {
            Ok(p) => assert_eq!(1, p),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn pop_piece_from_bar_test() {
        let current_player_number = 1;
        let current_phase = Phase::RollPhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let location = Location {
           kind: PointKind::Bar,
           number: None,
        };

        match game_state.pop_piece(1, &location) {
            Ok(p) => assert_eq!(1, p),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn pop_piece_from_off_board_test() {
        let current_player_number = 1;
        let current_phase = Phase::RollPhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point] };
        let off_board = OffBoard { 
            player_one_piece_count: 1,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let location = Location {
           kind: PointKind::OffBoard,
           number: None,
        };

        match game_state.pop_piece(1, &location) {
            Ok(_) => assert!(false, "cannot pop piece from off board"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn push_piece_on_point_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let piece = 1;
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let location = Location {
           kind: PointKind::Point,
           number: Some(1)
        };

        match game_state.push_piece(&location, piece) {
            Ok(_) => assert_eq!(game_state.points.points[0].player_one_piece_count, 1),
            Err(_) => assert!(false, "expected no error")
        }
    }

    #[test]
    fn push_piece_with_no_point_specified_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let piece = 1;
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let location = Location {
           kind: PointKind::Point,
           number: None 
        };

        match game_state.push_piece(&location, piece) {
            Ok(_) => assert!(false, "expected error"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn push_piece_with_invalid_point_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let piece = 1;
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let location = Location {
           kind: PointKind::Point,
           number: Some(5) 
        };

        match game_state.push_piece(&location, piece) {
            Ok(_) => assert!(false, "expected error"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn push_piece_on_blot_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let piece = 1;
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 1 
        };
        let points = PointSet { points: vec![point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let location = Location {
           kind: PointKind::Point,
           number: Some(1) 
        };

        match game_state.push_piece(&location, piece) {
            Ok(_) => {
                assert_eq!(game_state.points.points[0].player_one_piece_count, 1);
                assert_eq!(1, game_state.bar.player_two_piece_count); 
            },
            Err(_) => assert!(false, "expected no error")
        }
    }

    #[test]
    fn push_piece_on_bar_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let piece = 1;
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let location = Location {
           kind: PointKind::Bar,
           number: None 
        };

        match game_state.push_piece(&location, piece) {
            Ok(_) => assert_eq!(1, game_state.bar.player_one_piece_count), 
            Err(_) => assert!(false, "expected no error")
        }
    }

    #[test]
    fn push_piece_on_off_board_test() {
        let current_player_number = 1;
        let current_phase = Phase::MovePhase;
        let die = Die { number: Some(1), used: false };
        let dice = vec![die];
        let piece = 1;
        let bar = Bar { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let point = Point { 
            number: 1, 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let points = PointSet { points: vec![point] };
        let off_board = OffBoard { 
            player_one_piece_count: 0,
            player_two_piece_count: 0
        };
        let mut game_state = GameState {
            current_player_number,
            current_phase,
            dice,
            bar,
            points,
            off_board
        };

        let location = Location {
           kind: PointKind::OffBoard,
           number: None 
        };

        match game_state.push_piece(&location, piece) {
            Ok(_) => assert_eq!(game_state.off_board.player_one_piece_count, 1), 
            Err(_) => assert!(false, "expected no error")
        }
    }
}
