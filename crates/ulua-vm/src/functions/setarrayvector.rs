use core::{mem::size_of, slice::from_raw_parts_mut};

use crate::{
  functions::{lua_m_realloc::lua_m_realloc_, runerror::runerror},
  macros::{err_table_overflow::ERR_TABLE_OVERFLOW, maxbits::MAXSIZE, setnilvalue::setnilvalue},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn setarrayvector(l: *mut LuaState, t: *mut LuaTable, size: i32) {
  // SAFETY: 契约保证 l/t 存活、size 界内；realloc 返回的 newarray 对 size 个 TValue
  // 有效可写（分配契约），置 nil 仅写新数组尾部，无其它别名
  unsafe {
    if size > MAXSIZE {
      runerror(l, ERR_TABLE_OVERFLOW.as_ptr().cast());
    }

    let oldsize = (*t).sizearray;
    let newarray = lua_m_realloc_(
      l,
      (*t).array as *mut u8,
      oldsize as usize * size_of::<TValue>(),
      size as usize * size_of::<TValue>(),
      (*t).memcat,
    ) as *mut TValue;

    (*t).array = newarray;

    if size > oldsize {
      // 先分配换指针（上方 realloc + `(*t).array` 回写已完成），后派生窗：
      // 新增尾段改窗内遍历置 nil，消除 p/end 双指针游走（cpp ltable.cpp:505-509 逐槽同形）
      let tail = from_raw_parts_mut(newarray.add(oldsize as usize), (size - oldsize) as usize);
      for slot in tail {
        setnilvalue!(slot);
      }
    }

    (*t).sizearray = size;
  }
}
