use ulua_common::{fflag::DebugLuauUserDefinedClasses, macros::luau_assert::LUAU_ASSERT};

use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_local::AstLocal, ast_stat::AstStat,
    ast_stat_class::AstStatClass, location::Location,
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
      base: AstStat::new(<Self as AstNodeClass>::CLASS_INDEX, location),
      name,
      super_,
      members,
      exported,
      open,
    }
  }
}
