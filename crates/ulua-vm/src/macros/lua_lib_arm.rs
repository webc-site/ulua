//! 库 C 臂统一骨架宏。math/int64/vector/bit32/string 各库的「取实参 → 纯计算 → 压结果」
//! 臂共享同一式样：单行 crate 树 + 2-4 行 # Safety 文档 +
//! `pub unsafe extern "C-unwind" fn 名(l) -> i32` + 一行核心调用。本宏承包签名与文档
//! 属性，臂只补形参名与体块；`unsafe` 块由臂自写（宏定义内不含任何 metavariable 落进
//! unsafe 块的写法），故 `clippy::macro_metavars_in_unsafe` 无从触发，且「调用了不安全
//! 核心」这一事实留在调用侧可见。
//!
//! 用法：`lua_lib_arm! { 文档… vis fn 名 (l) { unsafe { 核心(l, 实参…) } } }`。
//! 形参名由臂给出，须与体块同卫生域（宏定义里写死的 `l` 对臂体不可见，与 `luau_f_arm`
//! 同型）；不读栈的臂也须给出形参名（约定统一写 `l`）。
//!
//! 覆盖核心：math 库 [math_map1]/[math_pred1]/[math_extreme]、int64 库
//! `int64_unop`/`int64_binop`/`int64_cmp`/`int64_uarith`/`int64_extreme`/`int64_fold_push`、
//! vector 库 `vector_map1`/`vector_minmax`、bit32 库 `bit_map1` 与 string 库
//! `str_find_aux`（均为 `(l, 实参…)` 骨架，臂按调用处实参序转发，`int64_fold`
//! 裸值核心经 `int64_fold_push` 包压栈后走本宏）。
//!
//! [math_map1]: crate::functions::math_shared::math_map1
//! [math_pred1]: crate::functions::math_shared::math_pred1
//! [math_extreme]: crate::functions::math_shared::math_extreme

#[macro_export]
macro_rules! lua_lib_arm {
  ($(#[$meta:meta])* $vis:vis fn $name:ident ($l:ident) $body:block) => {
    $(#[$meta])*
    $vis unsafe extern "C-unwind" fn $name(
      $l: *mut $crate::records::lua_state::LuaState,
    ) -> i32
    // Safety: 契约由臂文档约定，核心函数仅按序做读槽（可抛错回退）→纯计算→压结果
    $body
  };
}

pub use lua_lib_arm;
