use core::ptr::from_ref;

use ulua_vm::{
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

use crate::records::vm_frame::VmFrame;

/// FORGLOOP 表哈希段迭代一步（cpp `forgLoopNodeIter`）；Rust 侧由 `forg_loop_table_iter`
/// 数组段耗尽后直调，C ABI 侧经 [`forg_loop_node_iter_export`] 挂 NativeContext 槽供生成码直调。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`，`h` 指向存活 `LuaTable`，`ra` 为本帧迭代器
/// 槽（协议预留 `ra..ra+5`），`index` 为回推的哈希段游标。边界契约集中于
/// [`VmFrame::current`]，其余为安全逻辑。
pub(crate) unsafe fn forg_loop_node_iter(
  l: *mut LuaState,
  h: *mut LuaTable,
  index: i32,
  ra: *mut TValue,
) -> bool {
  // Safety: 本函数头 ABI 契约保证 `l` 为存活 LuaState；VmFrame::current 依 VM 调度不变量以 L->base 收帧，不解引用。
  let frame = unsafe { VmFrame::current(l) };

  let sizearray = frame.array_len(h);

  // 随后沿 hash 部分推进索引：从游标 `index − sizearray` 起迭代节点切片；负游标经
  // usize 回绕落至切片尾外（等价原 while 的 u32 wrapping_sub 界检，直接空迭代）。
  for (offset, n) in frame
    .table_nodes(h)
    .iter()
    .enumerate()
    .skip((index - sizearray) as usize)
  {
    let val = from_ref(&n.val);

    if !frame.is_nil(val) {
      let index = sizearray + offset as i32;
      frame.set_iterator_index(frame.slot_at(ra, 2), index);
      frame.node_key_into(n, frame.slot_at(ra, 3));
      frame.copy_value(frame.slot_at(ra, 4), val);

      return true;
    }
  }

  false
}

/// [`forg_loop_node_iter`] 的 `extern "C-unwind"` 壳（NativeContext `forg_loop_node_iter`
/// 槽位）：原样转发，契约同本体。
///
/// # Safety
/// 调用方（生成的机器码）按 codegen 回调 ABI 提供 `l`/`h`/`index`/`ra`，原样透传给同契约的本体。
pub unsafe extern "C-unwind" fn forg_loop_node_iter_export(
  l: *mut LuaState,
  h: *mut LuaTable,
  index: i32,
  ra: *mut TValue,
) -> bool {
  unsafe { forg_loop_node_iter(l, h, index, ra) }
}
