use std::{ thread::sleep, time::Duration};
use macroquad::prelude::*;

#[macroquad::main("Particle Simulator")]

async fn main() {
	let h2o = particle_create();
   	loop
	{
		let frametime = Duration::new(0, 1000000000/FPS);
		draw(h2o);
		sleep(frametime);
		next_frame().await;
   	}
}

fn draw(mut pticle: Particle)
{
	clear_background(WHITE);
	if (is_mouse_button_down(MouseButton::Left))
	{
		pticle.position= mouse_position();
		
	}
	draw_circle(pticle.position.0, pticle.position.1, pticle.radius, pticle.color);
} 

const GRAVITY:f32 = 9.81;
const FPS:u32 = 60;

fn particle_create() -> Particle
{
	let waterparticle = Particle
	{
		position: mouse_position(),
		delta_v_x: 0.0,
		delta_v_y: 0.0,
		mass: 10.0,
		viscosity: 1.0,
		color: color_u8!(0,0,255,120),
		radius: 5.0,
		temperature: 20.0,
	};
	return waterparticle;
}

#[derive(Copy, Clone)]
struct Particle 
{
	position: (f32, f32),
	delta_v_x: f32,
	delta_v_y: f32,

	mass: f32,
	viscosity: f32,
	color: Color,
	radius: f32,
	temperature: f32,

}