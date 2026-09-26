use alloc::vec::Vec;
use core::ptr::NonNull;

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

    for &type_or_pack in type_arguments.iter() {
      match type_or_pack {
        AstTypeOrPack::Type(ty) => {
          resolved_type_arguments.push(self.resolve_type(
            scope,
            NonNull::from(ty).as_ptr(),
            false,
            false,
            Polarity::Unknown,
          ));
        }
        AstTypeOrPack::Pack(pack) => {
          resolved_type_pack_arguments.push(
            self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
              scope,
              NonNull::from(pack).as_ptr(),
              false,
              false,
              Polarity::Unknown,
            ),
          );
        }
        // cpp 走 else 臂先 `LUAU_ASSERT(tp.typePack)` 再把 null 传进 resolveTypePack
        // （ConstraintGenerator.cpp:5714）；变体载荷恒非空，null 侧只剩断言上报，
        // 故此处只保留断言失败路径，不再向下游透传空槽。
        AstTypeOrPack::Error => LUAU_ASSERT!(false),
      }
    }

    (resolved_type_arguments, resolved_type_pack_arguments)
  }
}
