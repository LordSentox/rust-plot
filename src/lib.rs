extern crate sfml;

pub mod coordinate_system;
pub use coordinate_system::*;

#[cfg(test)]
mod tests {
	use sfml::graphics::Color;
	use coordinate_system::*;
	use std::cmp;

    #[test]
    fn coordinate_system() {
		let x: Scale = Scale::new("time in s", 0., 7., ScaleType::Linear);
		let y: Scale = Scale::new("height in m", 0., 50., ScaleType::Linear);
		let mut plot = CoordinateSystem::new(x, y, "Ball throw.");

		// Create the trace for a Ball throw. f(t) = (1/2)*a*t^2+v0*t+s0
		// for the test it's
		// a = -9.81m/s^2
		// v0 = [0m/s, 20m/s]
		// s0 = [0m, 2m]
		let a: f32 = -9.81;
		let v0: f32 = 30.;
		let s0: f32 = 2.;

		let f = |t: f32| -> f32 {
			let f_t = (a/2.)*t*t+v0*t+s0;
			if f_t > 0. { f_t }
			else { 0. }
		};
		let mut trace = Trace { name: "Ball throw".to_string(), data: Vec::new(), colour: Color::white() };

		// Watch the ball for 7 seconds.
		let mut t: f32 = 0.;
		while t <= 7. {
			trace.data.push((t, f(t)));
			t += 0.01;
		}

		plot.add_trace(trace);
		plot.draw_plot(700, 700);
	}
}
