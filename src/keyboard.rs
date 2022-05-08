use crate::state::{DayState, Result};
use colored::*;
use std::{fmt, iter};

#[derive(Debug, PartialEq)]
enum KeyState {
    Touched(Result),
    Untouched,
}

#[derive(Debug)]
pub struct Key {
    data: char,
    used: KeyState,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.used {
            KeyState::Touched(Result::Wrong) => {
                write!(f, "{}", self.data.to_string().truecolor(105, 105, 105))?
            }
            KeyState::Touched(Result::Correct) => {
                write!(f, "{}", self.data.to_string().truecolor(51, 255, 51))?
            }
            KeyState::Touched(Result::Partial) => {
                write!(f, "{}", self.data.to_string().truecolor(255, 255, 51))?
            }
            KeyState::Untouched => write!(f, "{}", self.data)?,
        };
        Ok(())
    }
}

pub struct Keyboard {
    pub keys: Vec<Vec<Key>>,
}

impl Keyboard {
    pub fn init(&mut self) {
        self.keys = vec!["qwertyuiop", "asdfghjkl", "zxcvbnm"]
            .iter()
            .map(|x| {
                x.chars()
                    .map(|y| Key {
                        data: y,
                        used: KeyState::Untouched,
                    })
                    .collect()
            })
            .collect()
    }

    pub fn set_key_with_guess(&mut self, state: &DayState, guess: &String) {
        'next: for (attempt, guess_char) in state.stat.attempts.last().unwrap().slots.iter().zip(
            guess
                .chars()
                .collect::<Vec<_>>()
                .iter()
                .chain(iter::repeat(&'\0')),
        ) {
            for row in self.keys.iter_mut() {
                for mut key in row.iter_mut() {
                    if key.data == *guess_char {
                        // found the character
                        key.used = match attempt {
                            Result::Wrong => KeyState::Touched(Result::Wrong),
                            Result::Correct => KeyState::Touched(Result::Correct),
                            Result::Partial => match key.used {
                                KeyState::Touched(Result::Correct) => {
                                    KeyState::Touched(Result::Correct)
                                }
                                _ => KeyState::Touched(Result::Partial),
                            },
                        };
                        continue 'next;
                    }
                }
            }
        }
    }
}

impl fmt::Display for Keyboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        let mut r = 0;
        for row in &self.keys {
            // spaces around
            for _ in 0..(2 * r - 1) {
                write!(f, " ")?;
            }
            for ch in row {
                write!(f, "{} ", ch)?;
            }
            write!(f, "\n")?;
            r += 1;
        }
        write!(f, "\n")?;
        Ok(())
    }
}
