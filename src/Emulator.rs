use Translation_Cache::Translation_Cache;
use Interpreter::Interpreter;
use Emitter::Emitter;
use std::mem;

fn placeholder(arg: u64) -> u64{
    return 0;
}

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
    pub exCache : fn(u64) -> u64,
}

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
            exCache: placeholder,
         }
    }

    pub fn runBlock(&mut self) {
        if true { // if untranslated code at pc. This is always true for now

            //We'll Translate pif ROM Code Later

            //Emitter.emit(Interpreter.interpret(&self.PC, &self.ROM));
            // interpret MIPS code and store to Translation_Cache

            //self.exCache = self.getFnPtr();
            // get pointer to newly re-dynareced function

            let testFunc: Vec<u8> = vec![0x8B, 0x44, 0x24, 0x04, 0x40, 0xC3];
            //Adds 1 to argument and returns

            self.Emitter.emit(& testFunc, &mut self.Translation_Cache);
        }
        self.exCache = self.getFnPtr();
        //println!("{}", self.exCache(24)); //Should print 25
    }

    pub fn getFnPtr(self) -> (fn(u64) -> u64) {
      unsafe { mem::transmute(self.Translation_Cache) }
    }
}
