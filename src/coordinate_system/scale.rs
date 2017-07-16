#[derive(Copy, Clone)]
pub enum ScaleType {
	Linear,
	Logarithmic
}

impl Default for ScaleType {
	fn default() -> ScaleType { ScaleType::Linear }
}

#[derive(Clone)]
pub struct Scale {
	label: String,
	ty: ScaleType,
	min: f32,
	max: f32
}

impl Scale where {
	pub fn new(label: &str, min: f32, max: f32, ty: ScaleType) -> Scale {
		Scale {
			label: label.to_string(),
			ty: ty,
			min: min,
			max: max
		}
	}


	/// Get the point where zero is on this scale.
	pub fn zero(&self, size: f32) -> f32 {
		match self.ty {
			ScaleType::Linear => self.zero_linear(size),
			ScaleType::Logarithmic => self.zero_log(size)
		}
	}

	fn zero_linear(&self, size: f32) -> f32 {
		if self.min >= 0. {
			return 0.
		}

		let range = self.max - self.min;
		assert!(range > 0.);

		-self.min*size/range
	}

	fn zero_log(&self, size: f32) -> f32 {
		unimplemented!();
	}

	/// Project the value on the scale. Size is the size of the scale on the
	/// Real window. The projection is from the absolute value to the relative
	/// of the scale. This is not yet the final position.
	pub fn project(&self, value: f32, size: f32) -> f32 {
		match self.ty {
			ScaleType::Linear => self.project_linear(value, size),
			ScaleType::Logarithmic => self.project_log(value, size)
		}
	}

	fn project_linear(&self, value: f32, size: f32) -> f32 {
		  (value - self.min) * size
		/ (self.max - self.min)
	}

	fn project_log(&self, value: f32, size: f32) -> f32 {
		unimplemented!();
	}
}
