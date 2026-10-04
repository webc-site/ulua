use core::{ffi::c_char, slice::from_raw_parts};

use crate::{
  enums::tms::TMS,
  functions::{
    cstr,
    lua_h_getstr::lua_h_getstr,
  },
  macros::{api_check::api_check, getstr::getstr, lua_utag_limit::LUA_UTAG_LIMIT, svalue::svalue},
  records::lua_state::LuaState,
};

const USERDATA_NAME: &[u8] = b"userdata";

/// 字节切片形态获取 userdata 类型名称（§10：Rust 内部调用方一律走此形，不构造 NUL 结尾缓冲；
/// review.md §7：原 `*const c_char` 中间垫片实测唯一消费者为本文件 C ABI 导出臂，已删，
/// 指针面由导出臂就地构造）。
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

/// # Safety
/// C ABI 边界臂：`l` 须为指向存活 `LuaState` 的非空指针，`tag` 落在 userdata 元表注册界内；
/// 返回值恒非空（VM 串缓冲或静态字面量），其存活期由该 tag 的元表名串决定。
pub unsafe extern "C-unwind" fn lua_getuserdataname_export(
  l: *mut LuaState,
  tag: i32,
) -> *const c_char {
  // SAFETY: 契约保证 `l` 非空且指向存活 `LuaState`；`&mut *l` 一次性重借用即 bytes 核心
  // 期望的接收者形，本帧不再解引用该指针，返回的裸指针不延长任何借用。
  unsafe {
    // C 观察面就地构造：命中取 VM 串缓冲的 NUL 结尾首址（TString 布局恒带终止符），
    // 未命中回退带尾 `\0` 的静态字面量——bytes 核心的 9 字节回退窗无终止符，
    // 不可直接充当 NUL 串，此为 C ABI 边界的合法镜像位（§10 豁免台账见 rt/sys.rs）
    api_check!(&mut *l, (tag as u32) < LUA_UTAG_LIMIT as u32);

    let mt = (*l).gs_ref().udatamt[tag as usize];
    if !mt.is_null()
      && let Some(type_) = lua_h_getstr(&*mt, (*l).gs_ref().tmname[TMS::TmType as usize])
      && type_.get().is_string()
    {
      return svalue!(type_.get());
    }

    cstr(b"userdata\0")
  }
}
