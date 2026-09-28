//! `type_function_deserializer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::String;
use core::ptr::null_mut;

use ulua_ast::records::location::Location;
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  methods::type_function_serde_checks::{builder_state_has_errors, exceeded_serde_iteration_limit},
  records::{
    runtime_error::RuntimeError, type_function_deserializer::TypeFunctionDeserializer,
    type_function_error::TypeFunctionError, type_function_function_type::TypeFunctionFunctionType,
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
  },
  type_aliases::{
    module_name_type::ModuleName,
    seen_type_packs_type_function_runtime_builder::DeserializedTypePacks,
    seen_types_type_function_runtime_builder::DeserializerSeenTypes,
    type_function_error_data::TypeFunctionErrorData, type_function_type_id::TypeFunctionTypeId,
    type_function_type_pack_id::TypeFunctionTypePackId, type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证 `f` 非空、对齐，指向类型函数反序列化期间由 arena/持有者保活、地址稳定的
  /// `TypeFunctionFunctionType`；本函数只读取其 `generics`/`generic_packs` 的长度以弹出登记。
  /// cpp `Analysis/src/TypeFunctionRuntimeBuilder.cpp:703`。单线程。
  pub unsafe fn close_function_scope(&mut self, f: *mut TypeFunctionFunctionType) {
    // Safety: `f` 为类型函数反序列化期间存活的函数类型节点（由 deserializer 的 arena/持有者
    // 保活），此处仅只读取 `generics` 的 `Vec::len()`，不解引用可变状态。
    let generics_len = unsafe { (*f).generics.len() };
    if generics_len > 0 {
      let generics_start = self.generic_types.len() - generics_len;
      LUAU_ASSERT!(self.generic_types.len() >= generics_len);
      self.generic_types.drain(generics_start..);
    }

    // Safety: 同上——`f` 指向存活函数类型节点，此处仅只读取 `generic_packs` 的 `Vec::len()`。
    let generic_packs_len = unsafe { (*f).generic_packs.len() };
    if generic_packs_len > 0 {
      let generic_packs_start = self.generic_packs.len() - generic_packs_len;
      LUAU_ASSERT!(self.generic_packs.len() >= generic_packs_len);
      self.generic_packs.drain(generic_packs_start..);
    }
  }
}

// Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:593-606`
//
// ```cpp
// TypeId deserialize(TypeFunctionTypeId ty)
// {
// shallowDeserialize(ty);
// run();
//
// if (hasExceededIterationLimit() || hasErrors())
// {
// TypeId error = state->ctx->builtins->error_type;
// types[ty] = error;
// return error;
// }
//
// return find(ty).value_or(state->ctx->builtins->error_type);
// }
// ```

impl TypeFunctionDeserializer {
  pub fn deserialize_type_function_type_id(&mut self, ty: TypeFunctionTypeId) -> TypeId {
    self.shallow_deserialize_type_function_type_id(ty);
    self.run();

    if self.has_exceeded_iteration_limit() || self.has_errors() {
      // Safety: self.state 由 builder 入口注入且在本轮反序列化期间于 builder 栈上存活；
      // (*self.state).ctx 是 Handle（类型编码非空），物化后指向调用方持有的存活
      // TypeFunctionContext，其 builtins 是构造期接线的 NonNull<BuiltinTypes>（as_ptr 恒非空、比 ctx 长寿），error_type 为按值读出的
      // TypeId 字段，全程只读不写。
      let error: TypeId = unsafe { (*(*self.state).ctx.get().builtins.as_ptr()).error_type };
      *self.types.get_or_insert(ty) = error;
      return error;
    }

    // Safety: 同上——state/ctx 存活、builtins 为 NonNull 恒非空，仅按值只读 error_type。
    let error: TypeId = unsafe { (*(*self.state).ctx.get().builtins.as_ptr()).error_type };
    self.find_type_function_type_id(ty).unwrap_or(error)
  }
}

impl TypeFunctionDeserializer {
  pub fn find_type_function_type_id(&self, ty: TypeFunctionTypeId) -> Option<TypeId> {
    self.types.get(&ty).copied()
  }

  pub fn find_type_function_type_pack_id(&self, tp: TypeFunctionTypePackId) -> Option<TypePackId> {
    self.packs.get(&tp).copied()
  }
}

impl TypeFunctionDeserializer {
  pub fn has_errors(&self) -> bool {
    builder_state_has_errors(self.state)
  }
}

impl TypeFunctionDeserializer {
  pub fn has_exceeded_iteration_limit(&self) -> bool {
    exceeded_serde_iteration_limit(self.steps, self.queue.len())
  }
}

impl TypeFunctionDeserializer {
  pub fn push_runtime_error(&mut self, message: String) {
    if self.state.is_null() {
      return;
    }

    unsafe {
      if fflag::LuauTypeFunctionStructuredErrors.get() {
        (*self.state).errors.push(TypeFunctionError {
          location: Location::default(),
          module_name: ModuleName::new(),
          data: TypeFunctionErrorData::V2(RuntimeError::new(message)),
        });
      } else {
        (*self.state).errors_deprecated.push(message);
      }
    }
  }
}

impl TypeFunctionDeserializer {
  pub fn run(&mut self) {
    while !self.queue.is_empty() {
      self.steps += 1;

      if self.has_exceeded_iteration_limit() || self.has_errors() {
        break;
      }

      if let Some((tfkind, kind)) = self.queue.pop() {
        self.deserialize_children_kind(tfkind, kind);
      }

      if let Some(scope) = self.function_scopes.last().cloned()
        && self.queue.len() == scope.old_queue_size
        && !self.has_errors()
      {
        unsafe { self.close_function_scope(scope.function) };
        self.function_scopes.pop();
      }
    }
  }
}

impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证 `state` 非空、对齐，指向反序列化期存活的 builder state，其 `ctx` 构造接线恒非空。
  /// 该指针与 runtime 地址存入 self 后须在 builder 生命周期内有效、arena 块地址不移动。
  /// cpp `Analysis/src/TypeFunctionRuntimeBuilder.cpp:588`。单线程独占。
  pub unsafe fn type_function_deserializer(&mut self, state: *mut TypeFunctionRuntimeBuilderState) {
    self.state = state;
    self.type_function_runtime = unsafe {
      // 契约：`TypeFunctionRuntimeBuilderState::new` 以存活 `&mut TypeFunctionContext`
      // 接线 ctx（`Handle` 类型编码非空，cpp `state->ctx->` 同前提直解），
      // 故句柄物化的借用恒有效。
      (*state).ctx.get().type_function_runtime.as_ptr()
    };
    self.queue = Vec::new();
    self.types = DeserializerSeenTypes::new(null_mut());
    self.packs = DeserializedTypePacks::new(null_mut());

    self.generic_types = Vec::new();
    self.generic_packs = Vec::new();
    self.function_scopes = Vec::new();
    self.steps = 0;
  }
}
