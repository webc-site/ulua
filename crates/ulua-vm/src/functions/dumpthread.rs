use core::{
  ffi::{c_int, c_void},
  mem::size_of,
  ptr::{addr_of, null_mut},
};

use crate::{
  functions::{
    c_file_write, c_file_write_bytes, c_file_write_str, dumpref::dumpref, dumprefs::dumprefs,
    dumpstringdata::dumpstringdata, lua_f_findlocal::luaF_findlocal,
  },
  macros::{
    ci_func::ci_func, clvalue::clvalue, getstr::getstr, is_lua::isLua,
    iscollectable::iscollectable, obj_2_gco::obj2gco, pc_rel::pcRel, ttisfunction::ttisfunction,
  },
  records::{
    call_info::CallInfo,
    closure::{CClosure, Closure, LClosure},
    loc_var::LocVar,
    lua_state::lua_State,
    proto::Proto,
  },
  type_aliases::{stk_id::StkId, t_value::TValue},
};

pub(crate) unsafe fn dumpthread(f: *mut c_void, th: *mut lua_State) {
  unsafe {
    let size = size_of::<lua_State>()
      + size_of::<TValue>() * (*th).stacksize as usize
      + size_of::<CallInfo>() * (*th).size_ci as usize;

    c_file_write(
      f,
      format_args!(
        "{{\"type\":\"thread\",\"cat\":{},\"size\":{}",
        (*th).hdr.memcat,
        size as c_int
      ),
    );

    c_file_write_bytes(f, b",\"env\":");
    dumpref(f, obj2gco!((*th).gt));

    let mut tcl: *mut Closure = null_mut();
    let mut ci = (*th).base_ci;
    while ci <= (*th).ci {
      if ttisfunction!((*ci).func) {
        tcl = clvalue!((*ci).func);
        break;
      }
      ci = ci.add(1);
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

    if (*th).top > (*th).stack {
      c_file_write_bytes(f, b",\"stack\":[");
      dumprefs(f, (*th).stack, (*th).top.offset_from((*th).stack) as usize);
      c_file_write_bytes(f, b"]");

      let mut ci = (*th).base_ci;
      let mut first = true;
      c_file_write_bytes(f, b",\"stacknames\":[");

      let mut v: StkId = (*th).stack;
      while v < (*th).top {
        if iscollectable!(v) {
          while ci < (*th).ci && v >= (*ci.add(1)).func {
            ci = ci.add(1);
          }

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
                  c"[C]".as_ptr()
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
                  c"".as_ptr()
                },
              );
              c_file_write_bytes(f, b"\"");
            }
          } else if isLua!(ci) {
            let cl = ci_func!(ci);
            let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
            let p = (*lcl).p;
            let pc = pcRel!((*ci).savedpc, p);
            let var: *const LocVar = luaF_findlocal(p, v.offset_from((*ci).base) as c_int, pc);

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

        v = v.add(1);
      }
      c_file_write_bytes(f, b"]");
    }

    c_file_write_bytes(f, b"}");
  }
}
