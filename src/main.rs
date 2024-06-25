use macroquad::{prelude::*};

#[macroquad::main("Particle Simulator")]

async fn main() {
   	loop
	{
		clear_background(BLUE);
		for i in 1..=(screen_height() as u32/CELLSIZE as u32) as i32 {
			for j in 1..=(screen_width() as u32/CELLSIZE as u32) as i32  {
			}
		}
		next_frame().await;
   	}
}

const CELLSIZE:u32 = 10;

const GRAVITY:f32 = 9.81;
const FPS:u32 = 60;

static WATER:Particle = Particle
{
	material_id: 1,
	mass: 1.0,
	viscosity: 1.0,
	color: BLUE,
};

#[derive(Copy, Clone)]
struct Particle 
{
	material_id: u16, // the identification number that helps identifying the material in the cell array
	mass: f32, // mass of a cm^3 volume of the material
	viscosity: f32, // viscosity of the material -> higher number = thicker material (viscosity of water is 1)
	color: Color, // color of the material
}