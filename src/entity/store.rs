use crate::entity::object::*;
use crate::entity::point::Point;

type Table = Vec<Vec<i64>>;

#[derive(Debug, Clone)]
pub struct Store {
    pub size: Point,
    pub table: Table,
    width: usize,
    height: usize,
}

impl Store {
    pub fn new(width: usize, height: usize) -> Self {
        let x_i64 = width.try_into().unwrap();
        let y_i64 = height.try_into().unwrap();
        Self {
            size: Point::new(x_i64, y_i64),
            table: vec![vec![0; width]; height],
            width,
            height,
        }
    }

    pub fn init_table(&mut self) {
        self.table = vec![vec![0; self.width]; self.height];
    }

    pub fn apply_appeal(&mut self, object: &Object) {
        if let Some(offset) = &object.offset {
            match object.shape {
                Shape::Square => self.apply_square(offset, object.appeal, object.appeal_length),
                Shape::Plus => self.apply_plus(offset, object.appeal, object.appeal_length),
                Shape::Cross => self.apply_cross(offset, object.appeal, object.appeal_length),
            }
        }
    }

    fn apply_square(&mut self, point: &Point, appeal: i64, length: i64) {
        for x in (point.x - length)..=(point.x + length) {
            for y in (point.y - length)..=(point.y + length) {
                if self.on_table(x, y) && !(point.x == x && point.y == y) {
                    self.add_appeal(x, y, appeal);
                }
            }
        }
    }

    fn apply_plus(&mut self, point: &Point, appeal: i64, length: i64) {
        for x in (point.x - length)..=(point.x + length) {
            if self.on_table(x, point.y) && x != point.x {
                self.add_appeal(x, point.y, appeal);
            }
        }
        for y in (point.y - length)..=(point.y + length) {
            if self.on_table(point.x, y) && y != point.y {
                self.add_appeal(point.x, y, appeal);
            }
        }
    }

    fn apply_cross(&mut self, point: &Point, appeal: i64, length: i64) {
        let mut add_if_on_table = |x, y| {
            if self.on_table(x, y) {
                self.add_appeal(x, y, appeal);
            }
        };
        for l in 1..=length {
            add_if_on_table(point.x + l, point.y + l);
            add_if_on_table(point.x + l, point.y - l);
            add_if_on_table(point.x - l, point.y + l);
            add_if_on_table(point.x - l, point.y - l);
        }
    }

    fn on_table(&self, x: i64, y: i64) -> bool {
        x >= 0 && x < self.size.x && y >= 0 && y < self.size.y
    }

    fn add_appeal(&mut self, x: i64, y: i64, appeal: i64) {
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        self.table[y][x] += appeal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_apply {
        ($fn:ident, w:$w:literal, h:$h:literal, x:$x:literal, y:$y:literal, appeal:$ap:literal, length:$len:literal, expect:$exp:expr) => {
            let mut store = Store::new($w, $h);
            store.$fn(&Point::new($x, $y), $ap, $len);
            assert_eq!(
                $exp, &store.table,
                "store {}x{}, x:{} y:{} appeal:{} lenght:{}",
                $w, $h, $x, $y, $ap, $len
            );
        };
    }

    #[test]
    fn apply_square() {
        test_apply!(apply_square, w:2, h:2, x:0, y:0, appeal:1, length:0, expect:&vec![
            vec![0, 0],
            vec![0, 0],
        ]);

        test_apply!(apply_square, w:3, h:3, x:1, y:1, appeal:1, length:1, expect:&vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1],
        ]);

        test_apply!(apply_square, w:3, h:3, x:2, y:2, appeal:1, length:1, expect:&vec![
            vec![0, 0, 0],
            vec![0, 1, 1],
            vec![0, 1, 0],
        ]);

        test_apply!(apply_square, w:5, h:5, x:1, y:1, appeal:1, length:2, expect:&vec![
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 1, 1, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0],
        ]);
    }

    #[test]
    fn apply_plus() {
        test_apply!(apply_plus, w:2, h:2, x:0, y:0, appeal:1, length:0, expect:&vec![
            vec![0, 0],
            vec![0, 0],
        ]);

        test_apply!(apply_plus, w:3, h:3, x:1, y:1, appeal:1, length:1, expect:&vec![
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0],
        ]);

        test_apply!(apply_plus, w:3, h:3, x:2, y:2, appeal:1, length:1, expect:&vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 0],
        ]);

        test_apply!(apply_plus, w:5, h:5, x:3, y:1, appeal:1, length:2, expect:&vec![
            vec![0, 0, 0, 1, 0],
            vec![0, 1, 1, 0, 1],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0],
        ]);
    }

    #[test]
    fn apply_cross() {
        test_apply!(apply_cross, w:2, h:2, x:0, y:0, appeal:1, length:0, expect:&vec![
            vec![0, 0],
            vec![0, 0],
        ]);

        test_apply!(apply_cross, w:3, h:3, x:1, y:1, appeal:1, length:1, expect:&vec![
            vec![1, 0, 1],
            vec![0, 0, 0],
            vec![1, 0, 1],
        ]);

        test_apply!(apply_cross, w:3, h:3, x:0, y:0, appeal:1, length:2, expect:&vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 1],
        ]);

        test_apply!(apply_cross, w:3, h:3, x:0, y:2, appeal:1, length:1, expect:&vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 0],
        ]);

        test_apply!(apply_cross, w:3, h:3, x:2, y:1, appeal:1, length:1, expect:&vec![
            vec![0, 1, 0],
            vec![0, 0, 0],
            vec![0, 1, 0],
        ]);

        test_apply!(apply_cross, w:3, h:4, x:2, y:1, appeal:1, length:2, expect:&vec![
            vec![0, 1, 0],
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![1, 0, 0],
        ]);

        test_apply!(apply_cross, w:5, h:5, x:3, y:3, appeal:1, length:2, expect:&vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 1],
        ]);
    }
}
