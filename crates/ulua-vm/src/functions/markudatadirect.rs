use core::ptr::addr_of_mut;

use ulua_common::fflag;

use crate::{
  macros::{markobject::markobject, markvalue::markvalue},
  records::global_state::global_State,
};

/// 补标 udatadirect 直接访问元方法缓存与直接字段派发表；markroot 与 atomic 两阶段共用，
/// 两处扫描集合必须保持一致（cpp 中同为 lgc.cpp:934/1018-1022 的同一段逻辑）。
///
/// DELIBERATE DEVIATION：本地 cpp（lgc.cpp:934/1031）无条件扫 udatadirect；本仓按更新上游
/// 登记 `LuauUdataDirectAccess6`（fflag.rs，Luau* 前缀默认使能），旗关时跳过该扫描
/// （本地旧快照行为）。
///
/// # Safety
/// 调用方须保证：`g` 为存活 global_State 且处于 GC mark 相关阶段，`udatadirect`/`udatadirectfields`
/// 数组元素要么为 null、要么指向未被回收的存活对象；否则 mark 系宏对悬垂对象写灰白标签即 UB。
pub(crate) unsafe fn markudatadirect(g: *mut global_State) {
  // Safety: 契约保证 `g` 指向存活 global_State，仅按登记位扫描并写各对象灰白标签
  unsafe {
    // cpp lgc.cpp:1018-1019（markudatadirectaccess）：直接访问元方法缓存
    if fflag::LuauUdataDirectAccess6.get() {
      for udatadirect in (*g).udatadirect.iter_mut() {
        markvalue!(g, addr_of_mut!(udatadirect.indextm));
        markvalue!(g, addr_of_mut!(udatadirect.newindextm));
        markvalue!(g, addr_of_mut!(udatadirect.namecalltm));
      }
    }
    // cpp lgc.cpp:854-860 + 1021-1022（markudatadirectfields）：直接字段派发表
    if fflag::LuauDirectFieldGet.get() {
      for &field in (*g).udatadirectfields.iter() {
        if !field.is_null() {
          markobject!(g, field);
        }
      }
    }
  }
}
