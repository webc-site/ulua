//! Source: `VM/src/lveclib.cpp:301-359` (hand-ported)

use crate::{
  functions::{
    createmetatable_lveclib::createmetatable,
    lua_l_register::lua_l_register,
    lua_pushvector_lapi::{
      lua_pushvector_lua_state_f32_f32_f32, lua_pushvector_lua_state_f32_f32_f32_f32,
    },
    lua_setfield::lua_setfield,
    vector_angle::vector_angle,
    vector_clamp::vector_clamp,
    vector_create::vector_create,
    vector_cross::vector_cross,
    vector_dot::vector_dot,
    vector_lerp::vector_lerp,
    vector_magnitude::vector_magnitude,
    vector_normalize::vector_normalize,
    vector_shared::{vector_abs, vector_ceil, vector_floor, vector_max, vector_min, vector_sign},
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  records::{lua_l_reg::LuaLReg, lua_state::LuaState},
};

static VECTOR_FUNCS: [LuaLReg; 14] = [
  LuaLReg::new(b"create", vector_create),
  LuaLReg::new(b"magnitude", vector_magnitude),
  LuaLReg::new(b"normalize", vector_normalize),
  LuaLReg::new(b"cross", vector_cross),
  LuaLReg::new(b"dot", vector_dot),
  LuaLReg::new(b"angle", vector_angle),
  LuaLReg::new(b"floor", vector_floor),
  LuaLReg::new(b"ceil", vector_ceil),
  LuaLReg::new(b"abs", vector_abs),
  LuaLReg::new(b"sign", vector_sign),
  LuaLReg::new(b"clamp", vector_clamp),
  LuaLReg::new(b"max", vector_max),
  LuaLReg::new(b"min", vector_min),
  LuaLReg::new(b"lerp", vector_lerp),
];

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn luaopen_vector(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_register(l, c"vector".as_ptr(), &VECTOR_FUNCS);

    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_lua_state_f32_f32_f32_f32(l, 0.0, 0.0, 0.0, 0.0);
      lua_setfield(l, -2, c"zero".as_ptr());
      lua_pushvector_lua_state_f32_f32_f32_f32(l, 1.0, 1.0, 1.0, 1.0);
      lua_setfield(l, -2, c"one".as_ptr());
    } else {
      lua_pushvector_lua_state_f32_f32_f32(l, 0.0, 0.0, 0.0);
      lua_setfield(l, -2, c"zero".as_ptr());
      lua_pushvector_lua_state_f32_f32_f32(l, 1.0, 1.0, 1.0);
      lua_setfield(l, -2, c"one".as_ptr());
    }

    createmetatable(l);

    1
  }
}
