use bevy_ecs::{
    schedule::Schedule,
    system::Resource,
    world::World,
};
use pixels::{Pixels, SurfaceTexture};
use std::{time::Instant, ops::{Deref, DerefMut}};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

pub use std::time::Duration;
pub use winit::event::VirtualKeyCode as Key;
pub use glam;

pub mod geometry;
pub mod canvas;
pub mod sprite;
pub mod utils;

mod line;

pub use canvas::Canvas;

pub struct PandaOptions<'a> {
    pub title: &'a str,
    pub width: u32,
    pub height: u32,
    pub scale: u32,
}

impl Default for PandaOptions<'_> {
    fn default() -> Self {
        Self {
            title: "panda!",
            width: 300,
            height: 200,
            scale: 3,
        }
    }
}

#[derive(Resource)]
pub struct Input(pub WinitInputHelper);

#[derive(Resource)]
pub struct Time(pub Duration);

pub struct Panda {
    pub world: World,
    event_loop: EventLoop<()>,
    window: Window,
}

impl Panda {
    pub fn new(options: PandaOptions) -> Self {
        let event_loop = EventLoop::new();

        let window = {
            let size = LogicalSize::new(
                options.width * options.scale,
                options.height * options.scale,
            );
            WindowBuilder::new()
                .with_title(options.title)
                .with_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let input = WinitInputHelper::new();

        let pixels = {
            let size = window.inner_size();
            let surface = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(options.width, options.height, surface).unwrap()
        };


        let mut world = World::new();
        world.insert_resource(Input(input));
        world.insert_resource(Canvas(pixels));
        world.insert_resource(Time(Duration::default()));

        Self {
            event_loop,
            window,
            world,
        }
    }

    pub fn run(mut self, mut schedule: Schedule) {
        let mut old = Instant::now();
        self.event_loop.run(move |event, _, control_flow| {
            if self.world.resource_mut::<Input>().0.update(&event) {
                let now = Instant::now();
                self.world.resource_mut::<Time>().0 = now.duration_since(old);
                old = now;

                schedule.run(&mut self.world);
                self.window.request_redraw();
            }

            if self.world.resource_mut::<Input>().0.close_requested() {
                *control_flow = ControlFlow::Exit;
            }

            if let Some(size) = self.world.resource_mut::<Input>().0.window_resized() {
                if let Err(why) = self.world.resource_mut::<Canvas>().0.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                    eprintln!("{}", why);
                }
            }

            if let Event::RedrawRequested(_) = event {
                if let Err(why) = self.world.resource_mut::<Canvas>().0.render() {
                    *control_flow = ControlFlow::Exit;
                    eprintln!("{}", why);
                }
            }
        })
    }
}

impl Deref for Input {
    type Target = WinitInputHelper;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Time {
    type Target = Duration;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Time {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

