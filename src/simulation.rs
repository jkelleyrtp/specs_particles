use specs::prelude::*;
use crate::components::{Position, Velocity, vintegrate::SysA};


pub struct ElectronSim<'a, 'b> {
    pub world: specs::World,
    pub dispatcher: specs::Dispatcher<'a,'b>,
}

impl <'a,'b> ElectronSim <'a,'b>{
    pub fn new() -> Self {

        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();

        let mut dispatcher = DispatcherBuilder::new().with(SysA{}, "sys_a", &[]).build();
       
        Self {
            world: world,
            dispatcher: dispatcher
        }
    }
}