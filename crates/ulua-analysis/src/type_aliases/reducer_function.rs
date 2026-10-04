use crate::{
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

// C++ (TypeFunction.h:124-125): std::function<TypeFunctionReductionResult<T>(
// T, const std::vector<TypeId>&, const std::vector<TypePackId>&,
// NotNull<TypeFunctionContext>)>. Defaulted to TypeId so the bare
// `ReducerFunction` in TypeFunction matches C++ `ReducerFunction<TypeId>`;
// TypePackFunction instantiates `ReducerFunction<TypePackId>`.
//
// WHY this shape (review.md §2, 方案 1「单一共享上下文 + 注册回调」): the cpp
// oracle registers one `TypeFunction` per builtin kind in `BuiltinTypeFunctions`
// (BuiltinTypeFunctions.cpp) and the *same* session `TypeFunctionContext` is
// handed to every reducer at dispatch (TypeChecker2/TypeFunctionReducer's
// `stepType`/`stepPack` pass their single `ctx`). Entries therefore store only
// the fn discriminant — a plain safe `fn` pointer, no `dyn` (§4), no per-entry
// context storage and no handle registry (option 2 would model ownership the
// oracle does not have). The context is a stack local of the constraint-solver
// dispatch frame that outlives the whole reduction loop, so the faithful Rust
// spelling of cpp `NotNull<TypeFunctionContext>` is an exclusive session borrow
// `&mut TypeFunctionContext` threaded only through the call, never stored in the
// registry. The concrete reducer fns in `functions/*_type_function.rs` all share
// this signature so they are assignable to the `reducer` field of
// `TypeFunction`/`TypePackFunction` without a cast — this is the project's
// MagicFunction-style fn-pointer wiring.
/// 调用序契约（正确性，非内存安全）：经本别名调用一个 reducer 时，`ctx` 借出的
/// `TypeFunctionContext` 须在本次调用返回前独占存活（分派点 `TypeFunctionReducer::step_*`
/// 从构造期以真实 `&mut` 借用接线的 `Handle<TypeFunctionContext>` 物化该借用），两个切片
/// 参数在调用返回前保持有效，且 `T`（`TypeId`/`TypePackId`）为指向存活类型 arena 节点的
/// 句柄。这些前提不是内存安全问题：`ctx`/切片的存活与独占由 `&mut`/`&` 引用形参承载，
/// arena 句柄的解引用一律经各自 safe 门面（`alias*`/`get_type*`/`NonNull` 折返）收口，
/// 违反调用序只会得到错误诊断或逻辑错乱，不会越出 Rust 内存安全边界——这是 review.md
/// §2 要求的签名收形（fn 指针类型去掉 `unsafe`，与全部注册条目一致）。
pub type ReducerFunction<T = TypeId> =
  fn(T, &[TypeId], &[TypePackId], &mut TypeFunctionContext) -> TypeFunctionReductionResult<T>;
