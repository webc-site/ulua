use core::{ffi::c_char, slice::from_raw_parts};

use crate::{
  enums::tms::TMS,
  functions::{cstr, lua_h_getstr::lua_h_getstr},
  macros::{api_check::api_check, getstr::getstr, lua_utag_limit::LUA_UTAG_LIMIT, svalue::svalue},
  records::lua_state::LuaState,
};

const USERDATA_NAME: &[u8] = b"userdata";

/// 字节切片形态获取 userdata 类型名称（§10：Rust 内部调用方一律走此形，不构造 NUL 结尾缓冲）。
///
/// r16-v21 收形：首参转 `&mut LuaState`——`l` 的存活与独占由类型承载；返回切片寿命 `'a` 与该
/// 借用解耦（名串挂在 `global_State::udatamt[tag]` 元表内），仍是类型表达不了的内存契约，故
/// 本核心的 `# Safety` 契约位保留，调用前提见下（形制对齐 `lua_tobuffer_bytes_ref`）。
///
/// # Safety
/// `tag` 须落在 userdata 元表注册界内（`(tag as u32) < LUA_UTAG_LIMIT`，否则 `udatamt[tag]`
/// 越界读）；命中的 `mt` 非空且指向存活 `LuaTable`（本函数内仅只读解引用），`__type` 名串须
/// 存活且借出的 `'a` 窗内不得有 GC/清扫令该对象失效。不抛错/不分配。
pub(crate) unsafe fn lua_getuserdataname_bytes<'a>(l: &mut LuaState, tag: i32) -> &'a [u8] {
  // SAFETY: 契约即本函数 `# Safety` 所列——api_check 钉 tag 界内、`udatamt[tag]` 槽址为
  // `gs_ref` 只读视图内的裸句柄，空槽即未命中回退静态名；命中窗内不穿插分配/GC 写点。
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);

    let mt = l.gs_ref().udatamt[tag as usize];
    if !mt.is_null()
      && let Some(type_) = lua_h_getstr(&*mt, l.gs_ref().tmname[TMS::TmType as usize])
      && type_.get().is_string()
    {
      let ts = type_.get().as_string_ptr();
      return from_raw_parts(getstr(ts).cast::<u8>(), (*ts).len as usize);
    }

    USERDATA_NAME
  }
}

/// userdata 类型名的 C-ABI 镜像垫片形（`*const c_char`）：命中路径直接返回 Lua 串缓冲
/// 的 NUL 结尾指针，未命中回退静态字面量 `"userdata"`。
///
/// 保留理由（实测）：其消费位仅剩本文件 [`lua_getuserdataname_export`] 一枚，而该 C ABI 边界臂
/// 必须交回 `*const c_char`；bytes 核心形的回退常量 `USERDATA_NAME`（9 字节、无尾 `\0`）不可
/// 充当 NUL 结尾串，改道即改变对外可观察形态，故不落入零消费垫片裁撤判据，仅随本票收形 `l`。
///
/// # Safety
/// 同 [`lua_getuserdataname_bytes`]；另返回指针指向的串缓冲（VM 持有的 `TString` 或静态字面量）
/// 在调用方读取期间须保持存活。
pub(crate) unsafe fn lua_getuserdataname(l: &mut LuaState, tag: i32) -> *const c_char {
  // SAFETY: 契约即本函数 `# Safety` 所列，与 bytes 核心形同一（仅返回式折成 `*const c_char`：
  // 命中取 `svalue!` 的 NUL 结尾串首址，未命中取带尾 `\0` 的静态字面量）。
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);

    let mt = l.gs_ref().udatamt[tag as usize];
    if !mt.is_null()
      && let Some(type_) = lua_h_getstr(&*mt, l.gs_ref().tmname[TMS::TmType as usize])
      && type_.get().is_string()
    {
      return svalue!(type_.get());
    }

    cstr(b"userdata\0")
  }
}

/// # Safety
/// C ABI 边界臂：`l` 须为指向存活 `LuaState` 的非空指针，`tag` 落在 userdata 元表注册界内；
/// 返回值恒非空（VM 串缓冲或静态字面量），其存活期由该 tag 的元表名串决定。
pub unsafe extern "C-unwind" fn lua_getuserdataname_export(
  l: *mut LuaState,
  tag: i32,
) -> *const c_char {
  // SAFETY: 契约保证 `l` 非空且指向存活 `LuaState`；`&mut *l` 一次性重借用即收形后垫片期望的
  // 接收者形，本帧不再解引用该指针，返回的裸指针不延长任何借用。
  unsafe { lua_getuserdataname(&mut *l, tag) }
}
