use core::ptr::NonNull;

use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_error::lua_error, lua_tolstring::lua_tolstring_ref, lua_touserdata::lua_touserdata,
    lua_yield::lua_yield,
  },
  macros::{lua_l_error::luaL_error, lua_upvalueindex::lua_upvalueindex},
  records::lua_state::LuaState,
};

use crate::{
  enums::status_require_impl::Status,
  functions::{
    check_registered_modules::check_registered_modules,
    lua_requirecont::{REQUIRE_STACK_VALUES, lua_requirecont},
    push_str::push_c_str,
    resolve_require::resolve_require,
  },
  records::navigation_context::{HostSlot, RequireHost},
};

/// cpp `lua_requireinternal` 取配置的样板（`lua_touserdata` → cast → 判空）在
/// 本 crate 的唯一收口：`push_closure::<C>` 把宿主机装箱进 [`HostSlot`]`<C>` 放进带
/// GC 析构器的 userdata 并收作闭包唯一 upvalue，这里按同一 `C` 重建其共享引用
/// （引用只用于读取宿主方法；装载路径上宿主可重入 require，故必须共享而非独占借用
/// ——宿主页面的可变性由实现方的内部可变性自持）。
///
/// 泛型参数 `C` 由闭包体的单态化实例（`lua_require::<C>` / `lua_proxyrequire::<C>`）
/// 带入：注入点写入与这里读回由同一个 `C` 约束，零 `dyn`、零虚分派（原
/// `&dyn RequireHost` 返回值的擦除理由已在 [`HostSlot`] 处作废）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`；`idx` 处须是 `push_closure::<C>` 构造、被闭包
/// upvalue 持有（与闭包同寿命）的宿主 userdata，且其装箱类型与本函数的 `C` 一致
/// （由闭包体与槽位同源单态化保证）。
unsafe fn borrowed_host<'ctx, C: RequireHost>(l: *mut LuaState, idx: i32) -> Option<&'ctx C> {
  // Safety: 契约保证 idx 处是与 `l` 同帧存活、以 `C` 装箱的宿主 userdata
  // （lua_touserdata 只取该槽地址、不移动栈）；`Option` 证非空后重建指向已构造
  // `Box<C>` 的共享引用，再经 `Box` 解引用借出宿主本体——共享引用可与重入导航的
  // 其它共享引用合法并存。
  unsafe {
    lua_touserdata(l, idx).map(|ud| {
      let slot = NonNull::from(ud).cast::<HostSlot<C>>().as_ref();
      &**slot
    })
  }
}

/// # Safety
/// `l` 必须指向存活的 `LuaState`；upvalue(1) 须为 `push_closure::<C>` 建立的宿主
/// userdata，栈顶为 require 路径参数（由 C 闭包调用约定保证）。
/// `requirer_chunkname` 为 requirer chunkname 字节串，由真 FFI 入口经 `cstr_bytes`
/// 门面一次性取得。
pub(crate) unsafe fn lua_requireinternal<C: RequireHost>(
  l: *mut LuaState,
  requirer_chunkname: &[u8],
) -> i32 {
  // Safety: l 是 VM 调 require 闭包时传入的当前有效状态（fn # Safety 契约），
  // 入口一次重建独占借用（不与其他别名冲突），后续均为同一存活帧上的 VM 栈
  // 操作与宿主调用。
  let l: &mut LuaState = unsafe { &mut *l };

  // 对应 cpp `lua_settop(L, 1)`：把闭包帧归一为 require 路径 1 个实参
  l.set_top(1);

  // Safety: 契约保证 upvalue(1) 处为 push_closure::<C> 构造、以 `C` 装箱的宿主
  // userdata（与闭包同寿命），重建共享引用只用于读取宿主方法。
  let Some(host) = (unsafe { borrowed_host::<C>(l, lua_upvalueindex(1)) }) else {
    // Safety: l 存活，luaL_error! 抛错发散（`lua_l_error_l` 为 unsafe fn）。
    unsafe { luaL_error!(l, "unable to find require configuration") };
  };
  // 对应 cpp `std::string path(luaL_checkstring(L, 1))`：取 VM 串字节视图后落
  // owned 快照（Lua 串非 UTF-8，不校验；r16-p28 锚定形——窗口须活过 resolve/load
  // 全程对 `l` 的重借，cpp 同为 std::string 拷贝，一次分配逐点位等价）。
  let path_bytes = l.check_bytes(1).to_vec();

  // cpp 前置：已注册模块缓存命中即直接返回（值留栈顶）
  if check_registered_modules(l, &path_bytes) {
    return 1;
  }

  // resolve_require 触发宿主导航回调（可能执行 VM 配置代码）；宿主方法全部
  // 只借共享引用，重入合法，且函数体已收为安全签名（l 以独占借用收参）。
  let resolved_require = resolve_require(host, l, requirer_chunkname, &path_bytes);

  match resolved_require.status {
    // cpp 命中缓存路径：is_cached 已把值留在栈顶，无需再压
    Status::Cached => return 1,
    Status::ErrorReported => {
      push_c_str(l, &resolved_require.error);
      // l 存活，lua_error 由 VM 状态机接续（`!` 发散收敛为 i32）。
      lua_error(&mut *l)
    }
    _ => {}
  }

  // 装载阶段（cpp `lua_requireinternal` 尾段）：压 cacheKey/chunkname/loadname
  // 三槽后取栈上串视图交宿主 load
  push_c_str(l, &resolved_require.cache_key);
  push_c_str(l, &resolved_require.chunkname);
  push_c_str(l, &resolved_require.loadname);

  let stack_values = l.get_top();
  ulua_common::LUAU_ASSERT!(stack_values == REQUIRE_STACK_VALUES);

  // Safety: -2/-1 为装载阶段刚压入的字符串槽（恒为字符串），lua_tolstring_ref
  // 返回指向被栈槽持有的 VM 串的字节视图；两调用均为只读栈访问，不占栈位。
  let (chunkname, loadname) = unsafe {
    (
      lua_tolstring_ref(l, -2).unwrap_or(&[]),
      lua_tolstring_ref(l, -1).unwrap_or(&[]),
    )
  };

  // 宿主装载：path/chunkname/loadname 以字节视图直传（cpp 传同一 VM 串的
  // NUL 结尾指针，宿主按 C 串读取；首个 NUL 截断由需要该语义的宿主自行处理）
  let num_results = host.load(l, &path_bytes, chunkname, loadname);

  if num_results == -1 {
    // 挂起路径：先复核栈未被改动（不一致即 luaL_error! 发散），
    // lua_yield 由协程状态机接续
    if l.get_top() != stack_values {
      // Safety: l 存活，luaL_error! 抛错发散。
      unsafe { luaL_error!(l, "stack cannot be modified when require yields") };
    }
    // Safety: l 存活，lua_yield 由协程状态机接续。
    unsafe { lua_yield(l, 0) }
  } else {
    // 同步装载完成，continuation 按自身契约收尾本帧栈
    // Safety: l 存活，lua_requirecont 按自身契约收尾本帧栈。
    unsafe { lua_requirecont(l, LuaStatus::Ok as i32) }
  }
}
