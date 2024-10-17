pub fn parse_measurement(line: &[u8]) -> (&[u8], i16) {
	let mut parts = line.split(|&b| b == b';');

	let name = parts.next().expect("Expected station name part");
	let temperature = parts.next().expect("Expected temperature part");
	let temperature = parse_temperature(temperature);

	(name, temperature)
}

fn parse_temperature(temp: &[u8]) -> i16 {
	let (sign, temp) = if temp[0] == b'-' {
		(-1, &temp[1..])
	} else {
		(1, temp)
	};

	let number = temp
		.iter()
		.filter(|b| b.is_ascii_digit())
		.map(|&b| b - b'0')
		.fold(0i16, |acc, item| acc * 10 + item as i16);

	sign * number
}
