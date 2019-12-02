use std::fs::read_to_string;

fn fuel(mass: u64) -> u64 {
	(mass / 3).checked_sub(2).unwrap_or(0)
}

fn recursive_fuel(mass: u64) -> u64 {
	let fuel = fuel(mass);
	if fuel == 0 {
		fuel
	}
	else {
		fuel + recursive_fuel(fuel)
	}
}

pub fn main() {
	let input = read_to_string("input/day1/input1.txt").unwrap();

	let mut fuel_sum = 0;
	let mut recursive_fuel_sum = 0;

	for line in input.lines() {
		let num: u64 = line.parse().unwrap();
		fuel_sum += fuel(num);
		recursive_fuel_sum += recursive_fuel(num);
	}

	println!("PART 1 -> necessary fuel: {}", fuel_sum);
	println!("PART 2 -> recursive fuel: {}", recursive_fuel_sum);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day1_test1() {
		assert_eq!(2, fuel(12));
	}

	#[test]
	fn day1_test2() {
		assert_eq!(2, fuel(14));
	}

	#[test]
	fn day1_test3() {
		assert_eq!(654, fuel(1969));
	}

	#[test]
	fn day1_test4() {
		assert_eq!(33583, fuel(100_756));
	}

	#[test]
	fn day1_test5() {
		assert_eq!(2, recursive_fuel(14));
	}

	#[test]
	fn day1_test6() {
		assert_eq!(966, recursive_fuel(1969));
	}

	#[test]
	fn day1_test7() {
		assert_eq!(50346, recursive_fuel(100_756));
	}
}
