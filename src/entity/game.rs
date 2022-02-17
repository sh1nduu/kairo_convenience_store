use rand::distributions::{Distribution, Uniform};

use crate::entity::object::Object;
use crate::entity::point::Point;
use crate::entity::store::Store;

pub struct Game {
    store: Store,
    objects: Vec<Object>,
}

impl Game {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn layout(&mut self) {
        let mut rng = rand::thread_rng();
        let die = Uniform::<usize>::from(0..3);
        for x in 0..self.store.size.x {
            for y in 0..self.store.size.y {
                if die.sample(&mut rng) != 0 {
                    continue;
                }
                if let Some(mut object) = self.choose_object() {
                    object.offset = Some(Point::new(x, y));
                }
            }
        }
    }

    pub fn print(&mut self) {
        println!("Store size is {}", self.store.size);
        println!("Objects are");
        for object in &self.objects {
            println!("{:?}", object);
        }

        println!("Location");

        let mut store = vec![];
        for i in 0..self.store.size.area() {
            if i > 0 && i % self.store.size.x == 0 {
                print!("\n");
            }
            let point = self.store.size.from1d(i);
            if let Some(object) = self.find_object(&point) {
                store.push(object.id.clone());
                print!("{:>5},", object.id);
            } else {
                store.push("".to_owned());
                print!("{:>5},", "#");
            }
        }
        print!("\n");

        println!("Appeals");
        for object in &self.objects {
            self.store.apply_appeal(&object);
        }

        for row in &self.store.table {
            for col in row {
                print!("{:>5b},", col);
            }
            print!("\n");
        }
    }

    fn choose_object<'a>(&'a mut self) -> Option<&'a mut Object> {
        if self.objects.len() == 0 {
            return None;
        }
        let mut rng = rand::thread_rng();
        let die = Uniform::<usize>::from(0..self.objects.len());
        Some(&mut self.objects[die.sample(&mut rng)])
    }

    fn find_object<'a>(&'a self, point: &Point) -> Option<&'a Object> {
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
