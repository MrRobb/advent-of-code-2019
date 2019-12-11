extern crate itertools;

use itertools::Itertools;
use std::fs::read_to_string;

////////////////////////////////////////
/// OPERATION
////////////////////////////////////////

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

////////////////////////////////////////
/// PARAMETER MODE
////////////////////////////////////////

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

////////////////////////////////////////
/// INSTRUCTION
////////////////////////////////////////

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
			Operation::Halt => {},
		}
		*i = new_i;
	}
}

////////////////////////////////////////
/// PROCESS
////////////////////////////////////////

#[derive(PartialEq)]
enum ExitCode {
	Halt,
	Wait,
	Print,
}

struct Process {
	ip: usize,
	code: Vec<i64>,
	input: Vec<i64>,
	output: Vec<i64>,
}

impl Process {
	fn new(code: Vec<i64>) -> Self {
		Process {
			ip: 0,
			code,
			input: Vec::new(),
			output: Vec::new(),
		}
	}

	fn run(&mut self) -> ExitCode {
		loop {
			let ins = Instruction::parse(self.code[self.ip]);
			if ins.opcode == Operation::Halt {
				return ExitCode::Halt;
			}
			if ins.opcode == Operation::Input && self.input.is_empty() {
				return ExitCode::Wait;
			}
			ins.eval(&mut self.code, &mut self.ip, &mut self.input, &mut self.output);
			if ins.opcode == Operation::Output {
				return ExitCode::Print;
			}
		}
	}
}

////////////////////////////////////////
/// PART 1
////////////////////////////////////////

fn get_max_signal(code: &Vec<i64>) -> (i64, Vec<i64>) {
	(0..5)
		.permutations(5)
		.filter_map(|phases| {
			let mut result = 0;
			for phase in &phases {
				let mut process = Process::new(code.clone());
				process.input = vec![*phase, result];
				process.run();
				result = process.output[0];
			}
			Some((result, phases))
		})
		.max_by_key(|v| v.0)
		.unwrap()
}

////////////////////////////////////////
/// PART 2
////////////////////////////////////////

fn get_max_signal_with_feedback(code: &Vec<i64>) -> (i64, Vec<i64>) {
	(5..10)
		.permutations(5)
		.filter_map(|phases| {
			let mut amplifiers: Vec<_> = phases
				.iter()
				.map(|phase| {
					let mut process = Process::new(code.clone());
					process.input = vec![*phase];
					process
				})
				.collect();

			let mut result = 0;

			loop {
				for (_, process) in amplifiers.iter_mut().enumerate() {
					process.input.push(result);
					let exitcode = process.run();
					if exitcode == ExitCode::Halt {
						return Some((result, phases));
					}
					result = process.output.pop().unwrap();
				}
			}
		})
		.max_by_key(|v| v.0)
		.unwrap()
}

////////////////////////////////////////
/// MAIN
////////////////////////////////////////

pub fn main() {
	let code_str = read_to_string("input/day7/input1.txt").unwrap();
	let code: Vec<i64> = code_str.split(',').map(|n| n.parse().unwrap()).collect();
	let res = get_max_signal(&code);
	println!("PART 1 -> Max thruster signal {} (phase: {:?})", res.0, res.1);
	let res = get_max_signal_with_feedback(&code);
	println!("PART 2 -> Max thruster signal {} (phase: {:?})", res.0, res.1);
}

////////////////////////////////////////
/// TESTS
////////////////////////////////////////

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day7_test1() {
		let code: Vec<i64> = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
		let res = get_max_signal(&code);
		assert_eq!(43210, res.0);
		assert_eq!(vec![4, 3, 2, 1, 0], res.1);
	}

	#[test]
	fn day7_test2() {
		let code: Vec<i64> = vec![
			3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0,
		];
		let res = get_max_signal(&code);
		assert_eq!(54321, res.0);
		assert_eq!(vec![0, 1, 2, 3, 4], res.1);
	}

	#[test]
	fn day7_test3() {
		let code: Vec<i64> = vec![
			3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32,
			31, 31, 4, 31, 99, 0, 0, 0,
		];
		let res = get_max_signal(&code);
		assert_eq!(65210, res.0);
		assert_eq!(vec![1, 0, 4, 3, 2], res.1);
	}

	#[test]
	fn day7_test4() {
		let code: Vec<i64> = vec![
			3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99,
			0, 0, 5,
		];
		let res = get_max_signal_with_feedback(&code);
		assert_eq!(139629729, res.0);
		assert_eq!(vec![9, 8, 7, 6, 5], res.1);
	}

	#[test]
	fn day7_test5() {
		let code: Vec<i64> = vec![
			3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5, 54, 1105, 1,
			12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6,
			99, 0, 0, 0, 0, 10,
		];
		let res = get_max_signal_with_feedback(&code);
		assert_eq!(18216, res.0);
		assert_eq!(vec![9, 7, 8, 5, 6], res.1);
	}
}
