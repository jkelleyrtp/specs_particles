// Logs the agents out to the commandline
use specs::{Component, VecStorage, System, WriteStorage, ReadStorage, Join, ParJoin};
use super::{Position, Velocity};


pub struct LogAgent;

impl<'a> System<'a> for LogAgent {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}
