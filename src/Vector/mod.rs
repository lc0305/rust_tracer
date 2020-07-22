extern crate ndarray;
use ndarray::*;

pub struct Vector3D {
	x: f32,
	y: f32,
	z: f32,
}

impl Vector3D {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}

	pub fn to_ndarray(&self) -> Array1<f32> {
		array![self.x, self.y, self.z]
	}
}