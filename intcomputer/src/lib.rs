use std::collections::VecDeque;

const LEN_IO_INSTR: usize = 2;
const LEN_BASE_INSTR: usize = 2;
const LEN_JUMP_INSTR: usize = 3;
const LEN_ARITH_INSTR: usize = 4;

pub struct IntComputer {
    pub program: Vec<i64>,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,

    base: i64,
    pc: usize,
    pub state: ProgramState,
}

#[derive(Debug, PartialEq)]
pub enum ProgramState {
    Finished,
    Running,
    WaitingForInput,
}

#[derive(Debug)]
struct OpCode {
    instr: Instruction,
    param_mode: (ParameterMode, ParameterMode, ParameterMode),
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
    JumpT,
    JumpF,
    Less,
    Equal,
    AdjBase,
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
    Invalid,
}

impl IntComputer {
    pub fn new(program: Vec<i64>) -> IntComputer {
        IntComputer {
            program: program,
            pc: 0,
            base: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: ProgramState::Running,
        }
    }

    // Run program until it halts
    pub fn run(&mut self) {
        self.state = ProgramState::Running;
        while self.state == ProgramState::Running {
            self.exec_instr();
        }
    }

    // Execute instruction at current PC
    fn exec_instr(&mut self) {
        let opcode = OpCode::new(self.program[self.pc]);

        match opcode.instr {
            Instruction::Add | Instruction::Multiply | Instruction::Less | Instruction::Equal => {
                self.arith(opcode);
            }
            Instruction::Input | Instruction::Output => {
                self.io(opcode);
            }
            Instruction::JumpF | Instruction::JumpT => {
                self.jump(opcode);
            }
            Instruction::AdjBase => {
                self.adjust_base(opcode);
            }
            Instruction::Halt => {
                self.state = ProgramState::Finished;
            }
        }
    }

    // Handle input/output instructions
    fn io(&mut self, opcode: OpCode) {
        let (mode1, _, _) = opcode.param_mode;
        match opcode.instr {
            Instruction::Input => {
                if self.input.is_empty() {
                    self.state = ProgramState::WaitingForInput;
                    return;
                } else {
                    let input = self.input.pop_front().unwrap();
                    self.write(1, mode1, input);
                }
            }
            Instruction::Output => {
                let output = self.read(1, mode1);
                self.output.push_back(output);
            }
            _ => {}
        }
        self.pc += LEN_IO_INSTR;
    }

    fn adjust_base(&mut self, opcode: OpCode) {
        let (mode1, _, _) = opcode.param_mode;
        let val = self.read(1, mode1);
        self.base += val;
        self.pc += LEN_BASE_INSTR;
    }

    fn jump(&mut self, opcode: OpCode) {
        let (mode1, mode2, _) = opcode.param_mode;
        let val = self.read(1, mode1);
        let dst = self.read(2, mode2) as usize;

        if (opcode.instr == Instruction::JumpT && val != 0)
            || (opcode.instr == Instruction::JumpF && val == 0)
        {
            if dst < self.program.len() {
                self.pc = dst
            }
        } else {
            self.pc += LEN_JUMP_INSTR;
        }
    }

    // Handle addition, multiplication and comparisons
    fn arith(&mut self, opcode: OpCode) {
        let (mode1, mode2, mode3) = opcode.param_mode;
        let op1 = self.read(1, mode1);
        let op2 = self.read(2, mode2);

        let result = match opcode.instr {
            Instruction::Add => op1 + op2,
            Instruction::Multiply => op1 * op2,
            Instruction::Less => (op1 < op2) as i64,
            Instruction::Equal => (op1 == op2) as i64,
            _ => 0,
        };

        self.write(3, mode3, result);
        self.pc += LEN_ARITH_INSTR;
    }

    // Write to memory given by the value at offset
    // Will interpret the value according to the supplied paramter mode
    fn write(&mut self, offset: i64, mode: ParameterMode, value: i64) {
        let val = self.program[self.pc + offset as usize];
        let idx = match mode {
            ParameterMode::Position => val as usize,
            ParameterMode::Relative => (self.base + val) as usize,
            _ => panic!("Write with unsupported Paramter Mode"),
        };
        if idx >= self.program.len() {
            self.extend_capacity(idx);
        }
        self.program[idx] = value;
    }

    // Read from offset relative to current instruction pointer
    // will consider the supplied parameter mode for direct/indirect read
    fn read(&mut self, offset: usize, param_mode: ParameterMode) -> i64 {
        let val = self.program[self.pc + offset];
        let idx: usize = match param_mode {
            ParameterMode::Immediate => return val,
            ParameterMode::Position => val as usize,
            ParameterMode::Relative => (self.base + val) as usize,
            ParameterMode::Invalid => panic!("Invalid read mode!"),
        };
        if idx >= self.program.len() {
            self.extend_capacity(idx as usize);
        }
        self.program[idx as usize]
    }

    // Create additional program space for reads/writes beyond the current end of the program
    fn extend_capacity(&mut self, offset: usize) {
        let mut extra_capacity: Vec<i64> = vec![0; 1 + offset - self.program.len()];
        self.program.append(&mut extra_capacity);
    }
}

impl OpCode {
    // Parse Instruction and mode flags
    fn new(opcode: i64) -> OpCode {
        let read_mode = |flag| match flag % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => ParameterMode::Invalid,
        };
        let mode1 = read_mode(opcode / 100);
        let mode2 = read_mode(opcode / 1000);
        let mode3 = read_mode(opcode / 10000);

        let operation = opcode % 100;
        let instr = match operation {
            1 => Instruction::Add,
            2 => Instruction::Multiply,
            3 => Instruction::Input,
            4 => Instruction::Output,
            5 => Instruction::JumpT,
            6 => Instruction::JumpF,
            7 => Instruction::Less,
            8 => Instruction::Equal,
            9 => Instruction::AdjBase,
            99 => Instruction::Halt,
            _ => panic!("Unknown Opcode"),
        };

        OpCode {
            instr: instr,
            param_mode: (mode1, mode2, mode3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let mut comp = IntComputer::new(vec![1, 0, 0, 0, 99]);
        comp.run();
        assert_eq!(comp.program, vec![2, 0, 0, 0, 99])
    }

    #[test]
    fn test_mul() {
        let mut comp = IntComputer::new(vec![2, 3, 0, 3, 99]);
        comp.run();
        assert_eq!(comp.program, vec![2, 3, 0, 6, 99])
    }

    #[test]
    fn test_input() {
        let mut comp = IntComputer::new(vec![3, 0, 99]);
        comp.input.push_back(42);
        comp.run();
        assert_eq!(comp.program, vec![42, 0, 99])
    }

    #[test]
    fn test_output() {
        let mut comp = IntComputer::new(vec![4, 0, 99]);
        comp.run();
        assert_eq!(comp.output.front(), Some(&4))
    }

    #[test]
    fn test_relative() {
        let mut comp = IntComputer::new(vec![109, 2019, 109, -19, 99]);
        comp.run();
        assert_eq!(comp.base, 2000);

        let mut comp = IntComputer::new(vec![204, 2, 99]);
        comp.run();
        assert_eq!(comp.output[0], 99);

        let mut comp = IntComputer::new(vec![109, 3, 204, -2, 99]);
        comp.input.push_back(42);
        comp.run();
        assert_eq!(comp.output[0], 3);
    }

    #[test]
    fn test_p1() {
        let mut comp = IntComputer::new(vec![2, 4, 4, 5, 99, 0]);
        comp.run();
        assert_eq!(comp.program, vec![2, 4, 4, 5, 99, 9801])
    }

    #[test]
    fn test_p2() {
        let mut comp = IntComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        comp.run();
        assert_eq!(comp.program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99])
    }

    #[test]
    fn test_p3() {
        let mut comp = IntComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        comp.run();
        assert_eq!(comp.program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99])
    }

    #[test]
    fn test_p4() {
        let mut comp = IntComputer::new(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        comp.input.push_back(9);
        comp.run();
        assert_eq!(comp.output.front(), Some(&1001))
    }

    #[test]
    fn test_p5() {
        let mut comp = IntComputer::new(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        comp.input.push_back(8);
        comp.run();
        assert_eq!(comp.output.front(), Some(&1000))
    }

    #[test]
    fn test_p6() {
        let mut comp = IntComputer::new(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        comp.input.push_back(7);
        comp.run();
        assert_eq!(comp.output.front(), Some(&999))
    }
}
