//! `VariantN::variant_t_enable_if_t_get_type_id_t`（C++ `Variant(T&&)` 的
//! TypeId 分发构造）。7 个元数共享同一模式，由宏展开。
//!
//! 顺序要点：先查 `get_type_id`；未命中（tid < 0）在 `forget` **之前** panic，
//! `value` 正常析构不泄漏；命中后 unsafe 仅覆盖按位重解释读取（`'static`
//! 类型上 TypeId 相等当且仅当类型同一），`forget` 防止双重析构。

use core::{mem::forget, ptr::read};

use crate::records::variant::{
  Variant1, Variant2, Variant3, Variant4, Variant5, Variant6, Variant7,
};

macro_rules! define_type_id_ctor {
  ($name:ident < $($t:ident),+ > { $($idx:literal => $v:ident < $ty:ident >),+ $(,)? }) => {
    impl<$($t: 'static),+> $name<$($t),+> {
      pub fn variant_t_enable_if_t_get_type_id_t<T: 'static>(value: T) -> Self {
        let tid = Self::get_type_id::<T>();
        if tid < 0 {
          panic!(concat!(stringify!($name), ": type not found in variant"));
        }
        unsafe {
          let ptr = &value as *const T;
          forget(value);
          match tid {
            $($idx => Self::$v(read(ptr as *const $ty)),)+
            // tid ∈ [0, 元数)，上方已排除 -1，此分支不可达。
            _ => core::hint::unreachable_unchecked(),
          }
        }
      }
    }
  };
}

define_type_id_ctor!(Variant1<T0> { 0 => V0<T0> });
define_type_id_ctor!(Variant2<T0, T1> { 0 => V0<T0>, 1 => V1<T1> });
define_type_id_ctor!(Variant3<T0, T1, T2> { 0 => V0<T0>, 1 => V1<T1>, 2 => V2<T2> });
define_type_id_ctor!(Variant4<T0, T1, T2, T3> {
  0 => V0<T0>, 1 => V1<T1>, 2 => V2<T2>, 3 => V3<T3>
});
define_type_id_ctor!(Variant5<T0, T1, T2, T3, T4> {
  0 => V0<T0>, 1 => V1<T1>, 2 => V2<T2>, 3 => V3<T3>, 4 => V4<T4>
});
define_type_id_ctor!(Variant6<T0, T1, T2, T3, T4, T5> {
  0 => V0<T0>, 1 => V1<T1>, 2 => V2<T2>, 3 => V3<T3>, 4 => V4<T4>, 5 => V5<T5>
});
define_type_id_ctor!(Variant7<T0, T1, T2, T3, T4, T5, T6> {
  0 => V0<T0>, 1 => V1<T1>, 2 => V2<T2>, 3 => V3<T3>, 4 => V4<T4>, 5 => V5<T5>,
  6 => V6<T6>
});
