use alloc::vec::Vec;
use core::ptr::NonNull;

use crate::{
  records::{
    type_function::TypeFunction, type_function_instance_type::TypeFunctionInstanceType,
    user_defined_function_data::UserDefinedFunctionData,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl TypeFunctionInstanceType {
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
