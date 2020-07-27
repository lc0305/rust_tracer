mod helper;
use helper::*;
mod vector;
use vector::Vector3D;
mod objects;
use objects::*;
mod raytracer;
use raytracer::*;
use std::sync::Arc;

const CPU_THREADS: usize = 16;

pub fn render_example_scene0() {
	let path = String::from("scene0.png");
	let mut scene_arc = Scene::new();
	let mut scene = Arc::get_mut(&mut scene_arc).unwrap();
	scene.add_object(Sphere::new(Vector3D::new(0.75, 0.1, 1.), 0.6, rgb(25, 25, 25), 0.95, 0.95, 0.95));
	scene.add_object(Sphere::new(Vector3D::new(-0.75, 0.1, 2.25), 0.6, rgb(139, 0, 139), 0.03, 0.95, 0.4));
	scene.add_object(Sphere::new(Vector3D::new(3.75, 0.1, 4.0), 0.6, rgb(32, 178, 170), 0.03, 0.95, 0.4));
	scene.add_object(Sphere::new(Vector3D::new(-2.75, 0.1, 3.5), 0.6, rgb(218, 165, 32), 0.95, 0.95, 0.95));
	scene.add_object(Checkerboard::new(Vector3D::new(0., -0.5, 0.), Vector3D::new(0., 1., 0.), 0.25, 0.75, 0.5, (rgb(0, 0, 0), rgb(255, 255, 255))));
	RayTracer::default().render_image(scene_arc.clone(), 1920 * 4, 1080 * 4, &path, CPU_THREADS);
}

pub fn render_example_scene1() {
	let path = String::from("scene1.png");
	let mut scene_arc = Scene::new();
	let mut scene = Arc::get_mut(&mut scene_arc).unwrap();
	scene.add_object(Sphere::new(Vector3D::new(0.75, 0.1, 1.), 0.6, rgb(77, 238, 234), 0.95, 0.95, 0.95));
	scene.add_object(Sphere::new(Vector3D::new(-0.75, 0.1, 2.25), 0.6, rgb(116, 238, 21), 0.04, 0.95, 0.4));
	scene.add_object(Sphere::new(Vector3D::new(3.75, 0.1, 4.0), 0.6, rgb(255, 231, 0), 0.04, 0.95, 0.4));
	scene.add_object(Sphere::new(Vector3D::new(-2.75, 0.1, 3.5), 0.6, rgb(240, 0, 255), 0.95, 0.95, 0.95));
	scene.add_object(Checkerboard::new(Vector3D::new(0., -0.5, 0.), Vector3D::new(0., 1., 0.), 0.25, 0.75, 0.5, (rgb(240, 0, 255), rgb(0, 30, 255))));
	RayTracer::new(
		0.1, 
		1.,
		1.,
		50,
		8,
		Camera::default(), 
		Light::new(rgb(210, 200, 50), Vector3D::new(5., 5., -10.)),
	)
		.render_image(scene_arc.clone(), 1920 * 4, 1080 * 4, &path, CPU_THREADS);
}

fn main() {
	render_example_scene0();
	render_example_scene1();
}
  