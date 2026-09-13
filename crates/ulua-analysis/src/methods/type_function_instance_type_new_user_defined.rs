use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_ast::records::ast_name::AstName;

use crate::{
  records::{
    type_function::TypeFunction, type_function_instance_type::TypeFunctionInstanceType,
    user_defined_function_data::UserDefinedFunctionData,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl TypeFunctionInstanceType {
  pub fn new_user_defined(
    function: &TypeFunction,
    type_arguments: Vec<TypeId>,
    pack_arguments: Vec<TypePackId>,
    user_func_name: AstName,
  ) -> Self {
    Self::new(
      NonNull::from(function),
      type_arguments,
      pack_arguments,
      Some(user_func_name),
      UserDefinedFunctionData::new_empty(),
    )
  }
}
