use ulua_ast::records::{
  ast_array::AstArray, ast_generic_type::AstGenericType, ast_generic_type_pack::AstGenericTypePack,
  ast_name::AstName, node_handle::Nodes,
};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{duplicate_generic_parameter::DuplicateGenericParameter, type_checker_2::TypeChecker2},
  type_aliases::type_error_data::TypeErrorData,
};
impl TypeChecker2 {
  pub fn visit_generics(
    &mut self,
    generics: AstArray<*mut AstGenericType>,
    generic_packs: AstArray<*mut AstGenericTypePack>,
  ) {
    // Safety: generics/generic_packs 数组元素是 parser 在 bump allocator 上分配的
    // AstGenericType/AstGenericTypePack 节点（块地址不移动，本次 AST 检查期内存活），
    // parser 对该列表元素结构性保证非空；此处只重建共享借用,后续统一走只读迭代。
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
        // SAFETY: default_value 已在上一行显式判空，非空时是 parser 在 arena 上
        // 分配的类型标注节点，本次检查期内地址稳定且存活，满足 visit_type 的
        // 节点存活契约。
        self.visit_type(unsafe { &*default_value.as_ptr() });
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
        // SAFETY: 同上——判空后的 arena 节点指针转共享引用传入 visit_type_pack。
        self.visit_type_pack(Some(unsafe { &*default_value.as_ptr() }));
      }
    }
  }
}
