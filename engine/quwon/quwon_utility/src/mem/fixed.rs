// quwon_utility/src/mem/fixed.rs
// (C) 2022 Taichi Ito.
//! ゲーム用の高速な固定長メモリを提供します。

use std::{
    alloc::{
        self,
        Layout, 
        LayoutError
    }, 
    fmt::Display, 
    error, mem::{
        size_of, 
        transmute
    }, 
    ptr::null_mut
};
use crate::mem::pool;

// テストです。
#[cfg(test)]
pub mod tests {
    use std::ptr::null_mut;
    use crate::mem::pool::Pool;

    // 2022/10/31動作正常確認済み
    #[test]
    pub fn run() {
        println!("quwon_utility/src/mem/pool.rs test start.");
        for i in 2..8 {
            let size = 1 << i;
            const LEN: usize = 32;
            println!("pool = Pool::new({}, {})", size, LEN);
            let mut pool = match Pool::new(size, LEN) {
                Ok(pool) => pool,
                Err(err) => return println!("{}", err.to_string()),
            };
            let mut arr = [null_mut::<u8>(); LEN];
            println!("arr[i] = pool.alloc()");
            for j in 0..LEN {
                arr[j] = unsafe { pool.alloc() };
            }
            println!("pool.is_empty() == {}", pool.is_empty());
            println!("arr[i] = i");
            for j in 0..LEN {
                unsafe { *arr[j] = j as u8 };
            }
            println!("--- println(arr[i]) ---");
            for j in 0..LEN {
                unsafe { println!("{}", *arr[j]) };
            }
            println!("-----------------------");
            println!("pool.dealloc()");
            for j in 0..LEN {
                unsafe { pool.dealloc(arr[j]) };
            }
            println!("pool.is_full() == {}", pool.is_full());
            let mut arr = [null_mut::<u8>(); LEN];
            println!("arr[i] = pool.alloc()");
            for j in 0..LEN {
                arr[j] = unsafe { pool.alloc() };
            }
            println!("pool.is_empty() == {}", pool.is_empty());
            println!("arr[i] = i");
            for j in 0..LEN {
                unsafe { *arr[j] = j as u8 };
            }
            println!("--- println(arr[i]) ---");
            for j in 0..LEN {
                unsafe { println!("{}", *arr[j]) };
            }
            println!("-----------------------");
            println!("pool.dealloc()");
            for j in 0..LEN {
                unsafe { pool.dealloc(arr[j]) };
            }
            println!("pool.is_full() == {}", pool.is_full());
        }
        println!("quwon_utility/src/mem/pool.rs test end.");
    }
}

/// 固定長メモリです。
pub struct FixedAlloc {
    size: usize,
    len: usize,
    pools: PoolList
}

/// 順序指定IDで整列させたメモリプールの生リストです。
struct PoolList {
    len: usize,
    pools: *mut pool::Pool,
}

/// エラーです。
#[derive(Debug)]
pub enum Error {
    /// プールエラーです。
    PoolError(pool::Error),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PoolError(err) => write!(f, "PoolError {}", err),
        }
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::PoolError(err) => err.source(),
        }
    }
}