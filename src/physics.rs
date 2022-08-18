use specs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{Position, Velocity};

pub struct Physics;
impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        use crate::components::Direction::*;
        for (pos, vel) in (&mut data.0, &data.1).join() {
            match vel.direction {
                Left => {
                    if pos.0.x > -387 {
                        pos.0 = pos.0.offset(-vel.speed, 0)
                    };
                }
                Right => {
                    if pos.0.x < 387 {
                        pos.0 = pos.0.offset(vel.speed, 0);
                    }
                }
                Up => {
                    if pos.0.y > -282 {
                        pos.0 = pos.0.offset(0, -vel.speed);
                    }
                }
                Down => {
                    if pos.0.y < 282 {
                        pos.0 = pos.0.offset(0, vel.speed);
                    }
                }
            }
        }
    }
}
