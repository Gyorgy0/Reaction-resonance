use std::ops::Not;

use macroquad::prelude::*;
mod materials;

const CELLSIZE: u32 = 3;
const GRAVITY: f32 = 9.81;
#[macroquad::main("Particle Simulator")]

async fn main() {
    let mut game_board: Board = Board {
        width: 1,
        height: 1,
        contents: vec![],
    };
    game_board.create_board(300, 200); // Initializes the game_board
    let mut materials: Vec<Material> = vec![
        materials::powder::SAND,
        materials::liquid::WATER,
        materials::solid::WOOD,
        materials::gas::METHANE,
    ];
    let mut is_paused = false;
    let mut selected_material = materials::gas::METHANE;
    loop {
        clear_background(RED);
        if is_paused {
            draw_text(
                &format!("FPS: {0}", get_fps()),
                40.0,
                40.0,
                55.0,
                color_u8!(150, 0, 0, 255),
            ); // Displays the FPS of the game
        } else if !is_paused {
            draw_text(&format!("FPS: {0}", get_fps()), 40.0, 40.0, 55.0, YELLOW);
            // Displays the FPS of the game
        }

        update_board(&mut game_board, &mut selected_material, is_paused);
        draw_board(&game_board); // This function draws the game_board

        let col_count = game_board.width;
        draw_clear_button(
            &mut game_board,
            (col_count as i32 * CELLSIZE as i32) as f32 + 15.0,
            60.0,
        );
        start_pause_button(
            &mut is_paused,
            (col_count as i32 * CELLSIZE as i32) as f32 + 15.0,
            100.0,
        );
        draw_material_buttons(
            &mut selected_material,
            &mut materials,
            (col_count as i32 * CELLSIZE as i32) as f32 + 15.0,
            140.0,
        );

        handle_key_inputs(&mut game_board, &mut is_paused); // This function monitors the pressed keys
        next_frame().await;
    }
}

fn draw_board(game_board: &Board) {
    let f: Vec<_> = game_board
        .contents
        .iter()
        .flat_map(|particle| {
            let color = vec![
                (particle.0.color.r * 255.0) as u8,
                (particle.0.color.g * 255.0) as u8,
                (particle.0.color.b * 255.0) as u8,
                (particle.0.color.a * 255.0) as u8,
            ];
            color
        })
        .collect();
    let board_cells: Texture2D = Texture2D::from_rgba8(game_board.width, game_board.height, &f);
    board_cells.set_filter(FilterMode::Nearest);
    draw_texture_ex(
        &board_cells,
        5.,
        60.,
        WHITE,
        DrawTextureParams {
            dest_size: Option::Some(vec2(
                (game_board.width * CELLSIZE as u16) as f32,
                (game_board.height * CELLSIZE as u16) as f32,
            )),
            source: Option::None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: Option::None,
        },
    );
}

fn update_board(game_board: &mut Board, selected_material: &mut Material, is_stopped: bool) {
    let row_count = game_board.height as i32;
    let col_count: i32 = game_board.width as i32;
    if !is_stopped {
        (0..row_count * col_count).for_each(|count| {
            let i = count / col_count;
            let j = count % col_count;
            game_board.solve_particle(game_board.contents[count as usize].0.phase, i, j);
        });
    }
    handle_mouse_input(game_board, selected_material, row_count, col_count);
}

fn handle_mouse_input(
    game_board: &mut Board,
    selected_material: &mut Material,
    row_count: i32,
    col_count: i32,
) {
    let btn = MouseButton::Left;
    let rbtn = MouseButton::Right;
    if is_mouse_button_down(btn) || is_mouse_button_down(rbtn) {
        let cursor_position = mouse_position();
        if cursor_position.0 > CELLSIZE as f32 - 5.0
            && cursor_position.0 < (CELLSIZE * col_count as u32) as f32
            && cursor_position.1 > CELLSIZE as f32 + 55.0
            && cursor_position.1 < (CELLSIZE * row_count as u32) as f32 + 60.0
        {
            let x = cursor_position.0 as u32 / CELLSIZE;
            let y = (cursor_position.1 - 60.0) as u32 / CELLSIZE;
            let material = if is_mouse_button_down(btn) {
                *selected_material
            } else {
                materials::solid::VOID
            };
            game_board.contents[(y * col_count as u32 + x) as usize] = Particle(
                material,
                vec2(0.0, 1.0),
                true,
                game_board.contents[(y * col_count as u32 + x) as usize].3,
            );
        }
    }
}

fn handle_key_inputs(game_board: &mut Board, is_paused: &mut bool) {
    if is_key_pressed(KeyCode::R) {
        game_board.create_board(game_board.width, game_board.height);
    }
    if is_key_pressed(KeyCode::Space) {
        *is_paused = is_paused.not();
    }
}

fn draw_clear_button(game_board: &mut Board, x: f32, y: f32) {
    let (btn_width, btn_height): (f32, f32) = (100.0, 30.0);
    let mouse_pos: (f32, f32) = mouse_position();
    let mouse_pressed: bool = is_mouse_button_pressed(MouseButton::Left);

    if mouse_pos.0 > x
        && mouse_pos.0 < x + btn_width
        && mouse_pos.1 > y
        && mouse_pos.1 < y + btn_height
        && mouse_pressed
    {
        game_board.create_board(game_board.width, game_board.height);
    }

    draw_rectangle(x, y, btn_width, btn_height, DARKGRAY);
    draw_text("Clear", x + 10.0, y + 20.0, 20.0, WHITE);
}

fn is_mouse_over_button(x: f32, y: f32, width: f32, height: f32) -> bool {
    let (mouse_x, mouse_y): (f32, f32) = mouse_position();
    mouse_x > x && mouse_x < x + width && mouse_y > y && mouse_y < y + height
}

pub fn start_pause_button(is_paused: &mut bool, x: f32, y: f32) {
    let button_width: f32 = 100.0;
    let button_height: f32 = 30.0;
    let button_color: Color = if *is_paused {
        color_u8!(150, 0, 0, 255)
    } else {
        GREEN
    };
    let label: &str = if *is_paused { "Start" } else { "Pause" };

    draw_rectangle(x, y, button_width, button_height, button_color);
    draw_text(label, x + 10.0, y + 20.0, 20.0, WHITE);

    if is_mouse_button_pressed(MouseButton::Left)
        && is_mouse_over_button(x, y, button_width, button_height)
    {
        *is_paused = !*is_paused;
    }
}

pub fn draw_material_buttons(
    selected_material: &mut Material,
    materials: &mut Vec<Material>,
    x: f32,
    y: f32,
) {
    let button_width: f32 = 100.0;
    let button_height: f32 = 30.0;
    for i in 0..materials.len() {
        let button_color: Color = if selected_material.name != materials[i].name {
            DARKGRAY
        } else {
            DARKBLUE
        };
        draw_rectangle(
            x,
            y + (40 * i) as f32,
            button_width,
            button_height,
            button_color,
        );
        draw_text(
            &materials[i].name,
            x + 10.0,
            (y + 20.0) + (40 * i) as f32,
            20.0,
            WHITE,
        );
        if is_mouse_button_pressed(MouseButton::Left)
            && is_mouse_over_button(x, y + (40 * i) as f32, button_width, button_height)
        {
            *selected_material = materials[i];
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Phase {
    Void,
    Solid,
    Powder { coarseness: f32 }, // Coarseness is the average diameter of a powder particle (between 0 and 1) (in cm), -> , the powder is less stackable it'll flow to the sides like a liquid
    Liquid { viscosity: f32 }, // Viscosity gives the rate, which the liquid spreads, for e.g. water has a viscosity of 1.0, the bigger the viscosity, the thicker the fluid
    Gas { viscosity: f32 },    // Viscosity gives the rate, which the gas fills the space
    Plasma { viscosity: f32 },
}

impl Phase {
    fn get_coarseness(&self) -> f32 {
        let mut returnval: f32 = 0.0;
        if let Phase::Powder { coarseness } = self {
            returnval = *coarseness
        };
        returnval
    }
    fn get_viscosity(&self) -> f32 {
        let mut returnval: f32 = 0.0;
        if let Phase::Liquid { viscosity } = self {
            returnval = *viscosity;
        };
        if let Phase::Gas { viscosity } = self {
            returnval = *viscosity;
        };
        if let Phase::Plasma { viscosity } = self {
            returnval = *viscosity;
        };
        returnval
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Material {
    name: &'static str, // Name of the material
    mass: f32,          // Mass of a cm^3 volume of the material
    phase: Phase,       // Phase of the material for the implemented phases check the "Phase" enum
    durability: i32, // Durability of a material - how much force it needs to disintegrate the material -> higher = more force
    //oxidizer: bool,
    flammability: f32, // Flammability of material -> higher number = more flammable (the flammability is calculated using normal atmospheric conditions (1 bar - 100 000 Pa pressure, 21% oxygen, 78% nitrogen))
    //conductor: bool,
    //resistance: f32,
    color: Color, // Color of the material
}

#[derive(Copy, Clone)]
pub struct Particle(Material, Vec2, bool, f32);
// 0 (Material) - 	Material of the particle
// 1 (Vec2) - 		Vectors of the particle (x, y)
// 2 (bool) -       Is it updated?
// 3 (f32)  -       Random number associated with the cell (for calculating phase behaviour)

#[derive(Clone)]
struct Board {
    width: u16,
    height: u16,
    contents: Vec<Particle>,
}

impl Board {
    fn get_width(&self) -> u16 {
        self.width
    }
    fn get_height(&self) -> u16 {
        self.height
    }
    fn create_board(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.contents = vec![
            Particle(materials::solid::VOID, vec2(0.0, 0.0), false, 0.0);
            (&self.width * &self.height) as usize
        ];
        (0..self.width * self.height).for_each(|count| {
            self.contents[count as usize].3 = rand::gen_range(0.0, 1.0);
        });
    }
    fn solve_particle(&mut self, phase: Phase, i: i32, j: i32) {
        let row_count: i32 = self.height as i32;
        let col_count: i32 = self.width as i32;
        let frame_time = get_frame_time();
        match phase {
            Phase::Void => {}

            Phase::Solid => {}
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // POWDER PHYSICS
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            Phase::Powder { coarseness: _f32 } => {
                let cellpos: usize = (i * col_count + j) as usize;
                // Gravity simulation
                self.contents[cellpos].1.y += GRAVITY * frame_time;
                for _k in 0..(self.contents[cellpos].1.y + 1.0) as i32 {
                    // Falling and checking if there is a particle with a larger mass/density also it marks a particle
                    // after every pixel it falls, so it doesn't appear instantly at the bottom
                    if (i + _k) < (row_count)
                        && self.contents[cellpos].0.mass
                            > self.contents[((i + _k) * col_count + j) as usize].0.mass
                        && self.contents[cellpos].2
                    {
                        self.contents
                            .swap(cellpos, ((i + _k) * col_count + j) as usize);
                        self.contents[((i + _k) * col_count + j) as usize].2 = false;
                    }
                    // Checks if the powder particle falls inside bounds, if not, then it corrects it's falling speed
                    else if (i + _k) >= (row_count) {
                        self.contents[cellpos].1.y = f32::abs((i - (col_count - 1)) as f32);
                    }
                    // Checks, whether there is a solid particle in the path of the falling powder particle, if there
                    // is, then the falling speed is adjusted, also marks the particle so it doesn't appear on the
                    // solid particle instantly
                    else if self.contents[((i + _k) * col_count + j) as usize].0.phase
                        == Phase::Solid
                    {
                        self.contents[cellpos].1.y = f32::abs((i - (i - _k)) as f32);
                        self.contents[((i + _k) * col_count + j) as usize].2 = false;
                    }
                }
                // We are generating a random number between 0 and 3 (1,2) these numbers correspond the side which
                // the powder particle falls
                let rnd: u8 = rand::gen_range(0, 3);
                // This checks if there is any obstruction to the left side, if not, then the particle falls to the left side
                if (i < row_count - 1
                    && j >= 0
                    && j < col_count - 1
                    && self.contents[cellpos].2
                    && (phase.get_coarseness() * 4.0) >= self.contents[cellpos].3)
                    && (self.contents[(i * col_count + j + 1) as usize].0.mass
                        < self.contents[cellpos].0.mass
                        && self.contents[(i * col_count + j + 1) as usize].0.phase != Phase::Solid
                        && self.contents[((i + 1) * col_count + j + 1) as usize].0.mass
                            < self.contents[cellpos].0.mass
                        && rnd == 1)
                {
                    self.contents
                        .swap(cellpos, ((i * col_count) + (j + 1)) as usize);
                }
                // This checks if there is any obstruction to the right side, if not, then the particle falls to the left side
                if (i < row_count - 1
                    && j < col_count
                    && j > 0
                    && self.contents[cellpos].2
                    && (phase.get_coarseness() * 4.0) >= self.contents[cellpos].3)
                    && (self.contents[(i * col_count + j - 1) as usize].0.mass
                        < self.contents[cellpos].0.mass
                        && self.contents[(i * col_count + j + 1) as usize].0.phase != Phase::Solid
                        && self.contents[((i + 1) * col_count + j - 1) as usize].0.mass
                            < self.contents[cellpos].0.mass
                        && rnd == 2)
                {
                    self.contents
                        .swap(cellpos, ((i * col_count) + (j - 1)) as usize)
                }
                // This marks that the particle's position has been calculated
                self.contents[cellpos].2 = true;
            }
            ///////////////////////////////////////////////////////////////////////////////////////////
            // LIQUID PHYSICS
            //////////////////////////////////////////////////////////////////////////////////////////
            Phase::Liquid { viscosity: _f32 } => {
                let cellpos: usize = (i * col_count + j) as usize;
                self.contents[cellpos].1.y += GRAVITY * frame_time;
                for _k in 0..(self.contents[cellpos].1.y + 1.0) as i32 {
                    if (i + _k) < (row_count)
                        && self.contents[cellpos].0.mass
                            > self.contents[((i + _k) * col_count + j) as usize].0.mass
                        && self.contents[cellpos].2
                    {
                        self.contents
                            .swap(cellpos, (((i + _k) * col_count) + j) as usize);
                        self.contents[((i + _k) * col_count + j) as usize].2 = false;
                    } else if (i + _k) >= (row_count) {
                        self.contents[cellpos].1.y = f32::abs((i - (row_count - 1)) as f32);
                    } else if self.contents[((i + _k) * col_count + j) as usize].0.phase
                        == Phase::Solid
                    {
                        self.contents[cellpos].1.y = f32::abs((i - (i - _k)) as f32);
                        self.contents[((i + _k) * col_count + j) as usize].2 = false;
                    }
                }
                let rnd: i32 = rand::gen_range(-(2.3 * col_count as f32) as i32, col_count);
                let rnd: i32 = rand::gen_range(-(2.3 * col_count as f32) as i32, col_count);
                self.contents[cellpos].1.x = rnd as f32 * (1.0 / phase.get_viscosity());
                for _k in 0..f32::abs(self.contents[cellpos].1.x) as i32 {
                    if j + (rnd.signum() * _k) < col_count && j + (rnd.signum() * _k) > -1 {
                        if (j + rnd.signum() * _k) < col_count
                            && (j + rnd.signum() * _k) > -1
                            && self.contents[(i * col_count + j + (rnd.signum() * _k)) as usize]
                                .0
                                .phase
                                == Phase::Void
                            && self.contents[(i * col_count + j + (rnd.signum() * _k)) as usize]
                                .0
                                .mass
                                <= self.contents[cellpos].0.mass
                            && self.contents[cellpos].2
                        {
                            self.contents
                                .swap(cellpos, (i * col_count + j + (rnd.signum() * _k)) as usize);
                            self.contents[(i * col_count + j + (rnd.signum() * _k)) as usize].2 =
                                false;
                        } else if (self.contents
                            [(i * col_count + j + (rnd.signum() * _k)) as usize]
                            .0
                            .mass
                            >= self.contents[cellpos].0.mass
                            && std::mem::discriminant(
                                &self.contents[(i * col_count + j + (rnd.signum() * _k)) as usize]
                                    .0
                                    .phase,
                            ) == std::mem::discriminant(&Phase::Powder { coarseness: 1.0 }))
                            || (self.contents[(i * col_count + j + (rnd.signum() * _k)) as usize]
                                .0
                                .phase
                                == Phase::Solid)
                        {
                            break;
                        }
                    }
                }
                self.contents[cellpos].2 = true;
            }

            Phase::Gas { viscosity: _f32 } => {
                let cellpos: usize = (i * col_count + j) as usize;
                let orientation: i32 = rand::gen_range(-2, 2);
                let mut rnd: i32 = rand::gen_range(-row_count, row_count);
                self.contents[cellpos].1.y = rnd as f32 * (1.0 / phase.get_viscosity());
                rnd = rand::gen_range(-col_count, col_count);
                self.contents[cellpos].1.x = rnd as f32 * (1.0 / phase.get_viscosity());
                if orientation == -1 {
                    for _k in 0..f32::abs(self.contents[cellpos].1.y) as i32 {
                        if i + (rnd.signum() * _k) < row_count && i + (rnd.signum() * _k) > -1 {
                            if (i + rnd.signum() * _k) < row_count
                                && (i + rnd.signum() * _k) > -1
                                && self.contents
                                    [((i + (rnd.signum() * _k)) * col_count + j) as usize]
                                    .0
                                    .phase
                                    == Phase::Void
                                && self.contents
                                    [((i + (rnd.signum() * _k)) * col_count + j) as usize]
                                    .0
                                    .mass
                                    <= self.contents[cellpos].0.mass
                                && self.contents[cellpos].2
                            {
                                self.contents.swap(
                                    cellpos,
                                    ((i + (rnd.signum() * _k)) * col_count + j) as usize,
                                );
                                self.contents
                                    [((i + (rnd.signum() * _k)) * col_count + j) as usize]
                                    .2 = false;
                            } else if (self.contents
                                [((i + (rnd.signum() * _k)) * col_count + j) as usize]
                                .0
                                .mass
                                >= self.contents[cellpos].0.mass
                                && std::mem::discriminant(
                                    &self.contents[((i + (rnd.signum() as i32 * _k)) * col_count
                                        + j)
                                        as usize]
                                        .0
                                        .phase,
                                ) == std::mem::discriminant(&Phase::Liquid { viscosity: 1.0 }))
                                || (self.contents
                                    [((i + (rnd.signum() * _k)) * col_count + j) as usize]
                                    .0
                                    .mass
                                    >= self.contents[cellpos].0.mass
                                    && std::mem::discriminant(
                                        &self.contents[((i + (rnd.signum() as i32 * _k))
                                            * col_count
                                            + j)
                                            as usize]
                                            .0
                                            .phase,
                                    ) == std::mem::discriminant(&Phase::Powder {
                                        coarseness: 1.0,
                                    }))
                                || (self.contents
                                    [((i + (rnd.signum() * _k)) * col_count + j) as usize]
                                    .0
                                    .phase
                                    == Phase::Solid)
                            {
                                break;
                            }
                        }
                    }
                } else if orientation == 1 {
                    for _k in 0..f32::abs(self.contents[cellpos].1.x) as i32 {
                        if j + (rnd.signum() * _k) < col_count && j + (rnd.signum() * _k) > -1 {
                            if (j + rnd.signum() * _k) < col_count
                                && (j + rnd.signum() * _k) > -1
                                && self.contents[(i * col_count + j + (rnd.signum() * _k)) as usize]
                                    .0
                                    .phase
                                    == Phase::Void
                                && self.contents[(i * col_count + j + (rnd.signum() * _k)) as usize]
                                    .0
                                    .mass
                                    <= self.contents[cellpos].0.mass
                                && self.contents[cellpos].2
                            {
                                self.contents.swap(
                                    cellpos,
                                    (i * col_count + j + (rnd.signum() * _k)) as usize,
                                );
                                self.contents[(i * col_count + j + (rnd.signum() * _k)) as usize]
                                    .2 = false;
                            } else if (self.contents
                                [(i * col_count + j + (rnd.signum() * _k)) as usize]
                                .0
                                .mass
                                >= self.contents[cellpos].0.mass
                                && std::mem::discriminant(
                                    &self.contents
                                        [(i * col_count + j + (rnd.signum() * _k)) as usize]
                                        .0
                                        .phase,
                                ) == std::mem::discriminant(&Phase::Liquid { viscosity: 1.0 }))
                                || (self.contents
                                    [(i * col_count + j + (rnd.signum() * _k)) as usize]
                                    .0
                                    .mass
                                    >= self.contents[cellpos].0.mass
                                    && std::mem::discriminant(
                                        &self.contents
                                            [(i * col_count + j + (rnd.signum() * _k)) as usize]
                                            .0
                                            .phase,
                                    ) == std::mem::discriminant(&Phase::Powder {
                                        coarseness: 1.0,
                                    }))
                                || (self.contents
                                    [(i * col_count + j + (rnd.signum() * _k)) as usize]
                                    .0
                                    .phase
                                    == Phase::Solid)
                            {
                                break;
                            }
                        }
                    }
                }
                self.contents[cellpos].2 = true;
            }

            Phase::Plasma { viscosity: _f32 } => {}
        }
    }
}
