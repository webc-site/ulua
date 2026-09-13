use core::{ffi::c_void, ptr::copy_nonoverlapping};

use ulua_vm::{
  macros::{
    lu_tag_iterator::LU_TAG_ITERATOR, setobj::setobj, setpvalue::setpvalue, ttisnil::ttisnil,
  },
  records::{lua_node::LuaNode, lua_table::LuaTable},
  type_aliases::{lua_state::lua_State, t_value::TValue, value::Value},
};

#[repr(C)]
struct LocalTKey {
  value: Value,
  extra: [i32; 1],
  tt_next: i32,
}

impl LocalTKey {
  #[inline]
  fn tt(&self) -> i32 {
    self.tt_next & 0xF
  }
}

#[repr(C)]
struct LocalLuaNode {
  val: TValue,
  key: LocalTKey,
}

unsafe fn node_gval(n: *const LuaNode) -> *const TValue {
  unsafe { core::ptr::addr_of!((*(n as *const LocalLuaNode)).val) }
}

unsafe fn get_node_key(l: *mut lua_State, obj: *mut TValue, node: *const LuaNode) {
  unsafe {
    let key = core::ptr::addr_of!((*(node as *const LocalLuaNode)).key);
    (*obj).value = (*key).value;
    copy_nonoverlapping(
      (*key).extra.as_ptr(),
      (*obj).extra.as_mut_ptr(),
      (*obj).extra.len(),
    );
    (*obj).tt = (*key).tt();
    ulua_vm::macros::checkliveness::checkliveness!((*l).global, obj);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn forg_loop_node_iter(
  l: *mut lua_State,
  h: *mut LuaTable,
  mut index: i32,
  ra: *mut TValue,
) -> bool {
  unsafe {
    let sizearray = (*h).sizearray;
    let sizenode = 1 << (*h).lsizenode;

    // then we advance index through the hash portion
    while (index as u32).wrapping_sub(sizearray as u32) < (sizenode as u32) {
      let n = (*h).node.add((index - sizearray) as usize);

      if !ttisnil!(node_gval(n)) {
        setpvalue!(
          ra.add(2),
          (index + 1) as usize as *mut c_void,
          LU_TAG_ITERATOR
        );
        get_node_key(l, ra.add(3), n);
        setobj!(l, ra.add(4), node_gval(n));

        return true;
      }

      index += 1;
    }

    false
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_forgLoopNodeIter")]
pub unsafe extern "C-unwind" fn forg_loop_node_iter_export(
  l: *mut lua_State,
  h: *mut LuaTable,
  index: i32,
  ra: *mut TValue,
) -> bool {
  unsafe { forg_loop_node_iter(l, h, index, ra) }
}
