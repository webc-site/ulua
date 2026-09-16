use crate::{
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_name::AstName, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_declare_function::AstStatDeclareFunction,
    ast_type_list::AstTypeList, ast_type_pack::AstTypePack, location::Location,
  },
  rtti::AstNodeClass,
  type_aliases::ast_argument_name::AstArgumentName,
};

/// `AstStatDeclareFunction` 构造参数集合。
/// 直译 C++ 构造函数有 11+ 个参数，超出 Rust 惯用上限，打包成 struct 传递；
/// 字段与 C++ `AstStatDeclareFunction` 构造函数形参一一对应。
pub struct AstStatDeclareFunctionArgs {
  pub location: Location,
  pub attributes: AstArray<*mut AstAttr>,
  pub name: AstName,
  pub name_location: Location,
  pub generics: AstArray<*mut AstGenericType>,
  pub generic_packs: AstArray<*mut AstGenericTypePack>,
  pub params: AstTypeList,
  pub param_names: AstArray<AstArgumentName>,
  pub vararg: bool,
  pub vararg_location: Location,
  pub ret_types: *mut AstTypePack,
}

impl AstStatDeclareFunction {
  /// `attributes` 取默认空数组，对应旧 `new_simple` 便捷构造。
  pub fn new_simple(a: AstStatDeclareFunctionArgs) -> Self {
    Self::new(AstStatDeclareFunctionArgs {
      attributes: AstArray::default(),
      ..a
    })
  }

  pub fn new(a: AstStatDeclareFunctionArgs) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location: a.location,
        },
        has_semicolon: false,
      },
      attributes: a.attributes,
      name: a.name,
      name_location: a.name_location,
      generics: a.generics,
      generic_packs: a.generic_packs,
      params: a.params,
      param_names: a.param_names,
      vararg: a.vararg,
      vararg_location: a.vararg_location,
      ret_types: a.ret_types,
    }
  }
}
