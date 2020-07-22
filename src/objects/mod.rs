extern crate ndarray;
use ndarray::*;
use super::helper::*;
use super::Vector::Vector3D;

pub struct Scene {
	objects: Vec<Box<dyn SolidObject>>,
}

impl Scene {

	pub fn new() -> Box<Self> {
		Box::new(Self { objects: Vec::new() })
	}

	pub fn add_object(&mut self, object: Box<dyn SolidObject>) {
		self.objects.push(object)
	}

	pub fn get_objects(&self) -> &Vec<Box<dyn SolidObject>> {
		&self.objects
	}

	pub fn get_object_at_index(&self, index: usize) -> &Box<dyn SolidObject> {
		&self.objects[index]
	}
}
    
pub struct Sphere {
	position: Array1<f32>, 
	radius: f32, 
	color: Array1<f32>, 
	reflection: f32,
	diffuse_c: f32, 
	specular_c: f32,
}

pub trait SolidObject {
	fn intersect(&self, ray_origin: &Array1<f32>, ray_direction: &Array1<f32>) -> f32;
	fn get_color(&self, intersection: &Array1<f32>) -> &Array1<f32>;
	fn get_normal(&self, intersection: &Array1<f32>) -> Array1<f32>;
	fn get_diffuse_c(&self) -> f32;
	fn get_specular_c(&self) -> f32;
	fn get_reflection(&self) -> f32;
}

impl Sphere {
	pub fn new(position: Vector3D, radius: f32, color: Array1<f32>, reflection: f32, diffuse_c: f32, specular_c: f32) -> Box<Self> {
		Box::new(Self { 	
			position: position.to_ndarray(), 
			radius, 
			color, 
			reflection,
			diffuse_c, 
			specular_c, 
		})
	}
}

impl SolidObject for Sphere {
	fn intersect(&self, ray_origin: &Array1<f32>, ray_direction: &Array1<f32>) -> f32 {
		let a = ray_direction.dot(ray_direction);
		let position_origin_vec = ray_origin - &self.position;
		let b = 2. * ray_direction.dot(&position_origin_vec);
		let c = position_origin_vec.dot(&position_origin_vec) - self.radius.powi(2);
		let disc = b.powi(2) - 4. * a * c;
		if disc > 0. {
			let dist_sqrt = disc.sqrt();
			let q = if b < 0. {
				(-b - dist_sqrt) / 2.0
			} else {
				(-b + dist_sqrt) / 2.0
			};
			let mut t0 = q / a;
			let mut t1 = c / q;
			t0 = f32::min(t0, t1);
			t1 = f32::max(t0, t1);
			if t1 >= 0. {
				return if t0 < 0. {
					t1
				} else {
					t0
				};
			}
		}
		return f32::INFINITY;
	}

	fn get_color(&self, intersection: &Array1<f32>) -> &Array1<f32> {
		&self.color
	}

	fn get_normal(&self, intersection: &Array1<f32>) -> Array1<f32> {
		normalize(intersection - &self.position)
	}

	fn get_diffuse_c(&self) -> f32 {
		self.diffuse_c
	}

	fn get_specular_c(&self) -> f32 {
		self.specular_c
	}

	fn get_reflection(&self) -> f32 {
		self.reflection
	}
}

pub struct Checkerboard {
	postion: Array1<f32>,
	normal: Array1<f32>,
	reflection: f32,
	diffuse_c: f32, 
	specular_c: f32,
	white: Array1<f32>,
	black: Array1<f32>,
}

impl Checkerboard {
	pub fn new(postion: Vector3D, normal: Vector3D, reflection: f32, diffuse_c: f32, specular_c: f32, colors: (Array1::<f32>, Array1::<f32>)) -> Box<Self> {
		Box::new(Checkerboard { 	
			postion: postion.to_ndarray(),
			normal: normal.to_ndarray(),
			reflection,
			diffuse_c, 
			specular_c,
			white: colors.1,
			black: colors.0,
		})
	}
}

impl SolidObject for Checkerboard {

	fn get_color(&self, intersection: &Array1<f32>) -> &Array1<f32> {
		let intersec_x = (intersection[0] * 2.) as i32 as u32;
		let intersec_z = (intersection[2] * 2.) as i32 as u32;
		if ((intersec_x + intersec_z) % 2) == 1 {
			&self.white
		} else {
			&self.black
		}
	}

	fn intersect(&self, ray_origin: &Array1<f32>, ray_direction: &Array1<f32>) -> f32 {
		let denom = ray_direction.dot(&self.normal);
		if denom.abs() < 1e-6 {
			return f32::INFINITY;
		}
		let d = (&self.postion - ray_origin).dot(&self.normal) / denom;
		return if d < 0. {
			f32::INFINITY
		} else {
			d
		};
	}

	fn get_normal(&self, intersection: &Array1<f32>) -> Array1<f32> {
		self.normal.clone()
	}

	fn get_diffuse_c(&self) -> f32 {
		self.diffuse_c
	}

	fn get_specular_c(&self) -> f32 {
		self.specular_c
	}

	fn get_reflection(&self) -> f32 {
		self.reflection
	}
}