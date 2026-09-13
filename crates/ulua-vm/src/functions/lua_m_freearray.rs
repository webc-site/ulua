use core::{
  ffi::c_void,
  ptr::{null, read_unaligned},
};

use crate::{
  functions::lua_g_getline::luaG_getline,
  macros::getstr::getstr,
  type_aliases::{
    lua_counter_function::LuaCounterFunction, lua_counter_value::LuaCounterValue,
    lua_state::lua_State, proto::Proto,
  },
};

pub(crate) unsafe fn getcounters(
  l: *mut lua_State,
  p: *mut Proto,
  context: *mut c_void,
  functionvisit: LuaCounterFunction,
  countervisit: LuaCounterValue,
) {
  unsafe {
    let p_ref = &*p;
    if !p_ref.execdata.is_null() {
      let l_ref = &*l;
      let global = l_ref.global;
      if !global.is_null() && !(*global).ecb.getcounterdata.is_none() {
        let mut count: usize = 0;
        let data = (*global).ecb.getcounterdata.unwrap()(l, p, &mut count as *mut usize);

        if !data.is_null() && count != 0 {
          let debugname = if !p_ref.debugname.is_null() {
            getstr(p_ref.debugname)
          } else {
            null()
          };
          let linedefined = p_ref.linedefined;

          if let Some(fv) = functionvisit {
            fv(context, debugname, linedefined);
          }

          for i in 0..count {
            let ptr = (data as *const u8).add(i * (4 + 4 + 8));
            let kind = read_unaligned(ptr as *const u32);
            let pcpos = read_unaligned(ptr.add(4) as *const u32);
            let hits = read_unaligned(ptr.add(8) as *const u64);

            let line = if pcpos == !0u32 {
              p_ref.linedefined
            } else {
              luaG_getline(p, pcpos as i32)
            };

            if let Some(cv) = countervisit {
              cv(context, kind as i32, line, hits);
            }
          }
        }
      }
    }

    for i in 0..p_ref.sizep {
      let child = *p_ref.p.add(i as usize);
      getcounters(l, child, context, functionvisit, countervisit);
    }
  }
}
