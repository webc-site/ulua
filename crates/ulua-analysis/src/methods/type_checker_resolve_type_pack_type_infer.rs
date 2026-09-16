use alloc::vec::Vec;

use ulua_ast::records::ast_type_list::AstTypeList;

use crate::{
  records::{type_checker::TypeChecker, type_pack::TypePack},
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};
impl TypeChecker {
  pub fn resolve_type_pack_scope_ptr_ast_type_list(
    &mut self,
    scope: ScopePtr,
    types: &AstTypeList,
  ) -> TypePackId {
    if types.types.size == 0 && !types.tail_type.is_null() {
      return self.resolve_type_pack_scope_ptr_ast_type_pack(scope, unsafe { &*types.tail_type });
    } else if types.types.size > 0 {
      let mut head = Vec::with_capacity(types.types.size);
      for &ann in types.types.as_slice() {
        let ty = self.resolve_type(scope.clone(), unsafe { &*ann });
        head.push(ty);
      }

      let tail = if !types.tail_type.is_null() {
        Some(
          self
            .resolve_type_pack_scope_ptr_ast_type_pack(scope.clone(), unsafe { &*types.tail_type }),
        )
      } else {
        None
      };

      return self.add_type_pack_type_pack(TypePack { head, tail });
    }

    self.add_type_pack_type_pack(TypePack {
      head: Vec::new(),
      tail: None,
    })
  }
}
