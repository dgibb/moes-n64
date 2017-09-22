use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

pub struct Interpreter {
    allocated_regs: [u8; 13],
}

impl Interpreter {
    pub fn new() -> Interpreter{
        Interpreter {
            allocated_regs: [0; 13],
        }
    }

    pub fn interpret_inst(&mut self, PC: &u32, ROM: &Vec<u8>) -> Vec<u8>{
        let mut rom = Cursor::new(&ROM);
        let inst: u32 = rom.read_u32::<BigEndian>().unwrap();
        println!("the interpreter read u32: {:X}", inst);
        let mut translation = self.translate(inst);
        return translation;
    }

    pub fn translate(&mut self, inst: u32) -> Vec<u8>{
        match inst>>26 {

            0x00...0x0E => {
                println!("instruction not implemented, opcode: {:X}", inst>>26);
                return vec![0xC3];
            }

            0x0F => {
                println!("Congratulations! You got the expected value: {:X}, a lui instruction", inst>>26);
                let mut x86_inst: Vec<u8> = vec![0x48, 0xC7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0xFF, 0xC7];
                //MOV rd, imm,
                //INC R15 (counter)
                //counting normally implemented increased at branch
                let rd:u8 = self.allocate_reg((inst>>15)&0x1F);
                //map t1 to RAX, but not really
                x86_inst[2]=(rd|0xC0);
                x86_inst[3]=((inst&0xFF) as u8 );
                x86_inst[4]=(((inst>>8)&0xFF) as u8);
                return x86_inst;
            }

            0x10...0x3F => {
                println!("instruction not implemented, opcode: {:X}", inst>>26);
                return vec![0xC3];
            }

            _ =>{
                println!("invalid opcode: {:X}, instruction decoding screwed up somewhere", inst>>26);
                return vec![0xC3];
            }
        }
    }

    pub fn allocate_reg(&mut self, reg: u32) -> u8 {
        //map MIPS regs to x86_64 regs
        //default RAX for now
        println!("register {:X} mapped to rax", reg);
        return 0;
    }

    pub fn compile_block(&mut self, PC: &u32, ROM: &Vec<u8>) -> Vec<u8>{
        //will run till unconditional branch or end of cache block

        //--PASS 1--//
        //disassembly
        //for now just 1 instruction
        let block = self.interpret_inst(PC, ROM);


        //--PASS 2--//
        //register allocation
        //todo
        //default RAX

        //--PASS 3--//
        //translation
        //done in pass 1 for now

        return block
    }

    pub fn init(&mut self){
        for index in 0..12{
            self.allocated_regs[index]=(index+8) as u8;
            //pre allocate regs R8 to R22 ($t0-$t7 and $s0-$s4)
        }

    }
}
