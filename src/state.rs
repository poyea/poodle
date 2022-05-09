use chrono::prelude::*;
use colored::*;
use std::{cmp::PartialEq, fmt};

#[derive(Debug, PartialEq)]
pub enum Result {
    Wrong,
    Correct,
    Partial,
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let block = match self {
            Result::Wrong => '⬛'.to_string().truecolor(105, 105, 105),
            Result::Correct => '🟩'.to_string().truecolor(51, 255, 51),
            Result::Partial => '🟨'.to_string().truecolor(255, 255, 51),
        };
        write!(f, "{}", block)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Attempt {
    pub slots: Vec<Result>,
}

impl fmt::Display for Attempt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for slot in &self.slots {
            write!(f, "{}", slot)?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Stat {
    pub attempts: Vec<Attempt>,
}

impl Stat {
    pub fn start() -> Stat {
        Stat {
            attempts: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct DayState {
    pub stat: Stat,
    riddle: String,
    pub date: String,
    pub total_guess: i64,
    pub remaining_guess: i64,
}

impl DayState {
    pub fn get_today() -> String {
        format!("{}", Local::now().format("%b %d, %Y"))
    }

    pub fn new(riddle: String, total_guess: i64) -> DayState {
        DayState {
            riddle: riddle,
            date: DayState::get_today(),
            total_guess: total_guess,
            remaining_guess: total_guess,
            stat: Stat::start(),
        }
    }

    pub fn input_hygiene(&self, guess: &String) -> bool {
        return guess.len() == self.riddle.len() && guess.chars().all(|c| c.is_ascii_lowercase());
    }

    pub fn input_allowed(&self, guess: &String, allowed: &Vec<String>) -> bool {
        allowed.binary_search_by(|p| p.cmp(&guess)).is_ok()
    }

    fn char_compare(_guess_char: &char, _actual_char: &char) -> bool {
        _guess_char == _actual_char
    }

    fn validate(_guess: &String, _actual: &String) -> Attempt {
        let mut res = Vec::new();
        let guess = _guess.chars().collect::<Vec<_>>();
        let actual = _actual.chars().collect::<Vec<_>>();
        let n = _actual.len();
        for i in 0..n {
            let mut result_holder = Result::Wrong;
            for j in 0..n {
                if DayState::char_compare(&guess[i], &actual[j]) {
                    if i == j {
                        result_holder = Result::Correct;
                        break;
                    } else {
                        result_holder = Result::Partial;
                    }
                }
            }
            res.push(result_holder);
        }
        Attempt { slots: res }
    }

    pub fn guess(&mut self, word: &String) -> String {
        self.remaining_guess -= 1;
        self.stat
            .attempts
            .push(DayState::validate(&word, &self.riddle));
        format!("{}", self.stat.attempts.last().unwrap())
    }

    pub fn finished(&self) -> bool {
        return self.remaining_guess == 0
            || (!self.stat.attempts.is_empty()
                && self
                    .stat
                    .attempts
                    .last()
                    .unwrap()
                    .slots
                    .iter()
                    .all(|res| res == &Result::Correct));
    }
}

impl fmt::Display for DayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n<>==========<>\n")?;
        write!(
            f,
            "Poodle {} {}/{}\n",
            self.date,
            self.total_guess - self.remaining_guess,
            self.total_guess
        )?;
        for attempt in &self.stat.attempts {
            write!(f, "{}", attempt)?;
        }
        write!(f, "<>==========<>\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_correctness() {
        assert_eq!(
            DayState::validate(&String::from("aaaaa"), &String::from("aaaaa")),
            Attempt {
                slots: vec![
                    Result::Correct,
                    Result::Correct,
                    Result::Correct,
                    Result::Correct,
                    Result::Correct
                ]
            }
        );

        assert_eq!(
            DayState::validate(&String::from("bbbbb"), &String::from("ccccc")),
            Attempt {
                slots: vec![
                    Result::Wrong,
                    Result::Wrong,
                    Result::Wrong,
                    Result::Wrong,
                    Result::Wrong
                ]
            }
        );
    }

    #[test]
    fn validate_appearance() {
        assert_eq!(
            DayState::validate(&String::from("abyde"), &String::from("edxba")),
            Attempt {
                slots: vec![
                    Result::Partial,
                    Result::Partial,
                    Result::Wrong,
                    Result::Partial,
                    Result::Partial
                ]
            }
        );

        assert_eq!(
            DayState::validate(&String::from("aaxbb"), &String::from("bbxaa")),
            Attempt {
                slots: vec![
                    Result::Partial,
                    Result::Partial,
                    Result::Correct,
                    Result::Partial,
                    Result::Partial
                ]
            }
        );

        assert_eq!(
            DayState::validate(&String::from("arias"), &String::from("abyss")),
            Attempt {
                slots: vec![
                    Result::Correct,
                    Result::Wrong,
                    Result::Wrong,
                    Result::Partial,
                    Result::Correct
                ]
            }
        );

        assert_eq!(
            DayState::validate(&String::from("again"), &String::from("abyss")),
            Attempt {
                slots: vec![
                    Result::Correct,
                    Result::Wrong,
                    Result::Partial,
                    Result::Wrong,
                    Result::Wrong
                ]
            }
        );
    }

    #[test]
    fn validate_char_compare() {
        assert_eq!(DayState::char_compare(&'a', &'a'), true);
        assert_eq!(DayState::char_compare(&'\0', &'\0'), true);
        assert_eq!(DayState::char_compare(&'a', &'b'), false);
        assert_eq!(DayState::char_compare(&'z', &'x'), false);
    }
}
