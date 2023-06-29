* vec of struct structure

```rust
struct World {
    entities: Vec<Entity>
}

impl World {
    fn iter<T>() -> Vec<T> {
        todo!();
    }
}

struct Entity {
    position: Option<Position>,
    velocity: Option<Velocity>,
    sprite: Option<Sprite>
}

fn apply_velocity(world: &mut World, delta: f32) {
    for (position, velocity) in world.iter::<(Position, Velocity)>() {
        position.0 += veloicty.0 * delta;
    }
}

```

* dream ecs *v*

```rust
struct Position(Vec2);
struct Velocity(Vec2);
struct Sprite(Drawable);

fn apply_velocity(world: &mut World, delta: f32) {
    for (position, velocity) in world.iter::<(Position, Velocity)>() {
        position.0 += veloicty.0 * delta;
    }
}

fn draw(world: &World, frame: &mut Frame) {
    for sprite in world.iter::<Drawable>() {
        frame.draw_sprite(&sprite);
    }
}

fn main() {
    let mut world = World::new();

    let elephant = world.spawn((
        Position(10.0, 20.0),
        Velocity(20.0, 0.0),
    ));

    loop {
        apply_velocity(&mut world, delta);
        draw(&world, &mut frame);
    }
}
```

* but how to implement :< 

```rust
struct World {
    entity_count: usize,
    entity_components: Vec<ComponentStorage>
};



impl World {
    fn new() -> Self {}
    fn iter<T>() {}
    fn spawn<T>(components: T) {}
}
```

