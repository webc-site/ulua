use alloc::{collections::BTreeSet, vec::Vec};

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::{normalized_part::TABLE_OR_EXTERN_PARTS, reduction::Reduction},
  functions::{compute_keys_of::compute_keys_of, follow_type},
  records::{
    singleton_type::SingletonType, string_singleton::StringSingleton,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 逐参数契约（对照 cpp `keyofFunctionImpl`，BuiltinTypeFunctions.cpp:1766）：
/// - `type_params`：实参类型切片。契约要求长度恰为 1（`keyof` 的唯一操作数）；不等长时
///   先经 `ice`+`LUAU_ASSERT!(false)` 记录内部错误（`ice_string` 以 panic 发散），其后的
///   `type_params[0]` 索引仍依赖该形状成立。
/// - `pack_params`：打包实参切片，契约要求为空。
/// - `ctx`：只读借用，指向整段归约期间存活的 `TypeFunctionContext`。本函数按 cpp
///   `ctx->…` 解引用其 `ice/normalizer/builtins/arena` 句柄，并把同一上下文的共享引用
///   交给 `compute_keys_of`（其签名取 `&TypeFunctionContext`）；`try_normalize`/
///   `add_type` 的可变借用半径止于所属语句，单线程独占驱动（lib.rs 不变量 1）下无并存
///   别名，各 arena 节点随会话存活（不变量 2）。
/// - `is_raw`：纯 bool 标志（rawkeyof 时为 true，`compute_keys_of` 据此跳过
///   `__index` 元表链），无指针侧契约。
pub unsafe fn keyof_function_impl(
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &TypeFunctionContext,
  is_raw: bool,
) -> TypeFunctionReductionResult {
  // Safety: `ctx` 依函数级契约为整段归约内存活的只读借用；后续字段句柄与
  // 传给 `compute_keys_of` 的引用都由此派生。
  let ctx_ref = ctx;
  if type_params.len() != 1 || !pack_params.is_empty() {
    unsafe {
      // Safety: `ice` 是 NonNull<InternalErrorReporter>，as_ptr 取回的裸指针指向求解会话
      // 持有的存活报告器；`ice_string(&self, …)` 只读其元数据构造消息并 panic 发散。
      (*ctx_ref.ice.as_ptr()).ice_string(
                "keyof type function: encountered a type function instance without the required argument structure",
            )
    };
    LUAU_ASSERT!(false);
  }

  let make_result = |result, reduction_status| TypeFunctionReductionResult {
    result,
    reduction_status,
    blocked_types: Vec::new(),
    blocked_packs: Vec::new(),
    error: None,
    messages: Vec::new(),
  };

  let operand_ty = follow_type::follow(type_params[0]);
  // Safety: `normalizer` 为 NonNull<Normalizer>，as_ptr 后取 `&mut` 调 try_normalize
  // （写其内部归一化缓存），可变借用止于本语句，返回独立 `Arc<NormalizedType>`；
  // 失败返回 None 对应 cpp 空 shared_ptr 的短路分支。
  let Some(norm_ty) = (unsafe { (*ctx_ref.normalizer.as_ptr()).try_normalize(operand_ty) }) else {
    return make_result(None, Reduction::MaybeOk);
  };

  if norm_ty.has_tables() == norm_ty.has_extern_types() {
    return make_result(None, Reduction::Erroneous);
  }

  if norm_ty.has_parts_other_than(&TABLE_OR_EXTERN_PARTS) {
    return make_result(None, Reduction::Erroneous);
  }

  let mut keys = BTreeSet::new();

  if norm_ty.has_extern_types() {
    LUAU_ASSERT!(!norm_ty.has_tables());
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();

    let mut extern_types = norm_ty.extern_types.ordering.iter().copied();
    let Some(first) = extern_types.next() else {
      return make_result(None, Reduction::Erroneous);
    };

    if !compute_keys_of(first, &mut keys, &mut seen, is_raw, ctx_ref) {
      return make_result(
        // Safety: `builtins` 为 NonNull<BuiltinTypes>，指向会话期存活的内置类型表；
        // as_ref 后只读 string_type（Copy TypeId），共享借用瞬态有效。
        Some(unsafe { ctx_ref.builtins.as_ref().string_type }),
        Reduction::MaybeOk,
      );
    }

    for extern_ty in extern_types {
      seen.clear();
      let mut local_keys = BTreeSet::new();
      if compute_keys_of(extern_ty, &mut local_keys, &mut seen, is_raw, ctx_ref) {
        keys.retain(|key| local_keys.contains(key));
      }
    }
  }

  if norm_ty.has_tables() {
    LUAU_ASSERT!(!norm_ty.has_extern_types());
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();

    let mut tables = norm_ty.tables.order.iter().copied();
    let Some(first) = tables.next() else {
      return make_result(None, Reduction::Erroneous);
    };

    if !compute_keys_of(first, &mut keys, &mut seen, is_raw, ctx_ref) {
      return make_result(
        // Safety: 第二个首表失败分支，仍是对会话存活 builtins 表的瞬态只读 Copy 取值。
        Some(unsafe { ctx_ref.builtins.as_ref().string_type }),
        Reduction::MaybeOk,
      );
    }

    for table in tables {
      seen.clear();
      let mut local_keys = BTreeSet::new();
      if compute_keys_of(table, &mut local_keys, &mut seen, is_raw, ctx_ref) {
        keys.retain(|key| local_keys.contains(key));
      }
    }
  }

  if keys.is_empty() {
    return make_result(
      // Safety: 空键集归约为 never：读取会话存活 builtins 表的 Copy 字段，瞬态共享借用。
      Some(unsafe { ctx_ref.builtins.as_ref().never_type }),
      Reduction::MaybeOk,
    );
  }

  let mut singletons = Vec::new();
  for key in keys {
    // Safety: `arena` 为 NonNull<TypeArena>，as_ptr 后取 `&mut` 逐键追加
    // SingletonType(StringSingleton)；可变借用止于本语句，循环迭代间顺序无并存，
    // 单线程独占驱动下无其它 arena 借用。
    singletons.push(unsafe {
      (*ctx_ref.arena.as_ptr()).add_type(SingletonType::new(SingletonVariant::V1(
        StringSingleton::new(key),
      )))
    });
  }

  if singletons.len() == 1 {
    return make_result(Some(singletons[0]), Reduction::MaybeOk);
  }

  make_result(
    // Safety: 同上，向 arena 追加 UnionType（options 移入 singletons）；可变借用止于
    // 本表达式语句，此刻无并存 arena 借用。
    Some(unsafe {
      (*ctx_ref.arena.as_ptr()).add_type(UnionType {
        options: singletons,
      })
    }),
    Reduction::MaybeOk,
  )
}
