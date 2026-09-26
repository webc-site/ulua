//! GC 字符串 intern 表（§2 arena/自引用图的句柄模型首批）。
//!
//! 桶数组本体仍由 `frealloc` arena 分配（`global_State` 的裸指针字段契约见其文档，
//! 整体 `GcRef(u32)` arena 化为后续阶段），但桶号与桶链的全部算术收拢到本模块：
//! - [`BucketIdx`]：cpp `lmod(h, tb->size)` 裸下标落成 newtype 句柄，只能由
//!   `bucket_of` 构造；
//! - `buckets()/buckets_mut()`：带表借用生命周期的切片视图，取代散落的
//!   `tb.hash.add(i)` 裸指针算术与手写 null 哨兵守卫；
//! - `interned/link_front/unlink`：cpp `luaS_newlstr`/`luaS_buffinish` 扫描段、
//!   发布尾与 `unlinkstr` 的链算术，节点仍是页分配器拥有的侵入式
//!   `TString.next` 链，`&mut tstring` 借用只在方法内从裸指针受控导出；
//! - `wants_growth/doubled_size/wants_shrink/half_size/shrink_target`：cpp
//!   `shrinkbuffers(full)` 与发布尾的负载判定，除零/极值魔法数提为 `const`。
//!
//! 表不变式（构造后恒成立，`lua_newstate` 建空表、`lua_s_resize` 唯一换阵）：
//! `hash` 非 null 时指向 `frealloc` 分配的 `size` 个可读写 `*mut tstring` 槽，
//! 且 `size > 0`；`size == 0` 与 `hash == null` 同现（未分配态）。链上每个节点
//! 都是存活 `TString`，其 `data` 区 `(*el).len` 字节可读。越界桶号句柄经 `get`
//! 落空表分支：cpp 同位是越界 UB（空表期 intern 在 f_luaopen 建表前不可达），
//! 本实现取「查不中/不入链、串成孤儿由 `unlink` 未命中路径回收」的正确方向。
//! ——DELIBERATE DEVIATION: cpp lstring.cpp `newlstr` 空表发布尾为 OOB 写，Rust
//! 版收敛为无操作，理由如上；行为差异在 cpp 不可观测（前置顺序保证不可达）。

use core::{
  ffi::c_uint,
  mem,
  ptr::null_mut,
  slice::{from_raw_parts, from_raw_parts_mut},
};

use crate::{
  macros::{isdead::isdead, lua_minstrtabsize::LUA_MINSTRTABSIZE, whitebits::WHITEBITS},
  records::{gc_object::GCObject, global_state::global_State, t_string::tstring},
};

/// cpp `shrinkbuffers` 的负载下限除数：`nuse < size/4` 才收缩（lgc.cpp）。
const SHRINK_LOAD_DIVISOR: i32 = 4;
/// cpp `size > LUA_MINSTRTABSIZE * 2`：收缩下限，半步后仍不小于最小表。
const SHRINK_MIN_FACTOR: i32 = 2;
/// 倍增/减半系数（cpp `size * 2` / `size / 2` / `INT_MAX / 2` 的 2）。
const HALF_FACTOR: i32 = 2;
/// cpp 扩容上限 `size <= INT_MAX / 2`：防翻倍溢出。
const GROWTH_SIZE_CEILING: i32 = i32::MAX / HALF_FACTOR;

/// 桶号句柄（§2 新type Idx）：`bucket_of` 由哈希与表长导出，界内性由
/// `h & (size-1) ≤ size-1` 恒成立论证；`size == 0` 时为越界值，消费方一律
/// `get` 落空表分支。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BucketIdx(pub(crate) usize);

/// 串表（cpp `stringtable`）：桶数组 + 已用计数。字段仍 `pub(crate)` 供
/// `lua_newstate` 建空表与 `lua_s_resize` 换阵直写，其余模块一律走方法句柄。
#[derive(Debug)]
#[repr(C)]
pub struct Stringtable {
  pub(crate) hash: *mut *mut tstring,
  pub(crate) nuse: u32,
  pub(crate) size: i32,
}

impl Default for Stringtable {
  fn default() -> Self {
    Self {
      hash: null_mut(),
      nuse: 0,
      size: 0,
    }
  }
}

impl Stringtable {
  /// cpp `lmod(h, size)`：哈希 → 桶号句柄。
  #[inline]
  pub(crate) const fn bucket_of(h: c_uint, size: i32) -> BucketIdx {
    BucketIdx((h as usize) & (size as usize).wrapping_sub(1))
  }

  /// 桶数组只读切片视图；未分配空表给空视图（null 守卫收拢于单点）。
  #[inline]
  pub(crate) fn buckets(&self) -> &[*mut tstring] {
    if self.hash.is_null() {
      &[]
    } else {
      // Safety: 表不变式——hash 非 null 时指向 size 个可读 *mut tstring 槽
      unsafe { from_raw_parts(self.hash, self.size as usize) }
    }
  }

  /// 桶数组可变切片视图；未分配空表给空视图。
  #[inline]
  pub(crate) fn buckets_mut(&mut self) -> &mut [*mut tstring] {
    if self.hash.is_null() {
      &mut []
    } else {
      // Safety: 表不变式——hash 非 null 时指向 size 个可写 *mut tstring 槽
      unsafe { from_raw_parts_mut(self.hash, self.size as usize) }
    }
  }

  /// 沿 `h` 桶链查找与 `needle` 等长等字节的驻留串；命中且已死则翻转
  /// WHITEBITS 复活（逐位同 cpp `luaS_newlstr`/`luaS_buffinish` 扫描段）。
  /// 返回借自表链的 `&mut tstring`，借用止于下次表变更。
  pub(crate) fn interned(
    &mut self,
    g: *mut global_State,
    h: c_uint,
    needle: &[u8],
  ) -> Option<&mut tstring> {
    let b = Self::bucket_of(h, self.size);
    let head = *self.buckets_mut().get(b.0)?;
    let mut el = head;
    loop {
      if el.is_null() {
        return None;
      }
      // Safety: 表不变式——链上节点皆存活 TString，data 区 (*el).len 字节
      // 可读（空串零长度切片合法）
      let s = unsafe { &mut *el };
      let bytes = unsafe { from_raw_parts(s.data.as_ptr() as *const u8, needle.len()) };
      if s.len as usize == needle.len() && bytes == needle {
        // Safety: 表不变式——`el` 为链上存活 GCObject，isdead 仅读头 marked 位
        if unsafe { isdead!(g, el as *mut GCObject) } {
          s.hdr.marked ^= WHITEBITS as u8;
        }
        return Some(s);
      }
      el = s.next;
    }
  }

  /// 发布尾前段：`ts` 头插 `h` 桶并 `nuse++`（cpp 链插顺序：先取旧链头、
  /// 再写新链头、再计数）。空表态为无操作（见模块头 DELIBERATE DEVIATION）。
  pub(crate) fn link_front(&mut self, h: c_uint, ts: *mut tstring) {
    let b = Self::bucket_of(h, self.size);
    let Some(slot) = self.buckets_mut().get_mut(b.0) else {
      return;
    };
    // Safety: 表不变式 + 调用方契约——`ts` 为存活 TString，`slot` 界内
    unsafe {
      let s = &mut *ts;
      s.next = *slot;
      *slot = ts;
    }
    self.nuse = self.nuse.wrapping_add(1);
  }

  /// 把 `ts` 从其哈希桶链摘下（cpp `unlinkstr`）；true = 已除链，
  /// false = 不在表（孤儿缓冲，`(*ts).next` 应为 null 由调用方断言）。
  /// 不触碰 `nuse`（cpp 同构：递减归 `luaS_free` 的 removed 分支）。
  pub(crate) fn unlink(&mut self, ts: *mut tstring) -> bool {
    // Safety: 调用方契约——`ts` 为存活 TString（lua_s_free 保证 hash 可读）
    let h = unsafe { (*ts).hash };
    let b = Self::bucket_of(h, self.size);
    let Some(slot) = self.buckets_mut().get_mut(b.0) else {
      return false;
    };
    // Safety: 表不变式——桶槽与链节点皆存活；`ts` 存活由调用方保证
    unsafe {
      if *slot == ts {
        *slot = (*ts).next;
        return true;
      }
      let mut curr = *slot;
      while !curr.is_null() {
        let next = (*curr).next;
        if next == ts {
          (*curr).next = (*ts).next;
          return true;
        }
        curr = next;
      }
      false
    }
  }

  /// cpp 扩容判定 `nuse > size && size <= INT_MAX/2`（发布尾后查）。
  #[inline]
  pub(crate) const fn wants_growth(&self) -> bool {
    self.nuse > self.size as u32 && self.size <= GROWTH_SIZE_CEILING
  }

  /// cpp `luaS_resize(L, tb->size * 2)` 的目标桶数。
  #[inline]
  pub(crate) const fn doubled_size(&self) -> i32 {
    self.size * HALF_FACTOR
  }

  /// cpp 收缩判据 `nuse < size/4 && size > LUA_MINSTRTABSIZE * 2`。
  #[inline]
  const fn shrink_worthy(nuse: u32, size: i32) -> bool {
    nuse < (size / SHRINK_LOAD_DIVISOR) as u32 && size > LUA_MINSTRTABSIZE * SHRINK_MIN_FACTOR
  }

  /// cpp `shrinkbuffers` 的单步收缩判定。
  #[inline]
  pub(crate) const fn wants_shrink(&self) -> bool {
    Self::shrink_worthy(self.nuse, self.size)
  }

  /// 单步收缩目标（cpp `tb->size / 2`）。
  #[inline]
  pub(crate) const fn half_size(&self) -> i32 {
    self.size / HALF_FACTOR
  }

  /// cpp `shrinkbuffersfull` 的折半循环目标；等于现值即无需收缩。
  #[inline]
  pub(crate) const fn shrink_target(&self) -> i32 {
    let mut size = self.size;
    while Self::shrink_worthy(self.nuse, size) {
      size /= HALF_FACTOR;
    }
    size
  }

  /// 收表转手（§2 所有权转手显式动作）：摘出桶数组指针与槽数并置空表
  /// 初态，数组的 `frealloc` 释放责任移交调用方（close_state）。
  pub(crate) fn detach_buckets(&mut self) -> (*mut *mut tstring, usize) {
    let hash = mem::replace(&mut self.hash, null_mut());
    let count = mem::replace(&mut self.size, 0) as usize;
    self.nuse = 0;
    (hash, count)
  }
}
