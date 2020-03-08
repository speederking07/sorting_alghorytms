use std::ops::Add;

pub struct Stat {
    pub comp: i32,
    pub swap: i32,
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            comp: 0,
            swap: 0,
        }
    }

    pub fn swap(&mut self) {
        self.swap += 1;
    }

    pub fn comp(&mut self) {
        self.comp += 1;
    }
}

impl Add for Stat {
    type Output = Stat;

    fn add(self, other: Stat) -> Stat {
        Self { comp: self.comp + other.comp, swap: self.swap + other.swap }
    }
}

pub fn faaa() -> i32 {
    3
}
