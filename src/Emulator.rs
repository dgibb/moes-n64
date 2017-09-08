use Translation_Cache::Translation_Cache;
use Interpreter::Interpreter;
use Emitter::Emitter;
use std::mem;

pub struct Emulator {
    pub ROM: Vec<u8>,
    pub jumpTable: Vec<[u32;2]>,
    pub memory: [u8;0x1000],
    pub PC: u32,
    pub GPR: [u64;32],
    pub FPR: [u64;32],
    pub cp0: [u64;32],
    pub regMap: [u64;64],
    pub Translation_Cache: Translation_Cache,
    pub Interpreter : Interpreter,
    pub Emitter : Emitter,
    pub exCache : *mut fn(u64) -> u64,
}

unsafe impl Send for Emulator {}
unsafe impl Sync for Emulator {}

impl Emulator {
    pub fn new() -> Emulator{
        Emulator {
            ROM: vec![],
            jumpTable: vec![],
            memory: [0;0x1000],
            PC: 0,
            GPR: [0;32],
            FPR: [0;32],
            cp0: [0;32],
            regMap: [0;64],
            Translation_Cache: Translation_Cache::new(1),
            Interpreter: Interpreter::new(),
            Emitter: Emitter::new(),
            exCache: ::std::ptr::null_mut(),
         }
    }

    pub fn runBlock(&mut self) {
        if true { // if untranslated code at pc. This is always true for now

            //We'll Translate pif ROM Code Later

            //Emitter.emit(Interpreter.interpret(&self.PC, &self.ROM));
            // interpret MIPS code and store to Translation_Cache

            //self.exCache = self.getFnPtr();
            // get pointer to newly re-dynareced function
            //may or may not be neccesary

            let testFunc: Vec<u8> = vec![0x8B, 0x44, 0x24, 0x04, 0x40, 0xC3];
            //Adds 1 to argument and returns

            self.Emitter.emit(& testFunc, &mut self.Translation_Cache);
            //store test function in Translation_Cache
        }
        self.exCache = self.getFnPtr();
        //println!("{}", (*self.exCache)(24));
        //run machine code, print return value, should be 25
    }

    pub fn getFnPtr(&mut self) -> (*mut fn(u64) -> u64) {
      unsafe { mem::transmute(self.Translation_Cache.page) }
    }
}
