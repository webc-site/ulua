//! Faithful port of Luau's `Variant<Ts...>` — a `std::variant`-like tagged
//! union. Reference: `luau/Common/include/Luau/Variant.h`；行为对照
//! `luau/tests/Variant.test.cpp`（DefaultCtor, Create, Emplace, NonPOD copy,
//! Equality, Visit）。
//!
//! Rust has no variadic generics, so the C++ variadic `Variant<Ts...>` becomes a
//! fixed-arity enum family `Variant1<T0> .. Variant7<..>` (the arities Luau
//! actually instantiates; max is 7). A Rust `enum` *is* a tagged union, so this
//! is safe (no type-erased storage, no fn-pointer dispatch tables) and idiomatic.
//!
//! Mechanical mapping for callers (e.g. the eventual Analysis port):
//! - `Variant<A, B, C>`            -> `Variant3<A, B, C>`
//! - `Variant<A,B> x = a;`         -> `Variant2::V0(a)` (no blanket `From<Ti>` —
//!   Rust coherence forbids it since `T0` could equal `T1`; construct the variant
//!   directly at the position the type occupies)
//! - `v.get_if<B>()` (B is pos 1)  -> `v.get_if_1()` / `v.get_if_1_mut()`
//! - `v.emplace<B>(args)`          -> `v = Variant2::V1(B::from(args))`
//! - `v.index()`                   -> `v.index()`
//! - `visit(overloaded{...}, v)`   -> `match v { Variant3::V0(x) => …, … }`
//!
//! `==` (C++ `operator==`) and `Default` (C++ `Variant()` -> first alternative)
//! come from the derives + the generated first-alternative `Default` impl;
//! `valueless_by_exception()` is always `false`. b28 裁定 (b)：该方法是
//! cpp `std::variant` 公共面的忠实移植镜像工件（sync-cpp 对照维护），零调用
//! 不构成降级/删除理由，保留 `pub` 定形。
//!
//! **DELIBERATE DEVIATION**（出处 `Variant.h:23-36,119-136`）：按类型查找的
//! `get_type_id<T>()` / `get_if::<T>()` 两处与上游不同，均为 Rust 语义所迫且方向保守：
//! - `T` 不在选项集内：cpp 是编译期 `static_assert(tid >= 0, "unsupported T")`，
//!   Rust 无编译期类型集判定，`get_type_id` 返回 `-1`、`get_if` 返回 `None`
//!   （运行期不可命中，不会静默返回错值）。
//! - 取回引用：cpp 用 `reinterpret_cast<const T*>(&storage)` 直接重解释未定型存储，
//!   Rust 先以 `TypeId` 相等证明活跃选项就是 `T`，再走 `dyn Any` 安全向下转型
//!   （§2/§4：unsafe 与裸指针不外泄到业务层）。

use core::any::{Any, TypeId};

/// 生成 `VariantN` 的两个按类型查找方法（cpp `Variant.h` 的 `getTypeId<T>` 与
/// `std::get_if<T>`）：7 个元数共用同一形态，由 `define_variant*!` 内部调用，
/// 不再各自展开一份宏（原 `methods/variant_get_*.rs` 两枚碎片 + 21 次调用收口为此
/// 1 个宏 + 8 次内联调用）。
macro_rules! impl_variant_lookup {
  ($name:ident < $($t:ident),+ > { $($idx:literal => $g:ident < $ty:ident >),+ $(,)? }) => {
    impl<$($t: 'static),+> $name<$($t),+> {
      /// cpp `getTypeId<T>`：返回 `T` 所处的选项位置，未命中返回 `-1`。比较顺序即
      /// 选项声明序，首个命中胜出，与 C++ 的逐条 `if` 链一致。
      pub fn get_type_id<T: 'static>() -> i32 {
        $(
          if TypeId::of::<T>() == TypeId::of::<$ty>() {
            return $idx;
          }
        )+
        -1
      }

      /// cpp `std::get_if<T>(&variant)`：活跃选项类型为 `T` 时返回其引用，否则 `None`。
      ///
      /// 分发依据：`get_type_id::<T>() == index()` 已同时证明 `T` 与活跃选项是同一
      /// 类型（`'static` 类型上 TypeId 相等当且仅当类型同一），故 `dyn Any` 向下转型
      /// 必成；两个条件任一不满足都会在转型前以 `None` 返回，全程安全代码。
      pub fn get_if<U: 'static>(&self) -> Option<&U> {
        let tid = Self::get_type_id::<U>();
        // tid < 0：U 不在变体选项内；index 不符：U 对应选项非活跃。
        if tid < 0 || self.index() != tid as usize {
          return None;
        }
        match tid as usize {
            $($idx => self.$g().and_then(|v| (v as &dyn Any).downcast_ref::<U>()),)+
            // tid ∈ [0, 元数) 且 index() == tid，此分支不可达。
            _ => None,
        }
      }
    }
  };
}

macro_rules! define_variant1 {
  (
        $name:ident < $t0:ident >
        = $v0:ident($g0:ident, $g0m:ident)
    ) => {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum $name<$t0> {
      $v0($t0),
    }

    impl<$t0> $name<$t0> {
      pub fn index(&self) -> usize {
        0
      }

      pub fn valueless_by_exception(&self) -> bool {
        false
      }

      pub fn $g0(&self) -> Option<&$t0> {
        let Self::$v0(x) = self;
        Some(x)
      }
      pub fn $g0m(&mut self) -> Option<&mut $t0> {
        let Self::$v0(x) = self;
        Some(x)
      }
    }

    impl_variant_lookup!($name<$t0> { 0 => $g0<$t0> });

    impl<$t0: Default> Default for $name<$t0> {
      fn default() -> Self {
        Self::$v0(<$t0 as Default>::default())
      }
    }
  };
}

/// Generates one `VariantN` enum plus its `index`/`get_if_*`/`Default` API.
macro_rules! define_variant {
    (
        $name:ident < $t0:ident $(, $t:ident)* >
        = $v0:ident($g0:ident, $g0m:ident)
        $(, $idx:literal : $v:ident < $ty:ident > ($g:ident, $gm:ident) )*
    ) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum $name<$t0 $(, $t)*> {
            $v0($t0),
            $( $v($ty), )*
        }

        impl<$t0 $(, $t)*> $name<$t0 $(, $t)*> {
            /// `index()` / `typeId` — the active alternative's position.
            pub fn index(&self) -> usize {
                match self {
                    Self::$v0(_) => 0,
                    $( Self::$v(_) => $idx, )*
                }
            }

            /// Always `false` (this port has no valueless state). Matches the C++
            /// `valueless_by_exception`.
            pub fn valueless_by_exception(&self) -> bool {
                false
            }

            pub fn $g0(&self) -> Option<&$t0> {
                match self {
                    Self::$v0(x) => Some(x),
                    _ => None,
                }
            }
            pub fn $g0m(&mut self) -> Option<&mut $t0> {
                match self {
                    Self::$v0(x) => Some(x),
                    _ => None,
                }
            }
            $(
                pub fn $g(&self) -> Option<&$ty> {
                    match self {
                        Self::$v(x) => Some(x),
                        _ => None,
                    }
                }
                pub fn $gm(&mut self) -> Option<&mut $ty> {
                    match self {
                        Self::$v(x) => Some(x),
                        _ => None,
                    }
                }
            )*
        }

        impl_variant_lookup!(
            $name<$t0 $(, $t)*> {
                0 => $g0<$t0> $(, $idx => $g<$ty>)*
            }
        );

        // C++ `Variant()` 默认构造第一个选项。
        impl<$t0: Default $(, $t)*> Default for $name<$t0 $(, $t)*> {
            fn default() -> Self {
                Self::$v0(<$t0 as Default>::default())
            }
        }
    };
}

define_variant1!(Variant1<T0> = V0(get_if_0, get_if_0_mut));
define_variant!(
    Variant2<T0, T1> = V0(get_if_0, get_if_0_mut),
    1: V1<T1>(get_if_1, get_if_1_mut)
);
define_variant!(
    Variant3<T0, T1, T2> = V0(get_if_0, get_if_0_mut),
    1: V1<T1>(get_if_1, get_if_1_mut),
    2: V2<T2>(get_if_2, get_if_2_mut)
);
define_variant!(
    Variant4<T0, T1, T2, T3> = V0(get_if_0, get_if_0_mut),
    1: V1<T1>(get_if_1, get_if_1_mut),
    2: V2<T2>(get_if_2, get_if_2_mut),
    3: V3<T3>(get_if_3, get_if_3_mut)
);
define_variant!(
    Variant5<T0, T1, T2, T3, T4> = V0(get_if_0, get_if_0_mut),
    1: V1<T1>(get_if_1, get_if_1_mut),
    2: V2<T2>(get_if_2, get_if_2_mut),
    3: V3<T3>(get_if_3, get_if_3_mut),
    4: V4<T4>(get_if_4, get_if_4_mut)
);
define_variant!(
    Variant6<T0, T1, T2, T3, T4, T5> = V0(get_if_0, get_if_0_mut),
    1: V1<T1>(get_if_1, get_if_1_mut),
    2: V2<T2>(get_if_2, get_if_2_mut),
    3: V3<T3>(get_if_3, get_if_3_mut),
    4: V4<T4>(get_if_4, get_if_4_mut),
    5: V5<T5>(get_if_5, get_if_5_mut)
);
define_variant!(
    Variant7<T0, T1, T2, T3, T4, T5, T6> = V0(get_if_0, get_if_0_mut),
    1: V1<T1>(get_if_1, get_if_1_mut),
    2: V2<T2>(get_if_2, get_if_2_mut),
    3: V3<T3>(get_if_3, get_if_3_mut),
    4: V4<T4>(get_if_4, get_if_4_mut),
    5: V5<T5>(get_if_5, get_if_5_mut),
    6: V6<T6>(get_if_6, get_if_6_mut)
);
