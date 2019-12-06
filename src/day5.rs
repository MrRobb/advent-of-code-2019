use std::fs::read_to_string;

#[derive(PartialEq)]
enum Operation {
	Add = 1,
	Mul = 2,
	Input = 3,
	Output = 4,
	JumpIfTrue = 5,
	JumpIfFalse = 6,
	LessThan = 7,
	Equals = 8,
	Halt = 99,
}

impl Operation {
	fn n_parameters(&self) -> usize {
		match self {
			Operation::Add => 3,
			Operation::Mul => 3,
			Operation::Input => 1,
			Operation::Output => 1,
			Operation::Halt => 0,
			Operation::JumpIfTrue => 2,
			Operation::JumpIfFalse => 2,
			Operation::LessThan => 3,
			Operation::Equals => 3,
		}
	}
}

impl From<i64> for Operation {
	fn from(n: i64) -> Self {
		match n {
			1 => Operation::Add,
			2 => Operation::Mul,
			3 => Operation::Input,
			4 => Operation::Output,
			5 => Operation::JumpIfTrue,
			6 => Operation::JumpIfFalse,
			7 => Operation::LessThan,
			8 => Operation::Equals,
			99 => Operation::Halt,
			_ => unimplemented!(),
		}
	}
}

#[derive(Debug)]
enum ParameterMode {
	Position = 0,
	Immediate = 1,
}

impl ParameterMode {
	fn get_parameter(&self, memory: &Vec<i64>, i: usize) -> i64 {
		match self {
			ParameterMode::Immediate => memory[i],
			ParameterMode::Position => memory[memory[i] as usize],
		}
	}
}

impl From<u32> for ParameterMode {
	fn from(n: u32) -> Self {
		match n {
			0 => ParameterMode::Position,
			1 => ParameterMode::Immediate,
			_ => unimplemented!(),
		}
	}
}

struct Instruction {
	opcode: Operation,
	parameter_modes: Vec<ParameterMode>,
}

impl Instruction {
	fn parse(opcode: i64) -> Self {
		let operation = Operation::from(opcode % 100);
		let par_modes = (0..operation.n_parameters())
			.map(|i| {
				let op_str = (opcode / 100).to_string();
				ParameterMode::from(
					op_str
						.as_str()
						.chars()
						.rev()
						.nth(i)
						.unwrap_or('0')
						.to_digit(10)
						.unwrap(),
				)
			})
			.collect();
		Self {
			opcode: operation,
			parameter_modes: par_modes,
		}
	}

	fn eval(&self, memory: &mut Vec<i64>, i: &mut usize, input: &mut Vec<i64>, output: &mut Vec<i64>) {
		let mut new_i = *i + self.opcode.n_parameters() + 1;
		match self.opcode {
			Operation::Add => {
				let x1 = self.parameter_modes[0].get_parameter(memory, *i + 1);
				let x2 = self.parameter_modes[1].get_parameter(memory, *i + 2);
				let out = memory[*i + 3] as usize;
				memory[out] = x1 + x2;
			},
			Operation::Mul => {
				let x1 = self.parameter_modes[0].get_parameter(memory, *i + 1);
				let x2 = self.parameter_modes[1].get_parameter(memory, *i + 2);
				let out = memory[*i + 3] as usize;
				memory[out] = x1 * x2;
			},
			Operation::Input => {
				let out = memory[*i + 1] as usize;
				memory[out] = input.remove(0);
			},
			Operation::Output => {
				let n = self.parameter_modes[0].get_parameter(memory, *i + 1);
				output.push(n);
			},
			Operation::JumpIfTrue => {
				let n = self.parameter_modes[0].get_parameter(memory, *i + 1);
				if n != 0 {
					new_i = self.parameter_modes[1].get_parameter(memory, *i + 2) as usize;
				}
			},
			Operation::JumpIfFalse => {
				let n = self.parameter_modes[0].get_parameter(memory, *i + 1);
				if n == 0 {
					new_i = self.parameter_modes[1].get_parameter(memory, *i + 2) as usize;
				}
			},
			Operation::LessThan => {
				let x1 = self.parameter_modes[0].get_parameter(memory, *i + 1);
				let x2 = self.parameter_modes[1].get_parameter(memory, *i + 2);
				let out = memory[*i + 3] as usize;
				memory[out] = if x1 < x2 { 1 } else { 0 };
			},
			Operation::Equals => {
				let x1 = self.parameter_modes[0].get_parameter(memory, *i + 1);
				let x2 = self.parameter_modes[1].get_parameter(memory, *i + 2);
				let out = memory[*i + 3] as usize;
				memory[out] = if x1 == x2 { 1 } else { 0 };
			},
			Operation::Halt => unreachable!(),
		}
		*i = new_i;
	}
}

fn calculator(mut code: Vec<i64>, mut input: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
	let mut i = 0;
	let mut output = Vec::new();
	loop {
		let ins = Instruction::parse(code[i]);
		if ins.opcode == Operation::Halt {
			break;
		}
		ins.eval(&mut code, &mut i, &mut input, &mut output);
	}
	(code, output)
}

pub fn main() {
	let code_str = read_to_string("input/day5/input1.txt").unwrap();
	let code: Vec<i64> = code_str.split(',').map(|n| n.parse().unwrap()).collect();

	let input = vec![1];
	let (_, output) = calculator(code.clone(), input.clone());
	println!("PART 1 -> Input: {:?}, Output: {:?}", input, output);

	let input = vec![5];
	let (_, output) = calculator(code.clone(), input.clone());
	println!("PART 2 -> Input: {:?}, Output: {:?}", input, output);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day5_test1() {
		assert_eq!(vec![2, 0, 0, 0, 99], calculator(vec![1, 0, 0, 0, 99], vec![]).0);
	}

	#[test]
	fn day5_test2() {
		assert_eq!(vec![2, 3, 0, 6, 99], calculator(vec![2, 3, 0, 3, 99], vec![]).0);
	}

	#[test]
	fn day5_test3() {
		assert_eq!(
			vec![2, 4, 4, 5, 99, 9801],
			calculator(vec![2, 4, 4, 5, 99, 0], vec![]).0
		);
	}

	#[test]
	fn day5_test4() {
		assert_eq!(
			vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
			calculator(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![]).0
		);
	}

	#[test]
	fn day5_test5() {
		assert_eq!(vec![1002, 4, 3, 4, 99], calculator(vec![1002, 4, 3, 4, 33], vec![]).0);
	}

	#[test]
	fn day5_test6() {
		assert_eq!(vec![1], calculator(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![8]).1);
	}

	#[test]
	fn day5_test7() {
		assert_eq!(vec![0], calculator(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![7]).1);
	}

	#[test]
	fn day5_test8() {
		assert_eq!(vec![1], calculator(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![7]).1);
	}

	#[test]
	fn day5_test9() {
		assert_eq!(vec![0], calculator(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![9]).1);
	}

	#[test]
	fn day5_test10() {
		assert_eq!(vec![1], calculator(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![8]).1);
	}

	#[test]
	fn day5_test11() {
		assert_eq!(vec![0], calculator(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![7]).1);
	}

	#[test]
	fn day5_test12() {
		assert_eq!(vec![1], calculator(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![7]).1);
	}

	#[test]
	fn day5_test13() {
		assert_eq!(vec![0], calculator(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![9]).1);
	}

	#[test]
	fn day5_test14() {
		assert_eq!(
			vec![1],
			calculator(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], vec![1]).1
		);
	}

	#[test]
	fn day5_test15() {
		assert_eq!(
			vec![0],
			calculator(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], vec![0]).1
		);
	}

	#[test]
	fn day5_test16() {
		assert_eq!(
			vec![1],
			calculator(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], vec![1]).1
		);
	}

	#[test]
	fn day5_test17() {
		assert_eq!(
			vec![0],
			calculator(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], vec![0]).1
		);
	}

	#[test]
	fn day5_test18() {
		let code = vec![
			3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125,
			20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
		];
		assert_eq!(vec![999], calculator(code, vec![7]).1);
	}

	#[test]
	fn day5_test19() {
		let code = vec![
			3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125,
			20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
		];
		assert_eq!(vec![1000], calculator(code, vec![8]).1);
	}

	#[test]
	fn day5_test20() {
		let code = vec![
			3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125,
			20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
		];
		assert_eq!(vec![1001], calculator(code, vec![9]).1);
	}
}
