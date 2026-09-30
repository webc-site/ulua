//! Source: `Analysis/include/Luau/TypeFunctionRuntimeBuilder.h`

extern crate alloc;

use alloc::{string::String, vec::Vec};

use crate::records::{
  arena_handle::Handle, type_function_context::TypeFunctionContext,
  type_function_error::TypeFunctionError,
};

#[derive(Debug)]
pub struct TypeFunctionRuntimeBuilderState {
  // cpp `NotNull<TypeFunctionContext> ctx`：序列化/反序列化整轮所别名的会话上下文。
  //
  // 为何是 [`Handle`] 而不是带寿命标注的 `&'a mut TypeFunctionContext`：本结构的
  // 存储态被 `Box` 后以指针形式挂进 `TypeFunctionRuntime::runtime_builder`
  // （cpp `ScopedAssign setRuntimeBuilder(...)`），再由 serializer/deserializer/
  // `evaluateTypeAliasCall`/`isSubtypeOf` 等 6 处横传（`state->ctx->`）。给 `ctx`
  // 加 `'a` 即要求整条 builder/runtime/serializer 链都带上寿命参数，而 cpp 侧
  // 本来允许同一份 ctx 在多个栈帧间别名共享（reducer 借出、serializer 复用），
  // 寿命标注会凭空发明「谁独占 ctx」这一 oracle 里不存在的语义。故取
  // [`Handle`]：非空由类型编码，构造点收敛为真实 `&mut` 借用（见 [`Self::new`]），
  // 全部解引用收进 `arena_handle` 一处，调用点不再出现裸指针与判空。
  pub ctx: Handle<TypeFunctionContext>,
  // List of errors that occur during serialization/deserialization
  // At every iteration, if this list is non-empty, the process halts.
  pub errors_deprecated: Vec<String>,
  pub errors: Vec<TypeFunctionError>,
}

impl TypeFunctionRuntimeBuilderState {
  /// cpp `TypeFunctionRuntimeBuilderState(NotNull<TypeFunctionContext> ctx)`。
  ///
  /// 形参刻意是 `&mut`（而非 cpp 的 `NotNull`/裸指针）：唯一构造点
  /// （`user_defined_type_function`）手上的正是分派帧借出的独占会话借用，
  /// 由引用物化句柄让「非空 + 构造期存活」成为类型级事实，null 分支无从出现。
  pub fn new(ctx: &mut TypeFunctionContext) -> Self {
    Self {
      ctx: Handle::from_mut(ctx),
      errors_deprecated: Vec::new(),
      errors: Vec::new(),
    }
  }
}
