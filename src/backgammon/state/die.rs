use std::convert::TryFrom;

pub struct Die {
    pub number: Option<i8>,
    pub used: bool
}

impl Die {
    pub fn mark_used(&mut self) -> bool {
       self.used = true;
       true
    }

    pub fn mark_unused(&mut self) -> bool {
       self.used = false;
       true
    }
}

impl Clone for Die {
    fn clone(&self) -> Die {
        Die {
            number: self.number,
            used: self.used
        }
    }
}

pub fn parse(encoded: char) -> Result<Die, &'static str> {
    let number = match encoded {
        '1'..='6' => {
            if let Some(n) = encoded.to_digit(10) {
                match i8::try_from(n) {
                    Ok(n) => Some(n),
                    Err(_) => return Err("Invalid Die Number")
                }
            } else {
                return Err("Invalid Die Number");
            }
        },
        '-' => None,
        _ => return Err("Invalid die Number")
    };

    let used = false;

    let die = Die { number, used };
    Ok(die)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_blank_test() {
        let encoded = '-';
        let die = parse(encoded).unwrap();
        assert_eq!(die.used, false);
        match die.number {
            Some(_) => assert!(false, "die must not be number"),
            None => assert!(true)
        }
    }

    #[test]
    fn parsing_die_number_test() {
        let encoded = '6';
        let die = parse(encoded).unwrap();
        assert_eq!(die.used, false);
        match die.number {
            Some(n) => assert_eq!(n, 6),
            None => assert!(false, "die must not be blank")
        }
    }

    #[test]
    fn parsing_high_number_test() {
        let encoded = '7';
        let die = parse(encoded);
        match die {
            Ok(_) => assert!(false, "die must be invalid"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn parsing_char_test() {
        let encoded = 'c';
        let die = parse(encoded);
        match die {
            Ok(_) => assert!(false, "die must be invalid"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn mark_used_test() {
        let mut die = Die { number: Some(1), used: false };
        die.mark_used();
        assert!(die.used);
    }
}
