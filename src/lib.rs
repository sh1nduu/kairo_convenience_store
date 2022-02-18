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

pub mod solver {
    mod random_solver;

    use crate::entity::{Object, Point, Store};

    use self::random_solver::RandomSolver;

    pub trait Solve {
        fn solve(&mut self, store: &Store, objects: &[Object]);
        fn answer<'a>(&'a self) -> &'a Option<Answer>;
    }
    pub enum Solver {
        Random(String, random_solver::RandomSolver),
    }

    impl Solver {
        pub fn random(name: &str) -> Self {
            Self::Random(name.to_owned(), RandomSolver::new())
        }

        pub fn name<'a>(&'a self) -> &'a str {
            match self {
                Self::Random(name, _) => &name,
            }
        }
    }

    impl Solve for Solver {
        fn solve(&mut self, store: &Store, objects: &[Object]) {
            match self {
                Self::Random(_, solver) => solver.solve(store, objects),
            }
        }

        fn answer<'a>(&'a self) -> &'a Option<Answer> {
            match self {
                Self::Random(_, solver) => solver.answer(),
            }
        }
    }

    pub struct Answer {
        pub store: Store,
        pub objects: Vec<Object>,
    }

    impl Answer {
        pub fn new(mut store: Store, objects: Vec<Object>) -> Self {
            store.init_table();

            for object in &objects {
                store.apply_appeal(object);
            }

            Self { store, objects }
        }

        pub fn find_object<'a>(&'a self, point: &Point) -> Option<&'a Object> {
            for object in self.objects.iter() {
                if let Some(offset) = &object.offset {
                    if offset == point {
                        return Some(&object);
                    }
                }
            }
            None
        }
    }
}
