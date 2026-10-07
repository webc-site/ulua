use core::ptr::{null, null_mut};

use ulua_common::functions::c_str::{cstr, cstr_bytes};
use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  macros::{lua_idsize::LUA_IDSIZE, lua_l_error::luaL_error},
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

use crate::{
  functions::{lua_requireinternal::lua_requireinternal, resolve_require::NOT_ALLOWED_MSG},
  records::navigation_context::RequireHost,
};

/// `lua_getinfo` 的选项串：仅取 `what` 字段（NUL 结尾字节串，经 `cstr` 收口点转
/// C 指针，review.md §10：不散落 `.as_ptr().cast()`）。
const GETINFO_WHAT_OPT: &[u8] = b"s\0";

/// cpp `LuaDebug ar;` 的零初值：整块在编译期成形，免 `mem::zeroed()` 的 unsafe
/// 与「POD 全零合法」的口头论证（与 ulua-web `run_code::ZERO_DEBUG` 同款形态）。
///
/// FFI: c-API 出参槽——`LuaDebug` 是 VM 的 C ABI 镜像结构（`#[repr(C)]`，出处
/// `VM/include/lua.h:488-502`），指针字段的本体即 C 侧 `char*`/`void*`；结构
/// 声明与写端均在 ulua-vm（本 crate 外），此处 `null()`/`null_mut()` 仅作 const
/// POD 初值，从不被本侧解引用。
const ZERO_DEBUG: LuaDebug = LuaDebug {
  name: null(),
  what: null(),
  source: null(),
  short_src: null(),
  linedefined: 0,
  currentline: 0,
  protoid: 0,
  bytecodeid: 0,
  nupvals: 0,
  nparams: 0,
  isvararg: 0,
  userdata: null_mut(),
  ssbuf: [0; LUA_IDSIZE as usize],
};

/// require 闭包体（cpp `requireLikeFunc`）：泛型参数 `C` 为注入时的宿主类型，
/// `luarequire_pushrequire::<C>` 以 `Some(lua_require::<C>)` 具名实例化后 coerce 为
/// `LuaCFunction`——擦除发生在函数指针层（每宿主一个单态化体），槽位读回零 `dyn`。
///
/// # Safety
///
/// `l` must be a valid pointer to a live `LuaState`；upvalue(1) 须为
/// `push_closure::<C>` 以同一 `C` 装箱的宿主 userdata（由注入点与闭包体同源单态化
/// 保证）。
pub(crate) unsafe extern "C-unwind" fn lua_require<C: RequireHost>(l: *mut LuaState) -> i32 {
  // cpp `LuaDebug ar;` 的编译期零初值（见 ZERO_DEBUG），只作 lua_getinfo 的出参
  // 槽，由该 C API 按语义整体填充。
  let mut ar = ZERO_DEBUG;
  // Safety: 真 FFI 入口，l 为 VM 调 require 闭包时传入的存活 state，入口一次
  // 重建独占借用（不与其他别名冲突），后续均为本帧上的 C API 调用。
  let l: &mut LuaState = unsafe { &mut *l };

  // Safety: lua_getinfo 按其 C API 契约回填 ar：自 1 起沿调用栈上溯停在首个非 C
  // 函数帧（cpp 的手动 level 游标），越界由返回 0 先行报错；ar.what/ar.source 为
  // 回填的 null 或调用期内存活的 C 串，判空/解引用统一收口在 cstr_bytes 门面
  // （source 为 null 时译成空 chunkname，cpp 直接传指针，Rust 保底避免 UB）；
  // what 为 NUL 结尾静态字节串（`cstr` 门面）。`l.as_mut_ptr()` 由上一句独占借用
  // 借出、窗止于当句；尾段 lua_requireinternal::<C> 已收形为安全 fn，其对
  // upvalue(1) 宿主槽与栈布局的要求即本闭包体的 `# Safety` 契约（同源单态化）。
  let requirer_chunkname = unsafe {
    for level in 1.. {
      if lua_getinfo(l.as_mut_ptr(), level, cstr(GETINFO_WHAT_OPT), &mut ar) == 0 {
        luaL_error!(l, "{NOT_ALLOWED_MSG}");
      }
      // `what` 单字符判定经 cstr_bytes 门面收口（null 译空串 → false）。
      if cstr_bytes(ar.what).first() != Some(&b'C') {
        break;
      }
    }
    cstr_bytes(ar.source)
  };
  lua_requireinternal::<C>(l, requirer_chunkname)
}
