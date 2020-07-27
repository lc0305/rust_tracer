extern crate ndarray;
use ndarray::*;

extern crate image;
use image::{ImageBuffer, Rgb};

extern crate itertools;
use itertools::Itertools;

extern crate crossbeam;

use std::default::Default;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use super::helper::*;
use super::objects::{Scene};
use super::vector::Vector3D;

struct ScreenCoordinates {
	x0: f32,
	y0: f32,
	x1: f32,
	y1: f32,
}

impl ScreenCoordinates {
	fn from_ratio(ratio: f32) -> Self {
		Self {
			x0: -1.,
			y0: -1. / ratio + 0.25,
			x1: 1.,
			y1: 1. / ratio + 0.25,
		}
	}
}

pub struct Camera {
	position: Array1<f32>,
	pointing_to: Array1<f32>,
}

impl Camera {
	pub fn new(position: Vector3D, direction: Vector3D) -> Self {
		Self {
			position: position.to_ndarray(),
			pointing_to: direction.to_ndarray(),
		}
	}
}

impl Default for Camera {
	fn default() -> Self {
		Self {
			position: array![0., 0.35, -1.],
			pointing_to: array![0., 0., 0.],
		}
	}
}

pub struct Light {
	direction: Array1<f32>,
	color: Array1<f32>,
}

impl Light {
	pub fn new(color: Array1<f32>, direction: Vector3D) -> Self {
		Self {
			color,
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

	pub fn render_image(&self, scene: Arc<Scene>, width: usize, height: usize, path: &String, threads: usize) {
		let img = Arc::new(Mutex::new(ImageBuffer::new(width as u32, height as u32)));
		let img_scope = img.clone();
		let now = SystemTime::now();
		let scope_result = crossbeam::scope(move |scope| {
			let screen_coordinates = ScreenCoordinates::from_ratio(get_ratio(width, height));
			let y_coordinates = Array::linspace(screen_coordinates.y0, screen_coordinates.y1, height);
			let x_coordinates = Array::linspace(screen_coordinates.x0, screen_coordinates.x1, width);
			let x_coordinates_chunks = x_coordinates.iter().cloned().enumerate().chunks((width as f64 / threads as f64).ceil() as usize);

			for x_coordinates_chunk in x_coordinates_chunks.into_iter() {
				let img_thread = img_scope.clone();
				let y_coordinates_clone = y_coordinates.clone();
				let scene_clone = scene.clone();
				let x_coordinates_chunk_clone = x_coordinates_chunk.collect_vec();

				let handle = scope.spawn(move |_| {
					let mut current_color: Array1<f32> = array![0., 0., 0.];
					let mut camera_pointing_to: Array1<f32> = self.camera.pointing_to.clone();
					for (index_x, x) in x_coordinates_chunk_clone {
						for (index_y, y) in y_coordinates_clone.iter().enumerate() {
							for index in 0..3 {
								current_color[index] = 0.;
							}
							camera_pointing_to[0] = x;
							camera_pointing_to[1] = *y;
							let mut current_reflection: f32 = 1.;
							let mut ray_direction = normalize(&camera_pointing_to - &self.camera.position);
							let mut ray_origin = self.camera.position.clone();

							for current_depth in 0..self.depth_max {
								let traced = self.trace_ray(&scene_clone, &ray_origin, &ray_direction);
								if let Some((reflection, M, N, color_ray, is_occluded)) = traced {
									current_color = current_color + current_reflection * color_ray;
									if is_occluded {
										break;
									}
									ray_origin = M + &N * 0.0001;
									ray_direction = normalize(ray_direction.clone() - 2. * &ray_direction.dot(&N) * &N);
									current_reflection *= reflection;
								} else {
									break;
								}
							}
							let color = &current_color * 255.;
							let pixel_color = Rgb([color[0] as u8, color[1] as u8, color[2] as u8]);
							let y = (height - index_y) as u32 - 1;
							let x = index_x as u32;
							let mut img = img_thread.lock().unwrap();
							img.put_pixel(x, y, pixel_color);
						}
					}
				});
			}
		});
		if let Ok(scope) = scope_result {
			println!("Rendering took {} seconds.", now.elapsed().unwrap().as_secs());
			img.lock().unwrap().save(path).unwrap();
		}
	}

	fn trace_ray(&self, scene: &Arc<Scene>, ray_origin: &Array1<f32>, ray_direction: &Array1<f32>) -> Option<(f32, Array1<f32>, Array1<f32>, Array1<f32>, bool)> {
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
			return None; //ray doesnt hit anything
		}
		let object = scene.get_object_at_index(obj_idx);
		let M = ray_origin + &(ray_direction * t);

		let N = object.get_normal(&M);

		let to_L = normalize(&self.light.direction - &M);

		let s = M.clone() + &N * 0.0001;
		let l: Vec<f32> = scene
			.get_objects()
			.iter()
			.enumerate()
			.filter(|&(index, object)| index != obj_idx)
			.map(|(index, object)| object.intersect(&s, &to_L))
			.collect();

		let color_obj = object.get_color(&M);
		let color_ray_ambient = self.ambient * color_obj;

		if !l.is_empty() && min(&l) < f32::INFINITY {
			return Some((object.get_reflection(), M, N, color_ray_ambient, true)); //is occluded
		}

		let to_O = normalize(&self.camera.position - &M);

		let color_ray = color_ray_ambient
			+ object.get_diffuse_c() * f32::max(N.dot(&to_L), 0.) * color_obj * &self.light.color
			+ object.get_specular_c()
				* f32::powi(
					f32::max(N.dot(&normalize(to_L + to_O)), 0.),
					self.specular_phong_exponent,
				) * &self.light.color;
		return Some((object.get_reflection(), M, N, color_ray, false));
	}
}
