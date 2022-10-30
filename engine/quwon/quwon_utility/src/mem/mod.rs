// quwon_utility/src/mem/lib.rs
// (C) 2022 Taichi Ito.
//! ゲーム用の高速なメモリを提供します。

pub mod pool;
pub mod fixed;
pub mod global;

// テストです。
#[cfg(test)]
pub mod tests {

    use crate::mem::{
        pool,
        global
    };

    #[test]
    pub fn run() {
        println!("quwon_utility/src/mem/lib.rs test start.");
        pool::tests::run();
        global::tests::run();
        println!("quwon_utility/src/mem/lib.rs test end.");
    }
}