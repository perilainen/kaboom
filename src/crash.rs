use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::components::{Health, Position};

pub struct Crash;

impl<'a> System<'a> for Crash {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Health>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        for healt in (&mut data.2).join() {
            let mut all = (&*data.0, &data.1).join();
            let player = all.next().unwrap();
            let playerx = player.1 .0.x;
            let playery = player.1 .0.y;
            // println!("{:?}", healt.unwrap());

            for (_, pos) in all {
                if pos.0.x + 15 >= playerx - 5
                    && pos.0.x - 15 <= playerx + 5
                    && pos.0.y + 40 >= playery - 11
                    && pos.0.y - 30 <= playery + 11
                {
                    println!(
                        "posx:{},posy:{},playerx:{}, playery:{}",
                        pos.0.x, pos.0.y, playerx, playery
                    );
                    healt.health -= 1;
                }
            }
        }
    }
}
