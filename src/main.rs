use rand::Rng;
mod ai;
mod animator;
mod components;
mod keyboard;
mod physics;
mod renderer;

use rand::thread_rng;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use specs::{Builder, DispatcherBuilder, World, WorldExt};
use std::time::Duration;

use crate::components::*;

pub enum MovementCommand {
    Stop,
    Move(Direction),
}

fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

fn character_animation_frames(
    spritesheet: usize,
    top_left_frame: Rect,
    direction: Direction,
) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height,
            ),
        })
    }
    frames
}

fn initialize_player(world: &mut World, player_spritesheet: usize) {
    let player_top_left_frame = Rect::new(0, 0, 26, 36);
    let player_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Up,
        ),
        down_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Down,
        ),
        left_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Left,
        ),
        right_frames: character_animation_frames(
            player_spritesheet,
            player_top_left_frame,
            Direction::Right,
        ),
    };
    world
        .create_entity()
        .with(KeyboardControlled)
        .with(Position(Point::new(0, 0)))
        .with(Velocity {
            speed: 0,
            direction: Direction::Right,
            mo_speed: 6,
        })
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();
}

fn initialize_enemy(world: &mut World, enemy_spritesheet: usize, position: Point) {
    let enemy_top_left_frame = Rect::new(0, 0, 32, 36);

    let enemy_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(
            enemy_spritesheet,
            enemy_top_left_frame,
            Direction::Up,
        ),
        down_frames: character_animation_frames(
            enemy_spritesheet,
            enemy_top_left_frame,
            Direction::Down,
        ),
        left_frames: character_animation_frames(
            enemy_spritesheet,
            enemy_top_left_frame,
            Direction::Left,
        ),
        right_frames: character_animation_frames(
            enemy_spritesheet,
            enemy_top_left_frame,
            Direction::Right,
        ),
    };
    // let mut rng = thread_rng();
    let speed = thread_rng().gen_range(1..5);

    world
        .create_entity()
        .with(Enemy)
        .with(Position(position))
        .with(Velocity {
            speed: 0,
            direction: Direction::Right,
            mo_speed: speed,
        })
        .with(enemy_animation.right_frames[0].clone())
        .with(enemy_animation)
        .build();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video().unwrap();

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not init video");

    let mut canvas = window.into_canvas().build().expect("could not make canvas");
    let texture_creator = canvas.texture_creator();
    let movement_command: Option<MovementCommand> = None;
    let textures = [
        texture_creator.load_texture("assets/bardo.png")?,
        texture_creator.load_texture("assets/reaper.png")?,
    ];
    let player_spritesheet = 0;
    let enemy_spritesheet = 1;
    let mut world = World::new();
    world.insert(movement_command);
    let mut dispatcher = DispatcherBuilder::new()
        .with(ai::AI, "AI", &[])
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(physics::Physics, "Physics", &["Keyboard", "AI"])
        .with(animator::Animator, "Animator", &["Keyboard", "AI"])
        .build();

    dispatcher.setup(&mut world);
    initialize_player(&mut world, player_spritesheet);
    // initialize_enemy(&mut world, enemy_spritesheet, Point::new(-150, -150));
    // initialize_enemy(&mut world, enemy_spritesheet, Point::new(150, -190));
    // initialize_enemy(&mut world, enemy_spritesheet, Point::new(-150, 170));
    // initialize_enemy(&mut world, enemy_spritesheet, Point::new(-150, 170));
    // initialize_enemy(&mut world, enemy_spritesheet, Point::new(-150, 170));
    // initialize_enemy(&mut world, enemy_spritesheet, Point::new(-150, 170));
    // initialize_enemy(&mut world, enemy_spritesheet, Point::new(-150, 170));
    // world
    //     .create_entity()
    //     .with(KeyboardControlled)
    //     .with(Position(Point::new(0, 0)))
    //     .with(Velocity {
    //         speed: 0,
    //     direction: Direction::Right,
    // })
    // .with(player_animation.right_frames[0].clone())
    // .with(player_animation)
    // .build();

    let mut event_pump = sdl_context.event_pump()?;
    // let mut i = 0;
    let i = 87;
    let mut ii = 0;
    'running: loop {
        ii += 1;
        if ii % 100 == 0 {
            let mut rng = thread_rng();
            let x = rng.gen_range(-400..400);
            let y = rng.gen_range(-300..300);

            initialize_enemy(&mut world, enemy_spritesheet, Point::new(x, y));
            ii = 0
        }
        let mut movement_command = None;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    movement_command = Some(MovementCommand::Stop);
                }

                _ => {}
            }
        }
        *world.write_resource() = movement_command;
        // i = (i + 1) % 255;
        // dispatcher.dispatch(&mut world);
        dispatcher.dispatch(&world);
        world.maintain();

        renderer::render(
            &mut canvas,
            Color::RGB(i, 64, 255 - i),
            &textures,
            world.system_data(),
        )?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
