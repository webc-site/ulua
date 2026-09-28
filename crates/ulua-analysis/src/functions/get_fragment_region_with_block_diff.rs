use ulua_ast::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{
    ast_stat::AstStat, ast_stat_block::AstStatBlock, location::Location, position::Position,
  },
};

use crate::{
  functions::{block_diff_start::block_diff_start, get_fragment_location::get_fragment_location},
  records::{
    fragment_region::FragmentRegion, nearest_likely_block_finder::NearestLikelyBlockFinder,
    nearest_statement_finder::NearestStatementFinder,
  },
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_fragment_region_with_block_diff(
  stale: *mut AstStatBlock,
  fresh: *mut AstStatBlock,
  cursor_pos: &Position,
) -> FragmentRegion {
  let mut nsf = NearestStatementFinder::new(*cursor_pos);
  // Safety: `fresh` 对应 C++ `NotNull<AstStatBlock*>`，由 fragment 差异驱动方保证
  // 非空且指向解析 arena 中存活的块根；`&mut` 仅为匹配 `ast_stat_block_visit` 签名
  // ——遍历只把子节点裸指针记入 `nsf`，不改写 AST 本体，本次调用内独占无并发借用。
  ast_stat_block_visit(unsafe { &mut *fresh }, &mut nsf);

  let parent = if !nsf.parent.is_null() {
    nsf.parent
  } else {
    fresh
  };

  let nearest = if !nsf.nearest.is_null() {
    nsf.nearest
  } else {
    fresh.cast::<AstStat>()
  };

  let mut lsf = NearestLikelyBlockFinder::new(parent);
  // C++ `stale->visit(&lsf)` — traverse the entire stale AST so the visitor
  // sees every nested block (e.g. the inner `do` block), not just the root.
  // Safety: `stale` 同 `fresh`——`NotNull<AstStatBlock*>`，非空且指向解析 arena
  // 中存活的块根；`&mut` 仅匹配 visit 签名，遍历只把候选块裸指针记入 `lsf`，
  // 不改动 AST，本调用内独占无并发借用。
  ast_stat_block_visit(unsafe { &mut *stale }, &mut lsf);

  if let Some(same_block) = lsf.found
    // same_block 是 lsf 在整棵 stale 树上记回的存活 AstStatBlock 节点；parent 为
    // fresh 块根或 nsf.parent；nearest 为其 AstStat 视图（repr(C) 首字段基址重合）。
    && let Some(fd) = block_diff_start(same_block, parent, nearest)
  {
    return FragmentRegion {
      fragment_location: Location::new(fd, *cursor_pos),
      nearest_statement: nearest,
      parent_block: parent,
    };
  }

  FragmentRegion {
    // Safety: `nsf.nearest` 是 `NearestStatementFinder` 在 fresh 遍历时记下的存活
    // AstStat 裸指针，或仍为初始 null——`get_fragment_location` 的契约允许 null 形参
    // 并在首行提前返回空区域，故此处传入（可空但恒非悬垂的）arena 指针合法。
    fragment_location: unsafe { get_fragment_location(nsf.nearest, cursor_pos) },
    nearest_statement: nearest,
    parent_block: parent,
  }
}
