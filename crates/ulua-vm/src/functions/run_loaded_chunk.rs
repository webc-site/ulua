//! cpp `runCode` 的加载/执行段（`CLI/src/Repl.cpp:239-300` 与 `CLI/src/Web.cpp:71-140`
//! 的共同主干）：加载 bytecode → 换协程 → resume → 成功打印返回值 / 失败组装
//! 错误文本。三处宿主（ulua-repl-cli、ulua、ulua-web）原本各持一份逐行同构的
//! 移植，现收口于此；宿主间差异经参数与返回值表达，不再各写一份。

use alloc::string::String;
use core::ptr::null_mut;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    error_with_trace::error_with_trace, lua_gettop::lua_gettop, lua_insert::lua_insert,
    lua_l_checkstack::lua_l_checkstack, lua_newthread::lua_newthread, lua_pcall::lua_pcall,
    lua_pushvalue::lua_pushvalue, lua_remove::lua_remove, lua_resume::lua_resume,
    lua_tolstring::lua_tolstring_ref, lua_xmove::lua_xmove, luau_load::luau_load,
  },
  macros::{
    lua_getglobal::lua_getglobal, lua_isnil::lua_isnil, lua_memerrmsg::LUA_MEMERRMSG_STR,
    lua_minstack::LUA_MINSTACK, lua_pop::lua_pop,
  },
  records::lua_state::LuaState,
};

/// 结果打印的全局函数名（NUL 结尾字节串，仅 `lua_getglobal` 收口点转 C 指针）。
const PRETTYPRINT_NAME: &[u8] = b"_PRETTYPRINT\0";
const PRINT_NAME: &[u8] = b"print\0";

/// 运行已加载到 `l` 之外的 `bytecode`：加载、换协程执行、成功时打印返回值。
///
/// - `pretty_print_fallback`：非空返回值经 `_PRETTYPRINT` 打印；REPL 形态
///   （repl-cli / ulua）在 `_PRETTYPRINT` 为 nil 时回退标准 `print`
///   （cpp Repl.cpp 同款，传 `true`）。Web 形态直接用 `print`、不经
///   `_PRETTYPRINT`（cpp Web.cpp 同款，传 `false`）。
/// - 成功返回 `Ok(())`；失败返回 `Err(错误文本)`——加载失败是原始错误消息；
///   运行失败是「yield 文案或错误消息 + `"\nstack backtrace:\n"` + debugtrace」
///   （web 的 `short_src:line` 前缀由其调用点自行拼接，不在此处）。
/// - chunkname 固定 `"=stdin"`（三方原实现一致）。
///
/// # Safety
///
/// `l` 必须是有效、活跃且在本调用返回前不关闭的 `LuaState`（其全局表即闭包
/// env），且调用前栈已平衡——成功与失败两条出口都会把本函数压入的栈槽全部
/// 弹平（加载失败/线程分配失败弹错误消息或 chunk，正常出口弹 `lua_newthread`
/// 留下的线程槽）。
pub unsafe fn run_loaded_chunk(
  l: *mut LuaState,
  bytecode: &[u8],
  pretty_print_fallback: bool,
) -> Result<(), String> {
  // Safety: 本函数 `# Safety` 契约保证 `l` 是活跃状态机且调用前栈平衡；块内
  // 索引全部是相对栈顶的 -1/-2/-3，逐一由本块自己压入的元素支撑（load 留
  // 1 个闭包或错误消息、newthread 留 1 个线程、pushvalue/remove/xmove 与 cpp
  // `runCode` 同款平衡），每条出口的压弹都配平（见各块注释）。

  // Safety: bytecode 是本帧借用，luau_load 仅在本次调用窗口内读取；成功压入
  // 1 个闭包，失败压入错误消息（cpp `Repl.cpp:243` 的 chunkname 即 "=stdin"，
  // 错误回溯首列用户可见）。
  if unsafe { luau_load(l, "=stdin", bytecode, 0) } != 0 {
    // 读错误串与弹栈是同一个收尾动作（弹栈后串即失效），故并入同一处收口窗口。
    // Safety: l 存活；`lua_tolstring_ref` 在串上返回全字节切片，map 内立刻复制成
    // owned String，借用严格结束于 lua_pop 之前。
    // Faithful port of `std::string error(msg, len)`: 按长度取全部字节；`None`
    // （非字符串错误对象，旧 null 指针）与空切片同义，译成空串，无别名/悬挂。
    let error = unsafe {
      let text = lua_tolstring_ref(l, -1);
      let error = text.map_or_else(String::new, |s| String::from_utf8_lossy(s).into_owned());
      // Safety: 与 luau_load 压入的错误槽配平。
      lua_pop(l, 1);
      error
    };

    return Err(error);
  }

  // Safety: l 存活；分配失败返回 null 且不压槽，成功时线程由 l 栈槽持有
  //（Ok/Err 两条出口各一次 `lua_pop(l, 1)` 配平）。cpp 对分配失败不设防
  //（解引用 null）；此处判空弹平 chunk 后回报 VM 标准内存错误文本。
  let t = unsafe { lua_newthread(l) };
  if t.is_null() {
    // Safety: 弹出 luau_load 留下的 chunk（线程分配失败，不再使用它）。
    unsafe { lua_pop(l, 1) };
    return Err(LUA_MEMERRMSG_STR.to_owned());
  }

  // 闭包换入新协程：栈顶 -1 是本块 newthread 压入的线程、-2 是 luau_load 压入
  // 的闭包。
  // Safety: pushvalue/remove 把 -2 处闭包挪到线程之上，xmove 仅在同 VM 两线程
  // 间移动单值，三步是 cpp `runCode` 同款平衡序列。
  // lua_pushvalue(l, -2); lua_remove(l, -3); lua_xmove(l, t, 1);
  unsafe {
    lua_pushvalue(l, -2);
    lua_remove(l, -3);
    lua_xmove(l, t, 1);
  }

  // int status = lua_resume(t, NULL, 0);
  // 保留空指针：C 接口 lua_resume(L, from, nargs) 契约规定主线程首次
  // resume 时 from 传 NULL（无父调用方）
  // Safety: `t` 为上方新建且栈上已备好待执行函数；from=null 合法（见上注释）。
  let status = unsafe { lua_resume(t, null_mut(), 0) };

  if status == LuaStatus::Ok as i32 {
    // Safety: resume 正常返回后 t 仍由 l 栈槽持有；-n 为模块返回值。
    let n = unsafe { lua_gettop(t) };

    if n != 0 {
      // Safety: 上游 Repl.cpp:267 是 `luaL_checkstack(T, LUA_MINSTACK, "too many
      // results to print")`，预留失败即抛 Lua 错误（文本 `stack overflow (too
      // many results to print)`）；底层 lua_checkstack 只回报 0，不检查就继续
      // getglobal/insert/pcall 会写越 stack_last 的未分配栈槽（真 OOB 写）。
      // lua_l_checkstack 已对齐上游语义，其后的返回值在此处会被丢弃。
      // getglobal/insert/pcall 走「取 _PRETTYPRINT（缺则按参数回退 print）→
      // 插到参数前 → pcall(t, n, 0, 0)」的 cpp 同款打印路径。
      unsafe {
        lua_l_checkstack(t, LUA_MINSTACK, "too many results to print");
        // If _PRETTYPRINT is nil, then use the standard print function instead
        if pretty_print_fallback {
          lua_getglobal(t, PRETTYPRINT_NAME.as_ptr().cast());
          if lua_isnil!(t, -1) {
            lua_pop(t, 1);
            lua_getglobal(t, PRINT_NAME.as_ptr().cast());
          }
        } else {
          lua_getglobal(t, PRINT_NAME.as_ptr().cast());
        }
        lua_insert(t, 1);
        lua_pcall(t, n, 0, 0);
      }
    }

    // Safety: 弹走 lua_newthread 在 l 上留下的线程槽。
    unsafe { lua_pop(l, 1) };
    Ok(())
  } else {
    // 组装错误文本与弹走线程槽同属一个收尾动作，并入同一处收口窗口。
    // Safety: resume 出错后 t 存活、-1 为错误对象（error_with_trace 契约），其返回
    // 值是 owned String，故随后的 lua_pop 不再有任何借用依赖。
    let error = unsafe {
      let error = error_with_trace(t, status, "\nstack backtrace:\n");
      // Safety: 弹走 lua_newthread 在 l 上留下的线程槽（连带错误对象）。
      lua_pop(l, 1);
      error
    };

    Err(error)
  }
}
