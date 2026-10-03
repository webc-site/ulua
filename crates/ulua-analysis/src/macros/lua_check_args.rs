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
/// lua_check_args!(l, != 1, "type.value: expected 1 argument, but got {}");
/// ```
/// 格式串即收口前 `format_args!` 的同一字面量，抛出消息逐字不变。默认形展开
/// 引入的 `argument_count` 因宏 hygiene 仅守卫式可见；入口后续仍需按实参个数
/// 判别可选参数缺省时，用具名绑定形把计数绑定让给调用作用域：
/// ```ignore
/// lua_check_args!(argument_count = l, 2..=3, "type.setproperty: expected 2-3 arguments, but got {}");
/// ```
/// `$l` 为入口的 `&mut LuaState`。
macro_rules! lua_check_args {
  // 具名绑定形须先于匿名形匹配：否则 `argument_count = l` 会被 `$l:expr`
  // 贪婪解析为赋值表达式。
  ($count:ident = $l:expr, $op:tt $bound:literal, $fmt:literal) => {
    let $count = (*$l).get_top();
    if $count $op $bound {
      throw_type_error($l, format_args!($fmt, $count));
    }
  };
  ($count:ident = $l:expr, $lo:literal..=$hi:literal, $fmt:literal) => {
    let $count = (*$l).get_top();
    if !($lo..=$hi).contains(&$count) {
      throw_type_error($l, format_args!($fmt, $count));
    }
  };
  ($l:expr, $op:tt $bound:literal, $fmt:literal) => {
    let argument_count = (*$l).get_top();
    if argument_count $op $bound {
      throw_type_error($l, format_args!($fmt, argument_count));
    }
  };
  // 区间形态守卫（cpp `!(2 <= n && n <= 3)` 的 Rust 直译）。
  ($l:expr, $lo:literal..=$hi:literal, $fmt:literal) => {
    let argument_count = (*$l).get_top();
    if !($lo..=$hi).contains(&argument_count) {
      throw_type_error($l, format_args!($fmt, argument_count));
    }
  };
}

/// 生成「实参类型形态违例时连同其 tag 抛 lua type error」的守卫。
///
/// 展开为「先取 tag、再抛错」两步，与收口前各入口手抄的 9 行骨架逐字符等价。
/// tag 必须先落成局部量：[`get_tag`] 与 [`throw_type_error`] 都要借入同一个
/// `$l`，写成嵌套实参会让两次可变借用重叠。tag 绑定名受宏 hygiene 保护，不外泄。
macro_rules! lua_check_tag {
  ($l:expr, $cond:expr, $tag_arg:expr, $fmt:literal) => {
    if $cond {
      let tag = get_tag($l, $tag_arg);
      throw_type_error($l, format_args!($fmt, tag));
    }
  };
}

/// 生成「self 已 frozen 时拒改」的守卫（8 个 `type.set*` 入口共用）。
///
/// 展开与收口前手抄的 9 行骨架逐字符等价：`fflag` 与 `throw_type_error` 按宏
/// 展开点解析（各入口均已 import）；消息经 `concat!` 拼接为同一字面量。
macro_rules! lua_check_not_frozen {
  ($l:expr, $self_ty:expr, $prefix:literal) => {
    if fflag::LuauTypeFunctionSupportsFrozen.get() && (*$self_ty).frozen {
      throw_type_error(
        $l,
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
