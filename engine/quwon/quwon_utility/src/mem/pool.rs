// quwon_utility/src/mem/pool.rs
// (C) 2022 Taichi Ito.
//! ゲーム用の高速なメモリのメモリプールを提供します。

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

/// メモリプールです。
pub struct Pool {
    layout: Layout,      // バッファのメモリレイアウトです。
    buffer: *mut u8,     // バッファです。
    list: *mut *mut u8,  // 未使用ポインタの単方向連結リストの先頭です。
    use_cnt: usize,      // 使用カウントです。
}
impl Pool {
    /// プールを作成します。
    /// 
    /// # Arguments
    /// 
    /// * `size` - 1要素のサイズです。
    /// * `len` - 要素数です。
    /// 
    pub fn new(size: usize, len: usize) -> Result<Self, Error> {
        // サイズ、または、要素数が0の場合エラーです。
        if size == 0 || len == 0 {
            return Err(Error::Unsized);
        }
        // サイズがポインタサイズ以上になるように調整します。
        const PTR_SIZE: usize = size_of::<*mut u8>();
        let size = if size < PTR_SIZE { PTR_SIZE } else { size };
        // バッファのメモリレイアウトを作成します。
        let align = size.next_power_of_two();
        let buff_size = align * len;
        let buff_align = buff_size.next_power_of_two();
        let layout = match Layout::from_size_align(buff_size, buff_align) {
            Ok(layout) => layout,
            Err(err) => return Err(Error::LayoutError(err)),
        };
        // バッファを作成します。
        let buffer = unsafe { alloc::alloc(layout) };
        // 未使用ポインタのリストを作成します。
        let mut list = null_mut::<*mut u8>();
        for i in 0..len {
            //
            // (buffer) [elem, elem, elem, elem] 
            //          |   ^  |  ^  |  ^  |  ^  
            //  null <--'   '--'  '--'  '--'  '-- (list)
            //
            unsafe {
                let lp = buffer.add(i * align); 
                let lpp = transmute::<*mut u8, *mut *mut u8>(lp);
                let rp = transmute::<*mut *mut u8, *mut u8>(list);
                *lpp = rp;
                list = lpp; 
            }
        }
        Ok(Pool { 
            layout, 
            buffer,
            list,
            use_cnt:0
        })
    }
    /// メモリを確保します。
    /// この操作はプール内の未使用要素が存在しないか判定しません。
    pub unsafe fn alloc(&mut self) -> *mut u8 {
        // メモリを取り出します。
        let ptr = self.list;
        let ptr = transmute::<*mut *mut u8, *mut u8>(ptr);
        // 未使用リストの先頭を進めます。
        self.list = transmute::<*mut u8, *mut *mut u8>(*self.list);
        // 使用カウントを進めます。
        self.use_cnt += 1;
        ptr
    }
    /// メモリを解放します。  
    /// この操作はポインタがこのプールから確保されたものか確認しません。
    /// 
    /// # Arguments
    /// 
    /// * `ptr` - 解放するポインタです。
    /// 
    pub unsafe fn dealloc(&mut self, ptr: *mut u8) {
        let ptr = transmute::<*mut u8, *mut *mut u8>(ptr);
        *ptr = transmute::<*mut *mut u8, *mut u8>(self.list);
        self.list = ptr;
        self.use_cnt -= 1;
    }
    /// このプールの未使用要素数が最大が判定します。
    pub fn is_full(&self) -> bool {
        self.use_cnt == 0
    }
    /// このプールの未使用要素が存在しないか判定します。
    pub fn is_empty(&self) -> bool {
        self.list == null_mut()
    }
    /// 要素のサイズを取得します。
    pub fn size(&self) -> usize {
        self.layout.size()
    }
    /// 要素の調整したサイズを取得します。
    pub fn align(&self) -> usize {
        self.layout.align()
    }
    /// 管理位置の順序指定IDを取得します。
    pub fn id(&self) -> usize {
        self.buffer as usize
    }
}
impl Drop for Pool {
    fn drop(&mut self) {
        unsafe { alloc::dealloc(self.buffer, self.layout) }
    }
}

/// エラーです。
#[derive(Debug)]
pub enum Error {
    /// サイズがありません。
    Unsized, 
    /// レイアウトエラーです。
    LayoutError(LayoutError),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unsized => f.write_str("サイズ、または、要素数が0です。"),
            Error::LayoutError(err) => write!(f, "LayoutError {}", err),
        }
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Unsized => None,
            Error::LayoutError(err) => err.source(),
        }
    }
}