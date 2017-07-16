use sfml::graphics::{Color, PrimitiveType, RenderStates, RenderTarget, Vertex};
use sfml::system::Vector2f;

use coordinate_system::scale::*;

#[derive(Clone)]
pub struct Trace {
	pub name: String,
	pub data: Vec<(f32, f32)>,
	pub colour: Color
}

impl Trace {
	pub fn draw(&self, target: &mut RenderTarget, x_scale: &Scale, y_scale: &Scale, width: f32, height: f32) {
		let mut vertices: Vec<Vertex> = Vec::with_capacity(self.data.len());
		for &(ref x, ref y) in &self.data {
			let x = x_scale.project(*x, width);
			// y has to be adjusted, since y-coordinates of the window go up in
			// the different direction from the scale.
			let y = height - y_scale.project(*y, height);

			vertices.push(Vertex::new(Vector2f::new(x, y), self.colour, Vector2f::new(0., 0.)));
		}

		target.draw_primitives(&vertices, PrimitiveType::LineStrip, RenderStates::default());
	}
}
