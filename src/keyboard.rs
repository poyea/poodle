#[allow(dead_code)]
use std::fmt;

pub struct Key {
    data: char,
    used: bool,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.used {
            true => write!(f, "\x1b[37m{}\x1b[0m", self.data)?,
            false => write!(f, "\x1b[93m{}\x1b[0m", self.data)?,
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
                        used: false,
                    })
                    .collect()
            })
            .collect()
    }

    pub fn set(&mut self, guess: &String) {
        for guess_ch in guess.chars() {
            for row in self.keys.iter_mut() {
                for mut ch in row.iter_mut() {
                    if ch.data == guess_ch {
                        ch.used = true;
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
