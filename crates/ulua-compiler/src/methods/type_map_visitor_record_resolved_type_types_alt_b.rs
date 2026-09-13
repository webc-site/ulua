use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName, ast_type::AstType};
use ulua_common::{
  enums::luau_bytecode_type::{LBC_TYPE_ANY, LuauBytecodeType},
  records::dense_hash_set::DenseHashSet,
};

use crate::{functions::get_type::get_type, records::type_map_visitor::TypeMapVisitor};

impl<'a> TypeMapVisitor<'a> {
  pub fn record_resolved_type_ast_local_ast_type(
    &mut self,
    local: *mut AstLocal,
    ty: *const AstType,
  ) -> LuauBytecodeType {
    let ty_resolved = self.resolve_aliases_deprecated(ty);

    *self.resolved_locals.get_or_insert(local) = ty_resolved;

    let mut seen_aliases = DenseHashSet::new(AstName::new());
    let bty = unsafe {
      get_type(
        ty_resolved,
        Default::default(),
        &self.type_aliases,
        true,
        self.host_vector_type,
        self.userdata_types,
        self.bytecode,
        &mut seen_aliases,
      )
    };

    if bty != LBC_TYPE_ANY {
      *self.local_types.get_or_insert(local) = bty;
    }

    bty
  }
}
