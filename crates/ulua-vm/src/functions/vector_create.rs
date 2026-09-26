use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checknumber::lua_l_checknumber, vector_shared::vector_push,
  },
  macros::{lua_lib_fn::lua_lib_fn, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：以 `lua_gettop` 取实参数 count；索引 1、2 必需数值（缺失/非数值
/// `lua_l_checknumber` 抛错回退），索引 3、4 可选（count≥3/≥4 时才 checknumber，否则补 0.0）；`lua_pushvector`
/// 需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lveclib.cpp:10
pub(crate) unsafe fn vector_create(l: *mut LuaState) -> i32 {
  // Safety: 契约保证各索引为数值实参，新 vector 由 `lua_pushvector*` 内部扩栈后压入
  unsafe {
    let count = lua_gettop(l);

    let x = lua_l_checknumber(l, 1);
    let y = lua_l_checknumber(l, 2);
    let z = if count >= 3 {
      lua_l_checknumber(l, 3)
    } else {
      0.0
    };
    // 短路的 `LUA_VECTOR_SIZE == 4` 保证 3 分量配置下从不按索引 4 校验实参（缺参即抛错）
    let w = if LUA_VECTOR_SIZE == 4 && count >= 4 {
      lua_l_checknumber(l, 4)
    } else {
      0.0
    };

    vector_push(l, [x as f32, y as f32, z as f32, w as f32]);

    1
  }
}

lua_lib_fn!(pub(crate) fn vector_create, vector_create_arm);
