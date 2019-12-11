
use std::fs::read_to_string;
use std::collections::HashMap;

////////////////////////////////////////
/// OPERATION
////////////////////////////////////////

#[derive(PartialEq)]
pub enum Operation {
	Add = 1,
	Mul = 2,
	Input = 3,
	Output = 4,
	JumpIfTrue = 5,
	JumpIfFalse = 6,
	LessThan = 7,
	Equals = 8,
	RelativeBase = 9,
	Halt = 99,
}

impl Operation {
	pub fn n_parameters(&self) -> usize {
		match self {
			Operation::Add => 3,
			Operation::Mul => 3,
			Operation::Input => 1,
			Operation::Output => 1,
			Operation::Halt => 0,
			Operation::JumpIfTrue => 2,
			Operation::JumpIfFalse => 2,
			Operation::LessThan => 3,
			Operation::RelativeBase => 1,
			Operation::Equals => 3,
		}
	}
}

impl From<i128> for Operation {
	fn from(n: i128) -> Self {
		match n {
			1 => Operation::Add,
			2 => Operation::Mul,
			3 => Operation::Input,
			4 => Operation::Output,
			5 => Operation::JumpIfTrue,
			6 => Operation::JumpIfFalse,
			7 => Operation::LessThan,
			8 => Operation::Equals,
			9 => Operation::RelativeBase,
			99 => Operation::Halt,
			_ => unimplemented!(),
		}
	}
}

////////////////////////////////////////
/// MEMORY
////////////////////////////////////////

struct Memory {
    base: i128,
    values: HashMap<u128, i128>,
}

impl Memory {

    fn new(code: Vec<i128>) -> Self {
        let mut hash = HashMap::new();
        for (i, x) in code.iter().enumerate() {
            hash.insert(i as u128, *x);
        }
        Self {
            base: 0,
            values: hash,
        }
    }

    fn get(&self, i: u128, mode: ParameterMode) -> i128 {
        let address = self.get_address(i, mode);
        *self.values.get(&address).unwrap_or(&0)
    }

    fn get_address(&self, i: u128, mode: ParameterMode) -> u128 {
        match mode {
            ParameterMode::Immediate => i,
            ParameterMode::Position => *self.values.get(&i).unwrap_or(&0) as u128,
            ParameterMode::Relative => (*self.values.get(&i).unwrap_or(&0) + self.base) as u128,
        }
    }

    fn add_to_base(&mut self, increment: i128) {
        self.base += increment;
    }

    fn write(&mut self, i: u128, value: i128, mode: ParameterMode) {
        let address = self.get_address(i, mode);
        self.values.insert(address, value);
    }
}

////////////////////////////////////////
/// PARAMETER MODE
////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
enum ParameterMode {
	Position = 0,
	Immediate = 1,
	Relative = 2,
}

impl From<u32> for ParameterMode {
	fn from(n: u32) -> Self {
		match n {
			0 => ParameterMode::Position,
			1 => ParameterMode::Immediate,
			2 => ParameterMode::Relative,
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
	fn parse(opcode: i128) -> Self {
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

	fn eval(
        &self,
        i: &mut u128,
		memory: &mut Memory,
		input: &mut Vec<i128>,
		output: &mut Vec<i128>,
	) {
		let mut new_i = *i + self.opcode.n_parameters() as u128 + 1;
		match self.opcode {
			Operation::Add => {
                let x1 = memory.get(*i + 1, self.parameter_modes[0]);
                let x2 = memory.get(*i + 2, self.parameter_modes[1]);
                memory.write(*i + 3, x1 + x2, self.parameter_modes[2]);
			},
			Operation::Mul => {
                let x1 = memory.get(*i + 1, self.parameter_modes[0]);
                let x2 = memory.get(*i + 2, self.parameter_modes[1]);
                memory.write(*i + 3, x1 * x2, self.parameter_modes[2]);
			},
			Operation::Input => {
                memory.write(*i + 1, input.remove(0), self.parameter_modes[0]);
			},
			Operation::Output => {
                output.push(memory.get(*i + 1, self.parameter_modes[0]));
			},
			Operation::JumpIfTrue => {
                let n = memory.get(*i + 1, self.parameter_modes[0]);
				if n != 0 {
					new_i = memory.get(*i + 2, self.parameter_modes[1]) as u128;
				}
			},
			Operation::JumpIfFalse => {
				let n = memory.get(*i + 1, self.parameter_modes[0]);
				if n == 0 {
					new_i = memory.get(*i + 2, self.parameter_modes[1]) as u128;
				}
			},
			Operation::LessThan => {
                let x1 = memory.get(*i + 1, self.parameter_modes[0]);
                let x2 = memory.get(*i + 2, self.parameter_modes[1]);
                memory.write(*i + 3, if x1 < x2 { 1 } else { 0 }, self.parameter_modes[2]);
			},
			Operation::Equals => {
                let x1 = memory.get(*i + 1, self.parameter_modes[0]);
                let x2 = memory.get(*i + 2, self.parameter_modes[1]);
                memory.write(*i + 3, if x1 == x2 { 1 } else { 0 }, self.parameter_modes[2]);
			},
			Operation::RelativeBase => {
                memory.add_to_base(memory.get(*i + 1, self.parameter_modes[0]));
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
	ip: u128,
	memory: Memory,
	input: Vec<i128>,
	output: Vec<i128>,
}

impl Process {
	fn new(code: Vec<i128>) -> Self {
		Process {
			ip: 0,
			memory: Memory::new(code),
			input: Vec::new(),
			output: Vec::new(),
		}
	}

	fn run(&mut self) -> ExitCode {
		loop {
            let ins = self.memory.get(self.ip, ParameterMode::Immediate);
			let ins = Instruction::parse(ins);
			if ins.opcode == Operation::Halt {
				return ExitCode::Halt;
			}
			if ins.opcode == Operation::Input && self.input.is_empty() {
				return ExitCode::Wait;
			}
			ins.eval(
                &mut self.ip,
				&mut self.memory,
				&mut self.input,
				&mut self.output,
			);
			if ins.opcode == Operation::Output {
				return ExitCode::Print;
			}
		}
	}

	fn run_until_halt(&mut self) {
		while self.run() != ExitCode::Halt {}
	}
}

////////////////////////////////////////
/// MAIN
////////////////////////////////////////

pub fn main() {
	let code_str = read_to_string("input/day9/input1.txt").unwrap();
	let code: Vec<i128> = code_str.split(',').map(|n| n.parse().unwrap()).collect();
    
    let mut process = Process::new(code.clone());
	process.input.push(1);
	process.run_until_halt();
    println!("PART 1 -> BOOST keycode: {:?}", process.output);
    
    let mut process = Process::new(code);
	process.input.push(2);
	process.run_until_halt();
	println!("PART 2 -> BOOST keycode: {:?}", process.output);
}

////////////////////////////////////////
/// TESTS
////////////////////////////////////////

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day9_test1() {
		let code: Vec<i128> = vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
		let mut process = Process::new(code.clone());
		process.run_until_halt();
		assert_eq!(code, process.output);
	}

	#[test]
	fn day9_test2() {
		let code: Vec<i128> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
		let mut process = Process::new(code.clone());
		process.run_until_halt();
		assert!(
			1_000_000_000_000_000 <= process.output[0] && process.output[0] <= 9_999_999_999_999_999,
			"output: {}",
			process.output[0]
		);
	}

	#[test]
	fn day9_test3() {
		let code: Vec<i128> = vec![104, 1125899906842624, 99];
		let mut process = Process::new(code.clone());
		process.run_until_halt();
		assert_eq!(1_125_899_906_842_624, process.output[0]);
	}
}
