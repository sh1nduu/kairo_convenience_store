use rand::distributions::{Distribution, Uniform};

use crate::entity::{Object, Store};
use crate::solver::{Answer, Solve};

pub struct RandomSolver {
    answer: Option<Answer>,
}

impl RandomSolver {
    pub fn new() -> Self {
        Self { answer: None }
    }
}

impl Solve for RandomSolver {
    fn solve(&mut self, store: &Store, objects: &[Object]) {
        let mut rng = rand::thread_rng();

        let mut points = vec![];
        let die = Uniform::<usize>::from(0..store.size.area() - 1);
        let new_objects = objects
            .iter()
            .map(|object| {
                let mut new_object = object.to_owned();
                let mut point;
                loop {
                    let num = die.sample(&mut rng);
                    point = store.size.from1d(num);
                    if !points.contains(&point) {
                        break;
                    }
                }
                new_object.offset = Some(point.clone());
                points.push(point);
                new_object
            })
            .collect();

        let answer = Answer::new(store.to_owned(), new_objects);
        self.answer = Some(answer);
    }

    fn answer<'a>(&'a self) -> &'a Option<Answer> {
        &self.answer
    }
}
