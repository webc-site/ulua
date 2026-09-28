//! buffer 定宽元素的字节序与截断语义（cpp `lbuflib.cpp` 的 `buffer_swapbe<T>`
//! 模板与 `T val = T(value)` 对应）。

mod private {
  /// 密封父 trait：buffer 元素只能是本模块列出的定宽标量
  pub trait Sealed {}
}

use private::Sealed;

/// cpp `buffer_swapbe<T>`：按类型宽度翻转字节序（仅大端 cfg 下调用）。
///
/// 编译期分发到各类型自己的 `swap_bytes`，无运行时 `size_of` 分支、
/// 无跨类型 `transmute_copy`。
pub trait SwapBe: Sealed + Copy {
  fn swap_be(self) -> Self;
}

/// cpp `T val = T(value)`（`lbuflib.cpp:98`）：从 u32 做**数值**截断，
/// 始终保留低位（与字节序无关）；`transmute_copy` 在大端下会取到高位。
pub trait BufferInt: SwapBe {
  fn from_u32_trunc(value: u32) -> Self;
}

macro_rules! impl_swap_be {
  ($($t:ty),* $(,)?) => {
    $(
      impl Sealed for $t {}

      impl SwapBe for $t {
        #[inline(always)]
        fn swap_be(self) -> Self {
          self.swap_bytes()
        }
      }
    )*
  };
}

impl_swap_be!(i8, u8, i16, u16, i32, u32, i64, u64);

macro_rules! impl_buffer_int {
  ($($t:ty),* $(,)?) => {
    $(
      impl BufferInt for $t {
        #[inline(always)]
        fn from_u32_trunc(value: u32) -> Self {
          // `as` 截断低位并按补码解释，与 cpp `T(value)` 同语义
          value as $t
        }
      }
    )*
  };
}

impl_buffer_int!(i8, u8, i16, u16, i32);

impl BufferInt for u32 {
  #[inline(always)]
  fn from_u32_trunc(value: u32) -> Self {
    value
  }
}
