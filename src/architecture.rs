#[derive(Debug)]
pub enum Opcode {
    Noop,
    Stop,
    // direction and the two registers
    // MoveRegReg(bool, Register, Register),
    // MoveRegMem(bool, Register, u16),
    // MoveImmReg16(u16, Register),
    // MoveImmReg8(u8, Register),
    // MoveImmMem(u16, u16),
    Increment(Register),
    Decrement(Register),
}

#[derive(Debug)]
pub enum Flag {
    // Conditional Flags
    Carry,
    Auxiliary,
    Parity,
    Zero,
    Sign,
    Overflow,

    // Control,
    Trap,
    Interrupt,
    Direction,
}

#[derive(Debug)]
pub enum Register {
    // MAIN REGISTERS
    AX,
    AL,
    AH,
    BX,
    BH,
    BL,
    CX,
    CH,
    CL,
    DX,
    DH,
    DL,

    // SEGMENT REGISTERS
    CS,
    DS,
    ES,
    SS,

    // INDEX REGISTER
    SI,
    DI,
    BP,
    SP,

    // PROGRAM COUNTER
    IP,
}

#[derive(Debug)]
pub struct Processor {
    // Flags
    // Conditional Flags
    carry: bool,
    auxiliary: bool,
    parity: bool,
    zero: bool,
    sign: bool,
    overflow: bool,

    // Control flags,
    trap: bool,
    interrupt: bool,
    direction: bool,

    // Registers
    // main registers
    ax: u8,
    bx: u8,
    cx: u8,
    dx: u8,

    // segment registers
    cs: u16,
    ds: u16,
    es: u16,
    ss: u16,

    // index register
    si: u16,
    di: u16,
    bp: u16,
    sp: u16,

    // program counter
    ip: u16,

    byte_code: Vec<u8>,
    quasi_compiled: Vec<Opcode>,
}

impl Processor {
    pub fn default(hex_code: Vec<u8>) -> Self {
        Self {
            carry: false,
            auxiliary: false,
            parity: false,
            zero: false,
            sign: false,
            overflow: false,
            trap: false,
            interrupt: false,
            direction: false,
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            cs: 0,
            ds: 0,
            es: 0,
            ss: 0,
            si: 0,
            di: 0,
            bp: 0,
            sp: 0,
            ip: 0,
            byte_code: hex_code,
            quasi_compiled: Vec::new(),
        }
    }

    // Convert source code hex code to byte code (inc ax -> 40)
    pub fn source_code_to_byte_code(&self) {
        todo!()
    }

    // Get hex code on the index of IP
    pub fn get_current_hex_code() -> u8 {
        self.byte_code[<u16 as std::convert::Into<usize>>::into(self.ip)]
    }

    // Get opcodes in hex format and convert them into Opcode enum
    pub fn compile_byte_code_to_quasi_compiled(&mut self) {
        while usize::from(self.ip) < self.byte_code.iter().count() {
            self.quasi_compiled.push(match self.get_current_hex_code() {
                0x90 => Opcode::Noop,
                0x40 => Opcode::Increment(Register::AX),
                0x41 => Opcode::Increment(Register::CX),
                0x42 => Opcode::Increment(Register::DX),
                0x43 => Opcode::Increment(Register::BX),
                0x46 => Opcode::Increment(Register::SI),
                0x47 => Opcode::Increment(Register::DI),
                0x48 => Opcode::Decrement(Register::AX),
                0x49 => Opcode::Decrement(Register::CX),
                0x4A => Opcode::Decrement(Register::DX),
                0x4B => Opcode::Decrement(Register::BX),
                0xFE => {
                    self.ip += 1;
                    match get_current_hex_code() {
                        0xC0 => Opcode::Increment(Register::AL),
                        0xC1 => Opcode::Increment(Register::CL),
                        0xC2 => Opcode::Increment(Register::DL),
                        0xC3 => Opcode::Increment(Register::BL),
                        0xC8 => Opcode::Decrement(Register::AL),
                        0xC9 => Opcode::Decrement(Register::CL),
                        0xCA => Opcode::Decrement(Register::DL),
                        0xCB => Opcode::Decrement(Register::BL),
                        0xCC => Opcode::Decrement(Register::AH),
                        0xCD => Opcode::Decrement(Register::CH),
                        0xCE => Opcode::Decrement(Register::DH),
                        0xCF => Opcode::Decrement(Register::BH),
                    }
                }
                0xFC => {
                    self.ip += 1;
                    match get_current_hex_code() {
                        0xC4 => Opcode::Increment(Register::AH),
                        0xC5 => Opcode::Increment(Register::CH),
                        0xC6 => Opcode::Increment(Register::DH),
                        0xC7 => Opcode::Increment(Register::BH),
                    }
                }
                _ => Opcode::Noop,
            });

            self.ip += 1;
        }
    }

    // Execute opcode enum one by one (an interpreter!)
    pub fn execute_quasi_compiled(&mut self) {
        for opcode in self.quasi_compiled.iter() {
            println!("Opcode: {:?}", opcode);
            match opcode {
                Opcode::Noop => continue,
                Opcode::Increment(Register::AX) => self.ax += 1,
                Opcode::Increment(Register::BX) => self.bx += 1,
                Opcode::Increment(Register::CX) => self.cx += 1,
                Opcode::Increment(Register::DX) => self.dx += 1,
                _ => continue,
            }
        }
    }
}
