use bevy_ecs::{
    prelude::DetectChanges,
    schedule::{IntoSystemConfigs, Schedule},
    system::{Res, ResMut, Resource},
    world::World,
};
use panda_bevy::{canvas::Color, Canvas, Input, Key, Panda, PandaOptions};
use rand::{seq::SliceRandom, thread_rng};

const CELL_SIZE: i32 = 10;
const BOARD_HEIGHT: usize = 20;
const BOARD_WIDTH: usize = 10;

const VIEW_WINDOW: usize = 4;

const WIDTH: u32 = BOARD_WIDTH as u32 * CELL_SIZE as u32;
const HEIGHT: u32 = BOARD_HEIGHT as u32 * CELL_SIZE as u32;

fn main() {
    let mut panda = Panda::new(PandaOptions {
        title: "tetris!",
        width: WIDTH,
        height: HEIGHT,
        scale: 4,
    });

    setup(&mut panda.world);

    let mut scheulde = Schedule::new();
    scheulde.add_systems((clear, draw_board, draw_ghost).chain());
    scheulde.add_systems((move_x, turn_piece, hard_drop, extend_queue));
    panda.run(scheulde);
}

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Cell {
    fn from_num(num: i32) -> Self {
        match num {
            0 => Cell::I,
            1 => Cell::O,
            2 => Cell::T,
            3 => Cell::J,
            4 => Cell::L,
            5 => Cell::S,
            6 => Cell::Z,
            _ => unreachable!(),
        }
    }

    fn color(&self) -> Color {
        match self {
            Cell::Empty => Color::BLACK,
            Cell::I => Color::from(0x01ADEE),
            Cell::J => Color::from(0x1A74BB),
            Cell::L => Color::from(0xF6921E),
            Cell::O => Color::from(0xFFF200),
            Cell::S => Color::from(0x8BC540),
            Cell::T => Color::from(0x652D90),
            Cell::Z => Color::from(0xEC1A23),
        }
    }
}

struct Point(i32, i32);
struct Tetrimino {
    cell: Cell,
    points: [Point; 4],
}

impl Tetrimino {
    fn new(cell: Cell) -> Self {
        match cell {
            Cell::I => Self::create(cell, [(-1, 0), (0, 0), (1, 0), (2, 0)]),
            Cell::J => Self::create(cell, [(-1, -1), (-1, 0), (0, 0), (1, 0)]),
            Cell::L => Self::create(cell, [(-1, 0), (0, 0), (1, 0), (1, -1)]),
            Cell::O => Self::create(cell, [(0, 0), (1, 0), (1, 1), (0, 1)]),
            Cell::S => Self::create(cell, [(-1, 0), (0, 0), (0, -1), (1, -1)]),
            Cell::T => Self::create(cell, [(-1, 0), (0, 0), (1, 0), (0, -1)]),
            Cell::Z => Self::create(cell, [(-1, -1), (0, -1), (0, 0), (1, 0)]),
            Cell::Empty => panic!("cannot create empty shape!"),
        }
    }

    fn create(cell: Cell, points: [(i32, i32); 4]) -> Self {
        Self {
            cell,
            points: points.map(|(x, y)| Point(x, y)),
        }
    }

    fn turn_left(&mut self) {
        for point in &mut self.points {
            (point.0, point.1) = (point.1, point.0);
            point.1 *= -1;
        }
    }

    fn turn_right(&mut self) {
        for point in &mut self.points {
            (point.0, point.1) = (point.1, point.0);
            point.0 *= -1;
        }
    }
}

#[derive(Resource)]
struct Board([Cell; BOARD_WIDTH * BOARD_HEIGHT]);
impl Board {
    const EMPTY: Board = Board([Cell::Empty; BOARD_WIDTH * BOARD_HEIGHT]);

    fn get(&self, x: i32, y: i32) -> Option<&Cell> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        self.0.get(y * BOARD_WIDTH + x)
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        self.0.get_mut(y * BOARD_WIDTH + x)
    }

    fn check(&self, tetrimino: &Tetrimino, (center_x, center_y): (i32, i32)) -> bool {
        let mut cells = Vec::new();
        for Point(x, y) in &tetrimino.points {
            match self.get(center_x + x, center_y + y) {
                None => return false,
                Some(cell) => cells.push(cell),
            }
        }

        cells
            .iter()
            .all(|cell| if let Cell::Empty = cell { true } else { false })
    }

    fn where_insert(&self, tetrimino: &Tetrimino, center_x: i32) -> i32 {
        let mut center_y = 1; // ??
        while self.check(tetrimino, (center_x, center_y)) {
            center_y += 1;
        }
        center_y - 1
    }

    fn insert(&mut self, tetrimino: &Tetrimino, center_x: i32) {
        let center_y = self.where_insert(tetrimino, center_x);
        for Point(x, y) in &tetrimino.points {
            if let Some(cell) = self.get_mut(center_x + x, center_y + y) {
                *cell = tetrimino.cell;
            }
        }
    }
}

#[derive(Resource, Default)]
struct Queue(Vec<Tetrimino>);

impl Queue {
    fn extend(&mut self) {
        let mut indexes = [0, 1, 2, 3, 4, 5, 6];
        indexes.shuffle(&mut thread_rng());

        self.0.append(
            &mut indexes
                .into_iter()
                .map(|n| Tetrimino::new(Cell::from_num(n)))
                .collect::<Vec<Tetrimino>>(),
        );
    }
}

fn setup(world: &mut World) {
    world.insert_resource(Board::EMPTY);

    let mut queue = Queue::default();
    queue.extend();
    world.insert_resource(queue);

    world.insert_resource(CenterX(3)); // ??
}

#[derive(Resource)]
struct CenterX(i32);

fn move_x(mut center_x: ResMut<CenterX>, input: Res<Input>) {
    if input.key_pressed(Key::Left) {
        center_x.0 -= if center_x.0 > 0 { 1 } else { 0 };
    }

    if input.key_pressed(Key::Right) {
        center_x.0 += if center_x.0 < BOARD_WIDTH as i32 {
            1
        } else {
            0
        };
    }
}

fn turn_piece(mut queue: ResMut<Queue>, input: Res<Input>) {
    if input.key_pressed(Key::Z) {
        if let Some(tetrimino) = queue.0.first_mut() {
            tetrimino.turn_left();
        }
    }

    if input.key_pressed(Key::Up) {
        if let Some(tetrimino) = queue.0.first_mut() {
            tetrimino.turn_right();
        }
    }
}

fn hard_drop(
    mut board: ResMut<Board>,
    mut queue: ResMut<Queue>,
    mut center_x: ResMut<CenterX>,
    input: Res<Input>,
) {
    if input.key_pressed(Key::Space) {
        let tetrimino = &queue.0[0];
        board.insert(&tetrimino, center_x.0);
        center_x.0 = 3;
        queue.0.remove(0);
    }
}

fn extend_queue(mut queue: ResMut<Queue>) {
    if queue.is_changed() && queue.0.len() < VIEW_WINDOW + 1 {
        queue.extend();
    }
}

fn clear(mut canvas: ResMut<Canvas>) {
    canvas.clear(Color::BLACK.pixel());
}

fn draw_board(mut canvas: ResMut<Canvas>, board: Res<Board>) {
    for (i, cell) in board.0.iter().enumerate() {
        let x = i % BOARD_WIDTH;
        let y = i / BOARD_WIDTH;

        canvas.draw_rectangle(
            x as i32 * CELL_SIZE,
            y as i32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            cell.color().pixel(),
        );
    }
}

fn draw_ghost(
    mut canvas: ResMut<Canvas>,
    board: Res<Board>,
    center_x: Res<CenterX>,
    queue: Res<Queue>,
) {
    if let Some(tetrimino) = queue.0.first() {
        let center_y = board.where_insert(&tetrimino, center_x.0);
        for Point(x, y) in &tetrimino.points {
            canvas.draw_rectangle(
                (center_x.0 + *x) * CELL_SIZE,
                (center_y + *y) * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                Color::from(0x42414D).pixel(),
            );
        }
    }
}
