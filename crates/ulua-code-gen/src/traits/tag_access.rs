//! 寄存器 tag 读写抽象。
//!
//! C++ `propagateTagsFromPredecessors` 以 `std::function` 接收 get/set 闭包；
//! Rust 以 trait 静态分发替代，两个调用方状态各自实现，避免堆分配与裸指针。

pub trait TagAccess {
  fn get_tag(&self, i: usize) -> u8;
  fn set_tag(&mut self, i: usize, tag: u8);
}
