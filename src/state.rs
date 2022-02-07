use chrono::prelude::*;
use std::cmp::PartialEq;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Result {
    Wrong = -1,
    Correct = 1,
    Partial = 0,
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let block = match self {
            Result::Wrong => '⬛',
            Result::Correct => '🟩',
            Result::Partial => '🟨',
        };
        write!(f, "{}", block);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Attempt {
    slots: Vec<Result>,
}

impl fmt::Display for Attempt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for slot in &self.slots {
            write!(f, "{}", slot);
        }
        write!(f, "\n");
        Ok(())
    }
}

#[derive(Debug)]
pub struct Stat {
    attempts: Vec<Attempt>,
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
    stat: Stat,
    riddle: String,
    pub date: String,
    pub remaining: i64,
    crushed: bool,
}

impl DayState {
    pub fn get_today() -> String {
        format!("{}", Local::now().format("%b %d, %Y"))
    }

    pub fn new(riddle: String) -> DayState {
        DayState {
            crushed: false,
            riddle: riddle,
            date: DayState::get_today(),
            remaining: 6,
            stat: Stat::start(),
        }
    }

    fn validate(_guess: &String, _actual: &String) -> Attempt {
        let mut res = Vec::new();
        let guess = _guess.chars().collect::<Vec<_>>();
        let actual = _actual.chars().collect::<Vec<_>>();
        let mut same_pos = false;
        let n = _actual.len();
        'put: for i in 0..n {
            let mut parital = false;
            for j in 0..n {
                if guess[i] == actual[j] {
                    same_pos |= (i == j);
                    if same_pos {
                        res.push(Result::Correct);
                        continue 'put;
                    } else {
                        parital = true;
                    }
                }
            }
            if parital {
                res.push(Result::Partial);
            } else {
                res.push(Result::Wrong);
            }
        }
        Attempt { slots: res }
    }

    pub fn input_allowed(guess: &String, allowed: &Vec<String>) -> bool {
        if let Ok(_) = allowed.binary_search_by(|p| p.cmp(&guess)) {
            true
        } else {
            false
        }
    }

    pub fn input_hygiene(guess: &String) -> bool {
        return guess.len() == 5 && guess.chars().all(char::is_alphabetic);
    }

    pub fn guess(&mut self, word: String) -> String {
        self.remaining -= 1;
        self.stat
            .attempts
            .push(DayState::validate(&word, &self.riddle));
        if self
            .stat
            .attempts
            .last()
            .unwrap()
            .slots
            .iter()
            .all(|res| res == &Result::Correct)
        {
            self.crushed = true;
        }
        format!("{}", self.stat.attempts.last().unwrap())
    }

    pub fn finished(&self) -> bool {
        return self.crushed;
    }
}

impl fmt::Display for DayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n<>==========<>\n");
        write!(f, "Poodle {} {}/6\n", self.date, 6 - self.remaining);
        for attempt in &self.stat.attempts {
            write!(f, "{}", attempt);
        }
        write!(f, "<>==========<>\n");
        Ok(())
    }
}
