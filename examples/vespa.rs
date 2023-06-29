use bevy_ecs::{
    prelude::Component,
    schedule::Schedule,
    system::{Query, Res, ResMut},
    world::World, query::With,
};
use panda_bevy::{
    geometry::Vec2, render::Color, sprite::Sprite, Canvas, Input, Key, Panda, PandaOptions, Time,
};

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

fn main() {
    let mut panda = Panda::new(PandaOptions {
        title: "vespa!",
        width: WIDTH,
        height: HEIGHT,
        scale: 4,
    });

    setup(&mut panda.world);

    let mut schedule = Schedule::new();
    schedule.add_systems((
        apply_velocity,
        apply_gravity,
        ground_collision,
        player_control,
        draw_sprites,
    ));
    panda.run(schedule);
}

#[derive(Component)]
struct Position(Vec2);
#[derive(Component)]
struct Velocity(Vec2);
#[derive(Component)]
struct Drawable(Sprite);

#[derive(Component)]
struct Player;

fn setup(world: &mut World) {
    world.spawn((
        Player,
        Position(Vec2::new(30.0, 30.0)),
        Velocity(Vec2::new(0.0, 0.0)),
        Drawable(Sprite::new("./assets/vespa.png")),
    ));
}

fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0 * time.0.as_secs_f32();
    }
}

const GRAVITY: Vec2 = Vec2::new(0.0, 150.0);
fn apply_gravity(mut query: Query<&mut Velocity>, time: Res<Time>) {
    for mut velocity in &mut query {
        velocity.0 += GRAVITY * time.0.as_secs_f32();
    }
}

fn ground_collision(mut query: Query<(&mut Position, &mut Velocity, &Drawable)>) {
    for (mut position, mut velocity, sprite) in &mut query {
        if position.0.y > (HEIGHT - sprite.0.height) as f32 {
            position.0.y = (HEIGHT - sprite.0.height) as f32;
            velocity.0.y = 0.0;
        }
    }
}

const JUMP: Vec2 = Vec2::new(0.0, -70.0);
fn player_control(mut query: Query<&mut Velocity, With<Player>>, input: Res<Input>) {
    for mut velocity in &mut query {
        if input.0.key_pressed(Key::Space) {
            velocity.0 = JUMP;
        }
    }
}

const SKY: Color = Color {
    r: 127,
    g: 207,
    b: 250,
    a: 255,
};

fn draw_sprites(query: Query<(&Drawable, &Position)>, mut canvas: ResMut<Canvas>) {
    canvas.clear(SKY.pixel());
    for (drawable, position) in &query {
        drawable.0.draw(&mut canvas, &position.0);
    }
}
