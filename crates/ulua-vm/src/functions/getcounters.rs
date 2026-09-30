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
/// `l` 必须指向存活 `LuaState`；`p` 须为指向存活 `Proto` 的共享引用（子原型数组元素亦须存活，递归下传），
/// 且所查询的输出记录按约定存活可写。本函数只读 Proto，仅经 C ABI 回调边界转手一次可变指针（见体内注释）。
pub(crate) unsafe fn getcounters(
  l: *mut LuaState,
  p: &Proto,
  context: *mut c_void,
  functionvisit: LuaCounterFunction,
  countervisit: LuaCounterValue,
) {
  /// ecb 回调返回的计数槽布局：kind(u32) + pcpos(u32) + hits(u64)，
  /// 与 cpp 一致用 sizeof 表达式而非魔法数字
  const COUNTER_SLOT_SIZE: usize = size_of::<u32>() * 2 + size_of::<u64>();

  // SAFETY: 契约保证 `p` 为存活 Proto、execdata 非空时反馈向量与 sizecode 一致，`counters` 为调用方可写输出数组
  unsafe {
    if !p.execdata.is_null() {
      let l_ref = &*l;
      let global = l_ref.global;
      // if let 替代 is_none + unwrap，Option 由类型系统收口非空
      if !global.is_null()
        && let Some(getcounterdata) = (*global).ecb.getcounterdata
      {
        let mut count: usize = 0;
        // SAFETY(review §2): `getcounterdata` 为 C ABI 宿主回调，签名固定收 `*mut Proto`；
        // 其契约为「只量取计数数据、不改 Proto」，故此处的可变指针只跨越该外部调用边界，
        // 本函数不据此解写。
        let data = getcounterdata(l, (p as *const Proto).cast_mut(), &mut count as *mut usize);

        if !data.is_null() && count != 0 {
          let debugname = if !p.debugname.is_null() {
            getstr(p.debugname)
          } else {
            null()
          };
          let linedefined = p.linedefined;

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
              p.linedefined
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

    // SAFETY:p 为有效 Proto，sizep 与子 proto 数组分配一致。
    for &child in c_slice(p.p, p.sizep as usize) {
      getcounters(l, &*child, context, functionvisit, countervisit);
    }
  }
}
