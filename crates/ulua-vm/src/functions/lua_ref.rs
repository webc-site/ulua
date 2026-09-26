use crate::{
  functions::{index_2_addr::index_2_addr, lua_h_getn::lua_h_getn, lua_h_setnum::lua_h_setnum},
  macros::{
    api_check::api_check, lua_c_barriert::luaC_barriert, lua_refnil::LUA_REFNIL,
    lua_registryindex::LUA_REGISTRYINDEX, registry::registry, setobj_2_t::setobj2t,
  },
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).global` 存活：`api_check!(idx != LUA_REGISTRYINDEX)`，`index_2_addr` 所得 `p` 为栈内
/// 合法槽（非 nil 时登记）；registry 表 `(*registry!(l)).as_table_ptr()` 须存活可写，`lua_h_setnum` 可能 rehash（写 `slot`、`luaC_barriert`
/// 置灰、GC 期间该引用不可被回收），`(*g).registryfree`/`totalbytes` 读写须处于正常 GC 态。返回引用号（LUA_REFNIL 表未登记）。
/// cpp VM/src/lapi.cpp:1867
pub unsafe fn lua_ref(l: *mut LuaState, idx: i32) -> i32 {
  unsafe {
    api_check!(l, idx != LUA_REGISTRYINDEX);

    let mut ref_ = LUA_REFNIL;
    let g = (*l).global;
    let p: StkId = index_2_addr(l, idx);

    if !(*p).is_nil() {
      let reg: *mut LuaTable = (*registry!(l)).as_table_ptr();

      if (*g).registryfree != 0 {
        ref_ = (*g).registryfree;
      } else {
        // no free elements
        ref_ = lua_h_getn(reg);
        ref_ += 1; // create new reference
      }

      let slot: *mut TValue = lua_h_setnum(l, reg, ref_);
      if (*g).registryfree != 0 {
        (*g).registryfree = (*slot).as_number() as i32;
      }

      setobj2t!(l, slot, p);

      luaC_barriert!(l, reg, p);
    }

    ref_
  }
}
