fn two_equal(v: &[u32]) -> bool {
	v.iter().zip(v.iter().skip(1)).any(|(a, b)| a == b)
}

fn just_two_adjacent(v: &[u32]) -> bool {
	let mut vd = [0; 10];
	v.iter().for_each(|d| vd[*d as usize] += 1);
	vd.contains(&2)
}

fn count_passwords(min: u32, max: u32) -> (u32, u32) {
	let mut sum_part1 = 0;
	let mut sum_part2 = 0;

	for d1 in 0..10 {
		for d2 in d1..10 {
			for d3 in d2..10 {
				for d4 in d3..10 {
					for d5 in d4..10 {
						for d6 in d5..10 {
							if two_equal(&[d1, d2, d3, d4, d5, d6]) {
								let n = d1 * 100_000 + d2 * 10_000 + d3 * 1000 + d4 * 100 + d5 * 10 + d6;
								if min <= n && n <= max {
									sum_part1 += 1;
									if just_two_adjacent(&[d1, d2, d3, d4, d5, d6]) {
										sum_part2 += 1;
									}
								}
							}
						}
					}
				}
			}
		}
	}

	(sum_part1, sum_part2)
}

pub fn main() {
	let min = 272_091;
	let max = 815_432;
	let (sum1, sum2) = count_passwords(min, max);
	println!("PART 1 -> count from {} to {}: {:?}", min, max, sum1);
	println!("PART 2 -> count from {} to {}: {:?}", min, max, sum2);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day4_test1() {
		assert_eq!(1, count_passwords(111_111, 111_111).0);
	}

	#[test]
	fn day4_test2() {
		assert_eq!(0, count_passwords(223_450, 223_450).0);
	}

	#[test]
	fn day4_test3() {
		assert_eq!(0, count_passwords(123_789, 123_789).0);
	}

	#[test]
	fn day4_test4() {
		assert_eq!(1, count_passwords(112_233, 112_233).1);
	}

	#[test]
	fn day4_test5() {
		assert_eq!(0, count_passwords(123_444, 123_444).1);
	}

	#[test]
	fn day4_test6() {
		assert_eq!(1, count_passwords(111_122, 111_122).1);
	}
}
