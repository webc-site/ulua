use crate::{
  records::{
    ast_array::AstArray, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_node::AstNode, ast_type::AstType,
    ast_type_function::AstTypeFunction, ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    location::Location,
  },
  rtti::AstNodeClass,
  type_aliases::ast_argument_name::AstArgumentName,
};

impl AstTypeFunction {
  pub fn ast_type_function_location_ast_array_ast_generic_type_ast_array_ast_generic_type_pack_ast_type_list_ast_array_optional_ast_argument_name_ast_type_pack(
    location: Location,
    generics: AstArray<*mut AstGenericType>,
    generic_packs: AstArray<*mut AstGenericTypePack>,
    arg_types: AstTypeList,
    arg_names: AstArray<Option<AstArgumentName>>,
    return_types: *mut AstTypePack,
  ) -> Self {
    ulua_common::LUAU_ASSERT!(arg_names.is_empty() || arg_names.len() == arg_types.types.len());

    Self {
      base: AstType {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      attributes: AstArray::default(),
      generics,
      generic_packs,
      arg_types,
      arg_names,
      return_types,
    }
  }
}
