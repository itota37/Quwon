// quwon_utility/src/lib.rs
// (C) 2022 Taichi Ito.
//! 他クレートに依存しないスタンドアロンな機能を提供します。

pub mod mem;

// テストです。
#[cfg(test)]
mod tests {

    use crate::mem;

    #[test]
    fn run() {
        println!("quwon_utility/src/lib.rs test start.");
        mem::tests::run();
        println!("quwon_utility/src/lib.rs test end.");
    }
}
