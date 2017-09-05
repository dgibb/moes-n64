extern crate libc;

use std::mem;
use std::marker;

extern{
    fn fill(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}

pub struct Translation_Cache {
    page : *mut u8
}

const PAGE_SIZE: usize = 8;

impl Translation_Cache {
    pub fn new(num_pages: usize) -> Translation_Cache {
        let page : *mut u8;
        unsafe {
            let cache_size = num_pages * PAGE_SIZE;
            let mut _page : *mut libc::c_void = mem::uninitialized();
            //libc::posix_memalign(&mut _contents, PAGE_SIZE, size);
            libc::mprotect(_page, cache_size, libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);
            fill(_page, 0xC3, cache_size);
            page = mem::transmute(_page);
        }
        Translation_Cache { page: page }
    }
}
