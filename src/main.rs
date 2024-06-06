use std::{ thread::sleep, time::Duration};

use macroquad::prelude::*;

#[macroquad::main("Particle Simulator")]

async fn main() {
	let mut h2o = particle_create();
   	loop
	{
		let frametime = Duration::new(0, 1000000000/FPS);
		sleep(frametime);
		next_frame().await;
   	}
}

fn collisions()
{	
}

fn draw(pticle: &Particle)
{
	clear_background(WHITE);
	draw_circle(pticle.pos_x as f32, pticle.pos_y as f32, pticle.radius, pticle.color);
} 

const GRAVITY:f32 = 9.81;
const FPS:u32 = 60;

fn particle_create() -> Particle
{
	let waterparticle = Particle
	{
		pos_x: 55.0,
		pos_y: 55.0,
		delta_v_x: 0.0,
		delta_v_y: 0.0,

		mass: 10.0,
		viscosity: 1.0,

		color: color_u8!(0,0,255,120),
		radius: 5.0,
	};
	return waterparticle;
}

#[derive(Copy, Clone)]
struct Particle 
{
	pos_x: f32,
	pos_y: f32,
	delta_v_x: f32,
	delta_v_y: f32,

	mass: f32,
	viscosity: f32,

	color: Color,
	radius: f32,
}