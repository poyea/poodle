struct Key {
    data: char,
    used: bool,
}

struct Keyboard {
    keys: Vec<Vec<Key>>,
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
}
