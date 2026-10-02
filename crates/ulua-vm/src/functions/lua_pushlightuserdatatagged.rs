use core::{
  ffi::c_void,
  ptr::{NonNull, null_mut},
};

use crate::{
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_lutag_limit::LUA_LUTAG_LIMIT,
    setpvalue::setpvalue,
  },
  records::lua_state::LuaState,
};

/// lightuserdata 入栈核心（`Option` 载荷形）：`p` 以 `Option<NonNull<c_void>>` 表达
/// 「可空载荷」——`None` 即 cpp `lua_pushlightuserdata(L, NULL)` 的合法空值（只入栈、
/// VM 不解引用），不再是与错误态混用的裸 null 哨兵。
///
/// 次序与 cpp `lapi.cpp:809` 逐指令一致：栈余量 → `api_check(tag)` → 写保留 top 槽 →
/// 抬栈顶。扩容先行（`ensure_stack_space(1)` 可移动栈），其后才派生本槽写面。
///
/// # Safety（内部窄窗契约，签名安全：调用方无需 unsafe 上下文）
/// `l` 为正在执行的 API 帧的存活 `LuaState`；`ensure_stack_space(1)` 后 `l.top` 落在分配
/// 栈界内且为可独占写入的空槽；`tag` 越界只在 debug 断言可见（release 按 cpp 原形写入
/// `extra` 字节，契约违约行为与旧形逐位一致）。
fn push_lightuserdata_slot(l: &mut LuaState, p: Option<NonNull<c_void>>, tag: i32) {
  l.ensure_stack_space(1);
  // 纯断言（无裸指针算式）留 unsafe 外
  api_check!(l, (tag as u32) < LUA_LUTAG_LIMIT as u32);

  // SAFETY: 契约保证扩容后 top 槽可独占写入；`map_or` 只把句柄折回载荷地址（None → null，
  // 与 cpp NULL 载荷同值），`setpvalue` 写该槽的 `value.p`/`extra[0]`/`tt` 三字段，不越帧界
  unsafe {
    setpvalue!(l.top, p.map_or(null_mut(), NonNull::as_ptr), tag);
    api_incr_top!(l);
  }
}

/// C-ABI 镜像垫片：把 [`push_lightuserdata_slot`] 的引用/句柄形折回 cpp
/// `lua_pushlightuserdatatagged`（`VM/src/lapi.cpp:809`）的 `(lua_State*, void*, int)` 形，
/// 仅做 `&mut *l` 重建与 `NonNull::new(p)` 折形，语义零差。
///
/// # Safety
/// `l` 须为存活 `LuaState`（非空、对齐、整个调用期单线程独占驱动——引用重建前提）；
/// `p` 只作不透明载荷入栈（VM 不解引用，可为 null 或整数编码值）；`tag` 须
/// `< LUA_LUTAG_LIMIT`。其余前提与 [`push_lightuserdata_slot`] 的 `# Safety` 契约逐条同一。
pub unsafe fn lua_pushlightuserdatatagged(l: *mut LuaState, p: *mut c_void, tag: i32) {
  // SAFETY: 契约保证 `l` 非空且指向存活 LuaState，本帧重建独占引用后即结束借用窗口；
  // `p` 为不透明载荷值（可空），由核心按 `Option<NonNull>` 收口后只入栈不解引用
  unsafe { push_lightuserdata_slot(&mut *l, NonNull::new(p), tag) }
}
