//! [`l_alloc`] 语义表逐分支直测（r7-test-src 依 §8 从 src 迁入：被测对象是
//! 边界契约测试：null 系 alloc 回调合法返回/入参（既有约定 review.md §2），见 §9.3 保留理由。
//! 公开面 `ulua_vm::functions::l_alloc::l_alloc`（pub 的 C 分配器契约函数），
//! 测试只用标准库指针/切片门面，零私有项依赖，属可外迁面）。
//!
//! 覆盖分支：
//! - `ptr == null`、`nsize > 0` → 纯分配，返回可用块；
//! - `ptr == null`、`nsize == 0` → `block_layout(0) == None`，无事可做回 null；
//! - `ptr != null`、`nsize == 0` → 释放并回 null；
//! - `ptr != null`、`nsize != 0` → 缩/扩块，C realloc 语义下前 `min(old,new)`
//!   字节内容保持；
//! - 契约违例防御：非 null `ptr` 配 `osize == 0`（无从复原 Layout）→ 回 null
//!   且**绝不触碰**原块（既不 dealloc 也不 realloc），原块存活可读。
//!
//! `Layout` 对齐/大小的正确性由 Miri 覆盖，本处仅钉返回值、内容与上述
//! 二次释放防御。真实分配器 OOM→null 且原块存活的路径无法在无 failpoint
//! 前提下确定性地触发，按评审授权**不**在此强行构造（见文末说明）。

use core::{
  ffi::c_void,
  ptr::null_mut,
  slice::{from_raw_parts, from_raw_parts_mut},
};

use ulua_vm::functions::l_alloc::l_alloc;

/// 图案字节：位置 `i` 的可辨识值（写读两侧的同一函数，杜绝两份算式漂移）。
const fn pattern_byte(i: usize) -> u8 {
  (i.wrapping_mul(7) ^ 0xA5) as u8
}

/// 写满 `n` 字节的可辨识图案（[`pattern_byte`]）。
///
/// # Safety
/// 调用方保证 `ptr..ptr+n` 为已分配可写块。
unsafe fn fill_pattern(ptr: *mut u8, n: usize) {
  // Safety: 契约保证 `ptr..ptr+n` 为已分配可写块；切片门面取代逐字节裸偏移写。
  let block = unsafe { from_raw_parts_mut(ptr, n) };
  for (i, byte) in block.iter_mut().enumerate() {
    *byte = pattern_byte(i);
  }
}

/// 核对 `ptr` 前 `n` 字节仍是 [`fill_pattern`] 写下的图案。
///
/// # Safety
/// 调用方保证 `ptr..ptr+n` 可读。
unsafe fn pattern_matches(ptr: *const u8, n: usize) -> bool {
  // Safety: 契约保证 `ptr..ptr+n` 可读；切片门面取代逐字节裸偏移读。
  unsafe { from_raw_parts(ptr, n) }
    .iter()
    .enumerate()
    .all(|(i, &b)| b == pattern_byte(i))
}

/// 分支 1：`ptr == null`、`nsize > 0` → 返回非空块，可整块写入与读回。
#[test]
fn null_ptr_allocates_usable_block() {
  let n = 16;
  // Safety: 契约分支「ptr 为 null 且 osize 为 0」= 纯分配；块由本测试收尾释放。
  let ptr = unsafe { l_alloc(null_mut::<c_void>(), null_mut(), 0, n) };
  assert!(!ptr.is_null(), "null+osize0+nsize>0 必走纯分配返回非空块");
  // Safety: 上一断言证明 `ptr..ptr+n` 为本分配器交回的可写块。
  unsafe {
    fill_pattern(ptr, n);
    assert!(pattern_matches(ptr, n), "分配块整段可写读且内容保真");
    // Safety: 块仍存活且从未释放，`n` 即分配时大小（复原 Layout 的前提）。
    assert!(l_alloc(null_mut::<c_void>(), ptr, n, 0).is_null());
  }
}

/// 分支 1 退化：`ptr == null`、`nsize == 0` → `block_layout(0)` 回 `None`，
/// 契约上无事可做，直接回 null（不分配）。
#[test]
fn null_ptr_zero_size_returns_null() {
  // Safety: 契约分支「ptr 为 null 且 osize 为 0」，nsize 0 无事可做。
  let ptr = unsafe { l_alloc(null_mut::<c_void>(), null_mut(), 0, 0) };
  assert!(ptr.is_null(), "null+nsize0 不得分配，应回 null");
}

/// 分支 2：`ptr != null`、`nsize == 0` → 释放并回 null。分配后立即释放，
/// 返回值必为 null；后续对同尺寸再分配证明释放路径未污染分配器状态。
#[test]
fn non_null_zero_size_frees_and_returns_null() {
  let n = 32;
  // Safety: 契约分支「ptr 为 null 且 osize 为 0」= 纯分配。
  let ptr = unsafe { l_alloc(null_mut::<c_void>(), null_mut(), 0, n) };
  assert!(!ptr.is_null());
  // Safety: `ptr` 为本分配器刚以大小 `n` 交回且未释放的块（上一行实证非空）。
  let ret = unsafe { l_alloc(null_mut::<c_void>(), ptr, n, 0) };
  assert!(ret.is_null(), "nsize==0 释放路径恒回 null");
  // 释放后再分配一块，分配器仍可用（间接确认 dealloc 被正确调用、参数匹配）。
  // Safety: 同上契约分支，纯分配。
  let again = unsafe { l_alloc(null_mut::<c_void>(), null_mut(), 0, n) };
  assert!(!again.is_null());
  // Safety: `again` 为上一行以大小 `n` 纯分配交回、未释放的块。
  unsafe {
    assert!(l_alloc(null_mut::<c_void>(), again, n, 0).is_null());
  }
}

/// 分支 3 扩容：16 → 32，前 16 字节内容保持（C realloc 语义），新增区可写读；
/// 用新块尺寸收尾释放。
#[test]
fn realloc_grow_preserves_content() {
  let small = 16;
  let large = 32;
  // Safety: 契约分支「ptr 为 null 且 osize 为 0」= 纯分配。
  let old = unsafe { l_alloc(null_mut::<c_void>(), null_mut(), 0, small) };
  assert!(!old.is_null());
  // Safety: `old` 为本分配器以大小 `small` 交回、未释放的块（realloc/dealloc
  // 的 `osize` 参数即分配时大小，契约同源）。
  unsafe {
    fill_pattern(old, small);
    let grown = l_alloc(null_mut::<c_void>(), old, small, large);
    assert!(!grown.is_null(), "扩容成功应回非空块");
    assert!(
      pattern_matches(grown, small),
      "realloc 后前 osize 字节必须原样保留（C realloc 语义）"
    );
    // 新增区（`grown[small..large]`）可写读，写常量图案后逐字节核对。
    let tail = from_raw_parts_mut(grown.add(small), large - small);
    for b in tail.iter_mut() {
      *b = 0x5A;
    }
    assert!(
      from_raw_parts(grown.add(small), large - small)
        .iter()
        .all(|&b| b == 0x5A),
      "新增区写读往返应保真"
    );
    // 写新区不得回头污染保留前缀。
    assert!(pattern_matches(grown, small), "写新区后保留前缀仍完好");
    assert!(l_alloc(null_mut::<c_void>(), grown, large, 0).is_null());
  }
}

/// 分支 3 缩容：32 → 8，前 8 字节内容保持；用新块尺寸收尾释放。
#[test]
fn realloc_shrink_preserves_content() {
  let large = 32;
  let small = 8;
  // Safety: 契约分支「ptr 为 null 且 osize 为 0」= 纯分配。
  let old = unsafe { l_alloc(null_mut::<c_void>(), null_mut(), 0, large) };
  assert!(!old.is_null());
  // Safety: `old` 为本分配器以大小 `large` 交回、未释放的块。
  unsafe {
    fill_pattern(old, large);
    let shrunk = l_alloc(null_mut::<c_void>(), old, large, small);
    assert!(!shrunk.is_null(), "缩容成功应回非空块");
    assert!(
      pattern_matches(shrunk, small),
      "缩容后保留区（前 nsize 字节）内容必须原样保留"
    );
    assert!(l_alloc(null_mut::<c_void>(), shrunk, small, 0).is_null());
  }
}

/// 契约违例防御（`osize == 0` 且 `ptr != null`）：无从复原 Layout，两条出口
/// （`nsize == 0` 释放臂、`nsize > 0` 缩扩臂）都必须**只回 null 且绝不触碰
/// 原块**——原块地址与内容保持存活可读，随后以正确 `osize` 释放仍成功。
/// 这正是「二次释放/错误尺寸释放」防御的 observable 面：错误的 `osize` 不会
/// 提前把本块交回分配器。
#[test]
fn zero_osize_with_non_null_ptr_returns_null_and_leaves_block_alive() {
  let n = 24;
  // Safety: 契约分支「ptr 为 null 且 osize 为 0」= 纯分配。
  let ptr = unsafe { l_alloc(null_mut::<c_void>(), null_mut(), 0, n) };
  assert!(!ptr.is_null());
  // Safety: `ptr` 为本分配器以大小 `n` 交回、未释放的块；下述两次错误
  // `osize` 调用按被测契约不得释放/移动它，块全程存活可读。
  unsafe {
    fill_pattern(ptr, n);

    // 释放臂（nsize==0）配错误 osize==0：回 null、不 dealloc。
    let free_ret = l_alloc(null_mut::<c_void>(), ptr, 0, 0);
    assert!(free_ret.is_null(), "osize==0 释放臂应回 null");
    assert!(
      pattern_matches(ptr, n),
      "osize==0 释放臂不得触碰原块：内容须仍可读且完整"
    );

    // 缩扩臂（nsize>0）配错误 osize==0：`block_layout(0)==None` → 回 null、不 realloc。
    let grow_ret = l_alloc(null_mut::<c_void>(), ptr, 0, 64);
    assert!(grow_ret.is_null(), "osize==0 缩扩臂应回 null");
    assert!(
      pattern_matches(ptr, 1),
      "osize==0 缩扩臂同样绝不得移动/释放原块，首字节仍为图案起点"
    );

    // 本块仍存活：以正确 osize 收尾释放，成功回 null（证明此前确未 double-free）。
    assert!(
      l_alloc(null_mut::<c_void>(), ptr, n, 0).is_null(),
      "先前未真正释放，则正确尺寸的释放应照常成功"
    );
  }
}

// 关于「分配失败 → null 且原块存活」：`System` 直通平台 malloc/realloc，其 OOM
// 分支无法在无全局分配器 failpoint 的前提下被确定性地触发（强行申请超大块在不同
// 平台要么成功、要么进程被 OOM killer 终止，均非可控断言），故本文件按评审授权
// 不覆盖该路径；其 Layout 正确性与 no-UB 语义由 Miri 在 `System` 契约下覆盖。
