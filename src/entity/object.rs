use crate::entity::point::Point;

#[derive(Debug, Clone)]
pub enum ObjectKind {
    Shelf,
    Equipment,
}

#[derive(Debug, Clone)]
pub enum Shape {
    Square,
    Plus,
    Cross,
}

#[derive(Debug, Clone)]
pub struct Object {
    pub id: String,
    pub kind: ObjectKind,
    pub shape: Shape,
    pub appeal: i64,
    pub appeal_length: i64,
    pub size: Point,
    pub offset: Option<Point>,
}

impl Object {
    pub fn new(
        id: &str,
        kind: ObjectKind,
        shape: Shape,
        appeal: i64,
        appeal_length: i64,
        width: i64,
        height: i64,
    ) -> Self {
        Self {
            id: id.to_owned(),
            kind,
            shape,
            appeal,
            appeal_length,
            size: Point::new(width, height),
            offset: None,
        }
    }
}
