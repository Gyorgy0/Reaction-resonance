use macroquad::prelude::*;

#[macroquad::main("Particle Simulator")]
async fn main() {
    let row_count = 100; // Number of rows
    let col_count = 100; // Number of collumns
    let mut game_board = setup_board(row_count, col_count); // Initializes the game_board
    loop {
        clear_background(RED);
        draw_text(&format!("FPS: {0}", get_fps()), 40.0, 40.0, 55.0, YELLOW); // Displays the FPS of the game

        game_board = update_board(&mut game_board, row_count, col_count); // This function updates the state of the particles on the game_board

        draw_board(&game_board, row_count, col_count); // This function draws the game_board

        handle_key_inputs(&mut game_board, row_count, col_count); // This function monitors the pressed keys
        next_frame().await;
    }
}

fn setup_board(row_count: i32, col_count: i32) -> Vec<Particle> {
    let cell_count = row_count * col_count;
    let mut game_board: Vec<Particle> =
        vec![Particle(VOID, vec2(0.0, 0.0), false); cell_count as usize];
    for i in 0..row_count {
        for j in 0..col_count {
            game_board[(i * col_count + j) as usize] = Particle(VOID, vec2(0.0, 0.0), false);
        }
    }
    game_board
}

fn draw_board(game_board: &[Particle], row_count: i32, col_count: i32) {
    for i in 0..row_count {
        for j in 0..col_count {
            let cell: Particle = game_board[((i * col_count) + j) as usize];
            draw_rectangle(
                (j as u32 * CELLSIZE) as f32,
                (i as u32 * CELLSIZE) as f32 + 60.0,
                CELLSIZE as f32,
                CELLSIZE as f32,
                cell.0.color,
            )
        }
    }
}

fn update_board(game_board: &mut [Particle], row_count: i32, col_count: i32) -> Vec<Particle> {
    for i in 0..row_count {
        for j in 0..col_count {
            solve_particle(
                game_board,
                game_board[(i * row_count + j) as usize].0.phase,
                row_count,
                col_count,
                i,
                j,
            )
        }
    }
    handle_mouse_input(game_board, row_count, col_count);
    game_board.to_vec()
}

fn handle_mouse_input(game_board: &mut [Particle], row_count: i32, col_count: i32) {
    let btn = MouseButton::Left;
    let rbtn = MouseButton::Right;
    if is_mouse_button_down(btn) || is_mouse_button_down(rbtn) {
        let cursor_position = mouse_position();
        if cursor_position.0 > CELLSIZE as f32
            && cursor_position.0 < (CELLSIZE * col_count as u32) as f32
            && cursor_position.1 > CELLSIZE as f32
            && cursor_position.1 < (CELLSIZE * row_count as u32) as f32
        {
            let x = (cursor_position.0 as u32 / CELLSIZE) - 1;
            let y = ((cursor_position.1 - 55.0) as u32 / CELLSIZE) - 1;
            let material = if is_mouse_button_down(btn) {
                WATER
            } else {
                SAND
            };
            game_board[(y * col_count as u32 + x) as usize] =
                Particle(material, vec2(0.0, 1.0), false);
        }
    }
}

fn handle_key_inputs(game_board: &mut Vec<Particle>, row_count: i32, col_count: i32) {
    if is_key_pressed(KeyCode::R) {
        *game_board = setup_board(row_count, col_count);
    }
}

#[derive(Copy, Clone)]
enum Phase {
    Void,
    Solid { hardness: u8 },
    Powder { coarseness: f32 },
    Liquid { viscosity: f32 },
    Gas { viscosity: f32 },
    Plasma { viscosity: f32 },
}

fn solve_particle(
    game_board: &mut [Particle],
    phase: Phase,
    row_count: i32,
    col_count: i32,
    i: i32,
    j: i32,
) {
    let frame_time = get_frame_time();
    match phase {
        Phase::Void => {}
        Phase::Solid { hardness: _u8 } => {}
        Phase::Powder { coarseness: _f32 } => {
            let cellpos: usize = (i * col_count + j) as usize;
            game_board[cellpos].1.y += game_board[cellpos].0.mass * GRAVITY * frame_time;
            for _k in 0..game_board[cellpos].1.y as i32 {
                if (i + _k) < (row_count)
                    && game_board[cellpos].0.mass
                        > game_board[((i + _k) * col_count + j) as usize].0.mass
                    && game_board[cellpos].2
                {
                    game_board.swap(cellpos, (((i + _k) * col_count) + j) as usize);
                    game_board[(((i + _k) * col_count) + j) as usize].2 = false;
                } else if (i + _k) >= (row_count) {
                    game_board[cellpos].1.y = f32::abs((i - (row_count - 1)) as f32);
                    continue;
                }
            }
            game_board[cellpos].2 = true;
        }
        Phase::Liquid { viscosity: _f32 } => {
            let cellpos: usize = (i * col_count + j) as usize;
            game_board[cellpos].1.y += game_board[cellpos].0.mass * GRAVITY * frame_time;
            for _k in 0..game_board[cellpos].1.y as i32 {
                if (i + _k) < (row_count)
                    && game_board[cellpos].0.mass
                        > game_board[((i + _k) * col_count + j) as usize].0.mass
                    && game_board[cellpos].2
                {
                    game_board.swap(cellpos, (((i + _k) * col_count) + j) as usize);
                    game_board[(((i + _k) * col_count) + j) as usize].2 = false;
                } else if (i + _k) >= (row_count) {
                    game_board[cellpos].1.y = f32::abs((i - (row_count - 1)) as f32);
                    continue;
                }
            }
            game_board[cellpos].2 = true;
        }
        Phase::Gas { viscosity: _f32 } => {}
        Phase::Plasma { viscosity: _f32 } => {}
    }
}

const CELLSIZE: u32 = 5;
const GRAVITY: f32 = 9.81;

static VOID: Material = Material {
    mass: 0.0,
    phase: Phase::Void,
    flammability: 0.0,
    color: color_u8!(0, 0, 0, 100),
};

static WATER: Material = Material {
    mass: 1.0,
    phase: Phase::Liquid { viscosity: 1.0 },
    flammability: 1.0,
    color: BLUE,
};

static SAND: Material = Material {
    mass: 1.682,
    phase: Phase::Powder { coarseness: 1.0 },
    flammability: 0.0,
    color: color_u8!(203, 189, 147, 255),
};

#[derive(Copy, Clone)]
struct Material {
    mass: f32,         // Mass of a cm^3 volume of the material
    phase: Phase,      // Phase of the material for the implemented phases check the "Phase" enum
    flammability: f32, // Flammability of material -> higher number = more flammable (the flammability is calculated using normal atmospheric conditions (1 bar - 100 000 Pa pressure, 21% oxygen, 78% nitrogen))
    color: Color,      // Color of the material
}

#[derive(Copy, Clone)]
struct Particle(Material, Vec2, bool);
// 0 (Material) - 	Material of the particle
// 1 (Vec2) - 		Vectors of the particle (x, y)
// 2 (bool) -       Is it updated?
