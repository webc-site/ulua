use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_table::{Item, ItemKind},
  },
  rtti::ast_node_is_ptr,
};
pub fn is_record(item: &Item) -> bool {
  if item.kind == ItemKind::Record {
    true
  } else if item.kind == ItemKind::General {
    if item.key.is_null() {
      return false;
    }

    // `ast_node_is_ptr` 边界门面先判空、null 恒为 false，非空才读 repr(C) 基类
    // 偏移 0 的 class_index，判别与原「重建引用再判」形态逐字等价；item.key 已在
    // 上方 is_null() 守卫排除空指针，且它指向 arena 内存活节点（bump 分配、块地址
    // 不移动，item 借用期内存活），只读比较 class index。
    if unsafe { ast_node_is_ptr::<AstExprConstantString>(item.key) } {
      return true;
    }

    false
  } else {
    false
  }
}
