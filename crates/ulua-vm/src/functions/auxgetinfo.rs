//! Source: `VM/src/ldebug.cpp:106`
//!
//! Fill a `lua_Debug` record from a closure + call-info, driven by the `what`
//! option string (`s` source/what/linedefined/short_src, `l` current line, `u`
//! upvalue count, `a` arity/vararg, `n` name, `f` push the function). Faithful
//! to the C++ field-by-field; returns the closure when `f` was requested.
//!
//! review.md §10 收形：选项串以原生 `&[u8]`（不含 NUL 终止符）传入并整窗迭代；
//! 记录字段全部落原生形态——`name/what/source` 直存串体字节（模板为静态字面量、
//! 源名经 `tstr_bytes` 单点），`short_src` 经 `lua_o_chunkid_ref` 就地写入
//! `ShortSrc` 定长载体，全链无 `c_char`/指针折转。

use core::ptr::null_mut;

use crate::{
  functions::{
    currentline::currentline,
    getfuncname::getfuncname,
    lua_o_chunkid::{ChunkId, lua_o_chunkid_ref},
    tstr_bytes::{cut_at_nul, tstr_bytes},
  },
  macros::{ci_func::ci_func, is_lua::isLua, short_src_c::SHORT_SRC_C},
  records::{call_info::CallInfo, closure::Closure, lua_debug::LuaDebug, proto::Proto},
};

// §10 模板字节：读取面即整窗比较（无终止 NUL），`Option` 表达原 null 哨兵。
const SRC_C: &[u8] = b"=[C]";
const WHAT_C: &[u8] = b"C";
const WHAT_LUA: &[u8] = b"Lua";

/// # Safety
/// `what` 为可读选项字节窗；`ar` 独占可写；所查询的调用帧/Proto/输出记录按约定
/// 存活可写，且 `ar.source/name` 借出的串体在读取窗内不失效（函数值存活契约，
/// 见 `LuaDebug` 文档注）。
pub(crate) unsafe fn auxgetinfo(
  what: &[u8],
  ar: &mut LuaDebug,
  f: *mut Closure,
  ci: *mut CallInfo,
) -> *mut Closure {
  // SAFETY: 契约保证 `ci` 为当前调用栈中的存活帧、`f` 与其一致（或为空由 ci 取得）、`ar`/`what` 可写/可读，块内字段填充与推值不越界
  unsafe {
    // 既有约定（review.md §2）：VM c-API 边界签名折返——返回 `*mut Closure`，`cl` 为局部裸指针哨兵，与 lua_getinfo 同链，边界体内保留
    let mut cl: *mut Closure = null_mut();

    // C++ `for (; *what; what++)`：§10 后选项串在边界（`lua_getinfo`/调用方）即收为
    // NUL-free 窗口，整窗迭代与旧 cstr_bytes 截读等长
    for &ch in what {
      match ch {
        b's' => {
          if (*f).is_c != 0 {
            ar.source = Some(SRC_C);
            ar.what = Some(WHAT_C);
            ar.linedefined = -1;
            ar.short_src.set(SHORT_SRC_C);
          } else {
            let proto: *mut Proto = (*f).inner.l.p;
            let source = (*proto).source;
            // SAFETY: 契约保证函数值存活——TString 串体经 Proto 可达、GC 不移动，
            // `tstr_bytes` 单点折出恰覆盖 payload 的字节窗
            let src = tstr_bytes(source);
            ar.source = Some(cut_at_nul(src));
            ar.what = Some(WHAT_LUA);
            ar.linedefined = (*proto).linedefined;
            // 截断核心就地写 `ShortSrc` scratch（cpp `luaO_chunkid` 直写 `ssbuf` 同形；
            // 入窗为全 payload = cpp `source->len`，源名的 NUL 截读只发生在观察面）
            match lua_o_chunkid_ref(ar.short_src.scratch(), src) {
              ChunkId::Source => ar.short_src.set_cut(&src[1..]),
              ChunkId::Buf(n) => ar.short_src.commit_cut(n),
            }
          }
        }
        b'l' => {
          if !ci.is_null() {
            ar.currentline = if isLua!(ci) { currentline(&*ci) } else { -1 };
          } else {
            ar.currentline = if (*f).is_c != 0 {
              -1
            } else {
              (*(*f).inner.l.p).linedefined
            };
          }
        }
        b'u' => {
          ar.nupvals = (*f).nupvalues;
        }
        b'a' => {
          if (*f).is_c != 0 {
            ar.isvararg = 1;
            ar.nparams = 0;
          } else {
            let proto: *mut Proto = (*f).inner.l.p;
            ar.isvararg = (*proto).is_vararg as u8;
            ar.nparams = (*proto).numparams;
          }
        }
        b'p' => {
          if (*f).is_c != 0 {
            ar.protoid = 0;
            ar.bytecodeid = -1;
          } else {
            let proto: *mut Proto = (*f).inner.l.p;
            ar.protoid = (*proto).funid as i32;
            ar.bytecodeid = (*proto).bytecodeid;
          }
        }
        b'n' => {
          ar.name = if !ci.is_null() {
            getfuncname(ci_func!(ci))
          } else {
            getfuncname(f)
          }
          .map(cut_at_nul);
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
