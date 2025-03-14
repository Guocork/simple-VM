/// LC-3 simple-VM
fn main() {
    let program = vec![
        OpCode::Push(5),
        OpCode::Push(10),
        OpCode::Add,
        OpCode::Push(2),
        OpCode::Mul,
        OpCode::Print,
        OpCode::Halt,
    ];
    
    println!("执行基于栈的虚拟机");
    let mut vm = StackVm::new(program);
    vm.run();
    vm.stack_dump();
}

enum Instruction {
    BR , // BRANCH
    ADD,  // ADD
    LD,   // LOAD
    ST,   // STORE
    JSP,  // JUMP REGISTER
    AND,  // BITWISE AND
    LDR,  // LOAD REGISTER
    RTI,  // UNUSED
    NOT,  // BITWISE NOT
    LDI,  // LOAD INDIRECT
    STI,  // STORE INDIIRECT
    JMP,  // JUMP
    RES,  // RESERVED(UNUSED)
    LEA,  // LOAD EFFECTIVE ADDRESS
    TRAP, // EXECUTE TRAP
}

/// Resgisters are the memory units in the CPU that temporarily store data and instructions and address.
enum Register {
    RR0,
    RR1,
    RR2,
    RR3,
    RR4,
    RR5,
    RR6,
    RR7,
    RPC,
    RCOND,
    RCOUNT,
}

enum ConditionFlags {
    FLPOS,
    FLZRO,
    FLNEG,
}


enum OpCode {
    Push(i32),
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Pop,
    Halt,
}

/// base stack
pub struct StackVm {
    stack: Vec<i32>,
    program: Vec<OpCode>,
    ip: usize, 
}

impl StackVm {
    fn new(program: Vec<OpCode>) -> Self {
        StackVm {
            stack: Vec::new(),
            program,
            ip: 0,
        }
    }

    fn run(&mut self) {
        while self.ip < self.program.len() {
            match &self.program[self.ip] {
                OpCode::Push(value) => {
                    self.stack.push(*value);
                    self.ip += 1;
                },
                OpCode::Add => {
                    if self.stack.len() < 2 {
                        panic!("Stack underflow");
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                    self.ip += 1;
                },
                OpCode::Sub => {
                    if self.stack.len() < 2 {
                        panic!("Stack underflow");
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                    self.ip += 1;
                },
                OpCode::Mul => {
                    if self.stack.len() < 2 {
                        panic!("Stack underflow");
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                    self.ip += 1;
                },
                OpCode::Div => {
                    if self.stack.len() < 2 {
                        panic!("Stack underflow");
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if b == 0 {
                        panic!("Division by zero");
                    }
                    self.stack.push(a / b);
                    self.ip += 1;
                },
                OpCode::Print => {
                    if let Some(value) = self.stack.last() {
                        println!("Output: {}", value);
                    } else {
                        println!("Stcak is empty");
                    }
                    self.ip += 1;
                },
                OpCode::Pop => {
                    self.stack.pop();
                    self.ip += 1;
                }
                OpCode::Halt => {
                    break;
                }
            }
        }
    }

    fn stack_dump(&self) {
        println!("Stack: {:?}", self.stack);
    }
}




// base register