use Translation_Cache::Translation_Cache;

pub struct Emitter {
    pub page: u64,
    pub iter: u64,
}

impl Emitter {
    pub fn new() -> Emitter{
        Emitter {
            page:0,
            iter:0,
        }
    }

    pub fn emit(&mut self, Dyna_Rec_Code: &Vec<u8>, Translation_Cache: &mut Translation_Cache){
        for (index,i) in Dyna_Rec_Code.iter().enumerate() {
            Translation_Cache[index] = *i;
            println!("Emitting! {} to index {}", *i, index);
        }
    }
}
