use alloc::string::ToString;

use ulua_ast::{
  records::{ast_stat::AstStat, ast_stat_type_alias::AstStatTypeAlias},
  rtti::{AstNodePtr, ast_node_try_as},
};

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, follow_type, get_type,
    magic_names::is_reserved_type_alias_name,
  },
  records::{
    free_type::FreeType, occurs_check_failed::OccursCheckFailed, type_checker::TypeChecker,
    type_error::TypeError,
  },
  type_aliases::{
    name_type::Name, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_variant::TypeVariant,
  },
};
impl TypeChecker {
  pub fn check_block_type_aliases(&mut self, scope: &ScopePtr, sorted: &mut [*mut AstStat]) {
    for &stat_ptr in sorted.iter() {
      // SAFETY: stat 指向 AST arena 节点；AstStat #[repr(C)] 单继承，
      // base(AstNode) 在偏移 0，cast 有效。
      let Some(typealias) =
        ast_node_try_as::<AstStatTypeAlias>(unsafe { &*(stat_ptr.as_ast_node()) })
      else {
        continue;
      };

      // AstName 由词法器 intern，必为合法 ASCII/UTF-8；空名（null）按 "" 处理。
      if is_reserved_type_alias_name(typealias.name.as_bytes()) {
        continue;
      }

      let bindings = if typealias.exported {
        &scope.exported_type_bindings
      } else {
        &scope.private_type_bindings
      };

      let name: Name = typealias.name.as_str_or_empty().to_string();

      if self
        .duplicate_type_aliases
        .contains(&(typealias.exported, name.clone()))
      {
        continue;
      }

      let Some(type_binding) = bindings.get(&name) else {
        continue;
      };

      let type_id = follow_type::follow(type_binding.r#type());

      if get_type::get::<FreeType>(type_id).is_some() {
        // SAFETY: as_mutable_type_id 去除 const（C++ asMutable 同义），type_id 有效。
        unsafe {
          (*as_mutable_type_id(type_id)).ty =
            TypeVariant::Bound(self.error_recovery_type_type_id(self.any_type));
        }

        let error_data = TypeErrorData::OccursCheckFailed(OccursCheckFailed::default());
        let error =
          TypeError::type_error_location_type_error_data(typealias.base.base.location, error_data);
        self.report_error_type_error(&error);
      }
    }
  }
}
