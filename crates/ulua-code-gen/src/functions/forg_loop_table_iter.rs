use ulua_vm::{
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

use crate::{functions::forg_loop_node_iter::forg_loop_node_iter, records::vm_frame::VmFrame};

/// 生成码回写的 FORGLOOP 表数组段迭代一步（cpp `forgLoopTableIter`）；数组段耗尽后
/// 转入哈希段例程。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`，`h` 指向存活 `LuaTable`，`ra` 为本帧迭代器
/// 槽（协议预留 `ra..ra+5`），`index` 为数组段游标。边界契约集中于 [`VmFrame::current`]，
/// 其余为安全逻辑。
pub unsafe extern "C-unwind" fn forg_loop_table_iter(
  l: *mut LuaState,
  h: *mut LuaTable,
  mut index: i32,
  ra: *mut TValue,
) -> bool {
  // Safety: 本函数头 ABI 契约保证 `l` 为存活 LuaState；VmFrame::current 依 VM 调度不变量以 L->base 收帧，不解引用。
  let frame = unsafe { VmFrame::current(l) };

  let sizearray = frame.array_len(h);

  // 数组段游标扫至段尾即转入哈希段例程（切片视图迭代替代 C 式下标 while 循环；
  // index >= sizearray 时不进循环、原样回推，与 cpp 语义一致）。
  if (index as u32) < (sizearray as u32) {
    for (i, e) in frame
      .table_array(h)
      .iter()
      .enumerate()
      .skip(index.max(0) as usize)
    {
      if !frame.is_nil(e) {
        let index = i as i32;
        frame.set_iterator_index(frame.slot_at(ra, 2), index);
        frame.set_number(frame.slot_at(ra, 3), (index + 1) as f64);
        frame.set_stack_value(frame.slot_at(ra, 4), e);

        return true;
      }
    }
    index = sizearray;
  }

  // Safety: 依本函数头契约把同一活帧的 l/h/index/ra 原样转交同 ABI 的哈希段例程。
  unsafe { forg_loop_node_iter(l, h, index, ra) }
}
