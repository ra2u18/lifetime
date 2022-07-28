
/// Expanding CPU to have 
/// 1) general purpose registers for calculations
/// 2) one special purpose register <position_in_memory>
struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000], // 4kb
    // After 16 nested function calls,
    // the program encounters a stack overflow
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2; 

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >>  8) as u8;
            let y = ((opcode & 0x00F0) >>  4) as u8;
            let d = ((opcode & 0x000F) >>  0) as u8;

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; }, // Short-circuit no-op code
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode)
            }
        }
    }

    // Calling a function in three steps
    //  1) store the current memory location on the stack
    //  2) increment the stack pointer
    //  3) set the current memory location to the intended memory address
    // Returning from a function involves reversing the calling process:
    //  1) Decrement stack pointer
    //  2) Retrieve the calling memory address from stack
    //  3) Set the current memory to the intended memory address
    fn call(&mut self, addr: u16){
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow")
        }
        let mut sp = self.stack_pointer;

        sp -= 1;
        self.position_in_memory = self.stack[sp] as usize;
    }

    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        self.registers[0xF] = if overflow { 1 } else { 0 }; 
    }
}

fn main(){
   let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0
   };

   // Initialize few registers with values
   cpu.registers[0] = 5;
   cpu.registers[1] = 10;

   let mem = &mut cpu.memory;
   mem[0x000] = 0x21; mem[0x001] = 0x00; // Sets opcode to 0x2100 -> CALL fn at 0x100
   mem[0x002] = 0x21; mem[0x003] = 0x00; // Sets opcode to 0x2100 -> CALL fn at 0x100
   mem[0x004] = 0x00; mem[0x005] = 0x00; // Sets opcode to 0x0000 -> HALT execution

   mem[0x100] = 0x80; mem[0x101] = 0x014; // Sets opcode to 0x0000 -> HALT
   mem[0x102] = 0x80; mem[0x103] = 0x014; // Sets opcode to 0x0000 -> HALT
   mem[0x104] = 0x00; mem[0x005] = 0x0EE; // Sets opcode to 0x0000 -> HALT

   cpu.run();

   assert_eq!(cpu.registers[0], 45);
   println!("{}", cpu.registers[0]);
}

// Load the function into the ram 
// Assume we want to add two numbers, these are the following opcodes
// let add_twice: [u16; 3] = [0x8014, 0x8014, 0x00EE]
// decomponse into bytes [u8; 6] =[0x80, 0x14, 0x80, 0x14, 0x00, 0xEE]
fn load_into_ram() {
    let mut memory: [u8; 4096] = [0; 4096];
    let mem = &mut memory;

    let add_twice = [
        0x80, 0x14,
        0x80, 0x14,
        0x00, 0xEE
    ];

    mem[0x100..0x106].copy_from_slice(&add_twice);
    println!("{:?}", &mem[0x100..0x106]);
}