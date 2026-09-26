use core::{ffi::c_char, mem::size_of};

use crate::{
  functions::{
    c_slice_mut, gettablemode::gettablemode, removeentry::removeentry,
    tableresizeprotected::tableresizeprotected,
  },
  macros::{
    dummynode::dummynode,
    gkey::{gkey, gval},
    iscleared::iscleared,
    setnilvalue::setnilvalue,
    sizenode::sizenode,
  },
  records::{gc_object::GCObject, lua_node::LuaNode, lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `mode` 必须为 null 或指向以 NUL 结尾、整段可读的 C 字符串（弱表 mode 字段）；
/// 否则逐字节扫描找终止符 's' 会越界读。cpp lgc.cpp:755（`strchr(modev, 's')`）。
#[inline]
unsafe fn contains_s(mut mode: *const c_char) -> bool {
  // Safety: 契约保证 `mode` 为 null 或 NUL 结尾可读字符串，块内仅逐字节读到终止符为止
  unsafe {
    while !mode.is_null() && *mode != 0 {
      if *mode == b's' as c_char {
        return true;
      }
      mode = mode.add(1);
    }
    false
  }
}

/// # Safety
/// `l` 须存活（读取 `global` 并供 `tableresizeprotected` 用）；`list` 必须是以 `gclist` 字段串联、
/// 尚未释放的 `LuaTable` 链表，且各表 array/sizearray 与 node/sizenode 自洽。
/// 违反将对已回收内存置 nil/搬运算法区，或按错误长度越界遍历。cpp lgc.cpp:709。
pub(crate) unsafe fn cleartable(l: *mut LuaState, mut list: *mut GCObject) -> usize {
  unsafe {
    let mut work = 0usize;

    while !list.is_null() {
      let h = list as *mut LuaTable;
      let hsize = sizenode!(h);
      // cpp lgc.cpp:692：工作量估算时空哈希部（node == dummynode）计 0；遍历仍按 sizenode
      let hashwork = if (*h).node == dummynode.cast_mut() {
        0
      } else {
        hsize as usize
      };
      work += size_of::<LuaTable>()
        + size_of::<TValue>() * (*h).sizearray as usize
        + size_of::<LuaNode>() * hashwork;

      // 数组段窗口：array 与 sizearray 自洽（契约），每格判清与否只看自身白性，
      // 逆序切片遍历与 cpp `while (i--)` 逐指令序等价，免 `array.add(i)` 裸走查
      for o in c_slice_mut((*h).array, (*h).sizearray as usize)
        .iter_mut()
        .rev()
      {
        // 读宏（iscleared/gcvalue）按值取槽指针，与收敛前 `array.add(i)` 同形
        let o = &raw mut *o;
        if iscleared!(o) {
          setnilvalue!(o);
        }
      }

      // 哈希段窗口：node 与 sizenode 自洽；gval/gkey/removeentry 均只作用于当前格，
      // 逆序切片遍历与原 i 递减走查同序等价（dummynode 表读单格 dummy，与 cpp 一致）
      let mut activevalues = 0;
      for n in c_slice_mut((*h).node, hsize as usize).iter_mut().rev() {
        if !(*gval!(n)).is_nil() {
          if iscleared!(gkey!(n)) || iscleared!(gval!(n)) {
            setnilvalue!(gval!(n));
            removeentry(n as *mut LuaNode);
          } else {
            activevalues += 1;
          }
        }
      }

      let modev = gettablemode((*l).global, h);
      if !modev.is_null() && contains_s(modev) && activevalues < hsize * 3 / 8 {
        tableresizeprotected(l, h, activevalues);
      }

      list = (*h).gclist;
    }

    work
  }
}
