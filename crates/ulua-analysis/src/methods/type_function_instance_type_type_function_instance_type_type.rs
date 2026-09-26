use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_ast::records::ast_name::AstName;

use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  records::{
    type_function::TypeFunction, type_function_instance_type::TypeFunctionInstanceType,
    user_defined_function_data::UserDefinedFunctionData,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeFunctionInstanceType {
  pub fn type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id_optional_ast_name_user_defined_function_data(
    function: NonNull<TypeFunction>,
    type_arguments: Vec<TypeId>,
    pack_arguments: Vec<TypePackId>,
    user_func_name: Option<AstName>,
    user_func_data: UserDefinedFunctionData,
  ) -> Self {
    Self {
      function,
      type_arguments,
      pack_arguments,
      user_func_name,
      user_func_data,
      state: TypeFunctionInstanceState::default(),
    }
  }

  pub fn type_function_instance_type_type_function_vector_type_id(
    function: &TypeFunction,
    type_arguments: Vec<TypeId>,
  ) -> Self {
    Self::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id_optional_ast_name_user_defined_function_data(
            NonNull::from(function),
            type_arguments,
            Vec::new(),
            None,
            UserDefinedFunctionData::new_empty(),
        )
  }

  pub fn type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id(
    function: NonNull<TypeFunction>,
    type_arguments: Vec<TypeId>,
    pack_arguments: Vec<TypePackId>,
  ) -> Self {
    Self::type_function_instance_type_not_null_type_function_vector_type_id_vector_type_pack_id_optional_ast_name_user_defined_function_data(
            function,
            type_arguments,
            pack_arguments,
            None,
            UserDefinedFunctionData::new_empty(),
        )
  }
}
