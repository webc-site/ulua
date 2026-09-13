use ulua_common::{FFlag::LuauCstExprGroup, macros::luau_assert::LUAU_ASSERT};

use crate::{
  records::{cst_expr_group::CstExprGroup, cst_node::CstNode, position::Position},
  rtti::CstNodeClass,
};

impl CstExprGroup {
  pub fn new(close_position: Position) -> Self {
    LUAU_ASSERT!(LuauCstExprGroup.get());

    Self {
      base: CstNode {
        class_index: Self::CLASS_INDEX,
      },
      close_position,
    }
  }
}

pub fn cst_expr_group_cst_expr_group(close_position: Position) -> CstExprGroup {
  CstExprGroup::new(close_position)
}
