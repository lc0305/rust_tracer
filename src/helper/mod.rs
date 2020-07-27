extern crate ndarray;
use ndarray::*;
use console::Term;

#[inline]
pub fn clear_screen() {
	Term::stdout().clear_screen().unwrap();
}

#[inline]
pub fn get_ratio(width: usize, height: usize) -> f32 {
	width as f32 / height as f32
}

#[inline]
pub fn normalize(x: Array1::<f32>) -> Array1::<f32> {
	let norm = x.iter().fold(0.0f32, |accumulator, current_val| accumulator + current_val.powi(2)).sqrt();
	x / norm
}

#[inline]
pub fn rgb(red: u8, green: u8, blue: u8) -> Array1::<f32> {
	array![red as f32, green as f32, blue as f32] / 255.
}

#[inline]
pub fn min(vec: &Vec<f32>) -> f32 {
	vec.iter().cloned().fold(f32::NAN, f32::min)
}