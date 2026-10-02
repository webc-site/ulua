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

/// 浮点的 `buffer_swapbe`：cpp 调用点（`lbuflib.cpp:146-198` 的 readfp/writefp）从不把
/// 浮点直接喂给 `buffer_swapbe<T>`，而是经同宽整型 `StorageType`（f32↔u32、f64↔u64）
/// 中转：`memcpy` 出整型位模式 → `htole32/htole64` 翻转字节 → `memcpy` 回浮点。
/// 本宏逐位复刻该路径：`to_bits`/`from_bits` 是 IEEE-754 位模式与整型的双射
/// （任意位模式含 NaN 均为合法值，与 `static_cast<StorageType>` 前的 memcpy 同义），
/// 翻转落在对应无符号整型宽度上，两端 `sizeof(T) == sizeof(StorageType)`
/// （cpp `static_assert`）由 `$bits` 选型在编译期固化。
macro_rules! impl_swap_be_float {
  ($($t:ty => $bits:ty),* $(,)?) => {
    $(
      impl Sealed for $t {}

      impl SwapBe for $t {
        #[inline(always)]
        fn swap_be(self) -> Self {
          Self::from_bits(<$bits>::swap_bytes(self.to_bits()))
        }
      }
    )*
  };
}

impl_swap_be_float!(f32 => u32, f64 => u64);

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
