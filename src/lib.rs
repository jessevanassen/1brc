mod chunks;
mod station_data;
mod measurement;

use chunks::read_as_chunks;
use measurement::parse_measurement;
use station_data::StationData;

use core::str;
use std::{
	collections::HashMap, fmt::Write as _, io::Read, sync::{mpsc, Arc, Mutex}, thread
};

type Measurements = HashMap<Box<[u8]>, StationData>;

pub fn run(read: impl Read) -> String {
	let available_threads = thread::available_parallelism().unwrap().get();

	let (tx, rx) = mpsc::sync_channel::<Box<[u8]>>(available_threads * 4);
	let rx = Arc::new(Mutex::new(rx));

	let join_handles = (0..available_threads)
		.map(|_| {
			let rx = rx.clone();
			thread::spawn(move || {
				let mut measurements = Measurements::new();

				loop {
					let Ok(chunk) = rx.lock().unwrap().recv() else {
						break;
					};
					collect_measurements(&mut measurements, chunk);
				}

				measurements
			})
		})
		.collect::<Vec<_>>();

	for chunk in read_as_chunks(read) {
		tx.send(chunk).unwrap();
	}

	drop(tx);

	let measurements = join_handles
		.into_iter()
		.map(|handle| handle.join().unwrap());
	let measurements = merge_measurements(measurements);

	return format_measurements(measurements);
}

fn collect_measurements(acc: &mut Measurements, chunk: Box<[u8]>) {
	for line in chunk.split(|&b| b == b'\n') {
		let (name, measurement) = parse_measurement(line);

		if let Some(existing_measurement) = acc.get_mut(name) {
			existing_measurement.add_measurement(measurement);
		} else {
			acc.insert(
				name.into(),
				StationData::from_initial_measurement(measurement),
			);
		}
	}
}

fn merge_measurements(iter: impl IntoIterator<Item = Measurements>) -> Measurements {
	iter.into_iter()
		.reduce(|mut acc, item| {
			for (k, v) in item.into_iter() {
				acc.entry(k)
					.and_modify(|measurement| *measurement += v)
					.or_insert(v);
			}

			acc
		})
		.expect("Expect at least one set of measurements")
}

fn format_measurements(measurements: HashMap<Box<[u8]>, StationData>) -> String {
	/// When formatting to decimal places, Rust by default rounds towards the
	/// even integer. This results in rounding errors in the output. This
	/// function rounds to the nearest integer.
	fn round_to_one_digit(n: f64) -> f64 {
		(n * 10.0).round() / 10.0
	}

	let mut result = String::from("{");
	let mut measurements = measurements.into_iter().collect::<Vec<_>>();
	measurements.sort_by(|(n1, _), (n2, _)| n1.cmp(n2));
	for (i, (name, data)) in measurements.into_iter().enumerate() {
		if i > 0 {
			result.push_str(", ");
		}

		let name = str::from_utf8(&name).expect("Expect name to be valid UTF-8");
		let min = data.min as f64 / 10.0;
		let mean = round_to_one_digit(data.sum as f64 / data.count as f64 / 10.0);
		let max = data.max as f64 / 10.0;
		write!(result, "{name}={min:.1}/{mean:.1}/{max:.1}").unwrap();
	}
	result.push('}');

	result
}
