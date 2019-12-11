use std::fs::read_to_string;

////////////////////////////////////////
/// PART 1
////////////////////////////////////////

fn layers(image: String, n_pixels: usize) -> Vec<[usize; 3]> {
	image
		.as_bytes()
		.chunks(n_pixels)
		.map(|pixels| {
			let mut counts = [0, 0, 0];
			for pixel in pixels {
				match pixel {
					48 => counts[0] += 1, // '0'
					49 => counts[1] += 1, // '1'
					50 => counts[2] += 1, // '2'
					c => panic!("Unrecognized character: {}", c),
				}
			}
			counts
		})
		.collect()
}

fn count(layers: Vec<[usize; 3]>) -> usize {
	let min_zero_count = layers.iter().min_by_key(|counts| counts[0]).unwrap();
	min_zero_count[1] * min_zero_count[2]
}

////////////////////////////////////////
/// PART 2
////////////////////////////////////////

fn render(image: String, n_pixels: usize) -> Vec<char> {
	image
		.as_bytes()
		.chunks(n_pixels)
		.fold(vec![' '; n_pixels], |display, layer| {
			(0..n_pixels)
				.map(|i| {
					if display[i] == ' ' {
						match layer[i] {
							48 => '░',
							49 => '█',
							50 => ' ',
							c => panic!("Unrecognized character: {}", c),
						}
					}
					else {
						display[i]
					}
				})
				.collect()
		})
}

////////////////////////////////////////
/// MAIN
////////////////////////////////////////

pub fn main() {
	let input = read_to_string("input/day8/input1.txt").unwrap();

	let n = count(layers(input.clone(), 25 * 6));
	println!("PART 1 -> Image: Layer with fewer 0: #1 * #2 = {}", n);
	println!("PART 2 -> Rendered image:");

	const WIDTH: usize = 25;
	const HEIGHT: usize = 6;
	let rendered = render(input, WIDTH * HEIGHT);
	for (i, pixel) in rendered.iter().enumerate() {
		if i % WIDTH == WIDTH - 1 {
			print!("{}\n", pixel);
		}
		else {
			print!("{}", pixel);
		}
	}
}

////////////////////////////////////////
/// TESTS
////////////////////////////////////////

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day8_test1() {
		assert_eq!(
			render("0222112222120000".to_string(), 16),
			vec!['░', ' ', ' ', ' ', '█', '█', ' ', ' ', ' ', ' ', '█', ' ', '░', '░', '░', '░']
		)
	}
}
