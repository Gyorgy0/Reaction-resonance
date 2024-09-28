use macroquad::color::*;
use macroquad::color_u8;

use crate::Phase;

/*#[derive(Copy, Clone)]
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
}*/

pub static METHANE: crate::Material = crate::Material {
    name: "Methane",
    mass: 0.657,
    phase: Phase::Gas { viscosity: 1.0 },
    durability: 50,
    flammability: 10.0,
    color: macroquad::color::YELLOW,
};
