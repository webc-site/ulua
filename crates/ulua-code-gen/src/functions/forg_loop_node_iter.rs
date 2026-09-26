use core::ptr::from_ref;

use ulua_vm::{
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

use crate::records::vm_frame::VmFrame;

/// 生成码回写的 FORGLOOP 表哈希段迭代一步（cpp `forgLoopNodeIter`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`，`h` 指向存活 `LuaTable`，`ra` 为本帧迭代器
/// 槽（协议预留 `ra..ra+5`），`index` 为回推的哈希段游标。边界契约集中于
/// [`VmFrame::current`]，其余为安全逻辑。
pub unsafe fn forg_loop_node_iter(
  l: *mut LuaState,
  h: *mut LuaTable,
  index: i32,
  ra: *mut TValue,
) -> bool {
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

/// # Safety
/// C-ABI 导出边界：由生成码按 codegen 回调约定调用，`l`/`h`/`index`/`ra` 的合法性与
/// 存活性与 [`forg_loop_node_iter`] 的契约一致（本函数仅原样透传）。
pub unsafe extern "C-unwind" fn forg_loop_node_iter_export(
  l: *mut LuaState,
  h: *mut LuaTable,
  index: i32,
  ra: *mut TValue,
) -> bool {
  // Safety: 导出 C ABI 入口原样转发 l/h/index/ra 给同契约 unsafe fn forg_loop_node_iter;
  // 调用方按 ABI 保证 h 为活 LuaTable、ra 为帧内活栈槽, 满足被调前置条件。
  unsafe { forg_loop_node_iter(l, h, index, ra) }
}
