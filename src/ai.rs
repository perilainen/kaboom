use rand::thread_rng;
use rand::Rng;
use specs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{Enemy, Velocity};

// const ENEMY_MOVEMENT_SPEED: i32 = 3;

pub struct AI;

impl<'a> System<'a> for AI {
    type SystemData = (ReadStorage<'a, Enemy>, WriteStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        let mut rng = thread_rng();
        for (_, vel) in (&data.0, &mut data.1).join() {
            if rng.gen_range(0..10) == 0 {
                vel.speed = vel.mo_speed;
                vel.direction = match rng.gen_range(0..4) {
                    0 => crate::components::Direction::Up,
                    1 => crate::components::Direction::Down,
                    2 => crate::components::Direction::Left,
                    3 => crate::components::Direction::Right,
                    _ => unreachable!(),
                }
            }
        }
    }
}
// add code here
