use alloc::vec::Vec;
use core::ptr::{NonNull, write};

use ulua_ast::records::ast_name::AstName;

use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  records::{type_function::TypeFunction, user_defined_function_data::UserDefinedFunctionData},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct TypeFunctionInstanceType {
  pub(crate) function: NonNull<TypeFunction>,
  pub(crate) type_arguments: Vec<TypeId>,
  pub(crate) pack_arguments: Vec<TypePackId>,
  pub(crate) user_func_name: Option<AstName>,
  pub(crate) user_func_data: UserDefinedFunctionData,
  pub(crate) state: TypeFunctionInstanceState,
}

impl TypeFunctionInstanceType {
  pub fn new(
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
      state: TypeFunctionInstanceState::Unsolved,
    }
  }

  pub fn new_with_args(function: &TypeFunction, type_arguments: Vec<TypeId>) -> Self {
    Self {
      function: NonNull::from(function),
      type_arguments,
      pack_arguments: Vec::new(),
      user_func_name: None,
      user_func_data: UserDefinedFunctionData::new_empty(),
      state: TypeFunctionInstanceState::Unsolved,
    }
  }

  pub fn new_with_pack_args(
    function: &TypeFunction,
    type_arguments: Vec<TypeId>,
    pack_arguments: Vec<TypePackId>,
  ) -> Self {
    Self {
      function: NonNull::from(function),
      type_arguments,
      pack_arguments,
      user_func_name: None,
      user_func_data: UserDefinedFunctionData::new_empty(),
      state: TypeFunctionInstanceState::Unsolved,
    }
  }
}

impl Drop for TypeFunctionInstanceType {
  fn drop(&mut self) {
    // Safety: `ptr::write` 只覆写字段、不读取也不析构旧值（旧值被有意泄漏，非双重释放），
    // 亦不解引用任何 arena/Type 指针——`TypeId`/`TypePackId` 是无 Drop 的裸指针，`function`
    // 的 NonNull 未被触碰。本实例记录嵌在 `Module` 持有的类型 arena 节点里，用空值替换跳过
    // `user_func_data`（内含 `Weak<Module>` 与哈希表）在 Module/arena 拆卸顺序中的析构，
    // 新写入的空值随后被平凡地 drop，无悬垂或别名。
    unsafe {
      write(&mut self.type_arguments, Vec::new());
      write(&mut self.pack_arguments, Vec::new());
      write(
        &mut self.user_func_data,
        UserDefinedFunctionData::new_empty(),
      );
    }
  }
}
