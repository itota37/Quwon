// quwon_utility/src/mem.rs
// (C) 2022 Taichi Ito.
//! ゲーム用の高速なメモリを提供します。

use std::mem::size_of;
use std::alloc::{
    alloc, dealloc, 
    Layout, 
    GlobalAlloc
};
use std::ptr::{
    null_mut, 
    drop_in_place
};
use std::sync::{
    Arc, 
    Mutex, 
    Once
};

// テストです。
#[cfg(test)]
pub mod tests {
    use std::{
        alloc::{
            Layout,
            GlobalAlloc
        }, 
        ptr::null_mut, mem::transmute
    };
    use crate::mem::global::Memory;

    #[test]
    pub fn run() {
        println!("quwon_utility/src/mem.rs test start.");
        const LEN: usize = 123;
        let mut ptrs = [(null_mut::<usize>(), Option::<Layout>::None); LEN];
        let alloc = Memory::new();
        let layout = Layout::new::<usize>();
        println!("Memory::alloc");
        for i in 0..LEN {
            ptrs[i] = unsafe { (transmute::<*mut u8, *mut usize>(alloc.alloc(layout)), Some(layout)) };
        }
        println!("ptrs[i] = i");
        for i in 0..LEN {
            *unsafe { ptrs[i].0.as_mut()}.expect("nullです。") = i;
        }
        println!("---------- print ----------");
        for i in 0..LEN {
            println!( "i = {}", *unsafe { ptrs[i].0.as_mut()}.expect("nullです。"));
        }
        println!("---------------------------");
        println!("Memory::dealloc");
        for i in 0..LEN {
            unsafe { alloc.dealloc(transmute::<*mut usize, *mut u8>(ptrs[i].0), ptrs[i].1.expect("noneです。")) };
        }
        println!("quwon_utility/src/mem.rs test end.");
    }
}

/// グローバルメモリです。
pub struct Memory {}
//static mut GLOBAL_MEMORY: Option<AtomDynMemory> = None;
static GLOBAL_MEMORY_ONCE: Once = Once::new();
impl Memory {
    
    /// メモリを作成します。
    pub const fn new() -> Self {Memory {}}
}
unsafe fn imit_memory() {

    GLOBAL_MEMORY_ONCE.call_once(||{
        todo!()
    });
}
unsafe impl GlobalAlloc for Memory {
    
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        
        imit_memory();

        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        
        imit_memory();
        
        todo!()
    }
}


