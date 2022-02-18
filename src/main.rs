use kairo_convenience_store::entity::*;
use kairo_convenience_store::solver::*;

fn main() {
    let store = Store::new(8, 8);
    let s1 = Object::new("s1-1", ObjectKind::Shelf, Shape::Square, 1, 1, 1, 1);
    let s2 = Object::new("p2-2", ObjectKind::Shelf, Shape::Plus, 2, 2, 1, 1);
    let e1 = Object::new("c2-4", ObjectKind::Equipment, Shape::Cross, 4, 2, 1, 1);
    let e2 = Object::new("s2-8", ObjectKind::Equipment, Shape::Square, 8, 2, 1, 1);

    let mut game = Game::new(store);
    game.add_object(s1);
    game.add_object(s2);
    game.add_object(e1);
    game.add_object(e2);

    game.add_solver(Solver::random("#1"));
    game.add_solver(Solver::random("#2"));
    game.add_solver(Solver::random("#3"));

    game.layout();
    game.print();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
