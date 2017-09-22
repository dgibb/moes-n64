use TranslationCache::TranslationCache;
use Interpreter::Interpreter;
use Emitter::Emitter;
use std::mem;

pub struct Emulator {
    pub rom: Vec<u8>,
    pub jump_table: Vec<[u32;2]>,
    pub memory: [u8;0x1000],
    pub pc: u32,
    pub gpr: [u64;32],
    pub fpr: [u64;32],
    pub cp0: [u64;32],
    pub translation_cache: TranslationCache,
    pub interpreter : Interpreter,
    pub emitter : Emitter,
    pub ex_cache : Option<fn() -> u64 >,
}

unsafe impl Send for Emulator {}
unsafe impl Sync for Emulator {}

impl Emulator {
    pub fn new() -> Emulator{
        Emulator {
            rom: vec![],
            jump_table: vec![],
            memory: [0;0x1000],
            pc: 0,
            gpr: [0;32],
            fpr: [0;32],
            cp0: [0;32],
            translation_cache: TranslationCache::new(1), //use 32 or 64MB cache currently 4KB
            interpreter: Interpreter::new(),
            emitter: Emitter::new(),
            ex_cache: None,
         }
    }

    pub fn run_block(&mut self) {
        if true { // if untranslated code at pc. This is always true for now

            self.emitter.emit(&self.interpreter.compile_block(&self.pc, &self.rom), &mut self.translation_cache);
            // interpret MIPS code and store to TranslationCache

            //test_func, left here for reference

            //let test_func: Vec<u8> = vec![0x48, 0xC7, 0xC0, 0x18, 0x00, 0x00, 0x00, 0x48, 0xFF, 0xC0, 0xC3];
            //Moves 0x18 to RAX, Adds 1, and returns
            //assumes return value in RAX
        }
        self.ex_cache = Some(self.get_fn_ptr());
        //self.evaluate_interrupt((self.ex_cache.unwrap())());
        //wil not compile
        let return_val = (self.ex_cache.unwrap())();
        self.evaluate_interrupt(return_val);
        //compiles, thats annoying
    }

    pub fn get_fn_ptr(&mut self) -> (fn() -> u64) {
      unsafe { mem::transmute(self.translation_cache.page) }
    }

    pub fn evaluate_interrupt(&mut self, t0: u64 ){
        match t0{

            0x3400 => {
                println!("congrats you got the correct exit code: {:X}", t0);
                self.gpr[9];
            }

            _ => {
                println!("Error! you got the wrong exit code: {:X}", t0);
            }
        }
    }

    pub fn init(&mut self){
        self.interpreter.init();
    }
}
