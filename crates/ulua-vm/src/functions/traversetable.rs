//! Source: `VM/src/lgc.cpp` (lgc.cpp:321-366, hand-ported)

use core::slice::from_raw_parts;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_type::LuaType,
  functions::{cstr_bytes, gettablemode::gettablemode, removeentry::removeentry},
  macros::{
    gkey::{gkey, gval},
    gnode::gnode,
    markobject::markobject,
    markvalue::markvalue,
    sizenode::sizenode,
    ttype::ttype,
  },
  records::{gc_object::GCObject, global_state::global_State, lua_table::LuaTable},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn traversetable(g: *mut global_State, h: *mut LuaTable) -> i32 {
  unsafe {
    if !(*h).metatable.is_null() {
      markobject!(g, (*h).metatable);
    }

    // is there a weak mode?
    // C++ `strchr(modev, 'k'/'v') != NULL`：contains 等价（mode 串仅 1~2 字节）
    let modev = gettablemode(g, &*h);
    let (weakkey, weakvalue) = if modev.is_null() {
      (0, 0)
    } else {
      let mode = cstr_bytes(modev);
      let weakkey = mode.contains(&b'k') as i32;
      let weakvalue = mode.contains(&b'v') as i32;
      if weakkey != 0 || weakvalue != 0 {
        // is really weak?
        (*h).gclist = (*g).weak; // must be cleared after GC, ...
        (*g).weak = h as *mut GCObject; // ... so put in the appropriate list
      }
      (weakkey, weakvalue)
    };

    if weakkey != 0 && weakvalue != 0 {
      return 1;
    }
    if weakvalue == 0 {
      // r12-E4 窗化（数组段取窗遍历）：倒序 `.rev()` 与 cpp `for (i = luaH_size(t); i--; )`
      // 逐位同序；markvalue 只读槽位、经 reallymarkobject 染灰**他对象**头部，不写不搬
      // 本表 array（窗存续期内无重分配）。sizearray==0 时 array 可为 null，cpp 零次
      // 迭代同形，判空后派生窗。
      if !(*h).array.is_null() {
        for e in from_raw_parts((*h).array, (*h).sizearray as usize)
          .iter()
          .rev()
        {
          markvalue!(g, e);
        }
      }
    }
    // r12-E4 窗化裁决（哈希段保留原形）：哨兵表 node 指向不可变 static 单格，出借
    // &mut 窗即别名违例，而 removeentry 收 *mut 桶指针且对可回收键就地写 DeadKey；
    // 白灰判据/倒序/removeentry 次序逐字保持，gnode! 依 E1 裁决不收编。
    // r12-w6d 逐点复核定性（保留面）：写侧收编门槛不满足——本路径未经 is_hash_dummy
    // 判据先行/换发实向量，哨兵表可达本桶循环，node_window_mut 对哨兵返回空窗，
    // 强行收编即把正常走查变成写拒 panic，属行为变更，维持保留。
    let mut i: i32 = sizenode!(h);
    while i > 0 {
      i -= 1;
      let n = gnode!(h, i);
      LUAU_ASSERT!(ttype!(gkey!(n)) != LuaType::DeadKey as u32 || (*gval!(n)).is_nil());
      if (*gval!(n)).is_nil() {
        removeentry(n); // remove empty entries
      } else {
        LUAU_ASSERT!(!(*gkey!(n)).is_nil());
        if weakkey == 0 {
          markvalue!(g, gkey!(n));
        }
        if weakvalue == 0 {
          markvalue!(g, gval!(n));
        }
      }
    }
    (weakkey != 0 || weakvalue != 0) as i32
  }
}
