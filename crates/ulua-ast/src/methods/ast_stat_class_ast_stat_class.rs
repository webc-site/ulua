use ulua_common::{FFlag::DebugLuauUserDefinedClasses, macros::luau_assert::LUAU_ASSERT};

use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_local::AstLocal, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_class::AstStatClass, location::Location,
  },
  rtti::AstNodeClass,
  type_aliases::ast_class_member::AstClassMember,
};

impl AstStatClass {
  pub fn new(
    location: Location,
    name: *mut AstLocal,
    super_: *mut AstExpr,
    members: AstArray<AstClassMember>,
    exported: bool,
    open: bool,
  ) -> Self {
    LUAU_ASSERT!(DebugLuauUserDefinedClasses.get());
    Self {
      base: AstStat {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      name,
      super_,
      members,
      exported,
      open,
    }
  }
}

pub fn ast_stat_class_ast_stat_class(
  location: Location,
  name: *mut AstLocal,
  super_: *mut AstExpr,
  members: AstArray<AstClassMember>,
  exported: bool,
  open: bool,
) -> AstStatClass {
  AstStatClass::new(location, name, super_, members, exported, open)
}
