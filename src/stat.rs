use std::ops::Add;
use std::cmp::max;

pub struct Stat {
    pub comp: u64, //liczba porównań
    pub swap: u64, //liczba przestawień
    pub mem: u64 //zużyta pamięć
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            comp: 0,
            swap: 0,
            mem: 0
        }
    }

    //inkrementacja przestawień
    pub fn swap(&mut self) {
        self.swap += 1;
    }

    //inkrementacja prównań
    pub fn comp(&mut self) {
        self.comp += 1;
    }

    pub fn add_mem(&mut self, m:i32){
        self.mem += m as u64;
    }
}

impl Add for Stat {
    type Output = Stat;

    fn add(self, other: Stat) -> Stat {
        Self { comp: self.comp + other.comp, swap: self.swap + other.swap, mem: max(self.mem, other.mem)}
    }
}
