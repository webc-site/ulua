use alloc::string::ToString;

use ulua_common::fflag;

use crate::{
  functions::{
    first::first, flatten_type_pack::flatten_type_pack_id, follow_type, follow_type_pack,
  },
  records::{
    count_mismatch::{CountMismatch, CountMismatchContext},
    magic_function_type_check_context::MagicFunctionTypeCheckContext,
  },
  type_aliases::{type_error_data::TypeErrorData, type_id::TypeId},
};

// MagicFreeze is a magic function because table.freeze is a bounded version of the identity function with a custom output (accepts any subtype of
// `table` and returns a read-only version of that table).
pub fn magic_freeze_type_check(ctx: &MagicFunctionTypeCheckContext) -> bool {
  if !fflag::LuauTableFreezeCheckIsSubtype.get() {
    return false;
  }

  // Safety: ctx.typechecker 由调用点以 `NonNull::new_unchecked(self as *mut TypeChecker2)` 写入
  // （派生自 `&mut self`，非空对齐）。magic 回调持有 `&MagicFunctionTypeCheckContext` 临时借用，
  // 经该裸指针重访 self 属 C++ 同构用法：单线程时序串行、回调返回后无保留别名，故重建唯一 &mut 无冲突。
  let typechecker = unsafe { &mut *ctx.typechecker.as_ptr() };
  // Safety: ctx.builtin_types 是 `NonNull<BuiltinTypes>`（调用点由 self.builtin_types 的 NotNull
  // 会话指针注入），as_ref 返回的共享借用指向不可变单例、比 ctx 长寿，无并存 &mut。
  let builtin_types = unsafe { ctx.builtin_types.as_ref() };
  // Safety: ctx.call_site 为当前访问的 `*const AstExprCall` AST 节点（visit_call 传入的 `call`），
  // 由 SourceModule 拥有、在本次类型检查期间存活且非空；仅建只读共享借用。
  let call_site = unsafe { &*ctx.call_site };

  let (param_types, param_tail) = flatten_type_pack_id(ctx.arguments);

  if param_types.is_empty() && param_tail.is_none() {
    typechecker.report_error_type_error_data_location(
      TypeErrorData::CountMismatch(CountMismatch {
        expected: 1,
        maximum: Some(1),
        actual: 0,
        context: CountMismatchContext::Arg,
        is_variadic: false,
        function: "table.freeze".to_string(),
      }),
      &call_site.base.base.location,
    );
    return true;
  }

  let mut first_param_type: Option<TypeId> = None;

  if !param_types.is_empty() {
    first_param_type = Some(param_types[0]);
  } else if let Some(param_tail) = param_tail {
    // TODO (CLI-185019): We ideally want to report a Count Mismatch error if there's no head but a variadic tail, but CountMismatch requires
    // actual count size, which we don't have with variadic tails, so we can't report it properly yet. Instead, we continue to typecheck with the
    // first argument in the variadic tail and report a type mismatch error based on that, which is more informative than reporting a count
    // mismatch where the head (paramTypes.size()) is 0.
    first_param_type = first(param_tail, false);
  }

  if let Some(first_param_type) = first_param_type {
    // If a type is found, check if it is a subtype of table.
    typechecker.test_is_subtype_type_id_type_id_location(
      follow_type::follow(first_param_type),
      builtin_types.table_type,
      call_site.base.base.location,
    );
  } else {
    // If we can't get a type from the type or type pack, we testIsSubtype against the entire context's argument type pack to report a Type Pack
    // Mismatch error.
    let table_ty_pack =
      // Safety: typechecker 为上方已证的活 `&mut`；其 .module 是 TypeChecker2 构造接线期非空、
      // 比 typechecker 长寿的 `*mut Module`，internal_types 为其内嵌 TypeArena。此刻独占借用该 arena
      // 仅用于 add_type_pack，单线程、无并存别名。
      unsafe { &mut (*typechecker.module).internal_types }
      // Safety: typechecker.builtin_types 为构造期 NotNull 注入的非空 BuiltinTypes 单例，只读共享
      // 借用取 table_type，与上面的 arena 借用作用于不同对象、不混叠。
      .add_type_pack_initializer_list_type_id(&[typechecker.builtin_types.get().table_type]);
    typechecker.test_is_subtype_type_pack_id_type_pack_id_location(
      follow_type_pack::follow(ctx.arguments),
      table_ty_pack,
      call_site.base.base.location,
    );
    return true;
  }

  // Also report error if there's more than 1 argument explicitly provided to table.freeze.
  if param_types.len() > 1 {
    typechecker.report_error_type_error_data_location(
      TypeErrorData::CountMismatch(CountMismatch {
        expected: 1,
        maximum: Some(1),
        actual: call_site.args.size,
        context: CountMismatchContext::Arg,
        is_variadic: false,
        function: "table.freeze".to_string(),
      }),
      &call_site.base.base.location,
    );
  }

  true
}
