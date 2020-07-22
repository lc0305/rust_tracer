extern crate ndarray;
use ndarray::*;
extern crate image;
use image::{ImageBuffer, RgbImage, Rgb};
use std::time::{SystemTime};
use std::default::Default;

use super::helper::*;
use super::objects::{Scene, SolidObject};
use super::Vector::Vector3D;

struct ScreenCoordinates {
	x0: f32,
	y0: f32,
	x1: f32,
	y1: f32,
}

impl ScreenCoordinates {
	fn fromRatio(ratio: f32) -> Self {
		Self { x0: -1., y0: -1. / ratio + 0.25, x1: 1., y1: 1. / ratio + 0.25 }
	}
}

pub struct Camera {
	position: Array1::<f32>,
	direction: Array1::<f32>,
}

impl Camera {
	pub fn new(position: Vector3D, direction: Vector3D) -> Self {
		Self {
			position: position.to_ndarray(), 
			direction: direction.to_ndarray(),
		}
	}
}

impl Default for Camera {
	fn default() -> Self {
		Self { 
			position: array![0., 0.35, -1.], 
			direction: array![0., 0., 0.],
		}
	}
}

pub struct Light {
	direction: Array1::<f32>,
	color: Array1::<f32>,
}

impl Light {
	pub fn new(color: Array1::<f32>, direction: Vector3D) -> Self {
		Self {
			color: color, 
			direction: direction.to_ndarray(),
		}
	}
}

impl Default for Light {
	fn default() -> Self {
		Self {
			direction: array![5., 5., -10.],
			color: array![1., 1., 1.],
		}
	}
}

pub struct RayTracer {
	ambient: f32,
	def_diffuse_c: f32,
	def_specular_c: f32,
	specular_phong_exponent: i32,
	depth_max: u32,
	camera: Camera,
	light: Light,
}

impl Default for RayTracer {
	fn default() -> Self {
		Self { 
			ambient: 0.05, 
			def_diffuse_c: 1., 
			def_specular_c: 1., 
			specular_phong_exponent: 50, 
			depth_max: 8, 
			camera: Camera::default(), 
			light: Light::default(),
		}
	}
}

impl RayTracer {

	pub fn new(ambient: f32, def_diffuse_c: f32, def_specular_c: f32, specular_phong_exponent: i32, depth_max: u32, camera: Camera, light: Light) -> Self {
		Self { 
			ambient, 
			def_diffuse_c, 
			def_specular_c, 
			specular_phong_exponent, 
			depth_max, 
			camera, 
			light,
		}
	}

	pub fn render_image(&self, scene: &Box<Scene>, width: u32, height: u32, path: &String) {
		let mut current_color: Array1<f32> = array![0., 0., 0.];
		let mut camera_pointing_to: Array1<f32> = self.camera.direction.clone();
		let mut img: RgbImage = ImageBuffer::new(width, height);
		let screen_coordinates = ScreenCoordinates::fromRatio(get_ratio(width, height));
		
		let now = SystemTime::now();
	
		for (index_x, x) in Array::linspace(screen_coordinates.x0, screen_coordinates.x1, width as usize).iter().enumerate() {
			for (index_y, y) in Array::linspace(screen_coordinates.y0, screen_coordinates.y1, height as usize).iter().enumerate() {
				current_color[0] = 0.;
				current_color[1] = 0.;
				current_color[2] = 0.;
				camera_pointing_to[0] = x.clone();
				camera_pointing_to[1] = y.clone();
				let mut current_reflection: f32 = 1.;
				let mut ray_direction = normalize(&camera_pointing_to - &self.camera.position);
				let mut ray_origin = self.camera.position.clone();

				for current_depth in 0..self.depth_max {
					let traced = self.trace_ray(scene, &ray_origin, &ray_direction);
					if let Some((reflection, M, N, color_ray)) = traced {
						ray_origin = M + &N * 0.0001;
						ray_direction = normalize(ray_direction.clone() - 2. * &ray_direction.dot(&N) * &N);
						current_color = current_color + current_reflection * color_ray;
						current_reflection *= reflection; 
					} else {
						break;
					}
				}
				let color = &current_color * 255.;
				img.put_pixel(index_x as u32, height - index_y as u32 - 1, Rgb([color[0] as u8, color[1] as u8, color[2] as u8]));
			}
		}
		println!("Rendering took {} seconds.", now.elapsed().unwrap().as_secs());
		img.save(path).unwrap();
	}

	fn trace_ray(&self, scene: &Box<Scene>, ray_origin: &Array1<f32>, ray_direction: &Array1<f32>) -> Option<(f32, Array1::<f32>, Array1::<f32>, Array1::<f32>)> {
		let mut t = f32::INFINITY;
		let mut obj_idx: usize = 0;
		for (index, object) in scene.get_objects().iter().enumerate() {
			let t_obj = object.intersect(ray_origin, ray_direction);
			if t_obj < t {
				t = t_obj;
				obj_idx = index;
			}
		}
		if t == f32::INFINITY {
			return None;
		}
		let object = scene.get_object_at_index(obj_idx);
		
		let M = ray_origin + &(ray_direction * t);

		let N = object.get_normal(&M);

		let to_L = normalize(&self.light.direction - &M);
		let to_O = normalize(&self.camera.position - &M);

		let s = M.clone() + &N * 0.0001;
		let l: Vec<f32> = scene.get_objects().iter().enumerate()
			.filter(|&(index, object)| index != obj_idx)
			.map(|(index, object)| object.intersect(&s, &to_L))
			.collect();

		if !l.is_empty() {
			if min(&l) < f32::INFINITY {
				return None;
			}
		}
	
		let color_ray = 
			self.ambient + 
			object.get_diffuse_c() * f32::max(N.dot(&to_L), 0.) * object.get_color(&M) +
			object.get_specular_c() * f32::powi(f32::max(N.dot(&normalize(to_L + to_O)), 0.), self.specular_phong_exponent) * &self.light.color;
		
		return Some((object.get_reflection(), M, N, color_ray));
	}
}