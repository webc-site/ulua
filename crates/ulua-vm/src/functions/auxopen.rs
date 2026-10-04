use crate::{
  functions::tstr_bytes::cut_at_nul,
  records::lua_state::LuaState,
  type_aliases::lua_c_function::LuaCFunction,
};

/// 库注册对：压入初始化函数 `u`、以其为上值再压主函数 `f`，并挂到 -2 处的库表。
///
/// review.md §10/判例3 收形：`l` 收独占引用（存活由类型承载）、`name` 为静态
/// 选项名字节窗（debugname 存引用、`set_field_bytes` 直写），签名无调用方裸
/// 指针解引用位，降为 safe `fn`。
///
/// 调用序契约（正确性，非内存安全）：`l` 必须是库 open 期间存活的调用帧，且
/// 栈顶（-2 处）已压入待填充的库表，供 `set_field_bytes` 写入；`f`/`u` 遵循
/// Lua C 函数约定。违反将写坏栈布局或调到悬垂的 C 函数指针。cpp lbaselib.cpp:453。
pub(crate) fn auxopen(l: &mut LuaState, name: &'static [u8], f: LuaCFunction, u: LuaCFunction) {
  let name = cut_at_nul(name);
  l.push_c_function(u, None);
  // cpp 以主函数自身名为 debugname（静态注册名窗，闭包存活契约同旧指针面）
  l.push_c_closure(f, Some(name), 1);
  l.set_field_bytes(-2, name);
}
