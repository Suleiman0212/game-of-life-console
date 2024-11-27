use rand::Rng;
use std::{self, env, thread, time::Duration};
use utils::{clear_terminal, update_terminal, wait_input};

mod utils;

const GRID_HEIGHT: i32 = 25;
const GRID_WIDTH: i32 = 35;

static NEIGHBORS_POSITION_OFFSET: [Offset; 8] = [
    Offset { x: -1, y: -1 },
    Offset { x: 0, y: -1 },
    Offset { x: 1, y: -1 },
    Offset { x: -1, y: 0 },
    Offset { x: 1, y: 0 },
    Offset { x: -1, y: 1 },
    Offset { x: 0, y: 1 },
    Offset { x: 1, y: 1 },
];

struct Offset {
    x: i32,
    y: i32,
}

enum UpdateType {
    Manualy,
    Auto(u64),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Expected arguments (auto --duratation_in_millis--, manualy)");
        return;
    }
    let arg1 = Some(args[1].clone());

    clear_terminal();
    let mut grid: Grid = Grid::new();
    grid.fill_random();

    match arg1 {
        Some(a) => {
            if a == "manualy".to_string() {
                update_loop(UpdateType::Manualy, &mut grid);
            } else if a == "auto".to_string() {
                let arg2 = Some(args[2].clone());
                let duratation: u64;
                match arg2 {
                    Some(d) => {
                        duratation = d.trim().parse().expect("Cant parse duratation!");
                        update_loop(UpdateType::Auto(duratation), &mut grid);
                    }
                    None => {
                        println!("Expected duratation in millis!");
                        return;
                    }
                }
            } else {
                println!("Unknown arguments!")
            }
        }
        None => {
            println!("Enter arguments! (auto, manualy)")
        }
    }
}

fn update_loop(update_type: UpdateType, grid: &mut Grid) {
    loop {
        update_terminal();
        update_cells(grid);
        display_grid(&grid);
        match update_type {
            UpdateType::Manualy => {
                wait_input();
            }
            UpdateType::Auto(d) => {
                thread::sleep(Duration::from_millis(d));
            }
        }
    }
}

fn update_cells(grid: &mut Grid) {
    let mut new_data = grid.data.clone();

    for idx in 0..grid.data.len() {
        let neighbors = grid.cell_neighbors(idx);
        match neighbors {
            2 => {}
            3 => new_data[idx].is_alive = true,
            _ => new_data[idx].is_alive = false,
        }
    }

    grid.data = new_data
}

fn display_grid(grid: &Grid) {
    let mut output: String = String::new();
    let mut current_row: i32 = 0;

    for cell in 0..grid.data.len() {
        if grid.data[cell].position.y != current_row {
            output.push_str("\n");
            current_row += 1;
        }

        if grid.data[cell].is_alive {
            output.push_str(" \x1b[32m#\x1b[0m ");
        } else {
            output.push_str(" \x1b[31m.\x1b[0m ");
        }
    }
    println!("{output}");
}

impl Grid {
    fn new() -> Self {
        Grid { data: vec![] }
    }

    fn add_cell(&mut self, cell: Cell) {
        self.data.push(cell);
    }

    #[allow(dead_code)]
    fn fill(&mut self) {
        for h in 0..GRID_HEIGHT {
            for w in 0..GRID_WIDTH {
                let new_cell = Cell {
                    position: Position { x: w, y: h },
                    is_alive: false,
                };
                self.add_cell(new_cell);
            }
        }
    }

    fn fill_random(&mut self) {
        for h in 0..GRID_HEIGHT {
            for w in 0..GRID_WIDTH {
                let rand_val: i32 = rand::thread_rng().gen_range(0..=4);
                let is_alive = match rand_val {
                    0 => true,
                    _ => false,
                };

                let new_cell = Cell {
                    position: Position { x: w, y: h },
                    is_alive,
                };
                self.add_cell(new_cell);
            }
        }
    }

    fn cell_neighbors(&self, cell: usize) -> i32 {
        let mut neightors: i32 = 0;
        for idx in 0..NEIGHBORS_POSITION_OFFSET.len() {
            let x = &self.data[cell].position.x + NEIGHBORS_POSITION_OFFSET[idx].x;
            let y = &self.data[cell].position.y + NEIGHBORS_POSITION_OFFSET[idx].y;
            let position: Position = Position { x, y };

            if let Some(c) = self.find_cell(position) {
                if c.is_alive {
                    neightors += 1;
                }
            }
        }
        neightors
    }

    fn find_cell(&self, cell_position: Position) -> Option<&Cell> {
        for cell in &self.data {
            if cell.position.x == cell_position.x && cell.position.y == cell_position.y {
                return Some(cell);
            }
        }
        None
    }
}

struct Grid {
    data: Vec<Cell>,
}

#[derive(Clone)]
struct Cell {
    position: Position,
    is_alive: bool,
}

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}
