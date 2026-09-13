use core::ffi::c_void;

use ulua_vm::{
  macros::{
    lu_tag_iterator::LU_TAG_ITERATOR, setnvalue::setnvalue, setobj_2_s::setobj2s, ttisnil::ttisnil,
  },
  records::{lua_state::lua_State, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn forg_loop_table_iter(
  l: *mut lua_State,
  h: *mut LuaTable,
  mut index: i32,
  ra: *mut TValue,
) -> bool {
  unsafe {
    let sizearray = (*h).sizearray;

    while (index as u32) < (sizearray as u32) {
      let e = (*h).array.add(index as usize);

      if !ttisnil!(e) {
        (*ra.add(2)).value.p = (index + 1) as usize as *mut c_void;
        (*ra.add(2)).tt = LU_TAG_ITERATOR;

        setnvalue!(ra.add(3), (index + 1) as f64);
        setobj2s!(l, ra.add(4), e);

        return true;
      }

      index += 1;
    }

    false
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_forgLoopTableIter")]
pub unsafe extern "C-unwind" fn forg_loop_table_iter_export(
  l: *mut lua_State,
  h: *mut LuaTable,
  index: i32,
  ra: *mut TValue,
) -> bool {
  unsafe { forg_loop_table_iter(l, h, index, ra) }
}
