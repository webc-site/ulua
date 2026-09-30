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
// the fn discriminant — a plain `unsafe fn` pointer, no `dyn` (§4), no per-entry
// context storage and no handle registry (option 2 would model ownership the
// oracle does not have). The context is a stack local of the constraint-solver
// dispatch frame that outlives the whole reduction loop, so the faithful Rust
// spelling of cpp `NotNull<TypeFunctionContext>` is an exclusive session borrow
// `&mut TypeFunctionContext` threaded only through the call, never stored in the
// registry. The concrete reducer fns in `functions/*_type_function.rs` all share
// this signature so they are assignable to the `reducer` field of
// `TypeFunction`/`TypePackFunction` without a cast — this is the project's
// MagicFunction-style fn-pointer wiring.
/// # Safety
/// 经本别名调用一个 reducer 时，调用方须保证：`ctx` 借出的 `TypeFunctionContext`
/// 在本次调用返回前独占存活（分派点 `TypeFunctionReducer::step_*` 从构造期以真实
/// `&mut` 借用接线的 `Handle<TypeFunctionContext>` 物化该借用），两个切片参数在调用返回前保持有效，
/// 且 `T`（`TypeId`/`TypePackId`）为指向存活类型 arena 节点的有效句柄。
///
/// 指针类型保留 `unsafe fn`（DELIBERATE DEVIATION 之外的取舍）：注册表内仍有
/// reducer（如 `user_defined_type_function` 等 VM 桥入口）依 cpp `ReducerFunction`
/// 统一函数指针形态保持 `unsafe fn`，别名收窄为安全 `fn` 会迫使全部条目同批
/// 改写；安全的 reducer 实现（`unm`/`len`/`not` 等已去签名 `unsafe`）作为安全
/// `fn` item 隐式可强制转换为本 `unsafe fn` 指针类型，派发点包裹的 `unsafe {}`
/// 只为兑现本契约，与具体条目是否 unsafe 无关。
pub type ReducerFunction<T = TypeId> = unsafe fn(
  T,
  &[TypeId],
  &[TypePackId],
  &mut TypeFunctionContext,
) -> TypeFunctionReductionResult<T>;
