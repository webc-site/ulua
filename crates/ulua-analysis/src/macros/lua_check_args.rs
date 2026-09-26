//! `types.*`/`luau.*` C 函数入口的实参形态守卫骨架单点。
//!
//! cpp 侧每个 builtin 类型函数 C 入口都以同一形状开场：`lua_gettop` 取实参数、
//! 与期望形态比较、不符即 `throwTypeError(L, "type.xxx: expected N arguments,
//! but got X")`。本仓库原先把这副骨架在 20+ 个模块里各手抄一遍（仅守卫算符、
//! 边界值与消息字面量不同）。[`lua_check_args!`] 展开逐字等价，调用点只剩
//! 「守卫三元组 + 消息字面量」。

/// 生成「取实参数并在形态违例时抛 lua type error」的入口守卫。
///
/// 用法：
/// ```ignore
/// lua_check_args!(vm_l, != 1, "type.value: expected 1 argument, but got {}");
/// ```
/// 格式串即收口前 `format_args!` 的同一字面量，抛出消息逐字不变。默认形展开
/// 引入的 `argument_count` 因宏 hygiene 仅守卫式可见；入口后续仍需按实参个数
/// 判别可选参数缺省时，用具名绑定形把计数绑定让给调用作用域：
/// ```ignore
/// lua_check_args!(argument_count = vm_l, 2..=3, "type.setproperty: expected 2-3 arguments, but got {}");
/// ```
macro_rules! lua_check_args {
  // 具名绑定形须先于匿名形匹配：否则 `argument_count = vm_l` 会被 `$vm_l:expr`
  // 贪婪解析为赋值表达式。
  ($count:ident = $vm_l:expr, $op:tt $bound:literal, $fmt:literal) => {
    let $count = lua_gettop($vm_l);
    if $count $op $bound {
      throw_type_error($vm_l, format_args!($fmt, $count));
    }
  };
  ($count:ident = $vm_l:expr, $lo:literal..=$hi:literal, $fmt:literal) => {
    let $count = lua_gettop($vm_l);
    if !($lo..=$hi).contains(&$count) {
      throw_type_error($vm_l, format_args!($fmt, $count));
    }
  };
  ($vm_l:expr, $op:tt $bound:literal, $fmt:literal) => {
    let argument_count = lua_gettop($vm_l);
    if argument_count $op $bound {
      throw_type_error($vm_l, format_args!($fmt, argument_count));
    }
  };
  // 区间形态守卫（cpp `!(2 <= n && n <= 3)` 的 Rust 直译）。
  ($vm_l:expr, $lo:literal..=$hi:literal, $fmt:literal) => {
    let argument_count = lua_gettop($vm_l);
    if !($lo..=$hi).contains(&argument_count) {
      throw_type_error($vm_l, format_args!($fmt, argument_count));
    }
  };
}

/// 生成「实参类型形态违例时连同其 tag 抛 lua type error」的守卫。
///
/// 展开为 `if $cond { throw_type_error($vm_l, format_args!($fmt, get_tag($l, $tag_arg))); }`，
/// 与收口前各入口手抄的 9 行骨架逐字符等价；`$l` 为入口原始 `lua_State*` 形参
/// （local 变量跨不进宏 hygiene，须由调用点显式传入）。
macro_rules! lua_check_tag {
  ($vm_l:expr, $cond:expr, $l:expr, $tag_arg:expr, $fmt:literal) => {
    if $cond {
      throw_type_error($vm_l, format_args!($fmt, get_tag($l, $tag_arg)));
    }
  };
}

/// 生成「self 已 frozen 时拒改」的守卫（8 个 `type.set*` 入口共用）。
///
/// 展开与收口前手抄的 9 行骨架逐字符等价：`fflag` 与 `throw_type_error` 按宏
/// 展开点解析（各入口均已 import）；消息经 `concat!` 拼接为同一字面量。
macro_rules! lua_check_not_frozen {
  ($vm_l:expr, $self_ty:expr, $prefix:literal) => {
    if fflag::LuauTypeFunctionSupportsFrozen.get() && (*$self_ty).frozen {
      throw_type_error(
        $vm_l,
        format_args!(concat!(
          $prefix,
          ": cannot be called to mutate a frozen type, use `types.copy` to make a copy"
        )),
      );
    }
  };
}

pub(crate) use lua_check_args;
pub(crate) use lua_check_not_frozen;
pub(crate) use lua_check_tag;
