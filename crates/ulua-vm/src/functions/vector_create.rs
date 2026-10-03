use crate::{
  functions::vector_shared::vector_push,
  macros::{lua_lib_fn::lua_lib_fn, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::LuaState,
};

/// cpp `vector.create`（VM/src/lveclib.cpp:10）：按 2..=4 个数值实参构造 vector 压回。
///
/// 调用序契约（正确性，非内存安全）：以 Lua 库函数约定被调，`l` 的存活与独占由 `&mut` 接收者
/// 承载且处于受保护帧——以 `get_top` 取实参数 count；索引 1、2 必需数值（缺失/非数值
/// `check_number` 抛错回退），索引 3、4 可选（count≥3/≥4 时才 check，否则补 0.0）；结果槽由
/// `vector_push` 内部 `ensure_stack` 预留，无需调用方扩栈；可触发 GC。
pub(crate) fn vector_create(l: &mut LuaState) -> i32 {
  let count = l.get_top();

  let x = l.check_number(1);
  let y = l.check_number(2);
  let z = if count >= 3 { l.check_number(3) } else { 0.0 };
  // 短路的 `LUA_VECTOR_SIZE == 4` 保证 3 分量配置下从不按索引 4 校验实参（缺参即抛错）
  let w = if LUA_VECTOR_SIZE == 4 && count >= 4 {
    l.check_number(4)
  } else {
    0.0
  };

  vector_push(l, [x as f32, y as f32, z as f32, w as f32]);

  1
}

lua_lib_fn!(pub(crate) fn vector_create @ref, vector_create_arm);
