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
pub unsafe fn forg_loop_table_iter(
  l: *mut LuaState,
  h: *mut LuaTable,
  mut index: i32,
  ra: *mut TValue,
) -> bool {
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

  unsafe { forg_loop_node_iter(l, h, index, ra) }
}

/// # Safety
/// C-ABI 导出边界：由生成码按 codegen 回调约定调用，`l`/`h`/`index`/`ra` 的合法性与
/// 存活性与 [`forg_loop_table_iter`] 的契约一致（本函数仅原样透传）。
pub unsafe extern "C-unwind" fn forg_loop_table_iter_export(
  l: *mut LuaState,
  h: *mut LuaTable,
  index: i32,
  ra: *mut TValue,
) -> bool {
  // Safety: 导出 C ABI 入口原样转发 l/h/index/ra 给同契约 unsafe fn forg_loop_table_iter;
  // 调用方按 ABI 保证 h 为活 LuaTable、ra 为帧内活栈槽且 index 有界, 满足被调前置条件。
  unsafe { forg_loop_table_iter(l, h, index, ra) }
}
