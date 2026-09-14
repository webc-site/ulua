//! Node: `cxx:Function:Luau.VM:VM/src/lveclib.cpp:341:luaopen_vector`
//! Source: `VM/src/lveclib.cpp:301-359` (hand-ported)

use core::{ffi::c_int, ptr::null};

use crate::{
  functions::{
    createmetatable_lveclib::createmetatable, lua_l_register::lua_l_register,
    lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32,
    lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32, lua_setfield::lua_setfield,
    vector_abs::vector_abs, vector_angle::vector_angle, vector_ceil::vector_ceil,
    vector_clamp::vector_clamp, vector_create::vector_create, vector_cross::vector_cross,
    vector_dot::vector_dot, vector_floor::vector_floor, vector_lerp::vector_lerp,
    vector_magnitude::vector_magnitude, vector_max::vector_max, vector_min::vector_min,
    vector_normalize::vector_normalize, vector_sign::vector_sign,
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  records::lua_l_reg::LuaLReg,
  type_aliases::lua_state::lua_State,
};

struct VectorFuncs([LuaLReg; 15]);
unsafe impl Sync for VectorFuncs {}

static VECTOR_FUNCS: VectorFuncs = VectorFuncs([
  LuaLReg {
    name: c"create".as_ptr(),
    func: Some(vector_create),
  },
  LuaLReg {
    name: c"magnitude".as_ptr(),
    func: Some(vector_magnitude),
  },
  LuaLReg {
    name: c"normalize".as_ptr(),
    func: Some(vector_normalize),
  },
  LuaLReg {
    name: c"cross".as_ptr(),
    func: Some(vector_cross),
  },
  LuaLReg {
    name: c"dot".as_ptr(),
    func: Some(vector_dot),
  },
  LuaLReg {
    name: c"angle".as_ptr(),
    func: Some(vector_angle),
  },
  LuaLReg {
    name: c"floor".as_ptr(),
    func: Some(vector_floor),
  },
  LuaLReg {
    name: c"ceil".as_ptr(),
    func: Some(vector_ceil),
  },
  LuaLReg {
    name: c"abs".as_ptr(),
    func: Some(vector_abs),
  },
  LuaLReg {
    name: c"sign".as_ptr(),
    func: Some(vector_sign),
  },
  LuaLReg {
    name: c"clamp".as_ptr(),
    func: Some(vector_clamp),
  },
  LuaLReg {
    name: c"max".as_ptr(),
    func: Some(vector_max),
  },
  LuaLReg {
    name: c"min".as_ptr(),
    func: Some(vector_min),
  },
  LuaLReg {
    name: c"lerp".as_ptr(),
    func: Some(vector_lerp),
  },
  LuaLReg {
    name: null(),
    func: None,
  },
]);

pub(crate) unsafe extern "C-unwind" fn luaopen_vector(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_register(l, c"vector".as_ptr(), VECTOR_FUNCS.0.as_ptr());

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
