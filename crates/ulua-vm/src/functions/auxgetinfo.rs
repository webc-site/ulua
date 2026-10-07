//! Source: `VM/src/ldebug.cpp:106`
//!
//! Fill a `lua_Debug` record from a closure + call-info, driven by the `what`
//! option string (`s` source/what/linedefined/short_src, `l` current line, `u`
//! upvalue count, `a` arity/vararg, `n` name, `f` push the function). Faithful
//! to the C++ field-by-field; returns the closure when `f` was requested.

use core::ptr::null_mut;

use crate::{
  functions::{
    cstr_bytes, currentline::currentline, getfuncname::getfuncname,
    lua_o_chunkid::lua_o_chunkid_owned,
  },
  macros::{ci_func::ci_func, getstr::getstr, is_lua::isLua, short_src_c::SHORT_SRC_C},
  records::{
    call_info::CallInfo,
    closure::Closure,
    lua_debug::{LuaDebug, LuaWhat},
  },
};
/// C 函数的 chunk 源名（cpp `getinfo` 的 `source` 占位，无终止 NUL）。
const SRC_C: &[u8] = b"=[C]";

/// # Safety
/// 所查询的调用帧/Proto/输出记录按约定存活可写。
pub(crate) unsafe fn auxgetinfo(
  what: &[u8],
  ar: *mut LuaDebug,
  f: *mut Closure,
  ci: *mut CallInfo,
) -> *mut Closure {
  // SAFETY: 契约保证 `ci` 为当前调用栈中的存活帧、`f` 与其一致（或为空由 ci 取得）、`ar` 可写，块内字段填充与推值不越界
  unsafe {
    // 既有约定（review.md §2）：VM c-API 边界签名折返——返回 `*mut Closure`，`cl` 为局部裸指针哨兵，与 lua_getinfo 同链，边界体内保留
    let mut cl: *mut Closure = null_mut();

    // C++ `for (; *what; what++)`：选项模板即字节切片，逐字节迭代（切片形无 NUL 语义）
    for &ch in what {
      match ch {
        b's' => {
          if (*f).is_c != 0 {
            (*ar).source = Some(SRC_C.to_vec());
            (*ar).what = LuaWhat::C;
            (*ar).linedefined = -1;
            // SHORT_SRC_C 是 `b"[C]\0"`，剥掉终止 NUL 后作为拥有的 Rust 字节串。
            (*ar).short_src = Some(SHORT_SRC_C[..SHORT_SRC_C.len() - 1].to_vec());
          } else {
            let proto = (*f).inner.l.p;
            let source = (*proto).source;
            // SAFETY: `source` 为存活 TString，`getstr` 取其数据首址、第 `len` 位
            // 为布局保证的终止 NUL，`cstr_bytes` 止于该 NUL 得到源名 payload。
            let src = cstr_bytes(getstr(source));
            (*ar).source = Some(src.to_vec());
            (*ar).what = LuaWhat::Lua;
            (*ar).linedefined = (*proto).linedefined;
            (*ar).short_src = Some(lua_o_chunkid_owned(src));
          }
        }
        b'l' => {
          if !ci.is_null() {
            (*ar).currentline = if isLua!(ci) { currentline(&*ci) } else { -1 };
          } else {
            (*ar).currentline = if (*f).is_c != 0 {
              -1
            } else {
              (*(*f).inner.l.p).linedefined
            };
          }
        }
        b'u' => {
          (*ar).nupvals = (*f).nupvalues;
        }
        b'a' => {
          if (*f).is_c != 0 {
            (*ar).isvararg = true;
            (*ar).nparams = 0;
          } else {
            let proto = (*f).inner.l.p;
            (*ar).isvararg = (*proto).is_vararg != 0;
            (*ar).nparams = (*proto).numparams;
          }
        }
        b'p' => {
          if (*f).is_c != 0 {
            (*ar).protoid = 0;
            (*ar).bytecodeid = -1;
          } else {
            let proto = (*f).inner.l.p;
            (*ar).protoid = (*proto).funid as i32;
            (*ar).bytecodeid = (*proto).bytecodeid;
          }
        }
        b'n' => {
          // SAFETY: `getfuncname` 契约保证返回 null 或存活至本帧结束、NUL 结尾的
          // 名字串（TString/静态字面量），`cstr_bytes` 止于其自带 NUL 取 payload。
          let ptr = if !ci.is_null() {
            getfuncname(ci_func!(ci))
          } else {
            getfuncname(f)
          };
          (*ar).name = if ptr.is_null() {
            None
          } else {
            Some(cstr_bytes(ptr).to_vec())
          };
        }
        b'f' => {
          cl = f;
        }
        _ => {}
      }
    }

    cl
  }
}
