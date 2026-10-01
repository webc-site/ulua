//! `type_function_serializer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::null;

use crate::{
  functions::{follow_type, follow_type_pack},
  methods::type_function_serde_checks::{builder_state_has_errors, exceeded_serde_iteration_limit},
  records::{
    arena_handle::Handle, type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
    type_function_serializer::TypeFunctionSerializer,
  },
  type_aliases::{
    seen_type_packs_type_function_runtime_builder::SeenTypePacks,
    seen_types_type_function_runtime_builder::SeenTypes, type_function_kind::TypeFunctionKind,
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
    type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId,
  },
};

impl TypeFunctionSerializer {
  pub fn find_type_id(&self, ty: TypeId) -> Option<TypeFunctionTypeId> {
    let ty = follow_type::follow(ty);
    self.types.get(&ty).copied()
  }

  pub(crate) fn find_type_pack_id(&self, tp: TypePackId) -> Option<TypeFunctionTypePackId> {
    let tp = follow_type_pack::follow(tp);
    self.packs.get(&tp).copied()
  }

  pub fn find_type_or_pack(&self, kind: TypeOrPack) -> Option<TypeFunctionKind> {
    match kind {
      TypeOrPack::V0(ty) => self.find_type_id(ty).map(TypeFunctionKind::V0),
      TypeOrPack::V1(tp) => self.find_type_pack_id(tp).map(TypeFunctionKind::V1),
    }
  }
}

impl TypeFunctionSerializer {
  pub fn has_errors(&self) -> bool {
    builder_state_has_errors(self.state)
  }
}

impl TypeFunctionSerializer {
  pub fn has_exceeded_iteration_limit(&self) -> bool {
    exceeded_serde_iteration_limit(self.steps, self.queue.len())
  }
}

impl TypeFunctionSerializer {
  /// C++ `TypeFunctionSerializer::run`（TypeFunctionRuntimeBuilder.cpp:101-111）：
  /// 先计步、再查限位/错误，与 deserializer/cloner 的循环同构。
  pub fn run(&mut self) {
    while !self.queue.is_empty() {
      self.steps += 1;

      if self.has_exceeded_iteration_limit() || self.has_errors() {
        break;
      }

      if let Some((kind, tfkind)) = self.queue.pop() {
        self.serialize_children_kind(kind, tfkind);
      }
    }
  }
}

impl TypeFunctionSerializer {
  pub fn serialize_type_id(&mut self, ty: TypeId) -> TypeFunctionTypeId {
    self.shallow_serialize_type_id(ty);
    self.run();

    if self.has_exceeded_iteration_limit() || self.has_errors() {
      null()
    } else {
      self.find_type_id(ty).unwrap_or(null())
    }
  }
}

impl TypeFunctionSerializer {
  /// cpp `TypeFunctionRuntimeBuilder.cpp:53` 的装配步。形参用 `&mut` 承接 cpp 的
  /// `TypeFunctionRuntimeBuilderState*`：非空与「序列化期内存活」由借用承载，
  /// 原 `# Safety` 契约与 `(*state).ctx` 裸解引用一并消失（review.md §2）。
  /// runtime 取自 `ctx`（`Handle` 编码非空），存入 self 后由 builder 生命周期保活。
  pub fn type_function_serializer(&mut self, state: &mut TypeFunctionRuntimeBuilderState) {
    let runtime = state.ctx.get().type_function_runtime;
    self.state = Some(Handle::from_mut(state));
    self.type_function_runtime = Some(Handle::from_nonnull(runtime));
    self.queue = Vec::new();
    self.types = SeenTypes::new(null());
    self.packs = SeenTypePacks::new(null());
    self.steps = 0;
  }
}
