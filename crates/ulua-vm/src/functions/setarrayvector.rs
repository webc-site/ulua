use core::mem::size_of;

use crate::{
  functions::{c_slice_mut, lua_m_realloc::lua_m_realloc_, runerror::runerror},
  macros::{err_table_overflow::ERR_TABLE_OVERFLOW, maxbits::MAXSIZE, setnilvalue::setnilvalue},
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn setarrayvector(l: *mut LuaState, t: *mut LuaTable, size: i32) {
  // Safety: 契约保证 l/t 存活、size 界内；realloc 返回的 newarray 对 size 个 TValue
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

    // 尾部新槽置 nil：i 只是数组游标，改切片迭代取代逐格 array.add(i) 指针算术；
    // 循环体内无任何再分配，基址恒为 newarray，get_mut 越界即 oldsize>=size 的空区间
    // （与原 `oldsize..size` 空 range 不执行逐位一致）
    if let Some(tail) = c_slice_mut(newarray, size as usize).get_mut(oldsize as usize..) {
      for slot in tail {
        setnilvalue!(slot);
      }
    }

    (*t).sizearray = size;
  }
}
