//! Builtin 二元运算类型函数包装骨架单点（`BuiltinTypeFunctions.cpp`）。
//!
//! C++ 侧 `add/sub/mul/div/pow/idiv/modTypeFunction` 七个入口是同一形状：校验
//! 实参形态（恰 2 个类型参、0 个类型包参）后转调
//! `numericBinopTypeFunction(..., "__xxx")`，只有方法名与元方法字面量不同。
//! 本仓库原先把这副骨架在 7 个模块里各手抄一遍（含逐字重复的 import 块与
//! `# Safety` 契约文本）。[`numeric_binop_wrapper!`] 保留全部对外路径、函数名、
//! 签名与守卫语义不变，调用点只剩「C++ 对应说明 + 方法名 + 元方法字面量」。
//! 比较族 `le/ltTypeFunction` 骨架同形（转调 `comparisonTypeFunction`），经第三
//! 参数显式指定目标函数复用同一宏体。

/// 生成一枚「校验二元实参形态并转发目标类型函数」的 `pub unsafe fn` 入口。
///
/// 用法：
/// ```ignore
/// numeric_binop_wrapper!(
///   /// 对应 C++ `addTypeFunction`（BuiltinTypeFunctions.cpp:490-505）。
///   add_type_function, "__add"
/// );
/// numeric_binop_wrapper!(
///   /// 对应 C++ `leTypeFunction`。
///   le_type_function, "__le",
///   crate::functions::comparison_type_function::comparison_type_function
/// );
/// ```
macro_rules! numeric_binop_wrapper {
  ($(#[$attr:meta])* $name:ident, $metamethod:literal) => {
    numeric_binop_wrapper!(
      $(#[$attr])*
      $name,
      $metamethod,
      crate::functions::numeric_binop_type_function::numeric_binop_type_function
    );
  };
  ($(#[$attr:meta])* $name:ident, $metamethod:literal, $target:path) => {
    $(#[$attr])*
    ///
    /// # Safety
    /// 本函数作为 `ReducerFunction`（type_aliases/reducer_function.rs）函数指针被
    /// TypeFunctionReducer 调用：`ctx` 须为整个 reduce 步进期内独占存活的
    /// `TypeFunctionContext` 借用（cpp `NotNull<TypeFunctionContext>`），并原样
    /// 下传给宏第三参指定的目标转调实现；`type_params` 须恰 2 项、
    /// `pack_params` 须为空（上方守卫即 cpp 同位 `LUAU_ASSERT`），`instance` 与
    /// 切片内句柄须为存活类型 arena 节点。
    pub unsafe fn $name(
      instance: crate::type_aliases::type_id::TypeId,
      type_params: &[crate::type_aliases::type_id::TypeId],
      pack_params: &[crate::type_aliases::type_pack_id::TypePackId],
      ctx: &mut crate::records::type_function_context::TypeFunctionContext,
    ) -> crate::records::type_function_reduction_result::TypeFunctionReductionResult {
      if type_params.len() != 2 || !pack_params.is_empty() {
        ulua_common::macros::luau_assert::LUAU_ASSERT!(false);
      }

      // SAFETY: 调用 `unsafe fn` 目标转调实现；其 ctx/切片/句柄前置条件由本函数
      // fn 级契约逐字承接（同一借用原样透传，不并存第二别名）。
      unsafe {
        $target(
          instance,
          type_params,
          pack_params,
          ctx,
          ::alloc::string::String::from($metamethod),
        )
      }
    }
  };
}

pub(crate) use numeric_binop_wrapper;
