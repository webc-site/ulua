use core::{
  ffi::{c_char, c_int, c_void},
  slice::from_raw_parts,
};

use ulua_common::functions::c_str::cstr_bytes;
use ulua_vm::{functions::lua_rawseti::lua_rawseti, records::lua_state::LuaState};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_coverage_callback(
  context: *mut c_void,
  function: *const c_char,
  linedefined: c_int,
  depth: c_int,
  hits: *const c_int,
  size: usize,
) {
  // `context` 由注册方按 `*mut LuaState` 传入，转成强类型局部（纯指针算术）。
  let l = context as *mut LuaState;

  // Safety: `l` 存活；为本回调新建结果表并置于栈顶，下方各段都以 -2 取该表。
  unsafe { (*l).new_table() };

  // Safety: `l` 存活且栈顶为结果表；`function` 是 VM 交回的 NUL 结尾函数名（或 null 哨兵），
  // push/set_field_str 成对消费栈槽。
  unsafe {
    if function.is_null() {
      (*l).push_nil();
    } else {
      (*l).push_bytes(cstr_bytes(function));
    }
    (*l).set_field_str(-2, "name");
  }

  // Safety: 同上——`l` 存活、栈顶为结果表；`linedefined` 是 VM 传入的整数值。
  unsafe {
    (*l).push_integer(linedefined);
    (*l).set_field_str(-2, "linedefined");
  }

  // Safety: 同上——`l` 存活、栈顶为结果表；`depth` 是 VM 传入的整数值。
  unsafe {
    (*l).push_integer(depth);
    (*l).set_field_str(-2, "depth");
  }

  // 切片视图跳过未命中的槽位，避免逐指针累加
  // Safety: `hits` + `size` 由 VM 保证为 `size` 个 `c_int` 的连续数组，回调期间可读。
  let hits = unsafe { from_raw_parts(hits, size) };
  for (i, &hit) in hits.iter().enumerate().filter(|&(_, &hit)| hit != -1) {
    // Safety: `l` 存活且栈顶为 hits 表；pushinteger/rawseti 成对消费栈槽。
    unsafe {
      (*l).push_integer(hit);
      lua_rawseti(l, -2, i as c_int);
    }
  }

  // Safety: `l` 存活；`lua_objlen(l, -2)` 先取外层表长度（参数按左到右求值），
  // 再把本表挂到其后一位。
  unsafe { lua_rawseti(l, -2, (*l).obj_len(-2) as i32 + 1) };
}
