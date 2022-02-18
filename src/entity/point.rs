#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn area(&self) -> usize {
        let area = self.x * self.y;
        area.try_into().unwrap()
    }

    pub fn from1d(&self, n: usize) -> Self {
        let n: i64 = n.try_into().unwrap();
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
