use std::time::Duration;

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

            for (ent, pos) in all {
                // ent.delete();
                if pos.0.x + 15 >= playerx - 5
                    && pos.0.x - 15 <= playerx + 5
                    && pos.0.y + 40 >= playery - 11
                    && pos.0.y - 30 <= playery + 11
                {
                    println!(
                        "posx:{},posy:{},playerx:{}, playery:{}",
                        pos.0.x, pos.0.y, playerx, playery
                    );
                    let _ = data.0.delete(ent);
                    // std::thread::sleep(Duration::from_secs(1));

                    healt.health -= 1;
                }
            }
        }
    }
}
