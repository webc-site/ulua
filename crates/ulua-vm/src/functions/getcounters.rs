use core::{
  ffi::c_void,
  mem::size_of,
  ptr::{null, read_unaligned},
};

use crate::{
  functions::{c_slice, lua_g_getline::lua_g_getline},
  macros::getstr::getstr,
  records::{lua_state::LuaState, proto::Proto},
  type_aliases::{lua_counter_function::LuaCounterFunction, lua_counter_value::LuaCounterValue},
};

/// # Safety
/// `l` 必须指向存活 `LuaState` 且所查询的调用帧/Proto/输出记录按约定存活可写。
pub(crate) unsafe fn getcounters(
  l: *mut LuaState,
  p: *mut Proto,
  context: *mut c_void,
  functionvisit: LuaCounterFunction,
  countervisit: LuaCounterValue,
) {
  /// ecb 回调返回的计数槽布局：kind(u32) + pcpos(u32) + hits(u64)，
  /// 与 cpp 一致用 sizeof 表达式而非魔法数字
  const COUNTER_SLOT_SIZE: usize = size_of::<u32>() * 2 + size_of::<u64>();

  // Safety: 契约保证 `p` 为存活 Proto、execdata 非空时反馈向量与 sizecode 一致，`counters` 为调用方可写输出数组
  unsafe {
    let p_ref = &*p;
    if !p_ref.execdata.is_null() {
      let l_ref = &*l;
      let global = l_ref.global;
      // if let 替代 is_none + unwrap，Option 由类型系统收口非空
      if !global.is_null()
        && let Some(getcounterdata) = (*global).ecb.getcounterdata
      {
        let mut count: usize = 0;
        let data = getcounterdata(l, p, &mut count as *mut usize);

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

          // 槽布局 kind(u32) + pcpos(u32) + hits(u64)：偏移同样由 sizeof 推导
          let slots = c_slice(data as *const u8, count * COUNTER_SLOT_SIZE);
          for slot in slots.as_chunks::<COUNTER_SLOT_SIZE>().0 {
            let kind = read_unaligned(slot.as_ptr() as *const u32);
            let pcpos = read_unaligned(slot[size_of::<u32>()..].as_ptr() as *const u32);
            let hits = read_unaligned(slot[size_of::<u32>() * 2..].as_ptr() as *const u64);

            let line = if pcpos == !0u32 {
              p_ref.linedefined
            } else {
              lua_g_getline(p, pcpos as i32)
            };

            if let Some(cv) = countervisit {
              cv(context, kind as i32, line, hits);
            }
          }
        }
      }
    }

    // Safety:p 为有效 Proto，sizep 与子 proto 数组分配一致。
    for &child in c_slice(p_ref.p, p_ref.sizep as usize) {
      getcounters(l, child, context, functionvisit, countervisit);
    }
  }
}
