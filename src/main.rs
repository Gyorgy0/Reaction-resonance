use macroquad::prelude::*;

#[macroquad::main("Particle Simulator")]
async fn main() {
    let row_count = 100;
    let col_count = 100;
    let mut game_board = setup_board(row_count, col_count);

    loop {
        clear_background(WHITE);  // only changed to know what version is running in comparisons

        let text = format!("FPS: {}", get_fps());  
        draw_text(&text, 40.0, 40.0, 55.0, YELLOW);
        
        game_board = update_board(&mut game_board, row_count, col_count);
        draw_board(&game_board, row_count, col_count);

        handle_shortcuts(&mut game_board, row_count, col_count);  //R-reset board

        next_frame().await;
    }
}

fn setup_board(row_count: i32, col_count: i32) -> Vec<Particle> {
    let cell_count = row_count * col_count;
    let mut game_board: Vec<Particle> = vec![Particle(VOID, vec2(0.0, 0.0), false); cell_count as usize];
    for i in 0..row_count {
        for j in 0..col_count {
            game_board[(i * col_count + j) as usize] = Particle(VOID, vec2(0.0, 0.0), false);
        }
    }
    return game_board;
}

fn draw_board(game_board: &Vec<Particle>, row_count: i32, col_count: i32) {
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

fn update_board(game_board: &mut Vec<Particle>, row_count: i32, col_count: i32) -> Vec<Particle> {
    let frame_time = get_frame_time();
    for i in (0..row_count).rev() { 
        for j in 0..col_count {
            let cellpos: usize = (i * col_count + j) as usize;
            if game_board[cellpos].0.mass > 0.0 {
                game_board[cellpos].1.y += game_board[cellpos].0.mass * GRAVITY * frame_time;

                let mut k = game_board[cellpos].1.y as i32;
                while k > 0 {
                    let new_i = (i + k) as usize;
                    if new_i < row_count as usize {
                        let new_pos = new_i * col_count as usize + j as usize;
                        if game_board[new_pos].0.mass == 0.0 || game_board[new_pos].0.mass < game_board[cellpos].0.mass {
                            game_board.swap(cellpos, new_pos);
                            game_board[new_pos].2 = false;
                        } else {
                            game_board[cellpos].1.y = 0.0;
                            break;
                        }
                    } else {
                        game_board[cellpos].1.y = 0.0;
                        break;
                    }
                    k -= 1;
                }
                game_board[cellpos].2 = true;
            }
        }
    }
    handle_mouse_input(game_board, row_count, col_count);
    game_board.to_vec()
}

fn handle_mouse_input(game_board: &mut Vec<Particle>, row_count: i32, col_count: i32) {
    let btn = MouseButton::Left;
    let rbtn = MouseButton::Right;
    if is_mouse_button_down(btn) || is_mouse_button_down(rbtn) {
        let cursor_position = mouse_position();
        if cursor_position.0 > CELLSIZE as f32 && cursor_position.0 < (CELLSIZE * col_count as u32) as f32 &&
            cursor_position.1 > CELLSIZE as f32 && cursor_position.1 < (CELLSIZE * row_count as u32) as f32 {
            let x = (cursor_position.0 as u32 / CELLSIZE) - 1;
            let y = (cursor_position.1 as u32 / CELLSIZE) - 1;
            let material = if is_mouse_button_down(btn) { WATER } else { SAND };
            game_board[(y * col_count as u32 + x) as usize] = Particle(material, vec2(0.0, 2.0), false);
        }
    }
}

fn handle_shortcuts(game_board: &mut Vec<Particle>, row_count: i32, col_count: i32) {
    if is_key_pressed(KeyCode::R) {
        *game_board = setup_board(row_count, col_count);
    }
}

const CELLSIZE: u32 = 5;
const GRAVITY: f32 = 9.81;

static VOID: Material = Material {
    mass: 0.0,
    phase: 0,
    viscosity: 0.0,
    color: color_u8!(0, 0, 0, 100),
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
