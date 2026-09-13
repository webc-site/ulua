use ulua_ast::records::ast_type_pack::AstTypePack;

use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  records::non_strict_type_checker::NonStrictTypeChecker, type_aliases::type_pack_id::TypePackId,
};

impl NonStrictTypeChecker {
  pub fn lookup_pack_annotation(&self, annotation: *mut AstTypePack) -> Option<TypePackId> {
    let module = unsafe { &*self.module };
    let tp = module
      .ast_resolved_type_packs
      .find(&(annotation as *const AstTypePack));
    tp.map(|tp| unsafe { follow_type_pack_id(*tp) })
  }
}
