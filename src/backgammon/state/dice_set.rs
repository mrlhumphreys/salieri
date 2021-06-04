use crate::backgammon::state::die::Die;
use crate::backgammon::state::die::parse as parse_die;

pub struct DiceSet {
    pub dice: Vec<Die>
}

impl Clone for DiceSet {
    fn clone(&self) -> DiceSet {
        DiceSet {
            dice: self.dice.clone()
        }
    }
}

pub fn parse_dice_set(encoded: &str) -> Result<DiceSet, &'static str> {
    if encoded.len() == 2 {
        let mut dice = Vec::new();
    
        for c in encoded.chars() {
            match parse_die(c) {
                Ok(d) => dice.push(d),
                Err(e) => return Err(e)
            }
        }

        let dice_set = DiceSet { dice };
        Ok(dice_set)
    } else {
        Err("invalid dice set")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_dice_set_test() {
       let encoded = "12"; 
       let dice_set = parse_dice_set(encoded).unwrap();
       let dice = dice_set.dice;
       assert_eq!(dice.len(), 2);
       let die = &dice[0];
       assert_eq!(die.number.unwrap(), 1);
    }

    #[test]
    fn parsing_one_die_test() {
       let encoded = "1";    
       let dice_set = parse_dice_set(encoded);

       match dice_set {
            Ok(_) => assert!(false, "should not return dice set"),
            Err(_) => assert!(true)
       }
    }

    #[test]
    fn parsing_three_die_test() {
       let encoded = "123";    
       let dice_set = parse_dice_set(encoded);

       match dice_set {
            Ok(_) => assert!(false, "should not return dice set"),
            Err(_) => assert!(true)
       }
    }
}
