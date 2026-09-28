//! `type_function_cloner` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::null;

use ulua_common::dfint;

use crate::{
  records::{type_function_cloner::TypeFunctionCloner, type_function_runtime::TypeFunctionRuntime},
  type_aliases::{
    seen_type_packs_type_function_runtime::SeenTypePacks,
    seen_types_type_function_runtime::SeenTypes, type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId,
  },
};

impl TypeFunctionCloner {
  pub fn clone_type_function_type_id(&mut self, ty: TypeFunctionTypeId) -> TypeFunctionTypeId {
    self.shallow_clone_type_function_type_id(ty);
    self.run();

    if self.has_exceeded_iteration_limit() {
      return null();
    }

    self.find_type_function_type_id(ty).unwrap_or(null())
  }
}

impl TypeFunctionCloner {
  pub fn find_type_function_type_id(&self, ty: TypeFunctionTypeId) -> Option<TypeFunctionTypeId> {
    self.types.find(&ty).copied()
  }

  pub fn find_type_function_type_pack_id(
    &self,
    tp: TypeFunctionTypePackId,
  ) -> Option<TypeFunctionTypePackId> {
    self.packs.find(&tp).copied()
  }
}

impl TypeFunctionCloner {
  pub fn has_exceeded_iteration_limit(&self) -> bool {
    self.steps + self.queue.len() as i32 >= dfint::LuauTypeFunctionSerdeIterationLimit.get()
  }
}

impl TypeFunctionCloner {
  /// C++ `TypeFunctionCloner::run`（TypeFunctionRuntime.cpp:2665-2676）。
  pub fn run(&mut self) {
    while !self.queue.is_empty() {
      self.steps += 1;

      if self.has_exceeded_iteration_limit() {
        break;
      }

      let Some((ty, tfti)) = self.queue.pop() else {
        break;
      };

      self.clone_children_kind(&ty, &tfti);
    }
  }
}

impl TypeFunctionCloner {
  pub fn new(runtime: *mut TypeFunctionRuntime) -> Self {
    Self {
      type_function_runtime: runtime,
      queue: Vec::new(),
      types: SeenTypes::new(null()),
      packs: SeenTypePacks::new(null()),
      steps: 0,
    }
  }
}
