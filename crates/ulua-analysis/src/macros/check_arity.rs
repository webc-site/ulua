//! 「内建类型函数 arity 前奏」单点收口（cpp `BuiltinTypeFunctions.cpp` 各
//! `xxxTypeFunction` 入口的实参形态校验）。
//!
//! 原先 20 处散落在 `src/functions/*.rs` 的前奏逐字雷同：
//! `if type_params.len() != N || !pack_params.is_empty() { ice 上报; LUAU_ASSERT!(false); }`
//! （intersect/union/refine 三处条件形态略有变体），相互只有期望 arity、是否上报
//! ICE、以及文案里的函数名前缀三点差异。本宏把这几种现行形态收口到一处，报错文案
//! 与「先上报、后断言」的顺序再无逐点漂移的可能。

/// arity 守卫，各臂与现状逐字对应：
/// - `(ctx, tp, pp, N)`：上报通用文案（numeric_binop/comparison 现状）；
/// - `(ctx, tp, pp, N, "name")`：上报 `"<name> type function: …"` 文案，由
///   `concat!` 编译期拼接，运行期文本与原各点字面量逐字节一致；
/// - `(ctx, tp, pp, N, ice_msg <expr>)`：以上下文表达式为文案（and/or 共享骨架
///   由调用方传入完整消息的现状）；
/// - `(tp, pp, N, assert_only)`：仅断言、不上报（keyof/rawkeyof/setmetatable/
///   rawget/index 等外层形态——其 ICE 上报由下游 impl 完成，现状如此，不改）。
/// - `(ctx, guard <cond>, "name")`：形态条件为任意布尔表达式（refine 要求至少
///   2 个类型参）且首词不是 `!`（宏 expr 元变量对 `!` 开头操作数的解析限制，
///   若加括号又会触发 `unused_parens`），上报文案同由 `concat!` 编译期拼接。
/// - `(ctx, pp, no_packs, "name")`：类型参 arity 不限、仅禁 pack 参
///   （intersect/union 现状）。
///
/// 为什么是宏而不是 `TypeFunctionContext` 上的方法：`LUAU_ASSERT!` 展开时把
/// `file!()`/`line!()` 烧进断言上报负载，移进函数会让全部站点的断言现场变成
/// 收口文件一处；宏在每个调用点展开，保持逐站点的文件/行号。ice 访问统一经
/// `TypeFunctionContext::ice()` 安全访问器（NonNull 契约在其内部单点收口），
/// 与原各点 `as_ref()/as_ptr()` 手工解引用的运行期效果逐字一致。
macro_rules! check_arity {
  // 关键字臂必须先于通用 expr 臂：`expr` 元变量会吞下同为合法表达式的臂关键字。
  ($ctx:expr, $pack_params:expr, no_packs, $name:literal) => {
    if !$pack_params.is_empty() {
      $ctx.ice().ice_string(concat!(
        $name,
        " type function: encountered a type function instance without the required argument structure"
      ));
      ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
    }
  };
  ($ctx:expr, $type_params:expr, $pack_params:expr, $n:literal) => {
    if $type_params.len() != $n || !$pack_params.is_empty() {
      $ctx.ice().ice_string("encountered a type function instance without the required argument structure");
      ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
    }
  };
  ($ctx:expr, $type_params:expr, $pack_params:expr, $n:literal, $name:literal) => {
    if $type_params.len() != $n || !$pack_params.is_empty() {
      $ctx.ice().ice_string(concat!(
        $name,
        " type function: encountered a type function instance without the required argument structure"
      ));
      ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
    }
  };
  ($ctx:expr, $type_params:expr, $pack_params:expr, $n:literal, ice_msg $msg:expr) => {
    if $type_params.len() != $n || !$pack_params.is_empty() {
      $ctx.ice().ice_string($msg);
      ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
    }
  };
  ($ctx:expr, guard $cond:expr, $name:literal) => {
    if $cond {
      $ctx.ice().ice_string(concat!(
        $name,
        " type function: encountered a type function instance without the required argument structure"
      ));
      ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
    }
  };
  ($type_params:expr, $pack_params:expr, $n:literal, assert_only) => {
    if $type_params.len() != $n || !$pack_params.is_empty() {
      ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
    }
  };
}

pub(crate) use check_arity;
