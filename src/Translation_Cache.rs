extern crate libc;

use std::mem;
use std::marker;
use std::ops::{Index, IndexMut};

extern{
    fn memset(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}

pub struct Translation_Cache {
    pub page : *mut u8
}

unsafe impl Send for Translation_Cache {}
unsafe impl Sync for Translation_Cache {}

const PAGE_SIZE: usize = 4096;

impl Index<usize> for Translation_Cache {
    type Output = u8;

    fn index(&self, _index: usize) -> &u8 {
        unsafe {&*self.page.offset(_index as isize) }
    }
}

impl IndexMut<usize> for Translation_Cache {
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        unsafe {&mut *self.page.offset(_index as isize) }
    }
}

impl Translation_Cache {
    pub fn new(num_pages: usize) -> Translation_Cache {
        let page : *mut u8;
        unsafe {
            let cache_size = num_pages * PAGE_SIZE;
            let mut _page : *mut libc::c_void = mem::uninitialized();
            libc::posix_memalign(&mut _page, PAGE_SIZE, cache_size);
            libc::mprotect(_page, cache_size, libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);
            memset(_page, 0xC3, cache_size);
            page = mem::transmute(_page);
        }
        Translation_Cache { page: page }
    }
}
