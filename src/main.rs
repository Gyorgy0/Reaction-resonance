use macroquad::prelude::*;

#[macroquad::main("Particle Simulator")]

async fn main() {
    let mut start: bool = true;
    let mut game_board = vec![];
    let row_count = 50;
    let col_count = 50;
    loop {
        if start {
            game_board = SetupBoard(row_count, col_count);
            start = false;
        }
        clear_background(RED);
        DrawBoard(&game_board, row_count, col_count);
        game_board = UpdateBoard(&mut game_board, row_count, col_count);
        next_frame().await;
    }
}

fn SetupBoard(row_count: i32, col_count: i32) -> Vec<Particle> {
    let cell_count = row_count * col_count;
    let mut game_board: Vec<Particle> =
        vec![Particle(VOID, vec2(0.0, 0.0), false); cell_count as usize];
    for i in 0..row_count {
        for j in 0..col_count {
            game_board[(i * col_count + j) as usize] = Particle(VOID, vec2(0.0, 0.0), false);
        }
    }
    return game_board;
}

fn DrawBoard(game_board: &Vec<Particle>, row_count: i32, col_count: i32) {
    for i in 0..row_count {
        for j in 0..col_count {
            let cell: Particle = game_board[((i * col_count) + j) as usize];
            draw_rectangle(
                ((j + 1) as u32 * CELLSIZE) as f32,
                ((i + 1) as u32 * CELLSIZE) as f32,
                CELLSIZE as f32,
                CELLSIZE as f32,
                cell.0.color,
            )
        }
    }
}

fn UpdateBoard(game_board: &mut Vec<Particle>, row_count: i32, col_count: i32) -> Vec<Particle> {
    for i in 0..row_count {
        for j in 0..col_count {
            let mut cell: Particle = game_board[((i * col_count) + j) as usize];
            cell.1.y += (cell.0.mass * GRAVITY * get_frame_time());
            if (cell.0.mass > 0.0) {
                if (i + (cell.1.y as i32 - 1)) < row_count - 1 && (!cell.2) && cell.1.y > 1.0 {
                    let tmp: Particle =
                        game_board[(((i + cell.1.y as i32) * col_count) + j) as usize];
                    game_board[(((i + cell.1.y as i32) * col_count) + j) as usize] = cell;
                    game_board[(((i + cell.1.y as i32) * col_count) + j) as usize].2 = true;
                    game_board[((i * col_count) + j) as usize] = tmp;
                } else if ((i as i32 - 1) - (row_count - 1)) < cell.1.y as i32
                    && (!cell.2)
                    && cell.1.y > 1.0
                {
                    cell.1.y = (-1.0) * ((i as i32) - (row_count - 1)) as f32;
                    let tmp: Particle =
                        game_board[(((i + cell.1.y as i32) * col_count) + j) as usize];
                    game_board[(((i + cell.1.y as i32) * col_count) + j) as usize] = cell;
                    game_board[(((i + cell.1.y as i32) * col_count) + j) as usize]
                        .1
                        .y = 0.0;
                    game_board[(((i + cell.1.y as i32) * col_count) + j) as usize].2 = true;
                    game_board[((i * col_count) + j) as usize] = tmp;
                } else if (cell.1.y < 1.0) {
                    game_board[((i * col_count) + j) as usize].1.y = cell.1.y;
                }
            }
        }
    }
    for i in 0..row_count {
        for j in 0..col_count {
            game_board[((i * col_count) + j) as usize].2 = false;
        }
    }
    let btn: MouseButton = MouseButton::Left;
    let rbtn: MouseButton = MouseButton::Right;
    if is_mouse_button_down(btn) {
        let cursor_position = mouse_position();
        if (cursor_position.0 > 10.0
            && cursor_position.0 < 510.0
            && cursor_position.1 > 10.0
            && cursor_position.1 < 510.0)
        {
            let x = (cursor_position.0 as u32 / CELLSIZE) - 1;
            let y = (cursor_position.1 as u32 / CELLSIZE) - 1;
            game_board[(y * col_count as u32 + x) as usize] =
                Particle(WATER, vec2(0.0, 0.0), false);
        }
    }
    if is_mouse_button_down(rbtn) {
        let cursor_position = mouse_position();
        if (cursor_position.0 > 10.0
            && cursor_position.0 < 510.0
            && cursor_position.1 > 10.0
            && cursor_position.1 < 510.0)
        {
            let x = (cursor_position.0 as u32 / CELLSIZE) - 1;
            let y = (cursor_position.1 as u32 / CELLSIZE) - 1;
            game_board[(y * col_count as u32 + x) as usize] = Particle(SAND, vec2(0.0, 0.0), false);
        }
    }
    return game_board.to_vec();
}

const CELLSIZE: u32 = 10;
const GRAVITY: f32 = 9.81;

static VOID: Material = Material {
    mass: 0.0,
    phase: 0,
    viscosity: 0.0,
    color: color_u8!(0.0, 0.0, 0.0, 100.0),
};

static WATER: Material = Material {
    mass: 1.0,
    phase: 3,
    viscosity: 1.0,
    color: BLUE,
};

static SAND: Material = Material {
    mass: 1.682,
    phase: 2,
    viscosity: 0.0,
    color: color_u8!(203, 189, 147, 255),
};

#[derive(PartialEq, Debug, Copy, Clone, Default)]
struct Material {
    mass: f32,      // mass of a cm^3 volume of the material
    phase: u8, // phase of the material    -> 0 - void, 1 - solid, 2 - powder, 3 - liquid, 4 - gas, 5 - plasma
    viscosity: f32, // viscosity of the material -> higher number = thicker material (viscosity of water is 1)
    color: Color,   // color of the material
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
struct Particle(Material, Vec2, bool);
// 0 (Material) - 	material of the particle
// 1 (Vec2) - 		vectors of the particle (x, y)
// 2 (bool) -       is it updated
