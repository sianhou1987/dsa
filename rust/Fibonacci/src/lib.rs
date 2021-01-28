pub struct FibClass {
    f: u64,
    g: u64,
}

impl FibClass {
    pub fn new(n: u64) -> Self {
        let mut f = FibClass { f: 1, g: 0 };
        while f.g < n {
            f.next();
        }
        f
    }
    pub fn get(&self) -> u64 {
        self.g
    }
    pub fn next(&mut self) -> u64 {
        self.g = self.g + self.f;
        self.f = self.g - self.f;
        self.g
    }
    pub fn prev(&mut self) -> u64 {
        self.f = self.g - self.f;
        self.g = self.g - self.f;
        self.g
    }
}
