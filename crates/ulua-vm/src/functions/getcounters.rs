use core::{
  ffi::c_void,
  mem::size_of,
  ptr::{NonNull, null, read_unaligned},
};

use crate::{
  functions::{c_slice, lua_g_getline::lua_g_getline},
  macros::getstr::getstr,
  records::{lua_state::LuaState, proto::Proto},
  type_aliases::{lua_counter_function::LuaCounterFunction, lua_counter_value::LuaCounterValue},
};

/// 计数器导出（cpp ldebug.cpp:612 `getcounters`）：经宿主 `getcounterdata` 量取计数槽向量
/// （kind(u32)+pcpos(u32)+hits(u64)），逐槽回调宿主。宿主 C 回调（getcounterdata/functionvisit/
/// countervisit）属运行期开放边界，unsafe 关在体内最小块并以本 doc/公开边界 `# Safety` 收口：
/// `lua_getcounters` 的调用方契约（`context` 与回调型别匹配、回调只读计数数据）在此被信任。
/// `p`（含递归子原型）与 `execdata` 反馈向量的存活自洽由宿主执行回调不变量保证。
pub(crate) fn getcounters(
  l: &LuaState,
  p: &Proto,
  context: *mut c_void,
  functionvisit: LuaCounterFunction,
  countervisit: LuaCounterValue,
) {
  /// ecb 回调返回的计数槽布局：kind(u32) + pcpos(u32) + hits(u64)，
  /// 与 cpp 一致用 sizeof 表达式而非魔法数字
  const COUNTER_SLOT_SIZE: usize = size_of::<u32>() * 2 + size_of::<u64>();

  if !p.execdata.is_null() {
    // SAFETY: 契约（见函数 doc）保证 `l.global` 存活；`getcounterdata` 为 C ABI 宿主回调，
    // 签名固定收 `*mut LuaState`/`*mut Proto`——其「只量取计数数据、不写穿 l/p」契约由公开边界
    // `lua_getcounters` 的 `# Safety` 收口，故 `read_ptr()` 只读转发与 `p` 的可变指针重取
    // 只跨越该外部调用边界，本函数不据此解写。
    let (data, count) = unsafe {
      let global = l.global;
      // if let 替代 is_none + unwrap，Option 由类型系统收口非空
      let mut count: usize = 0;
      // 「global 缺席 / 回调未安装 / 宿主返回 null」三态经 Option/NonNull 并为同一
      // `None`（review.md §2 规则 1），不再折回裸 null 哨兵后判空
      let data = if !global.is_null()
        && let Some(getcounterdata) = (*global).ecb.getcounterdata
      {
        NonNull::new(getcounterdata(
          l.read_ptr(),
          (p as *const Proto).cast_mut(),
          &mut count as *mut usize,
        ))
      } else {
        None
      };
      (data, count)
    };

    if let Some(data) = data
      && count != 0
    {
      let data = data.as_ptr();
      let debugname = if p.debugname.is_null() {
        null()
      } else {
        // SAFETY: 契约保证 `debugname` 非空即指向存活 `tstring`，`getstr` 仅取串体首址（只读）。
        unsafe { getstr(p.debugname) }
      };
      let linedefined = p.linedefined;

      // SAFETY: 宿主回调开放边界，同 `getcounterdata` 块的契约锚（context 型别匹配由
      // `lua_getcounters` 公开边界收口）。
      unsafe {
        if let Some(fv) = functionvisit {
          fv(context, debugname, linedefined);
        }
      }

      // SAFETY: 契约保证宿主反馈向量覆盖 `count * COUNTER_SLOT_SIZE` 字节且在本轮消费期内
      // 存活；`read_unaligned` 逐槽非对齐读取限于该切片界内。
      unsafe {
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

  // SAFETY: 契约保证子原型数组按 `sizep` 存活（递归下传，同函数 doc）。
  let children = unsafe { c_slice(p.p, p.sizep as usize) };
  for &child in children {
    // SAFETY: 同上契约，`child` 为数组内存活 `Proto` 句柄，降共享引用递归下传。
    getcounters(l, unsafe { &*child }, context, functionvisit, countervisit);
  }
}
