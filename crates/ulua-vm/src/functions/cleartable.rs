use core::{ffi::c_char, mem::size_of};

use crate::{
  functions::{
    gettablemode::gettablemode, removeentry::removeentry,
    tableresizeprotected::tableresizeprotected,
  },
  macros::{
    gkey::{gkey, gval},
    iscleared::iscleared,
    setnilvalue::setnilvalue,
  },
  records::{gc_object::GCObject, lua_node::LuaNode, lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `mode` 必须为 null 或指向以 NUL 结尾、整段可读的 C 字符串（弱表 mode 字段）；
/// 否则逐字节扫描找终止符 's' 会越界读。cpp lgc.cpp:755（`strchr(modev, 's')`）。
#[inline]
unsafe fn contains_s(mut mode: *const c_char) -> bool {
  // SAFETY: 契约保证 `mode` 为 null 或 NUL 结尾可读字符串，块内仅逐字节读到终止符为止
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
      // 窗形口径：hsize ⇔ sizenode!(h)（实向量窗长 twoto(lsizenode)；哨兵表窗长恒 1，
      // 即 cpp「dummynode 表 size 计 1」口径）
      let hsize = (*h).node_window().len() as i32;
      // cpp lgc.cpp:692：工作量估算时空哈希部（is_hash_dummy 哨兵判据）计 0；遍历仍按窗长走查
      let hashwork = if (*h).is_hash_dummy() {
        0
      } else {
        hsize as usize
      };
      work += size_of::<LuaTable>()
        + size_of::<TValue>() * (*h).array_window().len()
        + size_of::<LuaNode>() * hashwork;

      // 数组段切 array_window_mut 共享窗：array 与 sizearray 自洽（契约），null 数组
      // 归空窗；每格判清与否只看自身白性，逆序切片遍历与 cpp `while (i--)` 逐指令
      // 序等价，免 `array.add(i)` 裸走查
      for o in (*h).array_window_mut().iter_mut().rev() {
        // 读宏（iscleared/gcvalue）按值取槽指针，与收敛前 `array.add(i)` 同形
        let o = &raw mut *o;
        if iscleared!(o) {
          setnilvalue!(o);
        }
      }

      // 哈希段切 node_window_mut：实向量窗与 hsize 同界，逆序切片遍历与原 i 递减
      // 走查同序等价；哨兵表按 E1 裁决写侧恒空窗——C 侧从不原地写哨兵，且 dummy
      // 单格 val 恒 nil ⇒ 空桶，cpp 走查一格即判空落过、无任何观测动作，故空窗
      // 零迭代与 cpp 逐位一致（activevalues 不增，弱表阈值 `hsize*3/8` 判据口径不变）。
      // gval/gkey/removeentry 均只作用于当前格，写路径只落实向量。
      let mut activevalues = 0;
      for n in (*h).node_window_mut().iter_mut().rev() {
        if !(*gval!(n)).is_nil() {
          if iscleared!(gkey!(n)) || iscleared!(gval!(n)) {
            setnilvalue!(gval!(n));
            removeentry(n as *mut LuaNode);
          } else {
            activevalues += 1;
          }
        }
      }

      let modev = gettablemode((*l).global, &*h);
      if !modev.is_null() && contains_s(modev) && activevalues < hsize * 3 / 8 {
        tableresizeprotected(l, h, activevalues);
      }

      list = (*h).gclist;
    }

    work
  }
}
