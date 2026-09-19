use ulua_ast::records::ast_type_pack::AstTypePack;

use crate::{
  functions::follow_type_pack::follow_type_pack_id, records::type_checker_2::TypeChecker2,
  type_aliases::type_pack_id::TypePackId,
};

impl TypeChecker2 {
  pub fn lookup_pack_annotation(&self, annotation: *mut AstTypePack) -> Option<TypePackId> {
    let tp = unsafe {
      (*self.module)
        .ast_resolved_type_packs
        .find(&(annotation as *const AstTypePack))
    };
    tp.map(|tp| unsafe { follow_type_pack_id(*tp) })
  }
}
