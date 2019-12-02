use std::fs::read_to_string;

fn calculator(mut code: Vec<u64>) -> Vec<u64> {
	let mut i = 0;
	loop {
		if code[i] == 99 {
			break;
		}

		let x1 = code[code[i + 1] as usize];
		let x2 = code[code[i + 2] as usize];
		let out = code[i + 3] as usize;

		code[out] = match code[i] {
			1 => x1 + x2,
			2 => x1 * x2,
			_ => unreachable!(),
		};

		i += 4;
	}
	code
}

fn find_noun_verb(code: Vec<u64>, value: u64) -> Option<(u64, u64)> {
	for noun in 0..100 {
		for verb in 0..100 {
			let mut new_code = code.clone();
			new_code[1] = noun;
			new_code[2] = verb;
			if calculator(new_code)[0] == value {
				return Some((noun, verb));
			}
		}
	}
	None
}

pub fn main() {
	let input = read_to_string("input/day2/input1.txt").unwrap();
	let code: Vec<u64> = input.split(',').map(|n| n.parse().unwrap()).collect();
	println!("PART 1 -> position 0: {}", calculator(code.clone())[0]);
	let nv = find_noun_verb(code.clone(), 19_690_720).unwrap();
	println!("PART 2 -> noun: {}, verb: {}, combined: {}", nv.0, nv.1, 100 * nv.0 + nv.1);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day2_test1() {
		assert_eq!(vec![2, 0, 0, 0, 99], calculator(vec![1, 0, 0, 0, 99]));
	}

	#[test]
	fn day2_test2() {
		assert_eq!(vec![2, 3, 0, 6, 99], calculator(vec![2, 3, 0, 3, 99]));
	}

	#[test]
	fn day2_test3() {
		assert_eq!(vec![2, 4, 4, 5, 99, 9801], calculator(vec![2, 4, 4, 5, 99, 0]));
	}

	#[test]
	fn day2_test4() {
		assert_eq!(
			vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
			calculator(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
		);
	}
}
