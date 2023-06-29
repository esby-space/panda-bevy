# panda_bevy!

`panda_bevy` is a rust crate for easily drawing things to the screen using [`bevy_ecs`'s data-driven
patterns](https://docs.rs/bevy_ecs/latest/bevy_ecs/). i find this combination of `ecs` and a pixel
buffer easy to create 2d simulations and games!

(i mean, you probably should  just use [Bevy](https://bevyengine.org/), but this is a little simpler).

## getting started

```rust
fn main() {
    let mut panda = Panda::new(PandaOptions {
        title: "panda!",
        width: 480,
        height: 360,
        ..default()
    });

    setup(&mut panda.world);

    let mut schedule = Schedule::new();
    schedule.add_system(update);
    panda.run(schedule);
}

fn setup(world: &mut World) {
    // code to run once before game loop
}

fn update() {
    // code to run inside game loop
    // use just like a bevy system
}
```

`panda_bevy` provides 3 resources to help you out!

1. `Input` - get input events like key pressed or mouse movement
2. `Time` - get time between frames
3. `Canvas` - get the pixel buffer to render things to the screen!

## examples

check out the `examples` to learn how to use!

## libraries used

panda_bevy is built on top of
- [pixels](https://github.com/parasyte/pixels) to provide a pixel frame buffer rendered with `wgpu`
- [bevy_ecs](https://github.com/bevyengine/bevy) for `ecs` design patterns
- [glam](https://github.com/bitshifter/glam-rs) for 2d vector math and geometry
- [winit](https://github.com/rust-windowing/winit) for windowing and events
- [winit_input_helper](https://github.com/rukai/winit_input_helper) to process winit events
- [image](https://github.com/image-rs/image) to process `png` images

## older panda projects

- `[panda-canvas](https://github.com/esby-space/panda-canvas)` - uses `HTML Canvas` for rendering
- `[panda-pixi](https://github.com/esby-space/pixi-panda)` - uses `Pixi.js` for rendering

