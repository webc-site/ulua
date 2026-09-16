use alloc::vec::Vec;

use ulua_ast::records::ast_name::AstName;

use crate::{
  records::pending_expansion_type::PendingExpansionType,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl PendingExpansionType {
  pub fn pending_expansion_type_pending_expansion_type(
    prefix: Option<AstName>,
    name: AstName,
    type_arguments: Vec<TypeId>,
    pack_arguments: Vec<TypePackId>,
  ) -> Self {
    Self {
      prefix,
      name,
      type_arguments,
      pack_arguments,
      index: Self::fresh_index(),
    }
  }
}
