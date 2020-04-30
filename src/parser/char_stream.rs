pub struct CharStream {
    pub code: Vec<char>,
    pub index: usize
}

impl CharStream {
    pub fn peek (&mut self) -> char {
        self.code[self.index]
    }

    pub fn read (&mut self) -> char {
        let c = self.code[self.index];
        self.index += 1;
        c
    }
}

pub fn new (code: String) -> CharStream {
    CharStream {
        code: code.chars().collect(),
        index: 0
    }
}
