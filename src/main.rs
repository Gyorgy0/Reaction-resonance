use macroquad::prelude::*;

#[macroquad::main("Particle Simulator")]

async fn main() {
    let vacuum = Particle(VOID, vec2(0.0, 0.0), vec2(0.0, 0.0));
    let cell_count =
        ((screen_height() / CELLSIZE as f32) * (screen_width() / CELLSIZE as f32)) as i32;
    let row_count = (screen_height() / CELLSIZE as f32) as i32;
    let col_count = (screen_width() / CELLSIZE as f32) as i32;
    let mut game_board: Vec<Particle> = vec![vacuum; cell_count as usize];
    for i in 1..row_count {
        for j in 1..col_count {
            game_board[(i * col_count + j) as usize] =
                Particle(VOID, vec2(j as f32, i as f32), vec2(0.0, 0.0));
        }
    }
    loop {
        clear_background(RED);
        DrawBoard(&game_board, row_count, col_count);
        next_frame().await;
    }
}

fn DrawBoard(game_board: &Vec<Particle>, row_count: i32, col_count: i32) {
    for i in 1..row_count {
        for j in 1..col_count {
            let cell: Particle = game_board[((i * col_count) + j) as usize];
            draw_rectangle(
                cell.1.x * CELLSIZE as f32,
                cell.1.y * CELLSIZE as f32,
                CELLSIZE as f32,
                CELLSIZE as f32,
                cell.0 .2,
            )
        }
    }
}

const CELLSIZE: u32 = 5;

const GRAVITY: f32 = 9.81;
const FPS: u32 = 60;

static VOID: Material = Material(0.0, 0.0, color_u8!(0.0, 0.0, 0.0, 100.0));

static WATER: Material = Material(1.0, 1.0, BLUE);

#[derive(PartialEq, Debug, Copy, Clone, Default)]
struct Material(f32, f32, Color);
// 0 (f32) - 	mass of a cm^3 volume of the material
// 1 (f32) - 	viscosity of the material -> higher number = thicker material (viscosity of water is 1)
// 2 (Color) - 	color of the material

#[derive(PartialEq, Debug, Clone, Copy, Default)]
struct Particle(Material, Vec2, Vec2);
// 0 (Material) - 	material of the particle
// 1 (Vec2) - 		visco
// 2 (Vec2) - 		color of the material
// kilroy was here