//! `type_function_deserializer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::String;
use core::ptr::null_mut;

use ulua_ast::records::location::Location;
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  methods::type_function_serde_checks::{
    STATE_WIRED, builder_state_has_errors, exceeded_serde_iteration_limit,
  },
  records::{
    arena_handle::{Handle, alias_ref},
    runtime_error::RuntimeError,
    type_function_deserializer::TypeFunctionDeserializer,
    type_function_error::TypeFunctionError,
    type_function_function_type::TypeFunctionFunctionType,
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
  pub(crate) unsafe fn close_function_scope(&mut self, f: *mut TypeFunctionFunctionType) {
    let generics_len = alias_ref(f).generics.len();
    if generics_len > 0 {
      let generics_start = self.generic_types.len() - generics_len;
      LUAU_ASSERT!(self.generic_types.len() >= generics_len);
      self.generic_types.drain(generics_start..);
    }

    let generic_packs_len = alias_ref(f).generic_packs.len();
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
      let builtins = self.state.expect(STATE_WIRED).get().ctx.get().builtins;
      // Safety: builtins 是 TypeFunctionContext 构造期接线的 NonNull<BuiltinTypes>，
      // 恒非空且比 ctx 长寿；此处仅按值只读 error_type。
      let error: TypeId = unsafe { builtins.as_ref() }.error_type;
      *self.types.get_or_insert(ty) = error;
      return error;
    }

    let builtins = self.state.expect(STATE_WIRED).get().ctx.get().builtins;
    // Safety: 同上——builtins 为 NonNull 恒非空，仅按值只读 error_type。
    let error: TypeId = unsafe { builtins.as_ref() }.error_type;
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
    let state = match self.state {
      Some(state) => state.get_mut(),
      None => return,
    };

    if fflag::LuauTypeFunctionStructuredErrors.get() {
      state.errors.push(TypeFunctionError {
        location: Location::default(),
        module_name: ModuleName::new(),
        data: TypeFunctionErrorData::V2(RuntimeError::new(message)),
      });
    } else {
      state.errors_deprecated.push(message);
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
  /// cpp `TypeFunctionRuntimeBuilder.cpp:588` 的装配步。`&mut` 形参承载 cpp
  /// `TypeFunctionRuntimeBuilderState*` 的非空与存活前提，原 `# Safety` 契约与
  /// `(*state).ctx` 裸解引用一并消失（review.md §2）。
  pub fn type_function_deserializer(&mut self, state: &mut TypeFunctionRuntimeBuilderState) {
    let runtime = state.ctx.get().type_function_runtime;
    self.state = Some(Handle::from_mut(state));
    self.type_function_runtime = Some(Handle::from_nonnull(runtime));
    self.queue = Vec::new();
    // 既有约定（review.md §2）：两个 seen map 的 `null_mut()` 是已反序列化类型/类型包裸指针身份键
    // 的缺省哨兵（与 records/result.rs `UpperBounds::new` 同族），重接线即 reset 语义，不改键类型。
    self.types = DeserializerSeenTypes::new(null_mut());
    self.packs = DeserializedTypePacks::new(null_mut());

    self.generic_types = Vec::new();
    self.generic_packs = Vec::new();
    self.function_scopes = Vec::new();
    self.steps = 0;
  }
}
