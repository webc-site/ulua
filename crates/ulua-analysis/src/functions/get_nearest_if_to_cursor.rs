use ulua_ast::{
  records::{ast_stat::AstStat, ast_stat_if::AstStatIf, position::Position},
  rtti::{ast_node_is, ast_node_try_as},
};

pub(crate) fn get_nearest_if_to_cursor<'a>(
  stmt: Option<&'a AstStat>,
  cursor_pos: &Position,
) -> Option<&'a AstStatIf> {
  let mut current = stmt;

  while let Some(current_stat) = current {
    if let Some(current_if) = ast_node_try_as::<AstStatIf>(&current_stat.base) {
      if let Some(else_stat) = current_if.elsebody.get()
        && else_stat.base.location.contains_closed(*cursor_pos)
        && ast_node_is::<AstStatIf>(&else_stat.base)
      {
        current = Some(else_stat);
      } else {
        return Some(current_if);
      }
    } else {
      break;
    }
  }

  None
}
