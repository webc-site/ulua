use alloc::vec::Vec;

use ulua_ast::records::{ast_array::AstArray, ast_type_or_pack::AstTypeOrPack};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  records::{constraint_generator::ConstraintGenerator, scope::Scope},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl ConstraintGenerator {
  pub fn resolve_type_arguments(
    &mut self,
    scope: *mut Scope,
    type_arguments: AstArray<AstTypeOrPack>,
  ) -> (Vec<TypeId>, Vec<TypePackId>) {
    let mut resolved_type_arguments = Vec::new();
    let mut resolved_type_pack_arguments = Vec::new();

    for type_or_pack in type_arguments.iter() {
      if !type_or_pack.r#type.is_null() {
        resolved_type_arguments.push(self.resolve_type(
          scope,
          type_or_pack.r#type,
          false,
          false,
          Polarity::Unknown,
        ));
      } else {
        LUAU_ASSERT!(!type_or_pack.type_pack.is_null());
        resolved_type_pack_arguments.push(
          self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
            scope,
            type_or_pack.type_pack,
            false,
            false,
            Polarity::Unknown,
          ),
        );
      }
    }

    (resolved_type_arguments, resolved_type_pack_arguments)
  }
}
