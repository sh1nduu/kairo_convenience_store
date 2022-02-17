#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn area(&self) -> i64 {
        self.x * self.y
    }

    pub fn from1d(&self, n: i64) -> Self {
        let y = n / self.x;
        let x = n - (y * self.x);
        Self::new(x, y)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
