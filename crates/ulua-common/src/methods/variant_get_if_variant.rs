//! `VariantN::get_if<T>()`（C++ `std::get_if<T>` 按 TypeId 分发）。7 个元数
//! 共享同一模式，由宏展开。
//!
//! SAFETY（展开体内全部 `&*(v as *const Ti as *const T)` 转换的统一依据）：
//! `get_type_id::<T>() == index()` 已同时证明 T 与活跃选项 Ti 是同一类型
//!（'static 类型上 TypeId 相等当且仅当类型同一），故按位重解释引用合法；
//! 两个条件任一不满足都会在进入 unsafe 前以 `None` 返回。

use crate::records::variant::{
  Variant1, Variant2, Variant3, Variant4, Variant5, Variant6, Variant7,
};

macro_rules! define_get_if {
  // `$v` 是该位置的 getter（`get_if_N`），`$ty` 是对应选项类型。
  ($name:ident < $($t:ident),+ > { $($idx:literal => $v:ident < $ty:ident >),+ $(,)? }) => {
    impl<$($t: 'static),+> $name<$($t),+> {
      /// Returns `Some(&T)` if the active alternative has type `T`, else `None`.
      pub fn get_if<T: 'static>(&self) -> Option<&T> {
        let tid = Self::get_type_id::<T>();
        // tid < 0：T 不在变体选项内；index 不符：T 对应选项非活跃。
        if tid < 0 || self.index() != tid as usize {
          return None;
        }
        match tid as usize {
          $(
            // SAFETY：TypeId 相等 + 活跃匹配，按位重解释引用合法（见模块注释）。
            $idx => self.$v().map(|v| unsafe { &*(v as *const $ty as *const T) }),
          )+
          // tid ∈ [0, 元数) 且 index() == tid，此分支不可达。
          _ => None,
        }
      }
    }
  };
}

define_get_if!(Variant1<T0> { 0 => get_if_0<T0> });
define_get_if!(Variant2<T0, T1> { 0 => get_if_0<T0>, 1 => get_if_1<T1> });
define_get_if!(Variant3<T0, T1, T2> {
  0 => get_if_0<T0>, 1 => get_if_1<T1>, 2 => get_if_2<T2>
});
define_get_if!(Variant4<T0, T1, T2, T3> {
  0 => get_if_0<T0>, 1 => get_if_1<T1>, 2 => get_if_2<T2>, 3 => get_if_3<T3>
});
define_get_if!(Variant5<T0, T1, T2, T3, T4> {
  0 => get_if_0<T0>, 1 => get_if_1<T1>, 2 => get_if_2<T2>, 3 => get_if_3<T3>,
  4 => get_if_4<T4>
});
define_get_if!(Variant6<T0, T1, T2, T3, T4, T5> {
  0 => get_if_0<T0>, 1 => get_if_1<T1>, 2 => get_if_2<T2>, 3 => get_if_3<T3>,
  4 => get_if_4<T4>, 5 => get_if_5<T5>
});
define_get_if!(Variant7<T0, T1, T2, T3, T4, T5, T6> {
  0 => get_if_0<T0>, 1 => get_if_1<T1>, 2 => get_if_2<T2>, 3 => get_if_3<T3>,
  4 => get_if_4<T4>, 5 => get_if_5<T5>, 6 => get_if_6<T6>
});
