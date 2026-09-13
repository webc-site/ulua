use ulua_ast::records::allocator::Allocator;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    type_pack_rehydration_visitor::TypePackRehydrationVisitor,
    type_rehydration_visitor::TypeRehydrationVisitor,
  },
  type_aliases::synthetic_names::SyntheticNames,
};

impl TypePackRehydrationVisitor {
  pub fn type_pack_rehydration_visitor_type_pack_rehydration_visitor(
    allocator: *mut Allocator,
    synthetic_names: *mut SyntheticNames,
    type_visitor: *mut TypeRehydrationVisitor,
  ) -> Self {
    LUAU_ASSERT!(!allocator.is_null());
    LUAU_ASSERT!(!synthetic_names.is_null());
    LUAU_ASSERT!(!type_visitor.is_null());

    Self {
      allocator,
      synthetic_names,
      type_visitor,
    }
  }
}
