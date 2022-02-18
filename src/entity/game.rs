use crate::entity::{Object, Store};
use crate::solver::{Solve, Solver};

pub struct Game {
    store: Store,
    objects: Vec<Object>,
    solvers: Vec<Solver>,
}

impl Game {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            objects: vec![],
            solvers: vec![],
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn add_solver(&mut self, solver: Solver) {
        self.solvers.push(solver);
    }

    pub fn layout(&mut self) {
        for solver in &mut self.solvers {
            solver.solve(&self.store, &self.objects);
        }
    }

    pub fn print(&mut self) {
        println!("Store size is {}", self.store.size);
        println!("Objects are");
        for object in &self.objects {
            println!("{:?}", object);
        }

        for solver in &self.solvers {
            println!("Answer by {}", solver.name());
            self.print_answer(solver);
        }
    }

    fn print_answer(&self, solver: &Solver) {
        if let Some(answer) = solver.answer() {
            println!("Location");

            for i in 0..self.store.size.area() {
                let i_i64: i64 = i.try_into().unwrap();

                if i > 0 && i_i64 % self.store.size.x == 0 {
                    print!("\n");
                }
                let point = self.store.size.from1d(i);
                if let Some(object) = answer.find_object(&point) {
                    print!("{:>5},", object.id);
                } else {
                    print!("{:>5},", "#");
                }
            }
            print!("\n");

            println!("Appeals");

            for row in &answer.store.table {
                for col in row {
                    print!("{:>5b},", col);
                }
                print!("\n");
            }
        } else {
            println!("no answer");
        }
    }
}
