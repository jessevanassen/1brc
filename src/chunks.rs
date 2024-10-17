use std::{io::Read, iter};

pub fn read_as_chunks(mut read: impl Read) -> impl Iterator<Item = Box<[u8]>> {
	const BUFFER_SIZE: usize = 1024 * 1024;

	let mut overflow = Vec::<u8>::new();

	iter::from_fn(move || {
		let mut buf = vec![0u8; BUFFER_SIZE];

		let start = if !overflow.is_empty() {
			let overflow_len = overflow.len();
			buf[..overflow_len].copy_from_slice(&overflow[..]);
			overflow.clear();
			overflow_len
		} else {
			0
		};

		let bytes_read = read
			.read(&mut buf[start..])
			.expect("Should be able to read from file");

		if bytes_read == 0 {
			return if start > 0 {
				buf.truncate(start);
				Some(buf.into_boxed_slice())
			} else {
				None
			};
		}

		let end = start + bytes_read;
		buf.truncate(end);

		if let Some(index) = buf.iter().rposition(|&b| b == b'\n') {
			overflow.extend_from_slice(&buf[(index + 1)..end]);
			buf.truncate(index);
		}

		Some(buf.into_boxed_slice())
	})
}
