use ulua_ast::records::{
  ast_array::AstArray, ast_generic_type::AstGenericType, ast_generic_type_pack::AstGenericTypePack,
  ast_name::AstName, node_handle::Nodes,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    arena_handle::alias_ref, duplicate_generic_parameter::DuplicateGenericParameter,
    type_checker_2::TypeChecker2,
  },
  type_aliases::type_error_data::TypeErrorData,
};
impl TypeChecker2 {
  pub fn visit_generics(
    &mut self,
    generics: AstArray<*mut AstGenericType>,
    generic_packs: AstArray<*mut AstGenericTypePack>,
  ) {
    self.visit_generics_impl(
      generics.iter().map(|g| alias_ref(*g)),
      generic_packs.iter().map(|g| alias_ref(*g)),
    );
  }

  /// `&Nodes` 槽位形态：`AstExprFunction{generics, generic_packs}` 句柄化后,
  /// `Node::get` 直出 arena 存活只读引用,与 `visit_generics` 同义。
  pub fn visit_generics_nodes(
    &mut self,
    generics: &Nodes<AstGenericType>,
    generic_packs: &Nodes<AstGenericTypePack>,
  ) {
    self.visit_generics_impl(generics.iter(), generic_packs.iter());
  }

  fn visit_generics_impl<'g>(
    &mut self,
    generics: impl Iterator<Item = &'g AstGenericType>,
    generic_packs: impl Iterator<Item = &'g AstGenericTypePack>,
  ) {
    let mut seen: DenseHashSet<AstName> = DenseHashSet::default();

    for generic in generics {
      let name = generic.name;

      if seen.contains(&name) {
        let parameter_name = name.as_str_or_empty().to_string();
        self.report_error_type_error_data_location(
          TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(parameter_name)),
          &generic.base.location,
        );
      } else {
        seen.insert(name);
      }

      if let Some(default_value) = generic.default_value {
        self.visit_type(alias_ref(default_value.as_ptr()));
      }
    }

    for generic_pack in generic_packs {
      let name = generic_pack.name;

      if seen.contains(&name) {
        let parameter_name = name.as_str_or_empty().to_string();
        self.report_error_type_error_data_location(
          TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(parameter_name)),
          &generic_pack.base.location,
        );
      } else {
        seen.insert(name);
      }

      if let Some(default_value) = generic_pack.default_value {
        self.visit_type_pack(Some(alias_ref(default_value.as_ptr())));
      }
    }
  }
}
