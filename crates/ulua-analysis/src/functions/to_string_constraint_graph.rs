//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ConstraintGraph.cpp:469:to_string`
//! Source: `Analysis/src/ConstraintGraph.cpp:469-487` (hand-ported)

use alloc::{format, string::String};

use crate::{
  functions::{
    to_string_to_string::to_string_type_pack_id_to_string_options_mut,
    to_string_to_string_alt_m::to_string_type_id_to_string_options,
    to_string_to_string_alt_q::to_string_constraint_to_string_options,
  },
  records::to_string_options::ToStringOptions,
  type_aliases::constraint_vertex::ConstraintVertex,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
/// C++ anonymous-namespace `std::string to_string(ConstraintVertex vertex)` —
/// `Luau::visit(overloaded{...}, vertex)` becomes a match over the Variant3.
pub unsafe fn to_string(vertex: ConstraintVertex) -> String {
  unsafe {
    match vertex {
      ConstraintVertex::V0(ty) => {
        let mut opts = ToStringOptions {
          exhaustive: true,
          ..Default::default()
        };
        format!(
          "Type {}",
          to_string_type_id_to_string_options(ty, &mut opts)
        )
      }
      ConstraintVertex::V1(tp) => {
        let opts = ToStringOptions {
          exhaustive: true,
          ..Default::default()
        };
        format!(
          "Type pack {}",
          to_string_type_pack_id_to_string_options_mut(tp, opts)
        )
      }
      ConstraintVertex::V2(c) => {
        let mut opts = ToStringOptions {
          exhaustive: true,
          ..Default::default()
        };
        format!(
          "Cons {}",
          to_string_constraint_to_string_options(&*c, &mut opts)
        )
      }
    }
  }
}

pub use to_string as to_string_constraint_vertex;
