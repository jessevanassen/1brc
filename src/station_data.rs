use std::ops::AddAssign;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StationData {
	pub min: i16,
	pub max: i16,
	pub sum: i32,
	pub count: u32,
}

impl StationData {
	pub fn from_initial_measurement(measurement: i16) -> Self {
		Self {
			min: measurement,
			max: measurement,
			sum: measurement as _,
			count: 1,
		}
	}

	pub fn add_measurement(&mut self, measurement: i16) {
		self.min = self.min.min(measurement);
		self.max = self.max.max(measurement);
		self.sum += measurement as i32;
		self.count += 1;
	}
}

impl AddAssign for StationData {
	fn add_assign(&mut self, rhs: Self) {
		self.min = self.min.min(rhs.min);
		self.max = self.max.max(rhs.max);
		self.sum = self.sum + rhs.sum;
		self.count = self.count + rhs.count;
	}
}
