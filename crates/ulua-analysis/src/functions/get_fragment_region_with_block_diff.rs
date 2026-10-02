use ulua_ast::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{
    ast_stat::AstStat, ast_stat_block::AstStatBlock, location::Location, position::Position,
  },
};

use crate::{
  functions::{block_diff_start::block_diff_start, get_fragment_location::get_fragment_location},
  records::{
    arena_handle::Handle, fragment_region::FragmentRegion,
    nearest_likely_block_finder::NearestLikelyBlockFinder,
    nearest_statement_finder::NearestStatementFinder,
  },
};

/// §2：入口已句柄化。`fresh` 为最近一次完整解析的树根（cpp 非空契约，
/// 由 `find_ancestry_for_fragment_parse` 判空早退后传入）；`stale` 为双树
/// 轮转中的旧 root——cpp `stale->visit(&lsf)` 在 `lastGoodParse` 非空时无条件
/// 解引用（nullptr 属 UB），此处保留 `Option` 形态直至该解引用点收口为
/// 确定性 panic，缺席分支不被吞。
pub(crate) fn get_fragment_region_with_block_diff(
  stale: Option<Handle<AstStatBlock>>,
  fresh: Handle<AstStatBlock>,
  cursor_pos: &Position,
) -> FragmentRegion {
  let mut nsf = NearestStatementFinder::new(*cursor_pos);
  ast_stat_block_visit(fresh.get_mut(), &mut nsf);

  let parent = nsf.parent.unwrap_or(fresh);
  // cpp `nearestStatement = nsf.nearest ? nsf.nearest : fresh`：根块 `AstStat`
  // 视图经 `repr(C)` 首字段基址重铸（原 `fresh.cast::<AstStat>()` 同形）。
  let nearest = nsf.nearest.or_else(|| Some(fresh.cast::<AstStat>()));

  let mut lsf = NearestLikelyBlockFinder::new(parent);
  // C++ `stale->visit(&lsf)` — traverse the entire stale AST so the visitor
  // sees every nested block (e.g. the inner `do` block), not just the root.
  let stale_root = stale.expect("stale 根在 lastGoodParse 非空时应在场（cpp 直解引用 UB 收口）");
  ast_stat_block_visit(stale_root.get_mut(), &mut lsf);

  if let Some(same_block) = lsf.found
    // same_block 是 lsf 在整棵 stale 树上记回的存活 AstStatBlock 节点；parent 为
    // fresh 块根或 nsf.parent；nearest 为其 AstStat 视图（repr(C) 首字段基址重合）。
    && let Some(fd) = block_diff_start(same_block, parent, nearest)
  {
    return FragmentRegion {
      fragment_location: Location::new(fd, *cursor_pos),
      nearest_statement: nearest,
      parent_block: Some(parent),
    };
  }

  FragmentRegion {
    fragment_location: get_fragment_location(nsf.nearest.map(|h| h.get()), cursor_pos),
    nearest_statement: nearest,
    parent_block: Some(parent),
  }
}
