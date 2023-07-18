use bevy_ecs::{
    prelude::DetectChanges,
    schedule::Schedule,
    system::{Res, ResMut, Resource},
    world::World,
};
use panda_bevy::{canvas::Color, utils::default, Canvas, Panda, PandaOptions};

const CELL_SIZE: i32 = 20;

const GRID_WIDTH: u32 = 16;
const GRID_HEIGHT: u32 = 16;
const RULE: u8 = 110;

fn main() {
    let mut panda = Panda::new(PandaOptions {
        width: GRID_WIDTH * CELL_SIZE as u32,
        height: GRID_HEIGHT * CELL_SIZE as u32,
        scale: 1,
        ..default()
    });

    setup(&mut panda.world);

    let mut schedule = Schedule::new();
    schedule.add_system(draw_grid);
    panda.run(schedule);
}

fn setup(world: &mut World) {
    let mut eca = ECA::new(GRID_WIDTH, GRID_HEIGHT, RULE);
    eca.grid.set(GRID_WIDTH as i32 - 1, 0, true);
    eca.update();

    world.insert_resource(eca);
    Rule::new(10);
}

fn draw_grid(eca: Res<ECA>, mut canvas: ResMut<Canvas>) {
    if eca.is_changed() {
        canvas.clear(Color::BLACK.pixel());
        for x in 0..eca.grid.width as i32 {
            for y in 0..eca.grid.height as i32 {
                if let Some(true) = eca.grid.get(x, y) {
                    canvas.draw_rectangle(
                        x * CELL_SIZE,
                        y * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        Color::WHITE.pixel(),
                    );
                }
            }
        }
    }
}

#[derive(Resource)]
struct ECA {
    grid: Grid,
    rule: Rule,
}

impl ECA {
    fn new(width: u32, height: u32, rule: u8) -> Self {
        Self {
            grid: Grid::new(width, height),
            rule: Rule::new(rule),
        }
    }

    fn update(&mut self) {
        // first row isn't touched
        for y in 1..self.grid.height as i32 {
            for x in 0..self.grid.width as i32 {
                let state = self.grid.state(x, y);
                if let Some(alive) = self.rule.check(state) {
                    self.grid.set(x, y, *alive);
                }
            }
        }
    }
}

struct Grid {
    width: u32,
    height: u32,
    cells: Vec<Vec<bool>>,
}

impl Grid {
    fn new(width: u32, height: u32) -> Self {
        let cells = vec![vec![false; width as usize]; height as usize];

        Self {
            width,
            height,
            cells,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&bool> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        Some(self.cells.get(y)?.get(x)?)
    }

    fn set(&mut self, x: i32, y: i32, alive: bool) -> Option<()> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        *self.cells.get_mut(y)?.get_mut(x)? = alive;
        Some(())
    }

    fn state(&self, x: i32, y: i32) -> u32 {
        (0..3).fold(0, |state, i| {
            let offset = -i + 1;
            if let Some(true) = self.get(x + offset, y - 1) {
                return state + 2_u32.pow((i) as u32);
            }
            state
        })
    }
}

struct Rule([bool; 8]);
impl Rule {
    fn new(rule: u8) -> Self {
        let binary = format!("{:0>8b}", rule);
        let mut rule = [false; 8];

        for (i, char) in binary.chars().enumerate() {
            match char {
                '0' => rule[7 - i] = false,
                '1' => rule[7 - i] = true,
                _ => panic!("binary with more than 2 chars??"),
            }
        }

        Self(rule)
    }

    fn check(&self, state: u32) -> Option<&bool> {
        self.0.get(state as usize)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grid_get() {
        let mut grid = Grid::new(10, 10);
        grid.cells[0][8] = true;
        grid.cells[4][2] = true;

        assert_eq!(grid.get(8, 0), Some(&true));
        assert_eq!(grid.get(2, 4), Some(&true));
    }

    #[test]
    fn grid_set() {
        let mut grid = Grid::new(10, 10);
        grid.set(8, 0, true);
        grid.set(2, 4, true);

        assert_eq!(grid.cells[0][8], true);
        assert_eq!(grid.cells[4][2], true);
    }

    #[test]
    fn grid_state() {
        // [ ] [*] [ ] [*] [*]
        let mut grid = Grid::new(6, 2);
        grid.set(1, 0, true);
        grid.set(3, 0, true);
        grid.set(4, 0, true);

        assert_eq!(grid.state(0, 1), 1); // [ ] [ ] [*]
        assert_eq!(grid.state(1, 1), 2); // [ ] [*] [ ]
        assert_eq!(grid.state(2, 1), 5); // [*] [ ] [*]
        assert_eq!(grid.state(3, 1), 3); // [ ] [*] [*]
        assert_eq!(grid.state(4, 1), 6); // [*] [*] [ ]
        assert_eq!(grid.state(5, 1), 4); // [*] [ ] [ ]
    }
    
    #[test]
    fn rule_check() {
        // 110 = (2 ^ 1) + (2 ^ 2) + (2 ^ 3) + (2 ^ 5) + (2 ^ 6)
        let rule = Rule::new(110);
        
        assert_eq!(rule.check(0), Some(&false));
        assert_eq!(rule.check(1), Some(&true));
        assert_eq!(rule.check(2), Some(&true));
        assert_eq!(rule.check(3), Some(&true));
        assert_eq!(rule.check(4), Some(&false));
        assert_eq!(rule.check(5), Some(&true));
        assert_eq!(rule.check(6), Some(&true));
        assert_eq!(rule.check(7), Some(&false));
    }

    #[test]
    fn eca_update() {
        let mut eca = ECA::new(16, 16, 110);
        eca.grid.set(15, 0, true);
        eca.update();

        assert_eq!(eca.grid.get(0, 15), Some(&true));
        assert_eq!(eca.grid.get(1, 15), Some(&true));
        assert_eq!(eca.grid.get(2, 15), Some(&false));
        assert_eq!(eca.grid.get(3, 15), Some(&true));
        assert_eq!(eca.grid.get(4, 15), Some(&false));
        assert_eq!(eca.grid.get(5, 15), Some(&true));
        assert_eq!(eca.grid.get(6, 15), Some(&true));
        assert_eq!(eca.grid.get(7, 15), Some(&false));
        assert_eq!(eca.grid.get(8, 15), Some(&false));
        assert_eq!(eca.grid.get(9, 15), Some(&true));
        assert_eq!(eca.grid.get(10, 15), Some(&true));
        assert_eq!(eca.grid.get(11, 15), Some(&true));
        assert_eq!(eca.grid.get(12, 15), Some(&true));
        assert_eq!(eca.grid.get(13, 15), Some(&true));
        assert_eq!(eca.grid.get(14, 15), Some(&false));
        assert_eq!(eca.grid.get(15, 15), Some(&true));
    }
}

