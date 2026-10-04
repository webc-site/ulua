use core::ptr::NonNull;

use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_error::lua_error, lua_tolstring::lua_tolstring_ref, lua_touserdata::lua_touserdata,
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

/// cpp `lua_requireinternal` 取配置的样板（`lua_touserdata` → cast → 判空）在本
/// crate 的唯一收口：`push_closure::<C>` 把宿主机装箱进 [`HostSlot`]`<C>` 放进带
/// GC 析构器的 userdata 并收作闭包唯一 upvalue，这里按同一 `C` 重建其共享引用
/// （引用只用于读取宿主方法；装载路径上宿主可重入 require，故必须共享而非独占借用
/// ——宿主页面的可变性由实现方的内部可变性自持）。
///
/// 泛型参数 `C` 由闭包体的单态化实例（`lua_require::<C>` / `lua_proxyrequire::<C>`）
/// 带入：注入点写入与这里读回由同一个 `C` 约束，零 `dyn`、零虚分派（原
/// `&dyn RequireHost` 返回值的擦除理由已在 [`HostSlot`] 处作废）。
///
/// 收形说明（review.md §2）：`l` 的存活/独占前提已由 `&mut LuaState` 引用形承载，
/// 不再折回裸指针；本函数**仍保留 `unsafe`**，因为契约不在「`l` 是否有效」而在两个
/// 无法由类型表达的前提（见下）。
///
/// # Safety
/// - `l`：存活 `LuaState` 的独占借用，`idx` 处栈槽属于该帧且在本次调用期内不被移动。
/// - `idx`：须是 `push_closure::<C>` 构造、被闭包 upvalue 持有（与闭包同寿命）的宿主
///   userdata 槽，且其装箱类型与本函数的 `C` 一致（由闭包体与槽位同源单态化保证）。
/// - 返回引用的 `'ctx`：由调用方选定，必须短于该 upvalue userdata 的存活期（即闭包
///   本体未被 GC 回收的窗口）；调用方不得把它降级为 `'static` 或跨帧缓存。
unsafe fn borrowed_host<'ctx, C: RequireHost>(l: &mut LuaState, idx: i32) -> Option<&'ctx C> {
  // SAFETY: 本函数 `# Safety` 保证 idx 处是与 `l` 同帧存活、以 `C` 装箱的宿主
  // userdata；`lua_touserdata` 只取该槽地址（不移动栈），`l.as_mut_ptr()` 由上方
  // 独占借用借出、窗止于当句。`Option` 证非空后重建指向已构造 `Box<C>` 的共享
  // 引用，再经 `Box` 解引用借出宿主本体——共享引用可与重入导航的其它共享引用
  // 合法并存；其寿命前提即 fn 契约第三条。
  unsafe {
    lua_touserdata(l.as_mut_ptr(), idx).map(|ud| {
      let slot = NonNull::from(ud).cast::<HostSlot<C>>().as_ref();
      &**slot
    })
  }
}

/// require 闭包的公共实现（cpp `lua_requireinternal`）：归一帧栈 → 取宿主 → 查已注册
/// 模块缓存 → 解析路径 → 装载并交 continuation 收尾。
///
/// 收形（review.md §2/§3）：`l` 由 `*mut LuaState` 收编为独占借用 `&mut LuaState`，
/// 存活与独占前提由类型承载，故降为安全 `fn`；两个调用点（`lua_require::<C>` /
/// `lua_proxyrequire::<C>` 这两个真 Lua/C 闭包）本就已物化好借用，直传即可，不再
/// 在边界上折回裸指针。体内残余的裸操作只落在两处 ulua-vm c-API
/// （`borrowed_host` 的 upvalue 读回、`lua_tolstring_ref` 的锚定视图）与本 crate 的
/// Lua/C 回调 `lua_requirecont` 上，各自下沉为带 `// SAFETY:` 论证的最小 `unsafe` 块。
///
/// 调用序契约（正确性，非内存安全）：`l` 为 require 闭包帧的当前状态，upvalue(1)
/// 须由 `push_closure::<C>` 以同一个 `C` 装箱宿主（由注入点与闭包体同源单态化保证），
/// 栈 1 为 require 路径参数（由 C 闭包调用约定与 `check_bytes` 的判型保证）；
/// `requirer_chunkname` 为发起方 chunkname 字节串，本次调用期内存活。
pub(crate) fn lua_requireinternal<C: RequireHost>(
  l: &mut LuaState,
  requirer_chunkname: &[u8],
) -> i32 {
  // 对应 cpp `lua_settop(L, 1)`：把闭包帧归一为 require 路径 1 个实参
  l.set_top(1);

  // SAFETY: `C` 与本闭包体的注入类型一致（fn 调用序契约），故 upvalue(1) 处的宿主
  // userdata 必以 `C` 装箱；借出的共享引用只在本次 require 窗口内使用，与 upvalue
  // userdata 同寿命（闭包在栈上即被 GC 视为根）。
  let Some(host) = (unsafe { borrowed_host::<C>(l, lua_upvalueindex(1)) }) else {
    luaL_error!(l, "unable to find require configuration");
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
      // `lua_error` 已前移 `&mut LuaState` 引用形（安全 fn，内部抛错不返回）。
      lua_error(l)
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

  // SAFETY: -2/-1 为装载阶段刚压入的字符串槽（恒为字符串），`lua_tolstring_ref` 的
  // `'ctx` 锚定形在此收口：返回视图指向被栈槽持有的 VM 串，只在紧随的 `host.load`
  // 调用窗口内消费（槽位不出现在其间的压弹中，两调用均只读、不占栈位）。
  let (chunkname, loadname) = unsafe {
    (
      lua_tolstring_ref(l.as_mut_ptr(), -2).unwrap_or(&[]),
      lua_tolstring_ref(l.as_mut_ptr(), -1).unwrap_or(&[]),
    )
  };

  // 宿主装载：path/chunkname/loadname 以字节视图直传（cpp 传同一 VM 串的
  // NUL 结尾指针，宿主按 C 串读取；首个 NUL 截断由需要该语义的宿主自行处理）
  let num_results = host.load(l, &path_bytes, chunkname, loadname);

  if num_results == -1 {
    // 挂起路径：先复核栈未被改动（不一致即 luaL_error! 发散），
    // lua_yield 由协程状态机接续
    if l.get_top() != stack_values {
      luaL_error!(l, "stack cannot be modified when require yields");
    }
    // `yield_thread` 为 `LuaState` 安全门面（本体 lua_yield 的裸指针折形收在方法内）
    l.yield_thread(0)
  } else {
    // 同步装载完成，continuation 按自身契约收尾本帧栈
    // SAFETY: `lua_requirecont` 是 Lua/C continuation 回调体，其 `# Safety` 只要求
    // `l` 为存活状态；本帧即该状态的当前协程，且 `_status` 形参按其契约忽略。
    unsafe { lua_requirecont(l.as_mut_ptr(), LuaStatus::Ok as i32) }
  }
}
