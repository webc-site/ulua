use core::{
  ffi::c_void,
  mem::size_of,
  ptr::{addr_of, null_mut},
  slice::{from_raw_parts, from_raw_parts_mut},
};

use crate::{
  functions::{
    c_file_write, c_file_write_bytes, c_file_write_str, dump_json_head, dumpref::dumpref,
    dumprefs::dumprefs, dumpstringdata::dumpstringdata, lua_f_findlocal::lua_f_findlocal,
  },
  macros::{
    ci_func::ci_func, getstr::getstr, is_lua::isLua, iscollectable::iscollectable,
    lua_emptystr::LUA_EMPTYSTR, obj_2_gco::obj2gco, pc_rel::pcRel, short_src_c::SHORT_SRC_C,
  },
  records::{
    call_info::CallInfo,
    closure::{CClosure, Closure, LClosure},
    loc_var::LocVar,
    lua_state::LuaState,
    proto::Proto,
  },
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `f` 须为有效可写 FILE；`th` 须为存活 LuaState（线程）：`stack..top` 与 `base_ci..ci` 为同数组内合法区间
/// （`ci >= base_ci`）、栈槽可解引用，`(*th).gt`、帧内闭包/Proto（`inner.l.p`、`source`、`debugname`）字段有效，
/// `savedpc` 相对 Proto 的 `pcRel` 及 `luaF_findlocal` 局部变量表可寻址。只读遍历，不改对象。cpp/VM/src/lgcdebug.cpp:465。
pub(crate) unsafe fn dumpthread(f: *mut c_void, th: *mut LuaState) {
  unsafe {
    let size = size_of::<LuaState>()
      + size_of::<TValue>() * (*th).stacksize as usize
      + size_of::<CallInfo>() * (*th).size_ci as usize;

    dump_json_head(f, "thread", (*th).hdr.memcat, size as i32);

    c_file_write_bytes(f, b",\"env\":");
    dumpref(f, obj2gco!((*th).gt));

    // 帧窗口 [base_ci, ci]：契约保证 base_ci <= ci 且同属 CallInfo 数组，一次定界；
    // cpp `while (ci <= L->ci)` 顺扫取首个 func 为函数的帧 → 切片迭代 + 提前 break
    let nci = (*th).ci.offset_from((*th).base_ci).max(0) as usize;
    let frames = from_raw_parts((*th).base_ci, nci + 1);
    let mut tcl: *mut Closure = null_mut();
    for ci in frames {
      let func = ci.func;
      if (*func).is_function() {
        tcl = (*func).as_closure_ptr();
        break;
      }
    }

    if !tcl.is_null() && (*tcl).is_c == 0 {
      let tcl_l = addr_of!((*tcl).inner.l).cast::<LClosure>();
      let tcl_p: *mut Proto = (*tcl_l).p;
      if !(*tcl_p).source.is_null() {
        let p: *mut Proto = tcl_p;
        c_file_write_bytes(f, b",\"source\":\"");
        dumpstringdata(f, getstr((*p).source), (*(*p).source).len as usize);
        c_file_write(f, format_args!("\",\"line\":{}", (*p).linedefined));
      }
    }

    // 栈窗口 [stack, top)：一次 offset_from 定界（top==stack 即原
    // `top > stack` 守卫不成立），后续槽名遍历改范围迭代取代指针游走
    let stack_count = (*th).top.offset_from((*th).stack).max(0) as usize;
    if stack_count > 0 {
      c_file_write_bytes(f, b",\"stack\":[");
      dumprefs(f, (*th).stack, stack_count);
      c_file_write_bytes(f, b"]");

      // cpp 的 ci 随 v 单调推进；此处以帧下标建模（ci = base_ci + ci_idx），
      // 下标界限 nci 即 `ci < L->ci` 的等价判定
      let mut ci_idx: usize = 0;
      let mut first = true;
      c_file_write_bytes(f, b",\"stacknames\":[");

      for slot in from_raw_parts_mut((*th).stack, stack_count) {
        // 栈槽指针是数据（与帧 func 指针比较、求局部变量偏移），从切片取回裸槽位
        let v: StkId = slot as *mut TValue;
        if iscollectable!(v) {
          while ci_idx < nci && v >= (*(*th).base_ci.add(ci_idx + 1)).func {
            ci_idx += 1;
          }
          let ci = (*th).base_ci.add(ci_idx);

          if !first {
            c_file_write_bytes(f, b",");
          }
          first = false;

          if v == (*ci).func {
            let cl = ci_func!(ci);
            if (*cl).is_c != 0 {
              let c = addr_of!((*cl).inner.c).cast::<CClosure>();
              c_file_write_bytes(f, b"\"frame:");
              c_file_write_str(
                f,
                if !(*c).debugname.is_null() {
                  (*c).debugname
                } else {
                  SHORT_SRC_C.as_ptr().cast()
                },
              );
              c_file_write_bytes(f, b"\"");
            } else {
              let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
              let p = (*lcl).p;
              c_file_write_bytes(f, b"\"frame:");
              if !(*p).source.is_null() {
                dumpstringdata(f, getstr((*p).source), (*(*p).source).len as usize);
              }
              c_file_write(f, format_args!(":{}", (*p).linedefined));
              c_file_write_bytes(f, b":");
              c_file_write_str(
                f,
                if !(*p).debugname.is_null() {
                  getstr((*p).debugname)
                } else {
                  LUA_EMPTYSTR.as_ptr().cast()
                },
              );
              c_file_write_bytes(f, b"\"");
            }
          } else if isLua!(ci) {
            let cl = ci_func!(ci);
            let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
            let p = (*lcl).p;
            let pc = pcRel!((*ci).savedpc, p);
            let var: *const LocVar = lua_f_findlocal(&*p, v.offset_from((*ci).base) as i32, pc);

            if !var.is_null() && !(*var).varname.is_null() {
              c_file_write_bytes(f, b"\"");
              c_file_write_str(f, getstr((*var).varname));
              c_file_write_bytes(f, b"\"");
            } else {
              c_file_write_bytes(f, b"null");
            }
          } else {
            c_file_write_bytes(f, b"null");
          }
        }
      }
      c_file_write_bytes(f, b"]");
    }

    c_file_write_bytes(f, b"}");
  }
}
