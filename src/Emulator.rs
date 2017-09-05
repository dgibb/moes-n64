use Translation_Cache::Translation_Cache;

pub struct Emulator {
    pub ROM: Vec<u8>,
    pub jumpTable: Vec<[u32;2]>,
    pub memory: [u8;0x1000],
    pub GPR: [u64;32],
    pub FPR: [u64;32],
    pub cp0: [u64;32],
    pub regMap: [u64;64],
    pub Translation_Cache: Translation_Cache,
}

impl Emulator {
    pub fn new() -> Emulator{
        Emulator {
            ROM: vec![],
            jumpTable: vec![],
            memory: [0;0x1000],
            GPR: [0;32],
            FPR: [0;32],
            cp0: [0;32],
            regMap: [0;64],
            Translation_Cache: Translation_Cache::Translation_Cache::new(1),
         }
    }

    pub fn runBlock(&self) {
        println!("rune bloke");
    }
}
