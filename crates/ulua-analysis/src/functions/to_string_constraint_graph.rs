//! Source: `Analysis/src/ConstraintGraph.cpp:469-487` (hand-ported)

use alloc::{format, string::String};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::to_string_to_string::{
    to_string_constraint_to_string_options, to_string_type_id_to_string_options,
    to_string_type_pack_id_to_string_options_mut,
  },
  records::{blocked_constraint_registry::resolve_constraint, to_string_options::ToStringOptions},
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
/// C++ anonymous-namespace `std::string to_string(ConstraintVertex vertex)` —
/// `Luau::visit(overloaded{...}, vertex)` becomes a match over the Variant3.
pub unsafe fn to_string(vertex: BlockedConstraintId) -> String {
  match vertex {
    BlockedConstraintId::V0(ty) => {
      let mut opts = ToStringOptions {
        exhaustive: true,
        ..Default::default()
      };
      format!(
        "Type {}",
        to_string_type_id_to_string_options(ty, &mut opts)
      )
    }
    BlockedConstraintId::V1(tp) => {
      let opts = ToStringOptions {
        exhaustive: true,
        ..Default::default()
      };
      format!(
        "Type pack {}",
        to_string_type_pack_id_to_string_options_mut(tp, opts)
      )
    }
    BlockedConstraintId::V2(c) => {
      let mut opts = ToStringOptions {
        exhaustive: true,
        ..Default::default()
      };
      // §2 收口：V2 现携带 ConstraintId 句柄，经 registry 解析而非裸解引用。
      // NULL 句柄对应旧 null() 哨兵键，该打印路径在 C++ 原实现中即解引用
      // 空指针（UB），此处以断言 + 占位符取代，正常流程不可达。
      match resolve_constraint(c) {
        Some(node) => format!(
          "Cons {}",
          to_string_constraint_to_string_options(node, &mut opts)
        ),
        None => {
          LUAU_ASSERT!(false);
          "Cons <null>".into()
        }
      }
    }
  }
}

pub use to_string as to_string_constraint_vertex;
