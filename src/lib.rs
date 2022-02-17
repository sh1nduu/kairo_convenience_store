pub mod entity {
    mod game;
    mod object;
    mod point;
    mod store;
    pub use game::Game;
    pub use object::{Object, ObjectKind, Shape};
    pub use point::Point;
    pub use store::Store;
}
