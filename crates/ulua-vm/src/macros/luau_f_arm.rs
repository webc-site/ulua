//! 快速调用内建臂统一签名宏。`LUAU_F_TABLE` 全部臂共享 `LuauFastCallable` 的
//! 六参 ABI 签名 `(l, res, arg0, nresults, args, nparams)`（cpp `luau_FastFunction`），
//! 此前 9 处各写 8 行签名，其中 6 处还各重复「`let Some(args) = args else { return -1 };
//! let args = args as *mut TValue;`」两行实参解除样板。本宏一次承包签名与解除样板，
//! 臂只留文档注释与函数体。
//!
//! 用法：`luau_f_arm! { 文档… vis fn 名 [<泛型实参>] (res, arg0, nresults, args, nparams)
//! [=> args] 体块 }`。
//! - 泛型走 `[…]` 方括号组原样展开为 `<…>`（非泛型臂传 `[]`；嵌套泛型界须写成
//!   `Into<f64> >` 加空格，避免 `>>` 塌成单个 Shr token）；
//! - 形参名由调用方给出（与体块同卫生域），不读的槽位须直接传 `_` 前缀名，宏体不
//!   设任何 `allow`；
//! - `=> args` 标记在体前生成 Option 解除两行（快速路径要求 ≥2 实参、`None` 即
//!   FASTCALL1 单实参派发回退慢路径；解除后转回裸槽指针，与 cpp 连续槽访问形态一致）。

#[macro_export]
macro_rules! luau_f_arm {
  (
    $(#[$meta:meta])*
    $vis:vis fn $name:ident [$($gen:tt)*]
    ($res:ident, $arg0:ident, $nresults:ident, $args:ident, $nparams:ident)
    $(=> $deref:ident)?
    $body:block
  ) => {
    $(#[$meta])*
    $vis unsafe extern "C-unwind" fn $name $($gen)* (
      _l: *mut $crate::records::lua_state::LuaState,
      $res: $crate::type_aliases::stk_id::StkId,
      $arg0: *mut $crate::type_aliases::t_value::TValue,
      $nresults: i32,
      $args: Option<&mut $crate::type_aliases::t_value::TValue>,
      $nparams: i32,
    ) -> i32 {
      $(
        // `None` = FASTCALL1 单实参派发，快速路径要求 ≥2 实参，回退慢路径；
        // 解除后转回裸槽指针（宏/下标算术按指针重复取标签/值），与 cpp 连续槽访问形态一致
        let Some($deref) = $deref else { return -1 };
        let $deref = $deref as *mut $crate::type_aliases::t_value::TValue;
      )?
      $body
    }
  };
}

pub use luau_f_arm;
