
struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    ip: usize, // Inscruction pointer
    stack: [u16; 16],
    sp: usize, // Stack Pointer
}

impl CPU {
    fn new() -> CPU {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            ip: 0,
            stack: [0; 16],
            sp: 0,
        }
    }

    fn read_opcode(&mut self) -> u16 {
        let op_byte1 = self.memory[self.ip];
        let op_byte2 = self.memory[self.ip + 1];

        (op_byte1 as u16) << 8 | (op_byte2 as u16)
    }

    fn decode_opcode(&self, opcode: u16) -> (u8, u8, u8, u8) {
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = (opcode & 0x000F) as u8;
        (c, x, y, d)
    }

    fn execute_opcode(&mut self, opcode: u16) -> Result<bool, ()> {
        let (c, x, y, d) = self.decode_opcode(opcode);
        match (c, x, y, d) {
            (0, 0, 0, 0) => {
                return Ok(true);
            }
            (0x8, _, _, 0x4) => Err(self.add_xy(x, y)),
            (0x2, _, _, _) => Err(self.call_fn(opcode & 0x0FFF)),
            (0x0, 0x0, 0xe, 0xe) => Err(self.ret_fn()),
            _ => todo!("opcode {:04x}", opcode),
        }
    }

    fn call_fn(&mut self, addr: u16) {
        // We want to be sure there is no overflow
        let clean_addr = addr & 0x0FFF;

        self.stack[self.sp] = self.ip as u16;
        self.sp += 1;
        self.ip = clean_addr as usize;
    }

    fn ret_fn(&mut self) {
        println!("RET");
        if self.sp == 0 {
            panic!("Stack underflow");
        }
        self.sp = self.sp - 1;
        self.ip = self.stack[self.sp] as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        println!("ADD {} + {}", arg1, arg2);

        let (res, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = res;
        self.registers[0xF] = overflow as u8;
    }

    fn run(&mut self) {
        loop {
            // Read the opcode to execute
            let opcode = self.read_opcode();
            // increment the instruction pointer
            self.ip += 2;

            let opres = self.execute_opcode(opcode);
            match opres {
                Ok(_) => return,
                Err(_) => continue,
            }
        }
    }

    fn decode_instruction(&self, opcode: u16) -> String {
        match  self.decode_opcode(opcode) {
            (0, 0, 0, 0) => String::from("EXIT"),
            
            (0x8, _, _, 0x4) => String::from("ADD"),
            (0x2, _, _, _) => String::from("CALL"),
            (0x0, 0x0, 0xe, 0xe) => String::from("RET"),
            _ => String::from("!!UNKNOWN!!"),
        }
    }

    fn print_program(&self, count: usize) {

        println!("PROGRAM");
        println!("#######");

        let mut opcode: u16= 0;
        for (i, op) in self.memory.iter().enumerate() {
            if i == count  {
                break;
            }
            
            if i % 2 != 0 {
                opcode = opcode | *op as u16;
                println!("{:04x} - {:04x} ({})", i, opcode, self.decode_instruction(opcode));
            } else {
                opcode = (*op as u16) << 8;

            }
        }
        println!("#######");
    }
}

fn main() {
    
    let mut cpu = CPU::new();
    cpu.registers[0] = 10;
    cpu.registers[1] = 2;

    // ADD op (0x8014)
    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x14;

    // CALL fn (0x2nnn)  addr 006
    cpu.memory[2] = 0x20;
    cpu.memory[3] = 0x08;

    // ADD op (0x8014)
    cpu.memory[4] = 0x80;
    cpu.memory[5] = 0x14;

    // EXIT op
    cpu.memory[6] = 0x00;
    cpu.memory[7] = 0x00;

    // fn body loaded a addr 8
    cpu.memory[8] = 0x80;
    cpu.memory[9] = 0x14;
    cpu.memory[10] = 0x80;
    cpu.memory[11] = 0x14;
    cpu.memory[12] = 0x00;
    cpu.memory[13] = 0xee;

    // EXIT op
    cpu.memory[14] = 0x00;
    cpu.memory[15] = 0x00;

    cpu.print_program(16);

    cpu.run();

    println!("Result is {}", cpu.registers[0]);
}
