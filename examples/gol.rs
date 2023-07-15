use bevy_ecs::{schedule::Schedule, system::{Resource, ResMut, Res, Local}, world::World};
use panda_bevy::{Panda, PandaOptions, Canvas, canvas::Color, Time, Input, Key, utils::default};
use rand::random;

const GRID_WIDTH: usize = 200;
const GRID_HEIGHT: usize = 200;

const CELL_SIZE: i32 = 2;
const TIME_STEP: f64 = 1.0 / 30.0;

fn main() {
    let mut panda = Panda::new(PandaOptions {
        title: "gol!",
        width: GRID_WIDTH as u32 * CELL_SIZE as u32,
        height: GRID_HEIGHT as u32 * CELL_SIZE as u32,
        ..default()
    });

    setup(&mut panda.world);

    let mut schedule = Schedule::new();
    schedule.add_systems((draw_grid, step_board, randomize_board));
    panda.run(schedule);
}

#[derive(Resource)]
struct Cells([bool; GRID_WIDTH * GRID_HEIGHT]);

impl Cells {
    fn get(&self, x: i32, y: i32) -> Option<&bool> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        self.0.get(y * GRID_WIDTH + x)
    }

    fn count_neighbors(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 { continue };
                if *self.get(x + i, y + j).unwrap_or(&false) {
                    count += 1;
                }
            }
        }

        count
    }

    fn step(&mut self) {
        let mut delta: Vec<(usize, bool)> = Vec::new();
        for (i, _) in self.0.iter().enumerate() {
            let x = (i % GRID_WIDTH) as i32;
            let y = (i / GRID_WIDTH) as i32;

            let neighbors = self.count_neighbors(x, y);
            if neighbors < 2 || neighbors > 3 { delta.push((i, false)) };
            if neighbors == 3 { delta.push((i, true)) };
        }

        for (i, state) in delta {
            if let Some(cell) = self.0.get_mut(i) { *cell = state };
        }
    }

    fn randomize(&mut self) {
        for cell in self.0.iter_mut() {
            *cell = random();
        }
    }
}

fn setup(world: &mut World) {
    let mut cells = Cells([false; GRID_WIDTH * GRID_HEIGHT]);
    cells.randomize();
    world.insert_resource(cells);
}

#[derive(Default)]
struct Timer(f64);
fn step_board(mut board: ResMut<Cells>, mut timer: Local<Timer>, time: Res<Time>) {
    timer.0 += time.as_secs_f64(); 
    if timer.0 > TIME_STEP {
        timer.0 = 0.0;
        board.step();
    }
}

fn randomize_board(mut board: ResMut<Cells>, input: Res<Input>) {
    if input.key_pressed(Key::R) {
        board.randomize();
    }
}

fn draw_grid(board: Res<Cells>, mut canvas: ResMut<Canvas>) {
    canvas.clear(Color::BLACK.pixel());
    for (i, &cell) in board.0.iter().enumerate() {
        if cell {
            let x = i % GRID_WIDTH;
            let y = i / GRID_WIDTH;
            canvas.draw_rectangle(
                x as i32 * CELL_SIZE,
                y as i32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                Color::WHITE.pixel()
             );
        }
    }
}

