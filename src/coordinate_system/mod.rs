pub mod scale;
pub mod trace;

pub use self::scale::*;
pub use self::trace::*;

use sfml::graphics::{Color, RenderWindow, RenderTarget};
use sfml::window::{ContextSettings, Event, style, VideoMode};

#[derive(Clone)]
pub struct CoordinateSystem {
	title: String,
	x: Scale,
	y: Scale,
	traces: Vec<Trace>
}

impl CoordinateSystem {
	/// Create a new coordinate system.
	/// x_scale represents the x-coordinates, y_scale the y-coordinates.
	pub fn new(x_scale: Scale, y_scale: Scale, title: &str) -> CoordinateSystem {
		CoordinateSystem {
			title: title.to_string(),
			x: x_scale,
			y: y_scale,
			traces: Vec::new()
		}
	}

	/// Adds the trace to the coordinate system and returns the id of the trace
	/// in the caller wants to edit the trace later.
	pub fn add_trace(&mut self, trace: Trace) -> usize {
		self.traces.push(trace);

		self.traces.len() - 1
	}

	/// Get the trace with the id provided. If the trace does not exist, this
	/// function returns None. If you want to edit the trace, use get_trace_mut
	pub fn get_trace(&self, id: usize) -> Option<&Trace> {
		self.traces.get(id)
	}

	/// Get the trace with the id provided. If the trace does not exist, this
	/// function returns None. The Trace can be edited using this function.
	pub fn get_trace_mut(&mut self, id: usize) -> Option<&mut Trace> {
		self.traces.get_mut(id)
	}

	/// Draws the plot for the coordinate system. This is a blocking call, which
	/// returns after the user has closed the plot.
	pub fn draw_plot(&self, w: u32, h: u32) {
		let mut window = match RenderWindow::new(VideoMode::new(w, h, 32), &self.title, style::CLOSE, &ContextSettings::default()) {
			Some(window) => window,
			None => { println!("Unable to draw plot. Could not create window."); return; }
		};

		window.set_framerate_limit(10);
		'l: loop {
			for e in window.events() {
				if let Event::Closed = e {
					break 'l;
				}
			}

			window.clear(&Color::rgb(100, 100, 130));

			// Draw the available traces
			for ref t in &self.traces {
				t.draw(&mut window, &self.x, &self.y, w as f32, h as f32);
			}

			window.display();
		}
	}
}
