use bevy_ecs::{
    prelude::Component,
    schedule::Schedule,
    system::{Query, ResMut},
    world::World,
};
use panda_bevy::{geometry::Circle, geometry::Vec2, render::Color, Canvas, Panda, PandaOptions};
use rand::random;

const WIDTH: u32 = 480;
const HEIGHT: u32 = 360;

const NUM_BOIDS: u32 = 10;
const RADIUS: f32 = 5.0;

fn main() {
    let mut panda = Panda::new(PandaOptions {
        title: "boids!",
        width: WIDTH,
        height: HEIGHT,
        scale: 2,
    });

    spawn_boids(&mut panda.world);

    let mut schedule = Schedule::new();
    schedule.add_system(draw_boids);
    panda.run(schedule);
}

#[derive(Component)]
struct Boid {
    circle: Circle,
    color: Color,
}

#[derive(Component)]
struct Velocity(Vec2);

fn spawn_boids(world: &mut World) {
    for _ in 0..NUM_BOIDS {
        world.spawn((
            Boid {
                circle: Circle::new(
                    random::<f32>() * WIDTH as f32,
                    random::<f32>() * HEIGHT as f32,
                    RADIUS,
                ),
                color: Color::new(
                    random::<u8>(),
                    random::<u8>(),
                    random::<u8>(),
                    random::<u8>(),
                 ),
            },
            Velocity(Vec2::new(0.0, 0.0)),
        ));
    }
}

fn draw_boids(query: Query<&Boid>, mut canvas: ResMut<Canvas>) {
    for boid in &query {
        boid.circle.draw(&mut canvas, boid.color.pixel());
    }
}
