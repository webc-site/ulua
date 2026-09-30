//! `function_type` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use crate::{
  records::{
    function_argument::FunctionArgument, function_definition::FunctionDefinition,
    function_type::FunctionType, type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl FunctionType {
  pub fn arg_names(&self) -> &[Option<FunctionArgument>] {
    &self.arg_names
  }
}

impl FunctionType {
  pub fn arg_types(&self) -> TypePackId {
    self.arg_types
  }
}

impl FunctionType {
  pub fn definition(&self) -> Option<&FunctionDefinition> {
    self.definition.as_ref()
  }
}

impl FunctionType {
  /// C++ `FunctionType(TypePackId argTypes, TypePackId retTypes, std::optional<FunctionDefinition> defn = {}, bool hasSelf = false)`
  /// — the global monomorphic function constructor (Type.cpp:610). All other members take their
  /// C++ default member initializers (Type.h:350-47).
  pub fn function_type_new(
    arg_types: TypePackId,
    ret_types: TypePackId,
    defn: Option<FunctionDefinition>,
    has_self: bool,
  ) -> Self {
    FunctionType {
      definition: defn,
      generics: Vec::new(),
      generic_packs: Vec::new(),
      arg_names: Vec::new(),
      tags: Vec::new(),
      level: TypeLevel::default(),
      arg_types,
      ret_types,
      magic: None,
      has_self,
      has_no_free_or_generic_types: false,
      is_checked_function: false,
      is_deprecated_function: false,
      deprecated_info: None,
    }
  }
}

impl FunctionType {
  pub fn generic_packs(&self) -> &Vec<TypePackId> {
    &self.generic_packs
  }
}

impl FunctionType {
  pub fn generics(&self) -> &Vec<TypeId> {
    &self.generics
  }
}

impl FunctionType {
  pub fn has_self(&self) -> bool {
    self.has_self
  }
}

impl FunctionType {
  pub fn new_with_generics(
    generics: Vec<TypeId>,
    generic_packs: Vec<TypePackId>,
    arg_types: TypePackId,
    ret_types: TypePackId,
    defn: Option<FunctionDefinition>,
    has_self: bool,
  ) -> Self {
    let mut result = Self::function_type_new(arg_types, ret_types, defn, has_self);
    result.generics = generics;
    result.generic_packs = generic_packs;
    result
  }
}

impl FunctionType {
  pub fn ret_types(&self) -> TypePackId {
    self.ret_types
  }
}
