//! `VariantN::get_type_id<T>()`（C++ `getTypeId<T>`：返回 T 所处位置，未命中
//! 返回 -1）。7 个元数共享同一模式，由宏展开；比较顺序即选项声明序，与
//! C++ 逐条 `if` 链一致（首个命中胜出）。

use core::any::TypeId;

use crate::records::variant::{
  Variant1, Variant2, Variant3, Variant4, Variant5, Variant6, Variant7,
};

macro_rules! define_get_type_id {
  ($name:ident < $($t:ident),+ > { $($idx:literal => $ty:ident),+ $(,)? }) => {
    impl<$($t: 'static),+> $name<$($t),+> {
      pub fn get_type_id<T: 'static>() -> i32 {
        $(
          if TypeId::of::<T>() == TypeId::of::<$ty>() {
            return $idx;
          }
        )+
        -1
      }
    }
  };
}

define_get_type_id!(Variant1<T0> { 0 => T0 });
define_get_type_id!(Variant2<T0, T1> { 0 => T0, 1 => T1 });
define_get_type_id!(Variant3<T0, T1, T2> { 0 => T0, 1 => T1, 2 => T2 });
define_get_type_id!(Variant4<T0, T1, T2, T3> { 0 => T0, 1 => T1, 2 => T2, 3 => T3 });
define_get_type_id!(Variant5<T0, T1, T2, T3, T4> {
  0 => T0, 1 => T1, 2 => T2, 3 => T3, 4 => T4
});
define_get_type_id!(Variant6<T0, T1, T2, T3, T4, T5> {
  0 => T0, 1 => T1, 2 => T2, 3 => T3, 4 => T4, 5 => T5
});
define_get_type_id!(Variant7<T0, T1, T2, T3, T4, T5, T6> {
  0 => T0, 1 => T1, 2 => T2, 3 => T3, 4 => T4, 5 => T5, 6 => T6
});
