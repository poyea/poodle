use crate::state::{DayState, Result};
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
        let wrong_color: &'static str = "\x1b[1;30m";
        let correct_color: &'static str = "\x1b[1;32m";
        let partial_color: &'static str = "\x1b[1;33m";
        let no_color: &'static str = "\x1b[0m";
        let wrapped_color = |color_str: &str, data: char| -> String {
            format!("{}{}{}", color_str, data, no_color)
        };
        match &self.used {
            KeyState::Touched(Result::Wrong) => {
                write!(f, "{}", wrapped_color(wrong_color, self.data))?
            }
            KeyState::Touched(Result::Correct) => {
                write!(f, "{}", wrapped_color(correct_color, self.data))?
            }
            KeyState::Touched(Result::Partial) => {
                write!(f, "{}", wrapped_color(partial_color, self.data))?
            }
            KeyState::Untouched => write!(f, "{}", wrapped_color(no_color, self.data))?,
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
        'next: for res in state.stat.attempts.last().unwrap().slots.iter().zip(
            guess
                .chars()
                .collect::<Vec<_>>()
                .iter()
                .chain(iter::repeat(&'\0')),
        ) {
            for row in self.keys.iter_mut() {
                for mut ch in row.iter_mut() {
                    if ch.data == *res.1 {
                        ch.used = match res.0 {
                            Result::Wrong => KeyState::Touched(Result::Wrong),
                            Result::Correct => KeyState::Touched(Result::Correct),
                            Result::Partial => match ch.used {
                                KeyState::Touched(Result::Wrong)
                                | KeyState::Untouched
                                | KeyState::Touched(Result::Partial) => {
                                    KeyState::Touched(Result::Partial)
                                }
                                KeyState::Touched(Result::Correct) => {
                                    KeyState::Touched(Result::Correct)
                                }
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
