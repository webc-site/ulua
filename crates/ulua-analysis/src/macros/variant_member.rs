//! cpp `Variant<Ts...>` 落点的 `get_if<T>` 门面脚手架单点。
//!
//! C++ 侧 `get_if<T>(variant)` 靠**模板重载**逐个成员型生成；本仓库的 Rust 直译
//! 于是给每个 variant 配一枚 `*Member` trait（`get_if`/`get_if_mut`）加一份逐成员
//! 实现。原先这套形状在 16 个模块里各写一遍：具名 enum 侧是 9 份逐字相同的
//! `macro_rules! *_member!`（各 ~18 行），定元 `VariantN` 别名侧是 7 份手写的
//! `impl XMember for T { v.get_if_k() }`（每条 ~9 行）。本宏把它收为单一门面，
//! 调用点只剩「成员表」本身，成员与槽位的对应关系一眼可核。
//!
//! 两种形态按 cpp 落点区分，语义与收口前逐字等价：
//! - [`variant_member!`] 的 `enum` 形态：本地具名 enum（成员数超过 `Variant7`、
//!   或需要具名变体的先例，如 `TypeVariant`），按**变体名**取槽位；
//! - [`variant_member!`] 的 `position` 形态：`ulua_common::records::variant` 的
//!   定元别名（`Variant2<..>` 等），按**位置方法** `get_if_k`/`get_if_k_mut` 转发。
//!
//! trait 定义（含原 doc 注释）一并由宏生成，路径与可见性不变，故所有
//! `use crate::type_aliases::…::XMember` 与 `<T as XMember>::get_if` 调用点零改动。

/// 生成 `*Member` trait 及其逐成员 `get_if`/`get_if_mut` 实现。
///
/// 用法（`enum` 形态，`from` 关键字追加 cpp 隐式转换的 `From` 面）：
/// ```ignore
/// variant_member! {
///   /// 原 doc 注释按行传入，文本不变。
///   enum ConstraintVMember: ConstraintV {
///     Subtype => SubtypeConstraint,
///     // …
///   }
/// }
/// ```
/// 用法（`position` 形态）：
/// ```ignore
/// variant_member! {
///   position SingletonVariantMember: SingletonVariant {
///     0 => BooleanSingleton,
///     1 => StringSingleton,
///   }
/// }
/// ```
macro_rules! variant_member {
  // 具名 enum：按变体名取槽位。
  (
    $(#[$attr:meta])*
    enum $trait:ident: $enum:ident {
      $($variant:ident => $member:ty),* $(,)?
    }
  ) => {
    $(#[$attr])*
    pub trait $trait: Sized {
      fn get_if(v: &$enum) -> Option<&Self>;
      fn get_if_mut(v: &mut $enum) -> Option<&mut Self>;
    }

    $(
      impl $trait for $member {
        fn get_if(v: &$enum) -> Option<&Self> {
          match v {
            $enum::$variant(x) => Some(x),
            _ => None,
          }
        }
        fn get_if_mut(v: &mut $enum) -> Option<&mut Self> {
          match v {
            $enum::$variant(x) => Some(x),
            _ => None,
          }
        }
      }
    )*
  };

  // 具名 enum + cpp `Variant` 到本类型的隐式转换面。
  (
    $(#[$attr:meta])*
    enum from $trait:ident: $enum:ident {
      $($variant:ident => $member:ty),* $(,)?
    }
  ) => {
    $crate::macros::variant_member! {
      $(#[$attr])*
      enum $trait: $enum { $($variant => $member),* }
    }

    $(
      impl From<$member> for $enum {
        fn from(value: $member) -> Self {
          $enum::$variant(value)
        }
      }
    )*
  };

  // 定元 `VariantN` 别名：按位置方法转发。
  (
    $(#[$attr:meta])*
    position $trait:ident: $enum:ident {
      $($index:tt => $member:ty),* $(,)?
    }
  ) => {
    $(#[$attr])*
    pub trait $trait: Sized {
      fn get_if(v: &$enum) -> Option<&Self>;
      fn get_if_mut(v: &mut $enum) -> Option<&mut Self>;
    }

    $(
      impl $trait for $member {
        fn get_if(v: &$enum) -> Option<&Self> {
          $crate::macros::variant_member!(@slot v, $index)
        }
        fn get_if_mut(v: &mut $enum) -> Option<&mut Self> {
          $crate::macros::variant_member!(@slot_mut v, $index)
        }
      }
    )*
  };

  // 位置方法名无法由字面量拼接（macro_rules 无 ident 合成），按 cpp 实际使用的
  // 元数 1..=7 列举。
  (@slot $v:expr, 0) => { $v.get_if_0() };
  (@slot $v:expr, 1) => { $v.get_if_1() };
  (@slot $v:expr, 2) => { $v.get_if_2() };
  (@slot $v:expr, 3) => { $v.get_if_3() };
  (@slot $v:expr, 4) => { $v.get_if_4() };
  (@slot $v:expr, 5) => { $v.get_if_5() };
  (@slot $v:expr, 6) => { $v.get_if_6() };

  (@slot_mut $v:expr, 0) => { $v.get_if_0_mut() };
  (@slot_mut $v:expr, 1) => { $v.get_if_1_mut() };
  (@slot_mut $v:expr, 2) => { $v.get_if_2_mut() };
  (@slot_mut $v:expr, 3) => { $v.get_if_3_mut() };
  (@slot_mut $v:expr, 4) => { $v.get_if_4_mut() };
  (@slot_mut $v:expr, 5) => { $v.get_if_5_mut() };
  (@slot_mut $v:expr, 6) => { $v.get_if_6_mut() };
}

pub(crate) use variant_member;

/// 生成具名 enum、`index(&self) -> i32`、`*Member` trait 及其逐成员 `get_if`/`get_if_mut` 实现（可选 `From`）。
///
/// 消除 enum 变体声明、`index()` 映射与 `variant_member!` 之间的 3 重重复罗列。
macro_rules! variant_enum {
  // 1. 展开核心：无 from，分离的 enum 与 trait 声明
  (
    $(#[$enum_attr:meta])*
    $vis:vis enum $enum:ident {
      $($variant:ident => $member:ty),* $(,)?
    }
    $(#[$trait_attr:meta])*
    $trait_vis:vis trait $trait:ident;
  ) => {
    $(#[$enum_attr])*
    $vis enum $enum {
      $(
        $variant($member),
      )*
    }

    impl $enum {
      /// C++ `v.index()` — the member's position in the Variant<...> list.
      pub fn index(&self) -> i32 {
        $crate::macros::variant_enum!(@index_match self, $enum, ($($variant)*), (
          0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
          10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
          20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
          30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
          40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
          50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
          60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
          70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
          80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
          90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
          100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
          110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
          120, 121, 122, 123, 124, 125, 126, 127
        ))
      }
    }

    $crate::macros::variant_member! {
      $(#[$trait_attr])*
      enum $trait: $enum {
        $($variant => $member),*
      }
    }
  };

  // 2. 展开核心：带 from，分离的 enum 与 trait 声明
  (
    $(#[$enum_attr:meta])*
    $vis:vis enum from $enum:ident {
      $($variant:ident => $member:ty),* $(,)?
    }
    $(#[$trait_attr:meta])*
    $trait_vis:vis trait $trait:ident;
  ) => {
    $(#[$enum_attr])*
    $vis enum $enum {
      $(
        $variant($member),
      )*
    }

    impl $enum {
      /// C++ `v.index()` — the member's position in the Variant<...> list.
      pub fn index(&self) -> i32 {
        $crate::macros::variant_enum!(@index_match self, $enum, ($($variant)*), (
          0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
          10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
          20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
          30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
          40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
          50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
          60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
          70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
          80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
          90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
          100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
          110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
          120, 121, 122, 123, 124, 125, 126, 127
        ))
      }
    }

    $crate::macros::variant_member! {
      $(#[$trait_attr])*
      enum from $trait: $enum {
        $($variant => $member),*
      }
    }
  };

  // 3. 冒号简写：带 from
  (
    $(#[$enum_attr:meta])*
    $vis:vis enum from $enum:ident: $trait:ident {
      $($variant:ident => $member:ty),* $(,)?
    }
  ) => {
    $crate::macros::variant_enum! {
      $(#[$enum_attr])*
      $vis enum from $enum {
        $($variant => $member),*
      }
      /// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
      pub trait $trait;
    }
  };

  // 4. 冒号简写：无 from
  (
    $(#[$enum_attr:meta])*
    $vis:vis enum $enum:ident: $trait:ident {
      $($variant:ident => $member:ty),* $(,)?
    }
  ) => {
    $crate::macros::variant_enum! {
      $(#[$enum_attr])*
      $vis enum $enum {
        $($variant => $member),*
      }
      /// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
      pub trait $trait;
    }
  };

  // 内部辅助：匹配展开
  (@index_match $self:expr, $enum:ident, ($($variants:ident)*), ($($indices:tt),*)) => {
    $crate::macros::variant_enum!(@index_arms $self, $enum, ($($variants)*), ($($indices)*) -> ())
  };

  // 内部递归基点：变体消耗完毕
  (@index_arms $self:expr, $enum:ident, (), ($($indices:tt)*) -> ($($arms:tt)*)) => {
    match $self {
      $($arms)*
    }
  };

  // 内部递归步长：每次消耗一个变体和一个索引
  (@index_arms $self:expr, $enum:ident, ($v_head:ident $($v_tail:ident)*), ($i_head:tt $($i_tail:tt)*) -> ($($arms:tt)*)) => {
    $crate::macros::variant_enum!(@index_arms $self, $enum, ($($v_tail)*), ($($i_tail)*) -> (
      $($arms)*
      $enum::$v_head(_) => $i_head,
    ))
  };
}

pub(crate) use variant_enum;
