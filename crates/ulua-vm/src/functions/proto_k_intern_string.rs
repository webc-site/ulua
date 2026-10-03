//! JIT call inlining（NAMECALL 阶段）：caller 常量表的字符串 intern。
//!
//! 观测路径内联发射时，callee 体的 `GETTABLEKS`/`LOADK` 字符串常量（挂在 callee
//! proto 的 `k` 表上）必须重定位进 caller 常量表——生成码的 `VmConst` 操作数经
//! 入口序言装载的 `R_CONSTANTS`（= 当前 proto 的 `k` 基址）寻址，内联体在 caller
//! 帧执行，callee 下标直读会命中 caller `k` 表的错误槽位。
//!
//! 追加策略：「新块拷贝 + 原子换指针」，旧块退役挂 `proto->k_retired` 链（见
//! `RetiredKArray`）随 proto 死亡统一释放——在途 native 帧的 `R_CONSTANTS` 指向
//! 旧块基址，立即 free 会被页池复用覆盖（活跃帧悬垂）；泄漏不释放则页无法归还、
//! `lua_close` 的记账校验炸。退役链两头都顾：指针稳定窗口覆盖本帧退出，释放点
//! （proto 死亡）绝无在途帧。
//!
//! - 追加项的 `TString` 同时被 callee `k` 表引用（守卫通过 ⟹ callee proto 存活，
//!   funid 全局单调不复用），且新块由 GC `traverseproto` 按新 `sizek` 全量扫描，
//!   字符串永远可达。
//! - 分配（`luaM_newarray`）可能触发 GC：分配点在换指针之前，此刻 `proto->k`/
//!   `sizek` 仍是旧一致态，扫描安全；换指针 + `sizek++` 无分配点，对 GC 原子。

use core::ptr::copy_nonoverlapping;

use crate::{
  macros::{lua_m_newarray::luaM_newarray, setsvalue::setsvalue},
  records::{lua_state::LuaState, proto::Proto, retired_k_array::RetiredKArray, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// 向 `proto` 常量表 intern 字符串 `ts`：命中既有同指针项即复用其下标（编译器
/// 对 intern 短字符串保证同指针，无须逐字节比对），未命中则追加新槽。返回常量
/// 下标（`< sizek`，可作 `VmConst` 操作数）。
///
/// # Safety
/// `l` 为存活 `LuaState`；`proto` 为存活 Proto 且在本调用期间无并发写者（编译
/// 会话单线程契约）；`ts` 须为 GC 存活的 TString（调用点由 callee proto `k` 表
/// 锚定）。分配可触发 GC 重入——见模块注的窗口一致性论证。
pub unsafe fn proto_k_intern_string(
  l: *mut LuaState,
  proto: *mut Proto,
  ts: *mut tstring,
) -> Option<u32> {
  unsafe {
    let sizek = (*proto).sizek;
    if sizek < 0 {
      return None;
    }
    let k = (*proto).k;

    // 复用扫描：同指针 intern 字符串直接给既有下标（暖重编译重复追加的幂等闸）
    for i in 0..sizek as usize {
      let kv = k.add(i);
      if (*kv).is_string() && (*kv).value.gc as *mut tstring == ts {
        return Some(i as u32);
      }
    }

    // 追加：新块拷贝旧内容（分配点在前，此刻 k/sizek 保持旧一致态供 GC 扫描）。
    // 旧块退役不释放——在途 native 帧的 R_CONSTANTS 仍指着它（指针稳定窗口
    // 覆盖到本帧退出，立即 free 会被页池复用覆盖），挂链随 proto 死亡统一回收。
    let new_k = luaM_newarray!(l, sizek as usize + 1, TValue, (*proto).hdr.memcat);
    if new_k.is_null() {
      return None;
    }
    copy_nonoverlapping(k, new_k, sizek as usize);
    setsvalue!(l, new_k.add(sizek as usize), ts);

    let node = luaM_newarray!(l, 1, RetiredKArray, (*proto).hdr.memcat);
    if node.is_null() {
      // 旧块退役登记失败即整体放弃（不换指针，语义保持旧表一致态）
      return None;
    }
    (*node).next = (*proto).k_retired;
    (*node).ptr = k;
    (*node).size = sizek as usize;
    (*node).memcat = (*proto).hdr.memcat;
    (*proto).k_retired = node;

    (*proto).k = new_k;
    (*proto).sizek = sizek + 1;
    Some(sizek as u32)
  }
}
