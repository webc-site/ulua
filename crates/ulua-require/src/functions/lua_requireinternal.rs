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
/// 本 crate 的唯一收口：`push_closure` 把宿主机装箱进 `HostSlot` 放进带 GC 析构器
/// 的 userdata 并收作闭包唯一 upvalue，这里重建其共享引用（引用只用于读取宿主
/// 方法；装载路径上宿主可重入 require，故必须共享而非独占借用——宿主页面的可变性
/// 由实现方的内部可变性自持）。
///
/// 返回 `&dyn RequireHost`：`dyn` 保留点即 [`HostSlot`] 定义处（其 DELIBERATE
/// DEVIATION 已锚定宿主集合运行期开放 + 非泛型 C 闭包体取回），本函数只是把那层
/// 擦除**借出**给下游导航，非再擦除，故不改成泛型（避免污染整条 require 调用链）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`；`idx` 处须是 `push_closure` 构造、被闭包
/// upvalue 持有（与闭包同寿命）的宿主 userdata。
unsafe fn borrowed_host<'ctx>(l: *mut LuaState, idx: i32) -> Option<&'ctx dyn RequireHost> {
  // Safety: 契约保证 idx 处是与 `l` 同帧存活的宿主 userdata（lua_touserdata 只取
  // 该槽地址、不移动栈）；`Option` 证非空后重建指向已构造 Box 的共享引用，再经
  // Box 解引用借出 trait 对象——共享引用可与重入导航的其它共享引用合法并存。
  unsafe {
    lua_touserdata(l, idx).map(|ud| {
      let host = NonNull::from(ud).cast::<HostSlot>().as_ref();
      host.as_ref()
    })
  }
}

/// # Safety
/// `l` 必须指向存活的 `LuaState`；upvalue(1) 须为 `push_closure` 建立的宿主
/// userdata，栈顶为 require 路径参数（由 C 闭包调用约定保证）。
/// `requirer_chunkname` 为 requirer chunkname 字节串，由真 FFI 入口经 `cstr_bytes`
/// 门面一次性取得。
pub(crate) unsafe fn lua_requireinternal(l: *mut LuaState, requirer_chunkname: &[u8]) -> i32 {
  // Safety: l 是 VM 调 require 闭包时传入的当前有效状态（fn /// # Safety）；
  // 以下均为同一存活帧上的 VM 栈操作与宿主调用，lua_error 由 VM 状态机接续
  // （返回 `!`，臂型自然收敛为 i32）。
  unsafe {
    // 对应 cpp `lua_settop(L, 1)`：把闭包帧归一为 require 路径 1 个实参
    (*l).set_top(1);

    let Some(host) = borrowed_host(l, lua_upvalueindex(1)) else {
      luaL_error!(l, "unable to find require configuration");
    };
    // 对应 cpp `std::string path(luaL_checkstring(L, 1))`：check_bytes 取 VM 串的
    // 字节视图（Lua 串非 UTF-8，零拷贝不校验）。
    let path_bytes = (*l).check_bytes(1);

    // cpp 前置：已注册模块缓存命中即直接返回（值留栈顶）
    if check_registered_modules(&mut *l, path_bytes) {
      return 1;
    }

    // Safety: resolve_require 触发宿主导航回调（可能执行 VM 配置代码），以裸 l
    // 转手；宿主方法全部只借共享引用，重入合法。
    let resolved_require = resolve_require(host, l, requirer_chunkname, path_bytes);

    match resolved_require.status {
      // cpp 命中缓存路径：is_cached 已把值留在栈顶，无需再压
      Status::Cached => return 1,
      Status::ErrorReported => {
        push_c_str(&mut *l, &resolved_require.error);
        lua_error(l)
      }
      _ => {}
    }

    // 装载阶段（cpp `lua_requireinternal` 尾段）：压 cacheKey/chunkname/loadname
    // 三槽后取栈上串视图交宿主 load
    {
      let l_ref = &mut *l;
      push_c_str(l_ref, &resolved_require.cache_key);
      push_c_str(l_ref, &resolved_require.chunkname);
      push_c_str(l_ref, &resolved_require.loadname);
    }

    let stack_values = (*l).get_top();
    ulua_common::LUAU_ASSERT!(stack_values == REQUIRE_STACK_VALUES);

    // Safety: -2/-1 为装载阶段刚压入的字符串槽（恒为字符串），lua_tolstring_ref
    // 返回指向被栈槽持有的 VM 串的字节视图；两调用均为只读栈访问，不占栈位。
    let (chunkname, loadname) = (
      lua_tolstring_ref(l, -2).unwrap_or(&[]),
      lua_tolstring_ref(l, -1).unwrap_or(&[]),
    );

    // 宿主装载：path/chunkname/loadname 以字节视图直传（cpp 传同一 VM 串的
    // NUL 结尾指针，宿主按 C 串读取；首个 NUL 截断由需要该语义的宿主自行处理）
    let num_results = host.load(l, path_bytes, chunkname, loadname);

    if num_results == -1 {
      // 挂起路径：先复核栈未被改动（不一致即 luaL_error! 发散），
      // lua_yield 由协程状态机接续
      if (*l).get_top() != stack_values {
        luaL_error!(l, "stack cannot be modified when require yields");
      }
      lua_yield(l, 0)
    } else {
      // 同步装载完成，continuation 按自身契约收尾本帧栈
      lua_requirecont(l, LuaStatus::Ok as i32)
    }
  }
}
