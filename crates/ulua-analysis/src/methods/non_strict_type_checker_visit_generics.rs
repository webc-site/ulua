use ulua_ast::records::{
  ast_array::AstArray, ast_generic_type::AstGenericType, ast_generic_type_pack::AstGenericTypePack,
  ast_name::AstName, node_handle::Nodes,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    duplicate_generic_parameter::DuplicateGenericParameter,
    non_strict_type_checker::NonStrictTypeChecker,
  },
  type_aliases::type_error_data::TypeErrorData,
};
impl NonStrictTypeChecker {
  pub fn visit_generics(
    &mut self,
    generics: AstArray<*mut AstGenericType>,
    generic_packs: AstArray<*mut AstGenericTypePack>,
  ) {
    // Safety: `generics`/`generic_packs` 是 parser arena 成对写入的 {data,size}
    // 数组，界内元素为已绑定的非空 `*mut AstGenericType(AstGenericTypePack)`
    // （C++ 循环亦直接解引用）；共享只读借用止于消费点,report_error/visit_ast_type
    // 只改 checker 状态,不写穿 AST 节点。
    self.visit_generics_impl(
      generics.iter().map(|g| unsafe { &**g }),
      generic_packs.iter().map(|g| unsafe { &**g }),
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
    // C++ `DenseHashSet<AstName> seen{AstName{}};` — empty/null AstName sentinel key.
    let mut seen: DenseHashSet<AstName> = DenseHashSet::default();

    for g in generics {
      let name = g.name;

      if seen.contains(&name) {
        let param_name = name.as_str_or_empty().to_string();
        self.report_error(
          TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(param_name)),
          &g.base.location,
        );
      } else {
        seen.insert(name);
      }

      if let Some(default_value) = g.default_value {
        self.visit_ast_type(default_value.as_ptr());
      }
    }

    for g in generic_packs {
      let name = g.name;

      if seen.contains(&name) {
        let param_name = name.as_str_or_empty().to_string();
        self.report_error(
          TypeErrorData::DuplicateGenericParameter(DuplicateGenericParameter::new(param_name)),
          &g.base.location,
        );
      } else {
        seen.insert(name);
      }

      if let Some(default_value) = g.default_value {
        self.visit_ast_type_pack(default_value.as_ptr());
      }
    }
  }
}
