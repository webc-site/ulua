use core::{ffi::c_char, ptr, slice::from_ref};

use crate::{
  functions::{c_slice, c_slice_mut, enumtopointer::enumtopointer},
  macros::{
    dummynode::{DUMMYNODE, LUA_H_DUMMYNODE},
    twoto::twoto,
  },
  records::{
    enum_context::EnumContext, gc_object::GCObject, lua_node::LuaNode, lua_t_value::TValue,
    lua_table::LuaTable,
  },
};

/// # Safety
/// `ctx` 须指向存活 `EnumContext` 且其 `node` 回调与 `context` 配套（可空则跳过）；`gco` 须为存活
/// GCObject 的可读句柄（要读 `gch.tt`/`gch.memcat` 头字段），`objname` 为有效 C 字符串。违反则悬垂回调调用/头字段越界读。
/// cpp lgcdebug.cpp:760。
pub(crate) unsafe fn enumnode(
  ctx: *mut EnumContext,
  gco: *const GCObject,
  size: usize,
  objname: *const c_char,
) {
  // SAFETY: 契约保证 `ctx` 存活且 node 回调与其 context 配套，`gco` 头字段可读，块内仅追加输出不写对象
  unsafe {
    let ctx_ref = &*ctx;
    if let Some(node_fn) = ctx_ref.node {
      node_fn(
        ctx_ref.context,
        enumtopointer(&*gco),
        (*gco).tt(),
        (*gco).memcat(),
        size,
        objname,
      );
    }
  }
}

/// LuaTable 切片窗访问器（E1 窄腰契约的复形，r12-E3 临时安置）。
///
/// 安置说明：E1 曾把这些访问器落在 `records/lua_table.rs`，后被上游零消费清理
/// （1823a49）整体删除；本票基线 9791b46 已无窗形可用，而 `records/*` 为禁触面。
/// Rust 允许同 crate 任意模块为本地类型补 `impl`，故按 E1 原契约在此复形，供表
/// 遍历/查询族（E3 名单文件）共享；扩面建议：待 E1 窗形回归 `records/lua_table.rs`
/// 时删除本安置并原样转调，签名与文档逐字对齐 E1。
impl LuaTable {
  /// 数组段窗：`array` 前 `sizearray` 个 `TValue` 槽的共享视图（cpp `t->array[0..sizearray]`）。
  ///
  /// 契约（与 E1 逐字对齐）：
  /// - 返回长度恒等于 `max(sizearray, 0)`；`array` 为 null（`luaH_new` 初值、
  ///   `setarrayvector(0)` 经 `frealloc` 归零）或长度 0 时为空窗，与 C 中
  ///   零长数组不可解引用的行为一致。
  /// - 视图存续期内该表不得经历 `setarrayvector`/`rehash`/`luaH_resizearray` 等
  ///   重排 `array` 指针的操作（cpp 同等约束：frealloc 后旧指针失效）；GC 为
  ///   非移搬式（page/block 稳定地址），借用期内 GC 步进不使视图失效。
  /// - 与裸字段算术逐位一致：`array_window()[i]` ⇔ `(*t).array.add(i)`，
  ///   唯 `i >= sizearray` 由 UB 降级为切片越界 panic。
  pub(crate) fn array_window(&self) -> &[TValue] {
    // SAFETY: `array` 由 `setarrayvector`/`rehash` 经 frealloc 分配恰 `sizearray` 个
    // `TValue` 槽（或 null 配 sizearray==0），GC/栈操作均不搬移该分配；`&self` 借用
    // 期内表存活，负 sizearray 属已损坏态，`max(0)` 归为空窗保持定义行为。
    unsafe { c_slice(self.array, self.sizearray.max(0) as usize) }
  }

  /// [`Self::array_window`] 的可写版本；独占借用 `&mut self` 即窗内槽的独占写权。
  pub(crate) fn array_window_mut(&mut self) -> &mut [TValue] {
    // SAFETY: 同 `array_window`，且 `&mut self` 保证视图存续期内无其它别名；
    // 写点合法性与写序合理性由调用方按各自 # Safety 契约维持。
    unsafe { c_slice_mut(self.array, self.sizearray.max(0) as usize) }
  }

  /// 哈希部分是否为编译期哨兵（cpp `t->node == dummynode`，即「空哈希表」判据）。
  ///
  /// 不变式（`luaH_new`/`setnodevector(0)` 建立，`rehash` 全程维持）：
  /// `is_hash_dummy() == true` ⇔ 哈希部分未分配实向量，此时 `lsizenode == 0`。
  pub(crate) fn is_hash_dummy(&self) -> bool {
    ptr::eq(self.node, DUMMYNODE)
  }

  /// 哈希段窗：`node` 前 `sizenode(t) = twoto(lsizenode)` 个 `LuaNode` 桶的共享视图。
  ///
  /// dummynode 折叠（与 `gnode!`/`sizenode!` 宏逐位一致的定形论证，同 E1）：
  /// - 哨兵表：`node == DUMMYNODE` 且不变式给出 `lsizenode == 0`，宏口径下桶区间
  ///   恰覆盖哨兵单格（`sizenode = twoto(0) = 1`），`gnode!(t, 0)` 读出键/值皆 nil
  ///   的哨兵。本访问器对哨兵分支恒定返回单格窗 [`LUA_H_DUMMYNODE`]，即便
  ///   `lsizenode` 被损坏为非零也不越过哨兵对象边界（宏口径在该损坏态下本就是
  ///   UB，此处收口为定义行为）。
  /// - 实向量表：窗长 `twoto(lsizenode)`，与 `gnode!(t, i)`（即 `node.add(i)`）对
  ///   `i ∈ 0..sizenode` 逐位等价；`lsizenode < 31` 由 `setnodevector` 的
  ///   `ERR_TABLE_OVERFLOW` 守卫保证，`twoto` 不溢出。
  /// - 消费方判空必须用桶内容（哨兵/已清键为 nil），不得以 `len() == 0` 判空——
  ///   哨兵表窗长为 1 且不可写，正是 C 侧「dummynode 表 size 计 1、遍历读单格
  ///   dummy」的语义（工作量估算另以 [`Self::is_hash_dummy`] 折 0）。
  pub(crate) fn node_window(&self) -> &[LuaNode] {
    if self.is_hash_dummy() {
      from_ref(&LUA_H_DUMMYNODE.0)
    } else {
      // SAFETY: 非哨兵分支 `node` 由 `setnodevector` 经 frealloc 分配恰
      // `twoto(lsizenode)` 个 `LuaNode`（ERR_TABLE_OVERFLOW 守卫 lsizenode<31），
      // GC 不搬移分配；`&self` 借用期内表存活。
      unsafe { c_slice(self.node, twoto(self.lsizenode) as usize) }
    }
  }

  /// [`Self::node_window`] 的可写版本。**哨兵表返回空窗**：哨兵是不可变 `static`，
  /// 向其出借 `&mut` 本身即别名违例；C 侧从不原地写哨兵——`luaH_newkey` 经
  /// `gnode!` 取到哨兵基址后以指针相等判据（`eq(mp, dummynode)`）转 `rehash` →
  /// `setnodevector` 换实向量，写路径始终只落在实向量上。
  /// 消费方若需对哨兵表写入，必须先过 [`Self::is_hash_dummy`] 判据换发实向量。
  pub(crate) fn node_window_mut(&mut self) -> &mut [LuaNode] {
    if self.is_hash_dummy() {
      &mut []
    } else {
      // SAFETY: 同 `node_window` 非哨兵分支（frealloc 实向量、GC 不搬移），
      // 且 `&mut self` 保证视图存续期内无其它别名。
      unsafe { c_slice_mut(self.node, twoto(self.lsizenode) as usize) }
    }
  }
}
