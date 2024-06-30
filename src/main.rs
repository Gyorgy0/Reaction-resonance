use macroquad::prelude::*;

#[macroquad::main("Particle Simulator")]

async fn main() {
    let cell_count =
        ((screen_height() / CELLSIZE as f32) * (screen_width() / CELLSIZE as f32)) as i32;
    let row_count = (screen_height() / CELLSIZE as f32) as i32;
    let col_count = (screen_width() / CELLSIZE as f32) as i32;
    let mut game_board: Vec<Particle> =
        vec![Particle(VOID, vec2(0.0, 0.0)); cell_count as usize];
    for i in 1..row_count {
        for j in 1..col_count {
            game_board[(i * col_count + j) as usize] =
                Particle(VOID, vec2(0.0, 0.0));
        }
    }
    game_board[525] = Particle(WATER, vec2(0.0, 0.0));
    loop {
        clear_background(RED);
        UpdateBoard(game_board, row_count, col_count);
        DrawBoard(&game_board, row_count, col_count);
        next_frame().await;
    }
}

fn DrawBoard(game_board: &Vec<Particle>, row_count: i32, col_count: i32) {
    for i in 1..row_count {
        for j in 1..col_count {
            let cell: Particle = game_board[((i * col_count) + j) as usize];
            draw_rectangle(
                (j as u32 * CELLSIZE) as f32,
                (i as u32 * CELLSIZE) as f32,
                CELLSIZE as f32,
                CELLSIZE as f32,
                cell.0.color,
            )
        }
    }
}

fn UpdateBoard(mut game_board: Vec<Particle>, row_count: i32, col_count: i32) {
    for i in 1..row_count {
        for j in 1..col_count {
            let mut cell: Particle = game_board[((i * col_count) + j) as usize];
            cell.1.y += cell.0.mass * GRAVITY * (1000 / 60) as f32;
            //for k in 1..cell.1.y as i32 {
            //    if cell.0.mass > game_board[(((i + k) * col_count) + j) as usize].0.mass 
            //    {
                    let tmp:Particle = game_board[(((i + cell.1.y as i32) * col_count) + j) as usize];
                    game_board[(((i + cell.1.y as i32) * col_count) + j) as usize] = cell;
                    game_board[(((i + (cell.1.y as i32-1)) * col_count) + j) as usize] = tmp;
            //    }
            //}
        }
    }
}

const CELLSIZE: u32 = 10;

const GRAVITY: f32 = 9.81;
const FPS: u32 = 60;

static VOID: Material = Material {
    mass: 0.0,
    viscosity: 0.0,
    color: color_u8!(0.0, 0.0, 0.0, 100.0),
};

static WATER: Material = Material {
    mass: 1.0,
    viscosity: 1.0,
    color: BLUE,
};

#[derive(PartialEq, Debug, Copy, Clone, Default)]
struct Material {
    mass: f32,      // mass of a cm^3 volume of the material
    viscosity: f32, // viscosity of the material -> higher number = thicker material (viscosity of water is 1)
    color: Color,   // color of the material
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
struct Particle(Material, Vec2);
// 0 (Material) - 	material of the particle
// 1 (Vec2) - 		vectors of the particle (x, y)
