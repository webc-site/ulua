// GC 位同族常量统一以 `whitebits`/`blackbit`（i32，与 bitmask/bit2mask 同域）为准，
// 此处仅转发保持旧路径可用，并保留 C++ lgc.h `maskmarks` 宏的等价物
pub use crate::macros::{
  blackbit::BLACKBIT,
  whitebits::{WHITE0BIT, WHITE1BIT, WHITEBITS},
};

#[macro_export]
macro_rules! maskmarks {
  () => {
    (!($crate::macros::bitmask::bitmask($crate::macros::maskmarks::BLACKBIT)
      | $crate::macros::maskmarks::WHITEBITS)) as u8
  };
}

pub use maskmarks;
