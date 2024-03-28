use crate::backgammon::state::die::Die;
use crate::backgammon::state::die::parse as parse_die;

pub fn parse_dice_set(encoded: &str) -> Result<Vec<Die>, &'static str> {
    if encoded.len() == 2 {
        let mut dice = Vec::new();

        if &encoded[0..1] == &encoded[1..2] {
            let characters = encoded.chars();
            for c in characters {
                match parse_die(c) {
                    Ok(d) => {
                        // double rolled, duplicate dice
                        dice.push(d.clone());
                        dice.push(d);
                    },
                    Err(e) => return Err(e)
                }
            }
        } else {
            let characters = encoded.chars();
            for c in characters {
                match parse_die(c) {
                    Ok(d) => dice.push(d),
                    Err(e) => return Err(e)
                }
            }
        }

        Ok(dice)
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
       let dice = parse_dice_set(encoded).unwrap();
       assert_eq!(dice.len(), 2);
       let die = &dice[0];
       assert_eq!(die.number.unwrap(), 1);
    }

    #[test]
    fn parsing_double_dice_set_test() {
       let encoded = "22";
       let dice = parse_dice_set(encoded).unwrap();
       assert_eq!(dice.len(), 4);
       let die = &dice[0];
       assert_eq!(die.number.unwrap(), 2);
    }

    #[test]
    fn parsing_one_die_test() {
       let encoded = "1";
       let dice = parse_dice_set(encoded);

       match dice {
            Ok(_) => assert!(false, "should not return dice set"),
            Err(_) => assert!(true)
       }
    }

    #[test]
    fn parsing_three_die_test() {
       let encoded = "123";
       let dice = parse_dice_set(encoded);

       match dice {
            Ok(_) => assert!(false, "should not return dice set"),
            Err(_) => assert!(true)
       }
    }
}
