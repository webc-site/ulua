use crate::{
  records::{
    arena_handle::Handle, substitution::Substitution, type_function_context::TypeFunctionContext,
  },
  type_aliases::type_id::TypeId,
};

/// cpp `RefineTypeScrubber`（`BuiltinTypeFunctions.cpp:1075-1088`）的 `NotNull<TypeFunctionContext> ctx`。
/// 与 [`crate::records::type_function_reducer::TypeFunctionReducer`] 同一取态：本对象要装进
/// `SubstitutionVtable` 的 `owner: *mut ()` 回调（fn 指针经 owner 回转调用 `clean_type_id`），
/// 借用无法穿越该 C 形状边界，故存储态用 [`Handle`]（非空由类型编码、解引用收拢一处），
/// 构造点仍收真实 `&mut` 借用。
#[derive(Debug, Clone)]
pub struct RefineTypeScrubber {
  pub(crate) base: Substitution,
  pub(crate) ctx: Handle<TypeFunctionContext>,
  pub(crate) needle: TypeId,
}
